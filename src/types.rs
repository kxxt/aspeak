use clap::{Args, ValueEnum};
use strum;
use strum::IntoStaticStr;

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

#[derive(Args, Debug, Default)]
pub struct TextOptions {
    #[clap(help = "The text to speak. \
                If neither text nor input file is specified, the text will be read from stdin.")]
    pub text: Option<String>,
    #[arg(short, long, value_parser = parse_pitch,
        help="Set pitch, default to 0. \
              Valid values include floats(will be converted to percentages), \
              percentages such as 20% and -10%, absolute values like 300Hz, \
              and relative values like -20Hz, +2st and string values like x-low. \
              See the documentation for more details.")]
    pub pitch: Option<String>,
    #[arg(short, long, value_parser = parse_rate,
        help=r#"Set speech rate, default to 0. \
                Valid values include floats(will be converted to percentages), \
                percentages like -20%%, floats with postfix "f" \
                (e.g. 2f means doubling the default speech rate), \
                and string values like x-slow. See the documentation for more details."# )]
    pub rate: Option<String>,
    #[arg(short = 'S', long, help = r#"Set speech style, default to "general""#)]
    pub style: Option<String>,
    #[arg(short = 'R', long)]
    pub role: Option<Role>,
    #[arg(
        short = 'd',
        long,
        value_parser = parse_style_degree,
        help = "Specifies the intensity of the speaking style. This only works for some Chinese voices!"
    )]
    pub style_degree: Option<f32>,
    #[arg(short, long, conflicts_with = "locale", help = "Voice to use")]
    pub voice: Option<String>,
    #[arg(short, long, help = "Locale to use, default to en-US")]
    pub locale: Option<String>,
}

fn is_float(s: &str) -> bool {
    return s.parse::<f32>().is_ok();
}

fn parse_pitch(arg: &str) -> Result<String, String> {
    if (arg.ends_with("Hz") && is_float(&arg[..arg.len() - 2]))
        || (arg.ends_with("%") && is_float(&arg[..arg.len() - 1]))
        || (arg.ends_with("st")
            && (arg.starts_with('+') || arg.starts_with('-'))
            && is_float(&arg[..arg.len() - 2]))
        || ["default", "x-low", "low", "medium", "high", "x-high"].contains(&arg)
    {
        Ok(arg.to_owned())
    } else if let Ok(v) = arg.parse::<f32>() {
        // float values that will be converted to percentages
        Ok(format!("{:.2}", v * 100f32))
    } else {
        Err(format!(
            "Please read the documentation for possible values of pitch."
        ))
    }
}

fn parse_rate(arg: &str) -> Result<String, String> {
    if (arg.ends_with("%") && is_float(&arg[..arg.len() - 1]))
        || ["default", "x-slow", "slow", "medium", "fast", "x-fast"].contains(&arg)
    {
        Ok(arg.to_owned())
    } else if arg.ends_with('f') && is_float(&arg[..arg.len() - 1]) {
        // raw float
        Ok(arg[..arg.len() - 1].to_owned())
    } else if let Ok(v) = arg.parse::<f32>() {
        // float values that will be converted to percentages
        Ok(format!("{:.2}", v * 100f32))
    } else {
        Err(format!(
            "Please read the documentation for possible values of rate."
        ))
    }
}

fn parse_style_degree(arg: &str) -> Result<f32, String> {
    if let Ok(v) = arg.parse::<f32>() {
        if 0.01f32 <= v && v <= 2.0f32 {
            Ok(v)
        } else {
            Err(format!("Value {v} out of range [0.01, 2]"))
        }
    } else {
        Err("Not a floating point number!".to_owned())
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, IntoStaticStr)]
#[clap(rename_all = "verbatim")]
#[non_exhaustive]
pub enum AudioFormat {
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
