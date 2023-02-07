use clap::{ArgAction, Parser};

use self::{
    args::{AuthArgs, InputArgs, ProfileArgs},
    commands::Command,
};
use aspeak::{callback_play_blocking, AspeakError, AudioFormat, Result};
use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
};

use color_eyre::Help;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

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
    #[command(flatten)]
    pub profile: ProfileArgs,
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

    pub(crate) fn process_input(args: InputArgs) -> color_eyre::Result<String> {
        let mut s = String::new();

        let file: Box<dyn io::Read> = if let Some(file) = args.file {
            Box::new(File::open(&file)?)
        } else {
            Box::new(io::stdin())
        };
        let mut decoder = if let Some(encoding) = args.encoding.as_deref() {
            let encoding = encoding_rs::Encoding::for_label(encoding.as_bytes()).ok_or(
                AspeakError::ArgumentError(format!("Unsupported encoding: {}", encoding)),
            )?;
            DecodeReaderBytesBuilder::new()
                .encoding(Some(encoding))
                .build(file)
        } else {
            DecodeReaderBytes::new(file)
        };
        decoder.read_to_string(&mut s).with_note(|| {
            "It is possibly due to incorrect encoding. \
             Please specify an encoding for your file manually"
        })?;
        Ok(s)
    }

    pub(crate) fn process_output(
        _format: AudioFormat,
        output: Option<String>,
    ) -> Result<Box<dyn FnMut(Option<&[u8]>) -> Result<()>>> {
        Ok(if let Some(file) = output {
            // todo: file already exists?
            let file = File::create(file)?;
            let mut buf_writer = BufWriter::new(file);
            Box::new(move |data| {
                Ok(if let Some(data) = data {
                    buf_writer.write_all(data)?
                } else {
                    buf_writer.flush()?
                })
            })
        } else {
            callback_play_blocking()
        })
    }
}
