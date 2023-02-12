use crate::{
    interpolate_ssml, msg::WebSocketMessage, AspeakError, AudioFormat, AuthOptions, Result,
    TextOptions, ORIGIN,
};
use chrono::Utc;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{debug, info};
use rodio::{Decoder, OutputStream, Sink};
use std::{cell::RefCell, io::Cursor};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::client::IntoClientRequest, tungstenite::http::HeaderValue,
    tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SynthesizerConfig<'a> {
    pub(crate) auth: AuthOptions<'a>,
    pub(crate) audio_format: AudioFormat,
}

const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"version":"1.25.0","name":"SpeechSDK","build":"Windows-x64"},"os":{"platform":"Windows","name":"Client","version":"10"}}}"#; // r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl<'a> SynthesizerConfig<'a> {
    pub fn new(auth: AuthOptions<'a>, audio_format: AudioFormat) -> Self {
        info!("Successfully created SynthesizerConfig");
        Self { auth, audio_format }
    }

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
        } else {
            headers.append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        }
        debug!("The initial request is {request:?}");
        let (wss, resp) = connect_async(request).await?;
        let (mut write, read) = wss.split();
        let mut now = Utc::now();
        debug!("The response to the initial request is {:?}", resp);
        write.send(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ,request_id = &request_id))).await?;
        now = Utc::now();
        let synthesis_context = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(self.audio_format)
        );
        info!("Synthesis context is: {}", synthesis_context);
        write.send(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_context}", 
            request_id = &request_id)),
        ).await?;
        info!("Successfully created Synthesizer");
        Ok(Synthesizer {
            request_id,
            write: RefCell::new(write),
            read: RefCell::new(read),
        })
    }
}

pub trait SynthesisCallback: FnMut(Option<&[u8]>) -> Result<()> {}
impl<T> SynthesisCallback for T where T: FnMut(Option<&[u8]>) -> Result<()> {}

#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct Synthesizer {
    request_id: String,
    write: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    read: RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl Synthesizer {
    pub async fn synthesize_ssml(
        &self,
        ssml: &str,
        mut callback: impl SynthesisCallback,
    ) -> Result<()> {
        let now = Utc::now();
        let request_id = &self.request_id;
        self.write.borrow_mut().send(Message::Text(format!(
            "Path: ssml\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}\r\nContent-Type: application/ssml+xml\r\n\r\n{ssml}"
        ))).await?;
        while let Some(raw_msg) = self.read.borrow_mut().next().await {
            let raw_msg = raw_msg?;
            let msg = WebSocketMessage::try_from(&raw_msg)?;
            match msg {
                WebSocketMessage::TurnStart | WebSocketMessage::Response { body: _ } => continue,
                WebSocketMessage::Audio { data } => {
                    callback(Some(data))?;
                }
                WebSocketMessage::TurnEnd => return callback(None),
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
            }
        }
        Ok(())
    }

    pub async fn synthesize_text(
        &self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
        callback: impl SynthesisCallback,
    ) -> Result<()> {
        let ssml = interpolate_ssml(text, options)?;
        self.synthesize_ssml(&ssml, callback).await
    }
}

pub fn callback_play_blocking() -> Box<dyn SynthesisCallback> {
    let mut buffer = Vec::new();
    Box::new(move |data| {
        if let Some(data) = data {
            buffer.extend_from_slice(data);
        } else {
            info!("Playing audio... ({} bytes)", buffer.len());
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let sink = Sink::try_new(&stream_handle).unwrap();
            let cursor = Cursor::new(Vec::from(&buffer[..]));
            let source = Decoder::new(cursor)?;
            sink.append(source);
            sink.sleep_until_end();
        }
        Ok(())
    })
}

#[cfg(feature = "python")]
pub(crate) fn register_python_items(
    _py: pyo3::Python<'_>,
    m: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    m.add_class::<Synthesizer>()?;
    // m.add_class::<SynthesizerConfig>()?;
    Ok(())
}
