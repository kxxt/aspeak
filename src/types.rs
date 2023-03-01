use std::borrow::Cow;

use clap::ValueEnum;

use reqwest::header::{HeaderName, HeaderValue};
use serde::Deserialize;
use strum::{self, EnumString};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

use crate::{get_default_voice_by_locale, AspeakError, QUALITY_MAP, QUALITY_RANGE_MAP};

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

#[derive(Debug, Clone)]
pub struct AuthOptions<'a> {
    pub endpoint: Cow<'a, str>,
    pub token: Option<Cow<'a, str>>,
    pub key: Option<Cow<'a, str>>,
    pub headers: Cow<'a, [(HeaderName, HeaderValue)]>,
}

#[derive(Debug, Clone)]
pub struct TextOptions<'a> {
    pub voice: Cow<'a, str>,
    pub pitch: Option<Cow<'a, str>>,
    pub rate: Option<Cow<'a, str>>,
    pub style: Option<Cow<'a, str>>,
    pub role: Option<Role>,
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
