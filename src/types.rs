use clap::ValueEnum;
use strum;
use strum::IntoStaticStr;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub(crate) enum ContainerFormat {
    Mp3,
    Ogg,
    Webm,
    #[default]
    Wav,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, ValueEnum, IntoStaticStr)]
#[clap(rename_all = "verbatim")]
pub(crate) enum Role {
    Girl,
    Boy,
    YoungAdultFemale,
    YoungAdultMale,
    OlderAdultFemale,
    OlderAdultMale,
    SeniorFemale,
    SeniorMale,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, IntoStaticStr)]
#[clap(rename_all = "verbatim")]
#[non_exhaustive]
pub(crate) enum AudioFormat {
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
    #[strum(to_string = "raw-16khz-16bit-mono-pcm")]
    Riff16Khz16BitMonoPcm,
    #[strum(to_string = "raw-22050hz-16bit-mono-pcm")]
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
