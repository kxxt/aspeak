use log::{debug, info};
use std::{cell::RefCell, net::TcpStream};
use tungstenite::{
    client::IntoClientRequest, connect, http::HeaderValue, stream::MaybeTlsStream, Message,
    WebSocket,
};
use uuid::Uuid;

use crate::{msg::WebSocketMessage, AspeakError, AudioFormat, Result, ORIGIN};
use chrono::Utc;

pub struct SynthesizerConfig {
    wss_endpoint: String,
}

const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl SynthesizerConfig {
    pub fn new(endpoint: &str) -> Self {
        let wss_endpoint = format!("wss://{endpoint}/cognitiveservices/websocket/v1");
        info!("Successfully created SynthesizerConfig");
        return Self { wss_endpoint };
    }

    pub fn connect(self, audio_format: AudioFormat) -> Result<Synthesizer> {
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let mut request = self.wss_endpoint.into_client_request()?;
        request
            .headers_mut()
            .append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        debug!("The initial request is {request:?}");
        let (mut wss, resp) = connect(request)?;
        let mut now = Utc::now();
        println!("{:?}", resp);
        wss.write_message(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ,request_id = &request_id)) )?;
        now = Utc::now();
        let synthesis_config = format!(
            r#"{{"synthesis":{{"audio":{{"metadataOptions":{{"sentenceBoundaryEnabled":false,"wordBoundaryEnabled":false}},"outputFormat":"{}"}}}}}}"#,
            Into::<&str>::into(&audio_format)
        );
        info!("Synthesis config is: {}", synthesis_config);
        wss.write_message(Message::Text(format!(
            "Path: synthesis.context\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{synthesis_config}", 
            request_id = & request_id)),
        )?;
        info!("Successfully created Synthesizer");
        Ok(Synthesizer {
            request_id,
            wss: RefCell::new(wss),
        })
    }
}

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
