use std::{borrow::Cow, error::Error};

use aspeak::{
    get_endpoint_by_region, AspeakError, AudioFormat, AuthOptions, Role, TextOptions,
    DEFAULT_ENDPOINT,
};
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use phf::phf_map;
use reqwest::header::{HeaderName, HeaderValue};
use strum::AsRefStr;

#[derive(Parser, Debug)]
#[command(author, version,
    bin_name = "aspeak",
    about = "Try speech synthesis service(Powered by Azure Cognitive Services) in your terminal!", 
    long_about = None,
    after_help = "Attention: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes and the program will not report an error. Unreasonable high/low values for pitch and rate will be clipped to reasonable values by Azure Cognitive Services. Please refer to the documentation for other limitations at https://github.com/kxxt/aspeak/blob/main/README.md#limitations. By the way, we don\'t store your data, and Microsoft doesn\'t store your data according to information available on https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long, action = ArgAction::Count,
        help = "Log verbosity, -v for INFO, -vv for DEBUG, -vvv for TRACE")]
    verbose: u8,
    #[arg(long, action = ArgAction::SetTrue, help = "Do not use profile")]
    no_profile: bool,
    #[arg(long, conflicts_with = "no_profile", help = "The profile to use")]
    profile: Option<String>,
    #[command(flatten)]
    pub auth: AuthArgs,
}

impl Cli {
    pub(crate) fn log_level(&self) -> log::LevelFilter {
        match self.verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        }
    }
}

/// Parse a single key-value pair
fn parse_header(
    s: &str,
) -> Result<(HeaderName, HeaderValue), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((
        HeaderName::from_bytes(s[..pos].as_bytes())?,
        HeaderValue::from_str(&s[pos + 1..])?,
    ))
}

#[derive(Args, Debug, Clone)]
pub struct AuthArgs {
    #[arg(short, long, help = "Endpoint of TTS API")]
    pub endpoint: Option<String>,
    #[arg(
        short,
        long,
        help = "If you are using official endpoints, you can specify a region instead of full endpoint url",
        conflicts_with = "endpoint"
    )]
    pub region: Option<String>,
    #[arg(short, long, help = "Auth token for speech service")]
    pub token: Option<String>,
    #[arg(short, long, help = "Speech resource key")]
    pub key: Option<String>,
    #[arg(short = 'H', long,value_parser = parse_header, help = "Additional request headers")]
    pub headers: Vec<(HeaderName, HeaderValue)>,
}

impl<'a> TryInto<AuthOptions<'a>> for &'a AuthArgs {
    type Error = AspeakError;

    fn try_into(self) -> Result<AuthOptions<'a>, Self::Error> {
        Ok(AuthOptions {
            endpoint: self
                .endpoint
                .as_deref()
                .map(Cow::Borrowed)
                .or_else(|| {
                    self.region
                        .as_deref()
                        .map(get_endpoint_by_region)
                        .map(Cow::Owned)
                })
                .unwrap_or(Cow::Borrowed(DEFAULT_ENDPOINT)),
            token: self.token.as_deref().map(Cow::Borrowed),
            key: self.key.as_deref().map(Cow::Borrowed),
            headers: Cow::Borrowed(&self.headers),
        })
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum ContainerFormat {
    Mp3,
    Ogg,
    Webm,
    #[default]
    Wav,
}

#[derive(Args, Debug, Default)]
pub(crate) struct InputArgs {
    #[arg(short, long, help = "Text/SSML file to speak, default to `-`(stdin)")]
    pub file: Option<String>,
    #[arg(short, long, help = "Text/SSML file encoding")]
    pub encoding: Option<String>,
}

#[derive(Args, Debug, Default)]
pub(crate) struct OutputArgs {
    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,
    #[arg(
        short,
        long,
        allow_negative_numbers = true,
        help = "Output quality, default to 0. Run `aspeak list-qualities` to list available quality levels"
    )]
    pub quality: Option<i32>,
    #[arg(short, long)]
    pub container_format: Option<ContainerFormat>,
    #[arg(
        short = 'F',
        long,
        conflicts_with = "quality",
        conflicts_with = "container_format",
        hide_possible_values = true,
        help = "Set output audio format (experts only). Run `aspeak list-formats` to list available formats"
    )]
    pub format: Option<AudioFormat>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "List information of available voices, optionally filtered by locale/voice")]
    ListVoices {
        #[arg(
            short,
            long,
            conflicts_with = "locale",
            help = "Voice to list, default to all voices"
        )]
        voice: Option<String>,
        #[arg(short, long, help = "Locale to list, default to all locales")]
        locale: Option<String>,
    },
    #[command(about = "List available qualities for all container formats")]
    ListQualities,
    #[command(about = "List available formats (for experts)")]
    ListFormats,
    #[command(about = "Speak text")]
    Text {
        #[command(flatten)]
        text_args: TextArgs,
        #[command(flatten)]
        input_args: InputArgs,
        #[command(flatten)]
        output_args: OutputArgs,
    },
    #[command(about = "Speak SSML")]
    SSML {
        #[clap(help = "The SSML to speak. \
                    If neither SSML nor input file is specified, the SSML will be read from stdin. \
                    Do not include the document type definition in your SSML.")]
        ssml: Option<String>,
        #[command(flatten)]
        input_args: InputArgs,
        #[command(flatten)]
        output_args: OutputArgs,
    },
    #[command(about = "Configure settings of aspeak")]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

impl Default for Commands {
    fn default() -> Self {
        Self::Text {
            text_args: TextArgs::default(),
            input_args: InputArgs::default(),
            output_args: OutputArgs::default(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum ConfigCommand {
    #[command(about = "Open the default profile in your default editor")]
    Edit,
    #[command(about = "Initialize a new profile with default settings")]
    Init {
        #[arg(short, long, help = "Path to new profile, default to `~/.aspeak.toml`")]
        path: Option<String>,
        #[arg(long, action = ArgAction::SetTrue, help="Overwrite existing profile")]
        force: bool,
    },
    #[command(about = "Show full path to the default profile")]
    Where,
}

#[derive(Args, Debug, Default)]
pub(crate) struct TextArgs {
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

impl<'a> TryInto<TextOptions<'a>> for &'a TextArgs {
    type Error = AspeakError;

    fn try_into(self) -> Result<TextOptions<'a>, Self::Error> {
        Ok(TextOptions {
            text: self.text.as_deref().unwrap(),
            voice: self
                .voice
                .as_deref()
                .or_else(|| {
                    DEFAULT_VOICES
                        .get(self.locale.as_deref().unwrap_or("en-US"))
                        .map(|x| *x)
                })
                .unwrap(),
            pitch: self.pitch.as_deref(),
            rate: self.rate.as_deref(),
            style: self.style.as_deref(),
            role: self.role,
            style_degree: self.style_degree,
        })
    }
}

fn is_float(s: &str) -> bool {
    return s.parse::<f32>().is_ok();
}

pub(crate) fn parse_pitch(arg: &str) -> Result<String, String> {
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

pub(crate) fn parse_rate(arg: &str) -> Result<String, String> {
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
        if validate_style_degree(v) {
            Ok(v)
        } else {
            Err(format!("Value {v} out of range [0.01, 2]"))
        }
    } else {
        Err("Not a floating point number!".to_owned())
    }
}

pub(crate) fn validate_style_degree(degree: f32) -> bool {
    0.01f32 <= degree && degree <= 2.0f32
}

static DEFAULT_VOICES: phf::Map<&'static str, &'static str> = phf_map! {
    "af-ZA"=> "af-ZA-AdriNeural",
    "am-ET"=> "am-ET-AmehaNeural",
    "ar-AE"=> "ar-AE-FatimaNeural",
    "ar-BH"=> "ar-BH-AliNeural",
    "ar-DZ"=> "ar-DZ-AminaNeural",
    "ar-EG"=> "ar-EG-SalmaNeural",
    "ar-IQ"=> "ar-IQ-BasselNeural",
    "ar-JO"=> "ar-JO-SanaNeural",
    "ar-KW"=> "ar-KW-FahedNeural",
    "ar-LY"=> "ar-LY-ImanNeural",
    "ar-MA"=> "ar-MA-JamalNeural",
    "ar-QA"=> "ar-QA-AmalNeural",
    "ar-SA"=> "ar-SA-HamedNeural",
    "ar-SY"=> "ar-SY-AmanyNeural",
    "ar-TN"=> "ar-TN-HediNeural",
    "ar-YE"=> "ar-YE-MaryamNeural",
    "bg-BG"=> "bg-BG-BorislavNeural",
    "bn-BD"=> "bn-BD-NabanitaNeural",
    "bn-IN"=> "bn-IN-BashkarNeural",
    "ca-ES"=> "ca-ES-JoanaNeural",
    "cs-CZ"=> "cs-CZ-AntoninNeural",
    "cy-GB"=> "cy-GB-AledNeural",
    "da-DK"=> "da-DK-ChristelNeural",
    "de-AT"=> "de-AT-IngridNeural",
    "de-CH"=> "de-CH-JanNeural",
    "de-DE"=> "de-DE-KatjaNeural",
    "el-GR"=> "el-GR-AthinaNeural",
    "en-AU"=> "en-AU-NatashaNeural",
    "en-CA"=> "en-CA-ClaraNeural",
    "en-GB"=> "en-GB-LibbyNeural",
    "en-HK"=> "en-HK-SamNeural",
    "en-IE"=> "en-IE-ConnorNeural",
    "en-IN"=> "en-IN-NeerjaNeural",
    "en-KE"=> "en-KE-AsiliaNeural",
    "en-NG"=> "en-NG-AbeoNeural",
    "en-NZ"=> "en-NZ-MitchellNeural",
    "en-PH"=> "en-PH-JamesNeural",
    "en-SG"=> "en-SG-LunaNeural",
    "en-TZ"=> "en-TZ-ElimuNeural",
    "en-US"=> "en-US-JennyNeural",
    "en-ZA"=> "en-ZA-LeahNeural",
    "es-AR"=> "es-AR-ElenaNeural",
    "es-BO"=> "es-BO-MarceloNeural",
    "es-CL"=> "es-CL-CatalinaNeural",
    "es-CO"=> "es-CO-GonzaloNeural",
    "es-CR"=> "es-CR-JuanNeural",
    "es-CU"=> "es-CU-BelkysNeural",
    "es-DO"=> "es-DO-EmilioNeural",
    "es-EC"=> "es-EC-AndreaNeural",
    "es-ES"=> "es-ES-AlvaroNeural",
    "es-GQ"=> "es-GQ-JavierNeural",
    "es-GT"=> "es-GT-AndresNeural",
    "es-HN"=> "es-HN-CarlosNeural",
    "es-MX"=> "es-MX-DaliaNeural",
    "es-NI"=> "es-NI-FedericoNeural",
    "es-PA"=> "es-PA-MargaritaNeural",
    "es-PE"=> "es-PE-AlexNeural",
    "es-PR"=> "es-PR-KarinaNeural",
    "es-PY"=> "es-PY-MarioNeural",
    "es-SV"=> "es-SV-LorenaNeural",
    "es-US"=> "es-US-AlonsoNeural",
    "es-UY"=> "es-UY-MateoNeural",
    "es-VE"=> "es-VE-PaolaNeural",
    "et-EE"=> "et-EE-AnuNeural",
    "fa-IR"=> "fa-IR-DilaraNeural",
    "fi-FI"=> "fi-FI-SelmaNeural",
    "fil-PH"=> "fil-PH-AngeloNeural",
    "fr-BE"=> "fr-BE-CharlineNeural",
    "fr-CA"=> "fr-CA-SylvieNeural",
    "fr-CH"=> "fr-CH-ArianeNeural",
    "fr-FR"=> "fr-FR-DeniseNeural",
    "ga-IE"=> "ga-IE-ColmNeural",
    "gl-ES"=> "gl-ES-RoiNeural",
    "gu-IN"=> "gu-IN-DhwaniNeural",
    "he-IL"=> "he-IL-AvriNeural",
    "hi-IN"=> "hi-IN-MadhurNeural",
    "hr-HR"=> "hr-HR-GabrijelaNeural",
    "hu-HU"=> "hu-HU-NoemiNeural",
    "id-ID"=> "id-ID-ArdiNeural",
    "is-IS"=> "is-IS-GudrunNeural",
    "it-IT"=> "it-IT-IsabellaNeural",
    "ja-JP"=> "ja-JP-NanamiNeural",
    "jv-ID"=> "jv-ID-DimasNeural",
    "kk-KZ"=> "kk-KZ-AigulNeural",
    "km-KH"=> "km-KH-PisethNeural",
    "kn-IN"=> "kn-IN-GaganNeural",
    "ko-KR"=> "ko-KR-SunHiNeural",
    "lo-LA"=> "lo-LA-ChanthavongNeural",
    "lt-LT"=> "lt-LT-LeonasNeural",
    "lv-LV"=> "lv-LV-EveritaNeural",
    "mk-MK"=> "mk-MK-AleksandarNeural",
    "ml-IN"=> "ml-IN-MidhunNeural",
    "mr-IN"=> "mr-IN-AarohiNeural",
    "ms-MY"=> "ms-MY-OsmanNeural",
    "mt-MT"=> "mt-MT-GraceNeural",
    "my-MM"=> "my-MM-NilarNeural",
    "nb-NO"=> "nb-NO-PernilleNeural",
    "nl-BE"=> "nl-BE-ArnaudNeural",
    "nl-NL"=> "nl-NL-ColetteNeural",
    "pl-PL"=> "pl-PL-AgnieszkaNeural",
    "ps-AF"=> "ps-AF-GulNawazNeural",
    "pt-BR"=> "pt-BR-FranciscaNeural",
    "pt-PT"=> "pt-PT-DuarteNeural",
    "ro-RO"=> "ro-RO-AlinaNeural",
    "ru-RU"=> "ru-RU-SvetlanaNeural",
    "si-LK"=> "si-LK-SameeraNeural",
    "sk-SK"=> "sk-SK-LukasNeural",
    "sl-SI"=> "sl-SI-PetraNeural",
    "so-SO"=> "so-SO-MuuseNeural",
    "sr-RS"=> "sr-RS-NicholasNeural",
    "su-ID"=> "su-ID-JajangNeural",
    "sv-SE"=> "sv-SE-SofieNeural",
    "sw-KE"=> "sw-KE-RafikiNeural",
    "sw-TZ"=> "sw-TZ-DaudiNeural",
    "ta-IN"=> "ta-IN-PallaviNeural",
    "ta-LK"=> "ta-LK-KumarNeural",
    "ta-SG"=> "ta-SG-AnbuNeural",
    "te-IN"=> "te-IN-MohanNeural",
    "th-TH"=> "th-TH-PremwadeeNeural",
    "tr-TR"=> "tr-TR-AhmetNeural",
    "uk-UA"=> "uk-UA-OstapNeural",
    "ur-IN"=> "ur-IN-GulNeural",
    "ur-PK"=> "ur-PK-AsadNeural",
    "uz-UZ"=> "uz-UZ-MadinaNeural",
    "vi-VN"=> "vi-VN-HoaiMyNeural",
    "zh-CN"=> "zh-CN-XiaoxiaoNeural",
    "zh-HK"=> "zh-HK-HiuMaanNeural",
    "zh-TW"=> "zh-TW-HsiaoChenNeural",
    "zu-ZA"=> "zu-ZA-ThandoNeural",
};
