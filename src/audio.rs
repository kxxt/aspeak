use clap::ValueEnum;
use phf::phf_map;
use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::AspeakError;

pub type QualityMap = phf::Map<i8, AudioFormat>;

static WAV_QUALITY_MAP: QualityMap = phf_map! {
    -2i8 => AudioFormat::Riff8Khz16BitMonoPcm,
    -1i8 => AudioFormat::Riff16Khz16BitMonoPcm,
    0i8  => AudioFormat::Riff24Khz16BitMonoPcm,
    1i8  => AudioFormat::Riff24Khz16BitMonoPcm,
};

static MP3_QUALITY_MAP: QualityMap = phf_map! {
    -4i8 => AudioFormat::Audio16Khz32KBitRateMonoMp3,
    -3i8 => AudioFormat::Audio16Khz64KBitRateMonoMp3,
    -2i8 => AudioFormat::Audio16Khz128KBitRateMonoMp3,
    -1i8 => AudioFormat::Audio24Khz48KBitRateMonoMp3,
    0i8  => AudioFormat::Audio24Khz96KBitRateMonoMp3,
    1i8  => AudioFormat::Audio24Khz160KBitRateMonoMp3,
    2i8  => AudioFormat::Audio48Khz96KBitRateMonoMp3,
    3i8  => AudioFormat::Audio48Khz192KBitRateMonoMp3,
};

static OGG_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Ogg16Khz16BitMonoOpus,
    0i8  => AudioFormat::Ogg24Khz16BitMonoOpus,
    1i8  => AudioFormat::Ogg48Khz16BitMonoOpus,
};

static WEBM_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Webm16Khz16BitMonoOpus,
    0i8  => AudioFormat::Webm24Khz16BitMonoOpus,
    1i8  => AudioFormat::Webm24Khz16Bit24KbpsMonoOpus,
};

pub static QUALITY_MAP: phf::Map<&'static str, &'static QualityMap> = phf_map! {
    "wav" => &WAV_QUALITY_MAP,
    "mp3" => &MP3_QUALITY_MAP,
    "ogg" => &OGG_QUALITY_MAP,
    "webm" => &WEBM_QUALITY_MAP,
};

pub static QUALITY_RANGE_MAP: phf::Map<&'static str, (i8, i8)> = phf_map! {
    "wav" => (-2, 1),
    "mp3" => (-4, 3),
    "ogg" => (-1, 1),
    "webm" => (-1, 1),
};

#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(Debug, Clone, Copy, Default, IntoStaticStr, EnumString, EnumIter, Deserialize)]
#[non_exhaustive]
pub enum AudioFormat {
    // I don't know if there are better ways to do this.
    // https://github.com/Peternator7/strum/issues/113
    #[strum(to_string = "amr-wb-16000hz")]
    #[serde(rename = "amr-wb-16000hz")]
    AmrWb16000Hz,
    #[strum(to_string = "audio-16khz-128kbitrate-mono-mp3")]
    #[serde(rename = "audio-16khz-128kbitrate-mono-mp3")]
    Audio16Khz128KBitRateMonoMp3,
    #[strum(to_string = "audio-16khz-16bit-32kbps-mono-opus")]
    #[serde(rename = "audio-16khz-16bit-32kbps-mono-opus")]
    Audio16Khz16Bit32KbpsMonoOpus,
    #[strum(to_string = "audio-16khz-32kbitrate-mono-mp3")]
    #[serde(rename = "audio-16khz-32kbitrate-mono-mp3")]
    Audio16Khz32KBitRateMonoMp3,
    #[strum(to_string = "audio-16khz-64kbitrate-mono-mp3")]
    #[serde(rename = "audio-16khz-64kbitrate-mono-mp3")]
    Audio16Khz64KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-160kbitrate-mono-mp3")]
    #[serde(rename = "audio-24khz-160kbitrate-mono-mp3")]
    Audio24Khz160KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-16bit-24kbps-mono-opus")]
    #[serde(rename = "audio-24khz-16bit-24kbps-mono-opus")]
    Audio24Khz16Bit24KbpsMonoOpus,
    #[strum(to_string = "audio-24khz-16bit-48kbps-mono-opus")]
    #[serde(rename = "audio-24khz-16bit-48kbps-mono-opus")]
    Audio24Khz16Bit48KbpsMonoOpus,
    #[strum(to_string = "audio-24khz-48kbitrate-mono-mp3")]
    #[serde(rename = "audio-24khz-48kbitrate-mono-mp3")]
    Audio24Khz48KBitRateMonoMp3,
    #[strum(to_string = "audio-24khz-96kbitrate-mono-mp3")]
    #[serde(rename = "audio-24khz-96kbitrate-mono-mp3")]
    Audio24Khz96KBitRateMonoMp3,
    #[strum(to_string = "audio-48khz-192kbitrate-mono-mp3")]
    #[serde(rename = "audio-48khz-192kbitrate-mono-mp3")]
    Audio48Khz192KBitRateMonoMp3,
    #[strum(to_string = "audio-48khz-96kbitrate-mono-mp3")]
    #[serde(rename = "audio-48khz-96kbitrate-mono-mp3")]
    Audio48Khz96KBitRateMonoMp3,
    #[strum(to_string = "ogg-16khz-16bit-mono-opus")]
    #[serde(rename = "ogg-16khz-16bit-mono-opus")]
    Ogg16Khz16BitMonoOpus,
    #[strum(to_string = "ogg-24khz-16bit-mono-opus")]
    #[serde(rename = "ogg-24khz-16bit-mono-opus")]
    Ogg24Khz16BitMonoOpus,
    #[strum(to_string = "ogg-48khz-16bit-mono-opus")]
    #[serde(rename = "ogg-48khz-16bit-mono-opus")]
    Ogg48Khz16BitMonoOpus,
    #[strum(to_string = "raw-16khz-16bit-mono-pcm")]
    #[serde(rename = "raw-16khz-16bit-mono-pcm")]
    Raw16Khz16BitMonoPcm,
    #[strum(to_string = "raw-16khz-16bit-mono-truesilk")]
    #[serde(rename = "raw-16khz-16bit-mono-truesilk")]
    Raw16Khz16BitMonoTrueSilk,
    #[strum(to_string = "raw-22050hz-16bit-mono-pcm")]
    #[serde(rename = "raw-22050hz-16bit-mono-pcm")]
    Raw22050Hz16BitMonoPcm,
    #[strum(to_string = "raw-24khz-16bit-mono-pcm")]
    #[serde(rename = "raw-24khz-16bit-mono-pcm")]
    Raw24Khz16BitMonoPcm,
    #[strum(to_string = "raw-24khz-16bit-mono-truesilk")]
    #[serde(rename = "raw-24khz-16bit-mono-truesilk")]
    Raw24Khz16BitMonoTrueSilk,
    #[strum(to_string = "raw-44100hz-16bit-mono-pcm")]
    #[serde(rename = "raw-44100hz-16bit-mono-pcm")]
    Raw44100Hz16BitMonoPcm,
    #[strum(to_string = "raw-48khz-16bit-mono-pcm")]
    #[serde(rename = "raw-48khz-16bit-mono-pcm")]
    Raw48Khz16BitMonoPcm,
    #[strum(to_string = "raw-8khz-16bit-mono-pcm")]
    #[serde(rename = "raw-8khz-16bit-mono-pcm")]
    Raw8Khz16BitMonoPcm,
    #[strum(to_string = "raw-8khz-8bit-mono-alaw")]
    #[serde(rename = "raw-8khz-8bit-mono-alaw")]
    Raw8Khz8BitMonoALaw,
    #[strum(to_string = "raw-8khz-8bit-mono-mulaw")]
    #[serde(rename = "raw-8khz-8bit-mono-mulaw")]
    Raw8Khz8BitMonoMULaw,
    #[strum(to_string = "riff-16khz-16bit-mono-pcm")]
    #[serde(rename = "riff-16khz-16bit-mono-pcm")]
    Riff16Khz16BitMonoPcm,
    #[strum(to_string = "riff-22050hz-16bit-mono-pcm")]
    #[serde(rename = "riff-22050hz-16bit-mono-pcm")]
    Riff22050Hz16BitMonoPcm,
    #[default]
    #[strum(to_string = "riff-24khz-16bit-mono-pcm")]
    #[serde(rename = "riff-24khz-16bit-mono-pcm")]
    Riff24Khz16BitMonoPcm,
    #[strum(to_string = "riff-44100hz-16bit-mono-pcm")]
    #[serde(rename = "riff-44100hz-16bit-mono-pcm")]
    Riff44100Hz16BitMonoPcm,
    #[strum(to_string = "riff-48khz-16bit-mono-pcm")]
    #[serde(rename = "riff-48khz-16bit-mono-pcm")]
    Riff48Khz16BitMonoPcm,
    #[strum(to_string = "riff-8khz-16bit-mono-pcm")]
    #[serde(rename = "riff-8khz-16bit-mono-pcm")]
    Riff8Khz16BitMonoPcm,
    #[strum(to_string = "riff-8khz-8bit-mono-alow")]
    #[serde(rename = "riff-8khz-8bit-mono-alow")]
    Riff8Khz8BitMonoALaw,
    #[strum(to_string = "riff-8khz-8bit-mono-mulaw")]
    #[serde(rename = "riff-8khz-8bit-mono-mulaw")]
    Riff8Khz8BitMonoMULaw,
    #[strum(to_string = "webm-16khz-16bit-mono-opus")]
    #[serde(rename = "webm-16khz-16bit-mono-opus")]
    Webm16Khz16BitMonoOpus,
    #[strum(to_string = "webm-24khz-16bit-24kbps-mono-opus")]
    #[serde(rename = "webm-24khz-16bit-24kbps-mono-opus")]
    Webm24Khz16Bit24KbpsMonoOpus,
    #[strum(to_string = "webm-24khz-16bit-mono-opus")]
    #[serde(rename = "webm-24khz-16bit-mono-opus")]
    Webm24Khz16BitMonoOpus,
}

impl AudioFormat {
    pub fn from_container_and_quality(
        container: &str,
        quality: i8,
        use_closest: bool,
    ) -> crate::Result<AudioFormat> {
        let map = QUALITY_MAP.get(container).ok_or_else(|| {
            AspeakError::ArgumentError(format!(
                "No quality map found for container: {container}. Please check if the container is correct."
            ))
        })?;
        if let Some(format) = map.get(&quality).copied() {
            Ok(format)
        } else if use_closest {
            let (min, max) = QUALITY_RANGE_MAP.get(container).unwrap();
            let closest = if quality < *min { *min } else { *max };
            Ok(*map.get(&closest).unwrap())
        } else {
            Err(AspeakError::ArgumentError(format!(
                        "Invalid quality found for container: {container} and quality: {quality}. Please check if the quality is correct."
            )))
        }
    }
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
    Ok(())
}
