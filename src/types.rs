use std::borrow::Cow;

use clap::ValueEnum;

use reqwest::header::{HeaderName, HeaderValue};
use serde::Deserialize;
use strum::IntoStaticStr;
use strum::{self};

use crate::get_default_voice_by_locale;

/// Speech role
#[cfg_attr(feature = "python", pyo3::pyclass)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, ValueEnum, IntoStaticStr, Deserialize)]
#[clap(rename_all = "verbatim")]
pub enum Role {
    Girl,
    Boy,
    YoungAdultFemale,
    YoungAdultMale,
    OlderAdultFemale,
    OlderAdultMale,
    SeniorFemale,
    SeniorMale,
}

/// Options for authentication
#[derive(Debug, Clone)]
pub struct AuthOptions<'a> {
    /// Endpoint of the service
    pub endpoint: Cow<'a, str>,
    /// Authentication token
    pub token: Option<Cow<'a, str>>,
    /// Azure Subscription Key for authentication. It currently doesn't work.
    pub key: Option<Cow<'a, str>>,
    /// Additional headers
    pub headers: Cow<'a, [(HeaderName, HeaderValue)]>,
}

/// Options for text-to-speech
#[derive(Debug, Clone)]
pub struct TextOptions<'a> {
    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub voice: Cow<'a, str>,
    /// Pitch string that will be inserted directly into SSML
    pub pitch: Option<Cow<'a, str>>,
    /// Rate string that will be inserted directly into SSML
    pub rate: Option<Cow<'a, str>>,
    /// Speech style
    pub style: Option<Cow<'a, str>>,
    /// Speech role
    pub role: Option<Role>,
    /// Speech style degree, which should be in range [0.01, 2]
    pub style_degree: Option<f32>,
}

impl Default for TextOptions<'_> {
    fn default() -> Self {
        Self {
            voice: Cow::Borrowed(get_default_voice_by_locale("en-US").unwrap()),
            pitch: Default::default(),
            rate: Default::default(),
            style: Default::default(),
            role: Default::default(),
            style_degree: Default::default(),
        }
    }
}

#[cfg(feature = "python")]
pub(crate) fn register_python_items(
    _py: pyo3::Python<'_>,
    m: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    m.add_class::<Role>()?;
    // m.add_class::<TextOptions>()?;
    Ok(())
}
