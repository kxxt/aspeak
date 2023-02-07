


use clap::{ArgAction, Parser};




use self::{args::AuthArgs, commands::Command};

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod config;

#[derive(Parser, Debug)]
#[command(author, version,
    bin_name = "aspeak",
    about = "Try speech synthesis service(Powered by Azure Cognitive Services) in your terminal!", 
    long_about = None,
    after_help = "Attention: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes and the program will not report an error. Unreasonable high/low values for pitch and rate will be clipped to reasonable values by Azure Cognitive Services. Please refer to the documentation for other limitations at https://github.com/kxxt/aspeak/blob/main/README.md#limitations. By the way, we don\'t store your data, and Microsoft doesn\'t store your data according to information available on https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
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
