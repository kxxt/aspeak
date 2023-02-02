use thiserror::Error;

#[derive(Error, Debug)]
pub enum AspeakError {
    #[error("websocket error")]
    WebSocketError(#[from] tungstenite::Error),
    #[error("Encountered invalid websocket message, invalid segment is: {0:?}")]
    InvalidWebSocketMessage(String),
    #[error("audio decoder error")]
    DecoderError(#[from] rodio::decoder::DecoderError),
    #[error("audio stream error")]
    StreamError(#[from] rodio::StreamError),
    #[error("audio play error")]
    PlayError(#[from] rodio::PlayError),
}
