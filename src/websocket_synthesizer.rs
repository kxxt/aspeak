use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::constants::{DEFAULT_ENDPOINT, ORIGIN};
use crate::msg;
use crate::net::{self, connect_directly, ProxyConnectError, WsStream};
use crate::{interpolate_ssml, msg::WebSocketMessage, AudioFormat, AuthOptions, TextOptions};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use hyper::header::InvalidHeaderValue;
use hyper::Request;
use log::{debug, info, warn};

use tokio_tungstenite::{
    tungstenite::client::IntoClientRequest, tungstenite::http::HeaderValue,
    tungstenite::protocol::Message,
};
use uuid::Uuid;

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

    fn generate_client_request(&self) -> Result<Request<()>, WebsocketSynthesizerError> {
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
            .map_err(|e| ProxyConnectError {
                kind: net::ProxyConnectErrorKind::BadUrl(self.auth.proxy.unwrap().to_string()),
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
                return Err(ProxyConnectError {
                    kind: net::ProxyConnectErrorKind::UnsupportedScheme(Some(
                        other_scheme.to_string(),
                    )),
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

/// The main struct for interacting with the Azure Speech Service.
pub struct WebsocketSynthesizer {
    audio_format: AudioFormat,
    stream: WsStream,
}

impl WebsocketSynthesizer {
    /// Synthesize the given SSML into audio(bytes).
    pub async fn synthesize_ssml(
        &mut self,
        ssml: &str,
    ) -> Result<Vec<u8>, WebsocketSynthesizerError> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple();
        let now = Utc::now();
        let synthesis_context = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false,"sessionEndEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(self.audio_format)
        );
        self.stream.send(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_context}", 
            request_id = &request_id)),
        ).await?;
        info!("Before sending the SSML to the server");
        self.stream.send(Message::Text(format!(
            "Path: ssml\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}\r\nContent-Type: application/ssml+xml\r\n\r\n{ssml}"
        ))).await?;
        const HEADER_SIZE: usize = 44;
        let mut buffer = Vec::with_capacity(HEADER_SIZE);
        while let Some(raw_msg) = self.stream.next().await {
            let raw_msg = raw_msg?;
            let msg = WebSocketMessage::try_from(&raw_msg)?;
            match msg {
                WebSocketMessage::TurnStart | WebSocketMessage::Response { body: _ } => continue,
                WebSocketMessage::Audio { data } => {
                    buffer.extend_from_slice(data);
                }
                WebSocketMessage::TurnEnd => {
                    break;
                }
                WebSocketMessage::Close(frame) => {
                    return Err(frame.map_or_else(
                        || {
                            WebsocketSynthesizerError::connection_closed(
                                "Unknown".to_string(),
                                "The server closed the connection without a reason".to_string(),
                            )
                        },
                        |fr| {
                            WebsocketSynthesizerError::connection_closed(
                                fr.code.to_string(),
                                fr.reason.to_string(),
                            )
                        },
                    ));
                }
                msg => warn!("Received a message that is not handled: {:?}", msg),
            }
        }
        Ok(buffer)
    }

    /// Synthesize the given text into audio(bytes).
    /// This is a convenience method that interpolates the SSML for you.
    pub async fn synthesize_text(
        &mut self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
    ) -> Result<Vec<u8>, WebsocketSynthesizerError> {
        debug!("Synthesizing text: {}", text.as_ref());
        let ssml = interpolate_ssml(text, options)?;
        self.synthesize_ssml(&ssml).await
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct WebsocketSynthesizerError {
    pub kind: WebsocketSynthesizerErrorKind,
    pub(crate) source: Option<anyhow::Error>,
}

impl WebsocketSynthesizerError {
    fn connection_closed(code: String, reason: String) -> Self {
        Self {
            kind: WebsocketSynthesizerErrorKind::WebsocketConnectionClosed { code, reason },
            source: None,
        }
    }
}

impl Display for WebsocketSynthesizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use WebsocketSynthesizerErrorKind::*;
        match &self.kind {
            WebsocketConnectionClosed { code, reason } => {
                write!(
                    f,
                    "ws synthesizer error: the websocket connection was closed with code {} and reason {}",
                    code, reason
                )
            }
            WebsocketError => write!(f, "ws synthesizer error: websocket error"),
            ProxyConnect => write!(f, "ws synthesizer error: proxy connect error"),
            InvalidRequest => write!(f, "ws synthesizer error: invalid request"),
            _ => write!(f, "{:?} error", self.kind),
        }
    }
}

impl Error for WebsocketSynthesizerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(feature = "python")]
impl From<WebsocketSynthesizerError> for pyo3::PyErr {
    fn from(value: WebsocketSynthesizerError) -> Self {
        pyo3::exceptions::PyOSError::new_err(format!("{:?}", color_eyre::Report::from(value)))
    }
}

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum WebsocketSynthesizerErrorKind {
    ProxyConnect,
    WebsocketConnectionClosed { code: String, reason: String },
    WebsocketError,
    InvalidRequest,
    InvalidMessage,
    SsmlError,
}

macro_rules! impl_from_for_ws_synthesizer_error {
    ($error_type:ty, $error_kind:ident) => {
        impl From<$error_type> for WebsocketSynthesizerError {
            fn from(e: $error_type) -> Self {
                Self {
                    kind: WebsocketSynthesizerErrorKind::$error_kind,
                    source: Some(e.into()),
                }
            }
        }
    };
}

impl_from_for_ws_synthesizer_error!(InvalidHeaderValue, InvalidRequest);
impl_from_for_ws_synthesizer_error!(url::ParseError, InvalidRequest);
impl_from_for_ws_synthesizer_error!(net::ProxyConnectError, ProxyConnect);
impl_from_for_ws_synthesizer_error!(tokio_tungstenite::tungstenite::Error, WebsocketError);
impl_from_for_ws_synthesizer_error!(crate::ssml::SsmlError, SsmlError);

impl From<msg::ParseError> for WebsocketSynthesizerError {
    fn from(e: msg::ParseError) -> Self {
        Self {
            kind: WebsocketSynthesizerErrorKind::InvalidMessage,
            source: Some(e.into()),
        }
    }
}
