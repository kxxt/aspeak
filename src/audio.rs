use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[cfg(feature = "binary")]
use clap::ValueEnum;
use phf::phf_map;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

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

#[cfg(feature = "audio")]
mod internal {
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};

    use rodio::{decoder::DecoderError, PlayError, StreamError};
    use rodio::{Decoder, OutputStream, Sink};
    #[allow(unused)]
    pub fn play_borrowed_audio_blocking(buffer: &[u8]) -> Result<(), AudioError> {
        play_owned_audio_blocking(buffer.to_vec())
    }

    pub fn play_owned_audio_blocking(buffer: Vec<u8>) -> Result<(), AudioError> {
        log::info!("Playing audio... ({} bytes)", buffer.len());
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle).unwrap();
        let cursor = std::io::Cursor::new(buffer);
        let source = Decoder::new(cursor)?;
        sink.append(source);
        sink.sleep_until_end();
        log::debug!("Done playing audio");
        Ok(())
    }

    #[derive(Debug)]
    #[non_exhaustive]
    /// An error that can occur when trying to play audio
    ///
    /// Possible reasons include:
    /// - The audio decoder failed to decode the audio
    ///     - Bad audio data (e.g. not a valid audio file)
    ///     - Unsupported audio format
    /// - Audio stream error
    pub struct AudioError {
        pub kind: AudioErrorKind,
        source: Option<anyhow::Error>,
    }

    impl Display for AudioError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "audio {:?} error", self.kind)
        }
    }

    impl Error for AudioError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source.as_ref().map(|e| e.as_ref() as _)
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum AudioErrorKind {
        Decoder,
        Stream,
        #[allow(unused)]
        Play,
    }

    macro_rules! impl_from_for_audio_error {
        ($error_type:ident, $error_kind:ident) => {
            impl From<$error_type> for AudioError {
                fn from(e: $error_type) -> Self {
                    Self {
                        kind: AudioErrorKind::$error_kind,
                        source: Some(e.into()),
                    }
                }
            }
        };
    }

    impl_from_for_audio_error!(StreamError, Stream);
    impl_from_for_audio_error!(DecoderError, Decoder);
    impl_from_for_audio_error!(PlayError, Decoder);

    #[cfg(feature = "python")]
    impl From<AudioError> for pyo3::PyErr {
        fn from(value: AudioError) -> Self {
            pyo3::exceptions::PyOSError::new_err(format!("{:?}", color_eyre::Report::from(value)))
        }
    }
}

#[cfg(feature = "audio")]
pub use internal::*;

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

/// All possible audio formats
///
/// Some endpoints only support a subset of these formats.
#[cfg_attr(feature = "python", pyo3::pyclass)]
#[derive(
    Debug, Clone, Copy, Default, IntoStaticStr, EnumString, EnumIter, Deserialize, Serialize,
)]
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
    #[strum(to_string = "riff-8khz-8bit-mono-alaw")]
    #[serde(rename = "riff-8khz-8bit-mono-alaw")]
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
    /// Convert a container format and quality level into an [`AudioFormat`].
    ///
    /// If `use_closest` is `true`, then if the quality level is not supported
    /// by the container, the closest supported quality level will be used.
    pub fn from_container_and_quality(
        container: &str,
        quality: i8,
        use_closest: bool,
    ) -> Result<AudioFormat, AudioFormatParseError> {
        let map = QUALITY_MAP
            .get(container)
            .ok_or_else(|| AudioFormatParseError {
                kind: AudioFormatParseErrorKind::InvalidContainer(container.to_string()),
            })?;
        if let Some(format) = map.get(&quality).copied() {
            Ok(format)
        } else if use_closest {
            let (min, max) = QUALITY_RANGE_MAP.get(container).unwrap();
            let closest = if quality < *min { *min } else { *max };
            Ok(*map.get(&closest).unwrap())
        } else {
            Err(AudioFormatParseError {
                kind: AudioFormatParseErrorKind::InvalidQuality {
                    container: container.to_string(),
                    quality,
                },
            })
        }
    }
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl AudioFormat {
    #[new]
    #[pyo3(signature = (container = "mp3", quality = 0, use_closest = false))]
    fn py_from_container_and_quality(
        container: &str,
        quality: i8,
        use_closest: bool,
    ) -> Result<AudioFormat, AudioFormatParseError> {
        AudioFormat::from_container_and_quality(container, quality, use_closest)
    }
}

/// We can't derive `ValueEnum` for `AudioFormat`
/// because we need to use the strum's string representation,
/// which is not supported by clap for now.
#[cfg(feature = "binary")]
impl ValueEnum for AudioFormat {
    fn value_variants<'a>() -> &'a [Self] {
        use strum::IntoEnumIterator;
        // It's fine to leak it,
        // because otherwise we need to store it as a static/const variable
        AudioFormat::iter().collect::<Vec<_>>().leak()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(Into::<&str>::into(self)))
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// An error that can occur in [`AudioFormat::from_container_and_quality`].
pub struct AudioFormatParseError {
    pub kind: AudioFormatParseErrorKind,
}

impl Display for AudioFormatParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "audio format parse error: ")?;
        match &self.kind {
            AudioFormatParseErrorKind::InvalidContainer(container) => {
                write!(f, "invalid container format: {}", container)
            }
            AudioFormatParseErrorKind::InvalidQuality { container, quality } => {
                write!(f, "invalid quality {} for container {}", quality, container)
            }
        }
    }
}

impl Error for AudioFormatParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(feature = "python")]
impl From<AudioFormatParseError> for pyo3::PyErr {
    fn from(value: AudioFormatParseError) -> Self {
        pyo3::exceptions::PyOSError::new_err(format!("{:?}", color_eyre::Report::from(value)))
    }
}

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum AudioFormatParseErrorKind {
    InvalidContainer(String),
    InvalidQuality { container: String, quality: i8 },
}

#[cfg(feature = "python")]
pub(crate) fn register_python_items(
    _py: pyo3::Python<'_>,
    m: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    m.add_class::<AudioFormat>()?;
    Ok(())
}
