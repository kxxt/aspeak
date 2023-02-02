use thiserror::Error;

#[derive(Error, Debug)]
pub enum AspeakError {
    #[error("websocket error")]
    WebSocketError(#[from] tungstenite::Error),
    #[error("Encountered invalid websocket message, invalid segment is: {0:?}")]
    InvalidWebSocketMessage(String),
}
