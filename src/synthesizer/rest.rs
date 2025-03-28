use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use bytes::Bytes;
use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use log::debug;
use reqwest::{Client, StatusCode};
use strum::AsRefStr;

use crate::{SsmlError, TextOptions, interpolate_ssml};

/// The synthesizer that uses the RESTful API.
pub struct RestSynthesizer {
    pub(super) client: Client,
    pub(super) endpoint: String,
}

impl RestSynthesizer {
    /// Synthesize the given SSML into audio([`Vec<u8>`]).
    pub async fn synthesize_ssml(&self, ssml: &str) -> Result<Vec<u8>, RestSynthesizerError> {
        Ok(self.synthesize_ssml_to_bytes(ssml).await?.to_vec())
    }

    /// Synthesize the given SSML into audio([`bytes::Bytes`]).
    pub async fn synthesize_ssml_to_bytes(
        &self,
        ssml: &str,
    ) -> Result<Bytes, RestSynthesizerError> {
        let res = self
            .client
            .post(&self.endpoint)
            .body(ssml.to_string())
            .send()
            .await
            .map_err(|e| RestSynthesizerError {
                kind: RestSynthesizerErrorKind::Connect,
                source: Some(e.into()),
            })?
            .error_for_status()
            .map_err(|e| {
                use RestSynthesizerErrorKind::*;
                let kind = match e.status() {
                    Some(code) => match code {
                        StatusCode::TOO_MANY_REQUESTS => TooManyRequests,
                        StatusCode::UNAUTHORIZED => Unauthorized,
                        StatusCode::BAD_REQUEST => InvalidRequest,
                        StatusCode::UNSUPPORTED_MEDIA_TYPE => UnsupportedMediaType,
                        _ => OtherHttp,
                    },
                    None => OtherHttp,
                };
                RestSynthesizerError {
                    kind,
                    source: Some(e.into()),
                }
            })?;
        let bytes = res.bytes().await.map_err(|e| RestSynthesizerError {
            kind: RestSynthesizerErrorKind::Connection,
            source: Some(e.into()),
        })?;
        Ok(bytes)
    }

    /// This is a convenience method that interpolates the SSML for you.
    pub async fn synthesize_text(
        &self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
    ) -> Result<Vec<u8>, RestSynthesizerError> {
        debug!("Synthesizing text: {}", text.as_ref());
        let ssml = interpolate_ssml(text, options)?;
        self.synthesize_ssml(&ssml).await
    }

    /// This is a convenience method that interpolates the SSML for you.
    pub async fn synthesize_text_to_bytes(
        &self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
    ) -> Result<Bytes, RestSynthesizerError> {
        debug!("Synthesizing text: {}", text.as_ref());
        let ssml = interpolate_ssml(text, options)?;
        self.synthesize_ssml_to_bytes(&ssml).await
    }
}

/// Errors that can occur while using the RESTful API.
#[derive(Debug)]
#[non_exhaustive]
pub struct RestSynthesizerError {
    pub kind: RestSynthesizerErrorKind,
    pub(crate) source: Option<anyhow::Error>,
}

impl Display for RestSynthesizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RestSynthesizerErrorKind::*;
        write!(f, "rest synthesizer error: ")?;
        match &self.kind {
            Connect => write!(f, "error while connecting to the server"),
            InvalidRequest => write!(
                f,
                "an invalid request is constructed or 400 status reported by the server"
            ),
            Unauthorized => write!(
                f,
                "you are unauthorized. Did you set up the auth key/token?"
            ),
            _ => write!(f, "{} error", self.kind.as_ref()),
        }
    }
}

impl Error for RestSynthesizerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(feature = "python")]
impl From<RestSynthesizerError> for pyo3::PyErr {
    fn from(value: RestSynthesizerError) -> Self {
        pyo3::exceptions::PyOSError::new_err(format!("{:?}", color_eyre::Report::from(value)))
    }
}

#[derive(Debug, PartialEq, Clone, AsRefStr)]
#[non_exhaustive]
#[strum(serialize_all = "title_case")]
pub enum RestSynthesizerErrorKind {
    /// Failed to connect to the endpoint.
    Connect,
    /// The request was invalid, either caught early by us or indicated by a BadRequest response from the server.
    InvalidRequest,
    /// You are unauthorized. Did you set up the correct auth key/token?
    Unauthorized,
    /// The server returned a 415 Unsupported Media Type response.
    UnsupportedMediaType,
    /// The server returned a 429 Too Many Requests response.
    TooManyRequests,
    /// Other HTTP errors.
    OtherHttp,
    /// Connection errors.
    Connection,
    /// Errors when interpolating SSML.
    Ssml,
}

macro_rules! impl_from_for_rest_synthesizer_error {
    ($error_type:ty, $error_kind:ident) => {
        impl From<$error_type> for RestSynthesizerError {
            fn from(e: $error_type) -> Self {
                Self {
                    kind: RestSynthesizerErrorKind::$error_kind,
                    source: Some(e.into()),
                }
            }
        }
    };
}

impl_from_for_rest_synthesizer_error!(InvalidHeaderValue, InvalidRequest);
impl_from_for_rest_synthesizer_error!(InvalidHeaderName, InvalidRequest);
impl_from_for_rest_synthesizer_error!(SsmlError, Ssml);

#[cfg(test)]
mod test {
    use crate::{
        AudioFormat, AuthOptionsBuilder, SynthesizerConfig, TextOptions,
        get_rest_endpoint_by_region,
        test::{Creds, creds},
    };
    use rstest::rstest;

    #[tokio::test]
    #[rstest]
    async fn test_invalid_key(creds: Creds) {
        let endpoint = get_rest_endpoint_by_region(&creds.region);
        let auth = AuthOptionsBuilder::new(endpoint).key("invalid_key").build();
        let config = SynthesizerConfig::new(auth, AudioFormat::Riff16Khz16BitMonoPcm);

        let syn = config.rest_synthesizer().unwrap();

        let r = syn.synthesize_text("Hello", &TextOptions::default()).await;

        r.expect_err("Connect using an invalid_key should fail");
    }

    #[tokio::test]
    #[rstest]
    async fn test_text(creds: Creds) {
        let endpoint = get_rest_endpoint_by_region(&creds.region);
        let auth = AuthOptionsBuilder::new(endpoint).key(&creds.key).build();
        let config = SynthesizerConfig::new(auth, AudioFormat::Riff16Khz16BitMonoPcm);

        let syn = config.rest_synthesizer().unwrap();

        let audio = syn
            .synthesize_text("Hello", &TextOptions::default())
            .await
            .expect("Synthesis via websocket should succeed with a valid key");
        assert!(!audio.is_empty(), "Audio data should not be empty");
    }

    #[tokio::test]
    #[rstest]
    #[case(
        true,
        "<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'><voice name='en-US-JennyNeural'>Hello, world!</voice></speak>"
    )]
    #[case(false, "")]
    async fn test_ssml(creds: Creds, #[case] valid: bool, #[case] ssml: &str) {
        let endpoint = get_rest_endpoint_by_region(&creds.region);
        let auth = AuthOptionsBuilder::new(endpoint).key(&creds.key).build();
        let config = SynthesizerConfig::new(auth, AudioFormat::Riff16Khz16BitMonoPcm);

        let syn = config.rest_synthesizer().unwrap();
        let result = syn.synthesize_ssml(ssml).await;
        assert_eq!(
            result.is_ok(),
            valid,
            "SSML is {} but request {}",
            if valid { "valid" } else { "invalid" },
            if result.is_ok() { "succeeds" } else { "fails" }
        );
        if result.is_ok() {
            let audio_data = result.unwrap();
            assert!(!audio_data.is_empty(), "Audio data should not be empty");
        }
    }
}
