use crate::{msg::WebSocketMessage, AspeakError, AudioFormat, Result, ORIGIN};
use chrono::Utc;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use log::{debug, info, trace};
use rodio::{Decoder, OutputStream, Sink};
use std::{cell::RefCell, io::Cursor, thread::sleep, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_tungstenite::{
    connect_async, tungstenite::client::IntoClientRequest, tungstenite::http::HeaderValue,
    tungstenite::protocol::Message, tungstenite::WebSocket, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Debug, Clone)]
pub struct SynthesizerConfig {
    pub(crate) endpoint: String,
    pub(crate) audio_format: AudioFormat,
}

const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"version":"1.25.0","name":"SpeechSDK","build":"Windows-x64"},"os":{"platform":"Windows","name":"Client","version":"10"}}}"#; // r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl SynthesizerConfig {
    pub fn new(endpoint: &str, audio_format: AudioFormat) -> Self {
        info!("Successfully created SynthesizerConfig");
        return Self {
            endpoint: endpoint.to_string(),
            audio_format,
        };
    }

    pub async fn connect(self) -> Result<Synthesizer> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let mut request = format!("{}?X-ConnectionId={}&Authorization=bearer%20eyJhbGciOiJFUz1NiIsImtpZCI6ImtleTEiLCJ0eXAiOiJKV1QifQ.eyJyZWdpb24iOiJlYXN0dXMiLCJzdWJzY3JpcHRpb24taWQiOiI2MWIxODBlMmJkOGU0YWI2OGNiNmQxN2UxOWE5NjAwMiIsInByb2R1Y3QtaWQiOiJTcGVlY2hTZXJ2aWNlcy5TMCIsImNvZ25pdGl2ZS1zZXJ2aWNlcy1lbmRwb2ludCI6Imh0dHBzOi8vYXBpLmNvZ25pdGl2ZS5taWNyb3NvZnQuY29tL2ludGVybmFsL3YxLjAvIiwiYXp1cmUtcmVzb3VyY2UtaWQiOiIvc3Vic2NyaXB0aW9ucy9jMjU1ZGYzNi05NzRjLTQ2MGEtODMwYi0yNTE2NTEzYWNlYjIvcmVzb3VyY2VHcm91cHMvY3MtY29nbml0aXZlc2VydmljZXMtcHJvZC13dXMyL3Byb3ZpZGVycy9NaWNyb3NvZnQuQ29nbml0aXZlU2VydmljZXMvYWNjb3VudHMvYWNvbS1zcGVlY2gtcHJvZC1lYXN0dXMiLCJzY29wZSI6InNwZWVjaHNlcnZpY2VzIiwiYXVkIjoidXJuOm1zLnNwZWVjaHNlcnZpY2VzLmVhc3R1cyIsImV4cCI6MTY3NTUxMzM3NiwiaXNzIjoidXJuOm1zLmNvZ25pdGl2ZXNlcnZpY2VzIn0.P-wpih8GaCGUF6VKiDGSOcs_KdUPIak0evmKXJyjpJlRWGniQyVnIU_34I0e5XXg0vi4Z-4L2vStAfJx3GSCHA", self.endpoint, request_id).into_client_request()?;
        let headers = request.headers_mut();
        headers.append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        // headers.append("Authorization", HeaderValue::from_str("Bearer ").unwrap());
        debug!("The initial request is {request:?}");
        let (mut wss, resp) = connect_async(request).await?;
        // let (write, read) = wss.split();
        let mut now = Utc::now();
        debug!("The response to the initial request is {:?}", resp);
        wss.send(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ,request_id = &request_id))).await?;
        now = Utc::now();
        let synthesis_context = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(&self.audio_format)
        );
        info!("Synthesis context is: {}", synthesis_context);
        wss.send(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_context}", 
            request_id = &request_id)),
        ).await?;
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
    wss: RefCell<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

static mut COUNTER: i32 = 0;

impl Synthesizer {
    pub async fn synthesize(
        &self,
        ssml: &str,
        mut callback: impl FnMut(Option<&[u8]>) -> Result<()>,
    ) -> Result<()> {
        let now = Utc::now();
        let request_id = &self.request_id;
        self.wss.borrow_mut().send(Message::Text(format!(
            "Path: ssml\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}\r\nContent-Type: application/ssml+xml\r\n\r\n{ssml}"
        ))).await?;
        while let Some(raw_msg) = self.wss.borrow_mut().next().await {
            let raw_msg = raw_msg?;
            let msg = WebSocketMessage::try_from(&raw_msg)?;
            match msg {
                WebSocketMessage::TurnStart | WebSocketMessage::Response { body: _ } => continue,
                WebSocketMessage::Audio { data } => {
                    unsafe {
                        trace!("Before receving {COUNTER} audio frame");
                        callback(Some(data))?;
                        trace!("After receving {COUNTER} audio frame");
                        COUNTER += 1;
                    }
                    // self.wss.borrow_mut().send(Message::Pong(Vec::new())).await?;
                    // sleep(Duration::from_millis(300));
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
