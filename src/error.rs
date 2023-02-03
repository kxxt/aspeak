use thiserror::Error;

#[derive(Error, Debug)]
pub enum AspeakError {
    #[error("websocket error")]
    WebSocketError(#[from] tungstenite::Error),
    #[error("Connection closed, code: {code}, reason: {reason}")]
    ConnectionCloseError { code: String, reason: String },
    #[error("Encountered invalid websocket message, invalid segment is: {0:?}")]
    InvalidWebSocketMessage(String),
    #[error("audio decoder error")]
    DecoderError(#[from] rodio::decoder::DecoderError),
    #[error("audio stream error")]
    StreamError(#[from] rodio::StreamError),
    #[error("audio play error")]
    PlayError(#[from] rodio::PlayError),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("No input text/SSML.")]
    InputError,
    #[error("Failed to create SSML!")]
    XmlError(#[from] xml::writer::Error),
}

pub type Result<T> = std::result::Result<T, AspeakError>;
