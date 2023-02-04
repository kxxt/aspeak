use log::{debug, info};
use rodio::{Decoder, OutputStream, Sink};
use std::{cell::RefCell, io::Cursor, net::TcpStream};
use tungstenite::{
    client::IntoClientRequest, connect, http::HeaderValue, stream::MaybeTlsStream, Message,
    WebSocket,
};
use uuid::Uuid;

use crate::{msg::WebSocketMessage, AspeakError, AudioFormat, Result, ORIGIN};
use chrono::Utc;

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Debug, Clone)]
pub struct SynthesizerConfig {
    pub(crate) wss_endpoint: String,
    pub(crate) audio_format: AudioFormat,
}

const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl SynthesizerConfig {
    pub(crate) fn format_endpoint_url(endpoint: &str) -> String {
        format!("wss://{endpoint}/cognitiveservices/websocket/v1",)
    }

    pub fn new(endpoint: &str, audio_format: AudioFormat) -> Self {
        let wss_endpoint = Self::format_endpoint_url(endpoint);
        info!("Successfully created SynthesizerConfig");
        return Self {
            wss_endpoint,
            audio_format,
        };
    }

    pub fn connect(self) -> Result<Synthesizer> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let mut request = self.wss_endpoint.into_client_request()?;
        request
            .headers_mut()
            .append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        debug!("The initial request is {request:?}");
        let (mut wss, resp) = connect(request)?;
        let mut now = Utc::now();
        debug!("The response to the initial request is {:?}", resp);
        wss.write_message(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ,request_id = &request_id)) )?;
        now = Utc::now();
        let synthesis_context = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(&self.audio_format)
        );
        info!("Synthesis context is: {}", synthesis_context);
        wss.write_message(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_context}", 
            request_id = & request_id)),
        )?;
        info!("Successfully created Synthesizer");
        Ok(Synthesizer {
            request_id,
            wss: RefCell::new(wss),
        })
    }
}

#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct Synthesizer {
    request_id: String,
    wss: RefCell<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Synthesizer {
    pub fn synthesize(
        &self,
        ssml: &str,
        mut callback: impl FnMut(Option<&[u8]>) -> Result<()>,
    ) -> Result<()> {
        let now = Utc::now();
        let request_id = &self.request_id;
        self.wss.borrow_mut().write_message(Message::Text(format!(
            "Path: ssml\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}\r\nContent-Type: application/ssml+xml\r\n\r\n{ssml}"
        )))?;
        loop {
            let raw_msg = self.wss.borrow_mut().read_message()?;
            let msg = WebSocketMessage::try_from(&raw_msg)?;
            match msg {
                WebSocketMessage::TurnStart | WebSocketMessage::Response { body: _ } => continue,
                WebSocketMessage::Audio { data } => callback(Some(data))?,
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
    }
}

pub fn callback_play_blocking() -> Box<dyn FnMut(Option<&[u8]>) -> Result<()>> {
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
    m.add_class::<SynthesizerConfig>()?;
    Ok(())
}
