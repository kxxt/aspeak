use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use aspeak::{
    get_default_voice_by_locale, get_rest_endpoint_by_region, get_websocket_endpoint_by_region,
    AudioFormat, Role,
};
use color_eyre::eyre::{anyhow, bail};

use serde::Deserialize;

use super::args::{ContainerFormat, SynthesizerMode};

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

    pub fn load<P: AsRef<Path>>(path: Option<P>) -> color_eyre::Result<Option<Self>> {
        let text = if let Some(path) = path {
            Some(fs::read_to_string(path)?)
        } else {
            // return None if the default config file does not exist
            let path = Self::default_location()?;
            if !path.exists() {
                return Ok(None);
            }
            Some(fs::read_to_string(path)?)
        };
        Ok(text.as_deref().map(toml::from_str).transpose()?)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthConfig {
    #[serde(flatten)]
    pub endpoint_config: Option<EndpointConfig>,
    pub key: Option<String>,
    pub token: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    pub proxy: Option<String>,
    pub voice_list_api: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum EndpointConfig {
    Endpoint { endpoint: String },
    Region { region: String },
}

impl EndpointConfig {
    pub(crate) fn to_cow_str(&self, mode: SynthesizerMode) -> Cow<str> {
        match self {
            EndpointConfig::Endpoint { endpoint } => Cow::Borrowed(endpoint),
            EndpointConfig::Region { region } => Cow::Owned(match mode {
                SynthesizerMode::Websocket => get_websocket_endpoint_by_region(region.as_str()),
                SynthesizerMode::Rest => get_rest_endpoint_by_region(region.as_str()),
            }),
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

impl TextConfig {
    pub fn rate(&self) -> color_eyre::Result<Option<Cow<'_, str>>> {
        Ok(match self.rate.as_ref() {
            Some(toml::Value::String(s)) => Some(super::parse::parse_rate(s)?),
            Some(toml::Value::Integer(i)) => {
                Some(Cow::Owned(format!("{:.2}%", (*i as f32) * 100f32)))
            }
            Some(toml::Value::Float(f)) => {
                Some(Cow::Owned(format!("{:.2}%", (*f as f32) * 100f32)))
            }
            None => None,
            _ => return Err(anyhow!("Got invalid rate from profile: {:?}", self.rate)),
        })
    }

    pub fn pitch(&self) -> color_eyre::Result<Option<Cow<'_, str>>> {
        Ok(match self.pitch.as_ref() {
            Some(toml::Value::String(s)) => Some(super::parse::parse_pitch(s)?),
            Some(toml::Value::Integer(i)) => {
                Some(Cow::Owned(format!("{:.2}%", (*i as f32) * 100f32)))
            }
            Some(toml::Value::Float(f)) => {
                Some(Cow::Owned(format!("{:.2}%", (*f as f32) * 100f32)))
            }
            None => None,
            _ => return Err(anyhow!("Got invalid pitch from profile: {:?}", self.pitch)),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum VoiceConfig {
    Voice { voice: String },
    Locale { locale: String },
}

impl VoiceConfig {
    pub fn try_as_str(&self) -> color_eyre::Result<&str> {
        Ok(match self {
            VoiceConfig::Voice { voice } => voice.as_str(),
            VoiceConfig::Locale { locale } => get_default_voice_by_locale(locale)
                .ok_or_else(|| anyhow!("Could not find default voice for locale: {}", locale))?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputConfig {
    pub format: Option<AudioFormat>,
    pub container: Option<ContainerFormat>,
    pub quality: Option<i32>,
}
