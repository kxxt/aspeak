use crate::types::{AudioFormat, ContainerFormat, Role};
use clap::{Args, Parser, Subcommand};
use strum::{self};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version,
    bin_name = "aspeak",
    about = "Try speech synthesis service(Provided by Azure Cognitive Services) in your terminal!", 
    long_about = None,
    after_help = "Attention: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes and the program will not report an error. Unreasonable high/low values for pitch and rate will be clipped to reasonable values by Azure Cognitive Services. Please refer to the documentation for other limitations at https://github.com/kxxt/aspeak/blob/main/README.md#limitations. By the way, we don\'t store your data, and Microsoft doesn\'t store your data according to information available on https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long, default_value_t = String::from("eastus.api.speech.microsoft.com"))]
    pub endpoint: String,
}

#[derive(Args, Debug)]
pub(crate) struct InputArgs {
    #[arg(short, long, help = "Text/SSML file to speak, default to `-`(stdin)")]
    pub file: Option<String>,
    #[arg(
        short,
        long,
        help = r#"Text/SSML file encoding, default to "utf-8"(Not for stdin!)"#
    )]
    pub encoding: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,
    #[arg(short, long, help = "Output quality, default to 0")]
    pub quality: Option<i32>,
    #[arg(short, long)]
    pub container_format: Option<ContainerFormat>,
    #[arg(
        short = 'F',
        long,
        conflicts_with = "quality",
        conflicts_with = "container_format",
        help = "Set output audio format (experts only)"
    )]
    pub format: Option<AudioFormat>,
}

#[derive(Args, Debug)]
pub(crate) struct CommonArgs {
    #[arg(short, long, conflicts_with = "locale", help = "Voice to use")]
    pub voice: Option<String>,
    #[arg(short, long, help = "Locale to use, default to en-US")]
    pub locale: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct TextOptions {
    pub text: Option<String>,
    #[arg(short, long, value_parser = parse_pitch, help="Set pitch, default to 0. Valid values include floats(will be converted to percentages), percentages such as 20% and -10%, absolute values like 300Hz, and relative values like -20Hz, +2st and string values like x-low. See the documentation for more details.")]
    pub pitch: Option<String>,
    #[arg(short, long, value_parser = parse_rate, help =r#"Set speech rate, default to 0. Valid values include floats(will be converted to percentages), percentages like -20%%, floats with postfix "f" (e.g. 2f means doubling the default speech rate), and string values like x-slow. See the documentation for more details."# )]
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
    #[command(flatten)]
    pub common_args: CommonArgs,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    ListVoices {
        #[command(flatten)]
        common_args: CommonArgs,
    },
    ListQualitiesAndFormats,
    Text {
        #[command(flatten)]
        text_options: TextOptions,
        #[command(flatten)]
        input_args: InputArgs,
        #[command(flatten)]
        output_args: OutputArgs,
    },
    SSML {
        ssml: Option<String>,
        #[command(flatten)]
        input_args: InputArgs,
        #[command(flatten)]
        output_args: OutputArgs,
        #[command(flatten)]
        common_args: CommonArgs,
    },
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
