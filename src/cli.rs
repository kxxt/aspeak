use clap::{ArgAction, Parser};

use self::{
    args::{AuthArgs, InputArgs, ProfileArgs, TextArgs},
    commands::Command,
    config::TextConfig,
};
use aspeak::{
    callback_play_blocking, get_default_voice_by_locale, AspeakError, AudioFormat, Result,
    TextOptions,
};
use std::{
    borrow::Cow,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

use color_eyre::{eyre::anyhow, Help};
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod config;
mod parse;

#[derive(Parser, Debug)]
#[command(author, version,
    bin_name = "aspeak",
    about = "Try speech synthesis service(Powered by Azure Cognitive Services) in your terminal!", 
    long_about = None,
    after_help = "Please refer to the documentation for limitations at https://github.com/kxxt/aspeak/blob/main/README.md#limitations."
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
    fn log_level_by_verbosity(verbosity: u8) -> log::LevelFilter {
        match verbosity {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        }
    }

    pub(crate) fn get_log_level(&self, verbosity_config: Option<u8>) -> log::LevelFilter {
        match self.verbose {
            0 => verbosity_config
                .map(Self::log_level_by_verbosity)
                .unwrap_or(log::LevelFilter::Warn),
            v => Self::log_level_by_verbosity(v),
        }
    }

    pub(crate) fn process_input_text(args: &InputArgs) -> color_eyre::Result<String> {
        let mut s = String::new();

        let file: Box<dyn io::Read> = if let Some(file) = &args.file {
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

    pub(crate) fn process_text_options<'a>(
        args: &'a TextArgs,
        config: Option<&'a TextConfig>,
    ) -> color_eyre::Result<TextOptions<'a>> {
        Ok(TextOptions {
            text: args.text.as_deref().unwrap(),
            voice: Cow::Borrowed(
                match (args.voice.as_deref(), args.locale.as_deref(), &config) {
                    (Some(voice), _, _) => voice,
                    (None, Some(locale), _) => get_default_voice_by_locale(locale)?,
                    (None, None, config) => config
                        .map(|c| c.voice.as_ref().map(|v| v.try_as_str()).transpose())
                        .transpose()?
                        .flatten()
                        .unwrap_or_else(|| get_default_voice_by_locale("en-US").unwrap()),
                },
            ),
            pitch: {
                if let Some(pitch) = args.pitch.as_deref().map(Cow::Borrowed) {
                    Some(pitch)
                } else {
                    config
                        .map(|c| c.pitch())
                        .transpose()
                        .map_err(|e| anyhow!(e))?
                        .flatten()
                }
            },
            rate: {
                if let Some(rate) = args.rate.as_deref().map(Cow::Borrowed) {
                    Some(rate)
                } else {
                    config
                        .map(|c| c.rate())
                        .transpose()
                        .map_err(|e| anyhow!(e))?
                        .flatten()
                }
            },
            style: args
                .style
                .as_deref()
                .or_else(|| config.and_then(|c| c.style.as_deref()))
                .map(Cow::Borrowed),
            role: args.role.or_else(|| config.and_then(|c| c.role)),
            style_degree: args
                .style_degree
                .or_else(|| config.and_then(|c| c.style_degree)),
        })
    }
}
