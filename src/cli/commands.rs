use clap::{ArgAction, Subcommand};

use super::args::*;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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

impl Default for Command {
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
        overwrite: bool,
    },
    #[command(about = "Show full path to the default profile")]
    Where,
}
