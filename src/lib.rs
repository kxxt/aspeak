mod error;
mod msg;
mod ssml;
mod synthesizer;
mod types;
mod voice;

pub const ORIGIN: &str = "https://azure.microsoft.com";
pub const DEFAULT_ENDPOINT: &str =
    "wss://eastus.api.speech.microsoft.com/cognitiveservices/websocket/v1";

pub use error::{AspeakError, Result};
pub use ssml::interpolate_ssml;
pub use synthesizer::{callback_play_blocking, Synthesizer, SynthesizerConfig};
pub use types::*;
pub use voice::Voice;

#[cfg(feature = "python")]
pub mod python;
