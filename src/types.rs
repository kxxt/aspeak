use std::borrow::Cow;

use clap::ValueEnum;
use serde::Deserialize;
use strum::IntoStaticStr;

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

/// Options that are only available if rich ssml is enabled
#[derive(Debug, Clone, Default)]
pub struct RichSsmlOptions<'a> {
    /// Speech style
    pub(crate) style: Option<Cow<'a, str>>,
    /// Speech role
    pub(crate) role: Option<Role>,
    /// Speech style degree, which should be in range [0.01, 2]
    pub(crate) style_degree: Option<f32>,
}

impl<'a> RichSsmlOptions<'a> {
    /// Speech style
    pub fn style(&self) -> Option<&str> {
        self.style.as_deref()
    }
    /// Speech style
    pub fn style_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.style
    }
    /// Speech role
    pub fn role(&self) -> Option<Role> {
        self.role
    }
    /// Speech role
    pub fn role_mut(&mut self) -> &mut Option<Role> {
        &mut self.role
    }
    /// Speech style degree, which should be in range [0.01, 2]
    pub fn style_degree(&self) -> Option<f32> {
        self.style_degree
    }
    /// Speech style degree, which should be in range [0.01, 2]
    pub fn style_degree_mut(&mut self) -> &mut Option<f32> {
        &mut self.style_degree
    }
    /// Create a builder for [`RichSsmlOptions`]
    pub fn builder() -> RichSsmlOptionsBuilder<'a> {
        RichSsmlOptionsBuilder::new()
    }
}

/// Builder for [`RichSsmlOptions`]
#[derive(Default)]
pub struct RichSsmlOptionsBuilder<'a> {
    style: Option<Cow<'a, str>>,
    role: Option<Role>,
    style_degree: Option<f32>,
}

impl<'a> RichSsmlOptionsBuilder<'a> {
    /// Create a new builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Speech style
    pub fn style(mut self, style: impl Into<Cow<'a, str>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Speech style
    pub fn optional_style(mut self, style: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.style = style.map(|s| s.into());
        self
    }

    /// Speech role
    pub fn role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    /// Speech role
    pub fn optional_role(mut self, role: Option<Role>) -> Self {
        self.role = role;
        self
    }

    /// Speech style degree, which should be in range [0.01, 2]
    pub fn style_degree(mut self, style_degree: f32) -> Self {
        self.style_degree = Some(style_degree);
        self
    }

    /// Speech style degree, which should be in range [0.01, 2]
    pub fn optional_style_degree(mut self, style_degree: Option<f32>) -> Self {
        self.style_degree = style_degree;
        self
    }

    /// Build [`RichSsmlOptions`]
    pub fn build(self) -> RichSsmlOptions<'a> {
        RichSsmlOptions {
            style: self.style,
            role: self.role,
            style_degree: self.style_degree,
        }
    }
}

/// Options for text-to-speech
#[derive(Debug, Clone)]
pub struct TextOptions<'a> {
    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub(crate) voice: Cow<'a, str>,
    /// Pitch string that will be inserted directly into SSML
    pub(crate) pitch: Option<Cow<'a, str>>,
    /// Rate string that will be inserted directly into SSML
    pub(crate) rate: Option<Cow<'a, str>>,
    /// Rich SSML options
    pub(crate) rich_ssml_options: Option<RichSsmlOptions<'a>>,
}

impl Default for TextOptions<'_> {
    fn default() -> Self {
        Self {
            voice: Cow::Borrowed(get_default_voice_by_locale("en-US").unwrap()),
            pitch: Default::default(),
            rate: Default::default(),
            rich_ssml_options: Default::default(),
        }
    }
}

impl<'a> TextOptions<'a> {
    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub fn voice(&self) -> &str {
        &self.voice
    }

    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub fn voice_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.voice
    }

    /// Pitch string that will be inserted directly into SSML
    pub fn pitch(&self) -> Option<&str> {
        self.pitch.as_deref()
    }

    /// Pitch string that will be inserted directly into SSML
    pub fn pitch_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.pitch
    }

    /// Rate string that will be inserted directly into SSML
    pub fn rate(&self) -> Option<&str> {
        self.rate.as_deref()
    }

    /// Rate string that will be inserted directly into SSML
    pub fn rate_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.rate
    }

    /// Rich SSML options
    pub fn rich_ssml_options(&self) -> &Option<RichSsmlOptions> {
        &self.rich_ssml_options
    }

    /// Rich SSML options
    pub fn rich_ssml_options_mut(&mut self) -> &mut Option<RichSsmlOptions<'a>> {
        &mut self.rich_ssml_options
    }

    /// Create a builder for [`TextOptions`]
    pub fn builder() -> TextOptionsBuilder<'a> {
        TextOptionsBuilder::new()
    }
}

/// Builder for [`TextOptions`]
#[derive(Default)]
pub struct TextOptionsBuilder<'a> {
    voice: Option<Cow<'a, str>>,
    pitch: Option<Cow<'a, str>>,
    rate: Option<Cow<'a, str>>,
    rich_ssml_options: Option<RichSsmlOptions<'a>>,
}

impl<'a> TextOptionsBuilder<'a> {
    /// Create a new builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub fn voice(mut self, voice: impl Into<Cow<'a, str>>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// Voice identifier. It should be in the format of `locale-voice_name` like `en-US-JennyNeural`.
    pub fn optional_voice(mut self, voice: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.voice = voice.map(|v| v.into());
        self
    }

    /// Pitch string that will be inserted directly into SSML
    pub fn pitch(mut self, pitch: impl Into<Cow<'a, str>>) -> Self {
        self.pitch = Some(pitch.into());
        self
    }

    /// Pitch string that will be inserted directly into SSML
    pub fn optional_pitch(mut self, pitch: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.pitch = pitch.map(|p| p.into());
        self
    }

    /// Rate string that will be inserted directly into SSML
    pub fn rate(mut self, rate: impl Into<Cow<'a, str>>) -> Self {
        self.rate = Some(rate.into());
        self
    }

    /// Rate string that will be inserted directly into SSML
    pub fn optional_rate(mut self, rate: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.rate = rate.map(|r| r.into());
        self
    }

    /// Rich SSML options
    pub fn rich_ssml_options(mut self, rich_ssml_options: RichSsmlOptions<'a>) -> Self {
        self.rich_ssml_options = Some(rich_ssml_options);
        self
    }

    /// Rich SSML options
    pub fn optional_rich_ssml_options(
        mut self,
        rich_ssml_options: Option<RichSsmlOptions<'a>>,
    ) -> Self {
        self.rich_ssml_options = rich_ssml_options;
        self
    }

    /// Set the rich SSML options to the build result of a [`RichSsmlOptionsBuilder`].
    pub fn chain_rich_ssml_options_builder(
        mut self,
        rich_ssml_options_builder: RichSsmlOptionsBuilder<'a>,
    ) -> Self {
        self.rich_ssml_options = Some(rich_ssml_options_builder.build());
        self
    }

    /// Build the [`TextOptions`] from the builder
    pub fn build(self) -> TextOptions<'a> {
        TextOptions {
            voice: self.voice.unwrap_or_else(|| {
                Cow::Borrowed(
                    get_default_voice_by_locale("en-US").expect("No default voice for en-US!"),
                )
            }),
            pitch: self.pitch,
            rate: self.rate,
            rich_ssml_options: self.rich_ssml_options,
        }
    }
}

#[cfg(feature = "python")]
pub(crate) fn register_python_items(
    _py: pyo3::Python<'_>,
    m: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    m.add_class::<Role>()?;
    Ok(())
}
