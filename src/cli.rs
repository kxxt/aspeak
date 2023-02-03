use clap::{Args, Parser, Subcommand, ValueEnum};
use strum::{AsRefStr, IntoStaticStr};

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

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub(crate) enum ContainerFormat {
    Mp3,
    Ogg,
    Webm,
    #[default]
    Wav,
}

#[derive(Args, Debug)]
pub(crate) struct InputArgs {
    #[arg(short, long)]
    pub file: Option<String>,
    #[arg(short, long)]
    pub encoding: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub quality: Option<i32>,
    #[arg(short, long)]
    pub container_format: Option<ContainerFormat>,
    #[arg(
        short = 'F',
        long,
        conflicts_with = "quality",
        conflicts_with = "container_format"
    )]
    pub format: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct CommonArgs {
    #[arg(short, long, conflicts_with = "locale")]
    pub voice: Option<String>,
    #[arg(short, long)]
    pub locale: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, ValueEnum, IntoStaticStr)]
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
            "Please read the documentation for possible values for pitch."
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
            "Please read the documentation for possible values for pitch."
        ))
    }
}

#[derive(Args, Debug)]
pub(crate) struct TextOptions {
    pub text: Option<String>,
    #[arg(short, long, value_parser = parse_pitch)]
    pub pitch: Option<String>,
    #[arg(short, long, value_parser = parse_rate)]
    pub rate: Option<String>,
    #[arg(short = 'S', long)]
    pub style: Option<String>,
    #[arg(short = 'R', long)]
    pub role: Option<Role>,
    #[arg(short = 'd', long)]
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
