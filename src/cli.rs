use aspeak::{AudioFormat, TextOptions};
use clap::{Args, Parser, Subcommand, ValueEnum};
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
    #[arg(short, long,
        default_value_t = String::from("eastus.api.speech.microsoft.com"), 
        help = "Endpoint of Azure Cognitive Services")]
    pub endpoint: String,
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

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "List information of available voices, optionally filter by locale/voice")]
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
    #[command(about = "List available qualities and formats")]
    ListQualitiesAndFormats,
    #[command(about = "Speak text")]
    Text {
        #[command(flatten)]
        text_options: TextOptions,
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
}
