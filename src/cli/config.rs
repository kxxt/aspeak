use aspeak::{AudioFormat, Role};
use serde::Deserialize;


// pub(crate) const CONFIG_TEMPLATE: &str = include_str!("aspeak.toml");

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    auth: Option<AuthConfig>,
    text: Option<TextConfig>,
    output: Option<OutputConfig>,
    verbosity: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthConfig {
    endpoint: Option<EndpointConfig>,
    key: Option<String>,
    token: Option<String>,
    headers: Option<Vec<(String, String)>>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum EndpointConfig {
    Endpoint { endpoint: String },
    Region { region: String },
}

#[derive(Debug, Deserialize)]
pub(crate) struct TextConfig {
    #[serde(flatten)]
    voice: Option<VoiceConfig>,
    rate: Option<toml::Value>,
    pitch: Option<toml::Value>,
    style_degree: Option<f32>,
    role: Option<Role>,
    style: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum VoiceConfig {
    Voice { voice: String },
    Locale { locale: String },
}
#[derive(Debug, Deserialize)]
pub(crate) struct OutputConfig {
    #[serde(flatten)]
    format: OutputFormatConfig,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum OutputFormatConfig {
    AudioFormat {
        format: AudioFormat,
    },
    ContaierAndQuality {
        container: Option<String>,
        quality: Option<i32>,
    },
}
