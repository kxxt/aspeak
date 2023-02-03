mod error;
mod msg;
mod ssml;
mod synthesizer;
mod types;
mod voice;

pub const ORIGIN: &str = "https://azure.microsoft.com";

pub use error::{AspeakError, Result};
pub use ssml::interpolate_ssml;
pub use synthesizer::{Synthesizer, SynthesizerConfig};
pub use types::*;
pub use voice::Voice;
