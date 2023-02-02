use std::default;

use clap::{Arg, Args, Parser, Subcommand, ValueEnum};

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
    command: Option<Commands>,
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
    file: Option<String>,
    #[arg(short, long)]
    encoding: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    quality: Option<i32>,
    #[arg(short, long)]
    container_format: Option<ContainerFormat>,
    #[arg(
        short = 'F',
        long,
        conflicts_with = "quality",
        conflicts_with = "container_format"
    )]
    format: Option<String>,
}

#[derive(Args, Debug)]
pub(crate) struct CommonArgs {
    #[arg(short, long, conflicts_with = "locale")]
    voice: Option<String>,
    #[arg(short, long)]
    locale: Option<String>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    ListVoices,
    ListQualitiesAndFormats,
    Text {
        text: Option<String>,
        #[arg(short, long)]
        pitch: Option<String>,
        #[arg(short, long)]
        rate: Option<String>,
        #[arg(short = 'S', long)]
        style: Option<String>,
        #[arg(short = 'R', long)]
        role: Option<String>,
        #[arg(short = 'd', long)]
        style_degree: Option<f32>,
        #[command(flatten)]
        input_args: InputArgs,
        #[command(flatten)]
        output_args: OutputArgs,
        #[command(flatten)]
        common_args: CommonArgs,
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
