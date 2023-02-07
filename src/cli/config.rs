use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use aspeak::{get_endpoint_by_region, AspeakError, AudioFormat, Role, DEFAULT_VOICES};
use color_eyre::eyre::{anyhow, bail};

use serde::Deserialize;

use super::args::ContainerFormat;

pub(crate) const CONFIG_TEMPLATE: &str = include_str!("aspeak.toml");
pub(crate) const DEFAULT_PROFILE_NAME: &str = ".aspeak.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub auth: Option<AuthConfig>,
    pub text: Option<TextConfig>,
    pub output: Option<OutputConfig>,
    pub verbosity: Option<u8>,
}

impl Config {
    pub fn initialize(path: &Path, overwrite: bool) -> color_eyre::Result<()> {
        fn create_config(path: &Path) -> color_eyre::Result<()> {
            std::fs::write(path, CONFIG_TEMPLATE)?;
            Ok(())
        }
        if !path.exists() {
            create_config(path)?;
            println!("Created new config file: {}", path.display());
        } else if overwrite {
            create_config(path)?;
            println!("Overwritten existing config file: {}", path.display(),);
        } else {
            bail!(
                "Configuration file already exists! Refusing to overwrite {}",
                path.display()
            )
        }
        Ok(())
    }

    pub fn default_location() -> color_eyre::Result<PathBuf> {
        let path = dirs::home_dir()
            .ok_or(anyhow!("Could not find home directory"))?
            .join(DEFAULT_PROFILE_NAME);
        Ok::<PathBuf, color_eyre::eyre::ErrReport>(path)
    }

    pub fn load<P: AsRef<Path>>(path: Option<P>) -> color_eyre::Result<Self> {
        let text = if let Some(path) = path {
            fs::read_to_string(path)?
        } else {
            fs::read_to_string(Self::default_location()?)?
        };
        Ok(toml::from_str(&text)?)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthConfig {
    pub endpoint: Option<EndpointConfig>,
    pub key: Option<String>,
    pub token: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
}

#[derive(Debug, Deserialize)]
pub(crate) enum EndpointConfig {
    Endpoint { endpoint: String },
    Region { region: String },
}

impl<'a> From<&'a EndpointConfig> for Cow<'a, str> {
    fn from(endpoint: &'a EndpointConfig) -> Self {
        match endpoint {
            EndpointConfig::Endpoint { endpoint } => Cow::Borrowed(endpoint),
            EndpointConfig::Region { region } => {
                Cow::Owned(get_endpoint_by_region(region.as_str()))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TextConfig {
    #[serde(flatten)]
    pub voice: Option<VoiceConfig>,
    pub rate: Option<toml::Value>,
    pub pitch: Option<toml::Value>,
    pub style_degree: Option<f32>,
    pub role: Option<Role>,
    pub style: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum VoiceConfig {
    Voice { voice: String },
    Locale { locale: String },
}

impl TryFrom<VoiceConfig> for String {
    type Error = AspeakError;

    fn try_from(voice: VoiceConfig) -> Result<Self, Self::Error> {
        Ok(match voice {
            VoiceConfig::Voice { voice } => voice,
            VoiceConfig::Locale { locale } => DEFAULT_VOICES
                .get(locale.as_str())
                .ok_or_else(|| AspeakError::ArgumentError(format!("Invalid locale: {}", locale)))?
                .to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputConfig {
    #[serde(flatten)]
    pub format: OutputFormatConfig,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum OutputFormatConfig {
    AudioFormat {
        format: AudioFormat,
    },
    ContaierAndQuality {
        container: Option<ContainerFormat>,
        quality: Option<i32>,
    },
}
