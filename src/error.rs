use thiserror::Error;

#[derive(Error, Debug)]
pub enum AspeakError {
    #[error("websocket error")]
    WebSocketError(#[from] tungstenite::Error),
}
