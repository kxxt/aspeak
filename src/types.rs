use std::borrow::Cow;

use clap::ValueEnum;

use reqwest::header::{HeaderName, HeaderValue};
use strum::{self, EnumString};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, ValueEnum, IntoStaticStr)]
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

#[derive(Debug, Clone, Copy)]
pub struct TextOptions<'a> {
    pub text: &'a str,
    pub voice: &'a str,
    pub pitch: Option<&'a str>,
    pub rate: Option<&'a str>,
    pub style: Option<&'a str>,
    pub role: Option<Role>,
    pub style_degree: Option<f32>,
}

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Debug, Clone, Copy, Default, IntoStaticStr, EnumString, EnumIter)]
#[non_exhaustive]
pub enum AudioFormat {
    #[strum(to_string = "amr-wb-16000hz")]
    AmrWb16000Hz,
    #[strum(to_string = "audio-16khz-128kbitrate-mono-mp3")]
    Audio16Khz128KBitRateMonoMp3,
    #[strum(to_string = "audio-16khz-16bit-32kbps-mono-opus")]
    Audio16Khz16Bit32KbpsMonoOpus,
    #[strum(to_string = "audio-16khz-32kbitrate-mono-mp3")]
    Audio16Khz32KBitRateMonoMp3,
    #[strum(to_string = "audio-16khz-64kbitrate-mono-mp3")]
    Audio16Khz64KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-160kbitrate-mono-mp3")]
    Audio24Khz160KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-16bit-24kbps-mono-opus")]
    Audio24Khz16Bit24KbpsMonoOpus,
    #[strum(to_string = "audio-24khz-16bit-48kbps-mono-opus")]
    Audio24Khz16Bit48KbpsMonoOpus,
    #[strum(to_string = "audio-24khz-48kbitrate-mono-mp3")]
    Audio24Khz48KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-96kbitrate-mono-mp3")]
    Audio24Khz96KBitRateMonoMp3,
    #[strum(to_string = "audio-48khz-192kbitrate-mono-mp3")]
    Audio48Khz192KBitRateMonoMp3,
    #[strum(to_string = "audio-48khz-96kbitrate-mono-mp3")]
    Audio48Khz96KBitRateMonoMp3,
    #[strum(to_string = "ogg-16khz-16bit-mono-opus")]
    Ogg16Khz16BitMonoOpus,
    #[strum(to_string = "ogg-24khz-16bit-mono-opus")]
    Ogg24Khz16BitMonoOpus,
    #[strum(to_string = "ogg-48khz-16bit-mono-opus")]
    Ogg48Khz16BitMonoOpus,
    #[strum(to_string = "raw-16khz-16bit-mono-pcm")]
    Raw16Khz16BitMonoPcm,
    #[strum(to_string = "raw-16khz-16bit-mono-truesilk")]
    Raw16Khz16BitMonoTrueSilk,
    #[strum(to_string = "raw-22050hz-16bit-mono-pcm")]
    Raw22050Hz16BitMonoPcm,
    #[strum(to_string = "raw-24khz-16bit-mono-pcm")]
    Raw24Khz16BitMonoPcm,
    #[strum(to_string = "raw-24khz-16bit-mono-truesilk")]
    Raw24Khz16BitMonoTrueSilk,
    #[strum(to_string = "raw-44100hz-16bit-mono-pcm")]
    Raw44100Hz16BitMonoPcm,
    #[strum(to_string = "raw-48khz-16bit-mono-pcm")]
    Raw48Khz16BitMonoPcm,
    #[strum(to_string = "raw-8khz-16bit-mono-pcm")]
    Raw8Khz16BitMonoPcm,
    #[strum(to_string = "raw-8khz-8bit-mono-alaw")]
    Raw8Khz8BitMonoALaw,
    #[strum(to_string = "raw-8khz-8bit-mono-mulaw")]
    Raw8Khz8BitMonoMULaw,
    #[strum(to_string = "riff-16khz-16bit-mono-pcm")]
    Riff16Khz16BitMonoPcm,
    #[strum(to_string = "riff-22050hz-16bit-mono-pcm")]
    Riff22050Hz16BitMonoPcm,
    #[default]
    #[strum(to_string = "riff-24khz-16bit-mono-pcm")]
    Riff24Khz16BitMonoPcm,
    #[strum(to_string = "riff-44100hz-16bit-mono-pcm")]
    Riff44100Hz16BitMonoPcm,
    #[strum(to_string = "riff-48khz-16bit-mono-pcm")]
    Riff48Khz16BitMonoPcm,
    #[strum(to_string = "riff-8khz-16bit-mono-pcm")]
    Riff8Khz16BitMonoPcm,
    #[strum(to_string = "riff-8khz-8bit-mono-alow")]
    Riff8Khz8BitMonoALaw,
    #[strum(to_string = "riff-8khz-8bit-mono-mulaw")]
    Riff8Khz8BitMonoMULaw,
    #[strum(to_string = "webm-16khz-16bit-mono-opus")]
    Webm16Khz16BitMonoOpus,
    #[strum(to_string = "webm-24khz-16bit-24kbps-mono-opus")]
    Webm24Khz16Bit24KbpsMonoOpus,
    #[strum(to_string = "webm-24khz-16bit-mono-opus")]
    Webm24Khz16BitMonoOpus,
}

/// We can't derive `ValueEnum` for `AudioFormat`
/// because we need to use the strum's string representation,
/// which is not supported by clap for now.
impl ValueEnum for AudioFormat {
    fn value_variants<'a>() -> &'a [Self] {
        // It's fine to leak it,
        // because otherwise we need to store it as a static/const variable
        AudioFormat::iter().collect::<Vec<_>>().leak()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(Into::<&str>::into(self)))
    }
}

#[cfg(feature = "python")]
pub(crate) fn register_python_items(
    _py: pyo3::Python<'_>,
    m: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    m.add_class::<AudioFormat>()?;
    m.add_class::<Role>()?;
    // m.add_class::<TextOptions>()?;
    Ok(())
}
