use std::str;

use log::trace;

use tokio_tungstenite::{tungstenite::protocol::CloseFrame, tungstenite::Message};

use crate::error::AspeakError;

#[derive(Debug, Clone, Copy)]
pub(crate) enum WebSocketMessage<'a> {
    TurnStart,
    TurnEnd,
    #[allow(unused)]
    Response {
        body: &'a str,
    },
    Audio {
        data: &'a [u8],
    },
    Close(Option<&'a CloseFrame<'a>>),
    Ping,
    Pong,
}

impl<'a> TryFrom<&'a Message> for WebSocketMessage<'a> {
    type Error = AspeakError;

    fn try_from(value: &'a Message) -> Result<Self, Self::Error> {
        Ok(match *value {
            Message::Binary(ref data) => {
                let (int_bytes, rest) = data.split_at(std::mem::size_of::<u16>());
                let header_len = u16::from_be_bytes([int_bytes[0], int_bytes[1]]) as usize;
                let header = str::from_utf8(&rest[..header_len]).unwrap();
                let is_audio = {
                    let headers = header.split("\r\n");
                    let mut is_audio = false;
                    for header in headers {
                        trace!("Found header {header}");
                        if header.starts_with("Path") && header.ends_with("audio") {
                            is_audio = true;
                            break;
                        }
                    }
                    is_audio
                };
                if !is_audio {
                    return Err(AspeakError::InvalidWebSocketMessage(header.to_string()));
                }
                WebSocketMessage::Audio {
                    data: &rest[header_len..],
                }
            }
            Message::Text(ref text) => {
                let err_construct = || AspeakError::InvalidWebSocketMessage(text.to_string());
                let (header_text, body) = text.split_once("\r\n\r\n").ok_or_else(err_construct)?;
                let mut result = None;
                for header in header_text.split("\r\n") {
                    trace!("Found header {header}");
                    let (k, v) = header.split_once(':').ok_or_else(err_construct)?;
                    if k == "Path" {
                        match v.trim() {
                            "turn.end" => result = Some(WebSocketMessage::TurnEnd),
                            "turn.start" => result = Some(WebSocketMessage::TurnStart),
                            "response" => result = Some(WebSocketMessage::Response { body }),
                            _ => break,
                        }
                    }
                }
                result.ok_or_else(err_construct)?
            }
            Message::Close(ref frame) => WebSocketMessage::Close(frame.as_ref()),
            Message::Ping(_) => WebSocketMessage::Ping,
            Message::Pong(_) => WebSocketMessage::Pong,
            ref msg => {
                return Err(AspeakError::InvalidWebSocketMessage(format!(
                    "Niether Binary nor Text! Frame is {msg}"
                )))
            }
        })
    }
}
