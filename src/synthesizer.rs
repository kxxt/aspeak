use crate::constants::{DEFAULT_ENDPOINT, ORIGIN};
use crate::{
    interpolate_ssml, msg::WebSocketMessage, AspeakError, AudioFormat, AuthOptions, Result,
    TextOptions,
};
use chrono::Utc;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, info, warn};
use std::cell::RefCell;

use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::client::IntoClientRequest, tungstenite::http::HeaderValue,
    tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
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

    /// Connect to the Azure Speech Service and return a [`Synthesizer`] on success.
    pub async fn connect(self) -> Result<Synthesizer> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let uri = {
            let uri = format!("{}?X-ConnectionId={}", self.auth.endpoint, request_id);
            if let Some(auth_token) = self.auth.token {
                format!("{uri}&Authorization={auth_token}")
            } else {
                uri
            }
        };
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        if !self.auth.headers.is_empty() {
            // TODO: I don't know if this could be further optimized
            headers.extend(self.auth.headers.iter().map(Clone::clone));
        } else if self.auth.endpoint == DEFAULT_ENDPOINT {
            // Trial endpoint
            headers.append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        }
        debug!("The initial request is {request:?}");
        let (wss, resp) = connect_async(request).await?;
        let (mut write, read) = wss.split();
        let now = Utc::now();
        debug!("The response to the initial request is {:?}", resp);
        write.send(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ,request_id = &request_id))).await?;
        info!("Successfully created Synthesizer");
        Ok(Synthesizer {
            audio_format: self.audio_format,
            write: RefCell::new(write),
            read: RefCell::new(read),
        })
    }
}

/// The main struct for interacting with the Azure Speech Service.
pub struct Synthesizer {
    audio_format: AudioFormat,
    write: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    read: RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl Synthesizer {
    /// Synthesize the given SSML into audio(bytes).
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn synthesize_ssml(&self, ssml: &str) -> Result<Vec<u8>> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let now = Utc::now();
        let synthesis_context = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false,"sessionEndEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(self.audio_format)
        );
        self.write.borrow_mut().send(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_context}", 
            request_id = &request_id)),
        ).await?;
        info!("Before sending the SSML to the server");
        self.write.borrow_mut().send(Message::Text(format!(
            "Path: ssml\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}\r\nContent-Type: application/ssml+xml\r\n\r\n{ssml}"
        ))).await?;
        const HEADER_SIZE: usize = 44;
        let mut buffer = Vec::with_capacity(HEADER_SIZE);
        while let Some(raw_msg) = self.read.borrow_mut().next().await {
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
                        || AspeakError::ConnectionCloseError {
                            code: "Unknown".to_string(),
                            reason: "The server closed the connection without a reason".to_string(),
                        },
                        |fr| AspeakError::ConnectionCloseError {
                            code: fr.code.to_string(),
                            reason: fr.reason.to_string(),
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
        &self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
    ) -> Result<Vec<u8>> {
        debug!("Synthesizing text: {}", text.as_ref());
        let ssml = interpolate_ssml(text, options)?;
        self.synthesize_ssml(&ssml).await
    }
}
