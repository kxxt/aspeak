use clap::{ArgAction, Parser};
use log::{debug, info};
use rodio::{Decoder, OutputStream, Sink};

use self::{
    args::{AuthArgs, InputArgs, ProfileArgs, TextArgs},
    commands::Command,
    config::TextConfig,
};
use aspeak::{get_default_voice_by_locale, AspeakError, TextOptions};
use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    io::{self, Cursor, Read, Write},
    path::Path,
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
    about = "A simple text-to-speech client for Azure TTS API.", 
    long_about = None,
    after_help = "By default, we try to use a trial endpoint that doesn't require authentication\
                  But its availability is not guaranteed and its capability is restricted by Microsoft."
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

type OutputProcessor = Box<dyn FnOnce(&[u8]) -> color_eyre::Result<()> + Send>;

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

        let file: Box<dyn io::Read> = match args.file.as_deref() {
            Some(file) if file != "-" => Box::new(File::open(file)?),
            _ => Box::new(io::stdin()),
        };
        let mut decoder = if let Some(encoding) = args.encoding.as_deref() {
            let encoding = encoding_rs::Encoding::for_label(encoding.as_bytes()).ok_or(
                AspeakError::ArgumentError(format!("Unsupported encoding: {encoding}")),
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
        output: Option<String>,
        overwrite: bool,
    ) -> color_eyre::Result<OutputProcessor> {
        Ok(if let Some(file) = output.as_deref() {
            let file = Path::new(file);
            let mut file = match (file.exists(), overwrite) {
                (_, true) => File::create(file)?,
                (false, false) => OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(file)?,
                (true, false) => {
                    return Err(anyhow!("File {} already exists!", file.display())
                        .suggestion("You can use --overwrite to overwrite this file."))
                }
            };
            Box::new(move |buffer| {
                file.write_all(buffer)?;
                Ok(())
            })
        } else {
            Box::new(|buffer| {
                info!("Playing audio... ({} bytes)", buffer.len());
                let (_stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle).unwrap();
                let cursor = Cursor::new(Vec::from(buffer));
                let source = Decoder::new(cursor)?;
                sink.append(source);
                sink.sleep_until_end();
                debug!("Done playing audio");
                Ok(())
            })
        })
    }

    pub(crate) fn process_text_options<'a>(
        args: &'a TextArgs,
        config: Option<&'a TextConfig>,
    ) -> color_eyre::Result<TextOptions<'a>> {
        Ok(TextOptions {
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
