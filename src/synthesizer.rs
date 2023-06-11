use chrono::Utc;
use futures_util::SinkExt;
use hyper::http::HeaderValue;
use log::{debug, info};
use tokio_tungstenite::tungstenite::{
    client::IntoClientRequest, handshake::client::Request, Message,
};
use uuid::Uuid;

use crate::{
    constants::ORIGIN,
    net::{self, connect_directly, ConnectError},
    AudioFormat, AuthOptions, DEFAULT_ENDPOINT,
};

mod http;
mod websocket;

pub use websocket::*;

/// Initialize a new [`Synthesizer`] by creating a new [`SynthesizerConfig`] and call [`SynthesizerConfig::connect`].
#[derive(Debug, Clone)]
pub struct SynthesizerConfig<'a> {
    /// The authentication options.
    pub(crate) auth: AuthOptions<'a>,
    /// The audio format of the output audio.
    pub(crate) audio_format: AudioFormat,
}

const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"version":"1.25.0","name":"SpeechSDK","build":"Windows-x64"},"os":{"platform":"Windows","name":"Client","version":"10"}}}"#; // r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl<'a> SynthesizerConfig<'a> {
    /// Create a new [`SynthesizerConfig`] with the given [`AuthOptions`] and [`AudioFormat`].
    pub fn new(auth: AuthOptions<'a>, audio_format: AudioFormat) -> Self {
        info!("Successfully created SynthesizerConfig");
        Self { auth, audio_format }
    }

    fn generate_client_request(&self) -> Result<Request, WebsocketSynthesizerError> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let uri = {
            let mut url = url::Url::parse(&self.auth.endpoint)?;
            url.query_pairs_mut()
                .append_pair("X-ConnectionId", &request_id);
            if let Some(auth_token) = &self.auth.token {
                url.query_pairs_mut()
                    .append_pair("Authorization", auth_token);
            }
            url
        };
        let mut request = uri
            .into_client_request()
            .map_err(|e| WebsocketSynthesizerError {
                kind: WebsocketSynthesizerErrorKind::InvalidRequest,
                source: Some(e.into()),
            })?;
        let headers = request.headers_mut();
        if let Some(key) = &self.auth.key {
            headers.append("Ocp-Apim-Subscription-Key", HeaderValue::from_str(key)?);
        }
        if !self.auth.headers.is_empty() {
            // TODO: I don't know if this could be further optimized
            headers.extend(self.auth.headers.iter().map(Clone::clone));
        } else if Some(self.auth.endpoint.as_ref()) == DEFAULT_ENDPOINT {
            // Trial endpoint
            headers.append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        }
        debug!("The initial request is {request:?}");
        Ok(request)
    }

    /// Connect to the Azure Speech Service and return a [`Synthesizer`] on success.
    pub async fn connect(self) -> Result<WebsocketSynthesizer, WebsocketSynthesizerError> {
        let request = self.generate_client_request()?;
        let proxy_url = self
            .auth
            .proxy
            .as_deref()
            .map(reqwest::Url::parse)
            .transpose()
            .map_err(|e| ConnectError {
                kind: net::ConnectErrorKind::BadUrl(self.auth.proxy.unwrap().to_string()),
                source: Some(e.into()),
            })?;
        let mut wss = match proxy_url.as_ref().map(|x| x.scheme()) {
            Some("socks5") => {
                net::connect_via_socks5_proxy(request, proxy_url.as_ref().unwrap()).await?
            }
            Some("http") | Some("https") => {
                net::connect_via_http_proxy(request, proxy_url.as_ref().unwrap()).await?
            }
            None => connect_directly(request).await?,
            Some(other_scheme) => {
                return Err(ConnectError {
                    kind: net::ConnectErrorKind::UnsupportedScheme(Some(other_scheme.to_string())),
                    source: None,
                }
                .into())
            }
        };
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple();
        let now = Utc::now();
        wss.send(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ))).await?;
        info!("Successfully created Synthesizer");
        Ok(WebsocketSynthesizer {
            audio_format: self.audio_format,
            stream: wss,
        })
    }
}
