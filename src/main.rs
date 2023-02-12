mod cli;

use std::{path::PathBuf, borrow::Cow};

use cli::{commands::Command, Cli};

use aspeak::{
    AspeakError, AudioFormat, SynthesizerConfig, Voice, ORIGIN, QUALITY_MAP,
};
use clap::Parser;
use color_eyre::{eyre::anyhow, Help};
use colored::Colorize;

use log::debug;

use reqwest::header::{self, HeaderMap, HeaderValue};
use strum::IntoEnumIterator;
use tokio_tungstenite::tungstenite::{error::ProtocolError, Error as TungsteniteError};

use crate::cli::{commands::ConfigCommand, config::Config};

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let config = cli.profile.load_profile()?;
    env_logger::builder()
        .filter_level(cli.get_log_level(config.as_ref().and_then(|c| c.verbosity)))
        .init();
    debug!("Commandline args: {cli:?}");
    debug!("Profile: {config:?}");
    let Cli { command, auth, .. } = cli;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;
    rt.block_on(async {
        match command.unwrap_or_default() {
            Command::Ssml {
                ssml,
                input_args,
                output_args,
            } => {
                let ssml = ssml
                    .ok_or(AspeakError::InputError)
                    .or_else(|_| Cli::process_input_text(&input_args))?;
                let audio_format = output_args.get_audio_format(config.as_ref().and_then(|c|c.output.as_ref()))?;
                let callback = Cli::process_output(output_args.output, output_args.overwrite)?;
                let auth_options = auth.to_auth_options(config.as_ref().and_then(|c|c.auth.as_ref()))?;
                let synthesizer = SynthesizerConfig::new(auth_options, audio_format)
                    .connect()
                    .await?;
                synthesizer.synthesize_ssml(&ssml, callback).await?
            }
            Command::Text {
                text_args,
                input_args,
                output_args,
            } => {
                let text = 
                    text_args
                        .text.as_deref()
                        .map(Cow::Borrowed)
                        .ok_or(AspeakError::InputError)
                        .or_else(|_| Cli::process_input_text(&input_args).map(Cow::Owned))
                        ?;
                let audio_format = output_args.get_audio_format(config.as_ref().and_then(|c|c.output.as_ref()))?;
                let callback = Cli::process_output(output_args.output, output_args.overwrite)?;
                let auth_options = auth.to_auth_options(config.as_ref().and_then(|c|c.auth.as_ref()))?;
                let synthesizer = SynthesizerConfig::new(auth_options,audio_format)
                    .connect()
                    .await?;
                let options = &Cli::process_text_options(&text_args, config.as_ref().and_then(|c|c.text.as_ref()))?;
                let result = synthesizer.synthesize_text(text, options, callback).await;
                if let Err(AspeakError::WebSocketError(TungsteniteError::Protocol(
                    ProtocolError::ResetWithoutClosingHandshake,
                ))) = result
                {
                    return result.with_note(|| "This error usually indicates a poor internet connection or that the remote API terminates your service.")
                        .with_suggestion(|| "Retry if you are on a poor internet connection. \
                                             If this error persists and you are using the trial service, please shorten your input.");
                } else {
                    result?;
                }
            }
            Command::ListVoices {
                ref voice,
                ref locale,
            } => {
                let url = "https://eastus.api.speech.microsoft.com/cognitiveservices/voices/list";
                let headers =
                    HeaderMap::from_iter([(header::ORIGIN, HeaderValue::from_str(ORIGIN).unwrap())]);
                let client = reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .unwrap();
                let request = client.get(url).build()?;
                let voices = client.execute(request).await?.json::<Vec<Voice>>().await?;
                let voices = voices.iter();
                let locale_id = locale.as_deref();
                let voice_id = voice.as_deref();
                let voices: Box<dyn Iterator<Item = &Voice>> = {
                    if locale_id.is_some() {
                        Box::new(voices.filter(|voice| Some(voice.locale.as_str()) == locale_id))
                    } else if voice_id.is_some() {
                        Box::new(voices.filter(|voice| Some(voice.short_name.as_str()) == voice_id))
                    } else {
                        Box::new(voices)
                    }
                };
                for voice in voices {
                    println!("{voice}");
                }
            }
            Command::ListQualities => {
                for (container, qualities) in QUALITY_MAP.into_iter() {
                    println!(
                        "{} {}:",
                        "Qualities for".cyan(),
                        container.to_uppercase().cyan()
                    );
                    for (quality, format) in qualities.into_iter() {
                        println!("{:>3}: {}", quality, Into::<&str>::into(format));
                    }
                    println!()
                }
            }
            Command::ListFormats => {
                for format in AudioFormat::iter() {
                    println!("{}", Into::<&str>::into(format));
                }
            }
            Command::Config { command } => match command {
                ConfigCommand::Edit => {
                    let path = Config::default_location()?;
                    if !path.exists() {
                        Config::initialize(path.as_path(), false)?;
                    }
                    open::that(path)?;
                },
                ConfigCommand::Init { path, overwrite } => {
                    Config::initialize(
                        path.map(PathBuf::from)
                            .ok_or(anyhow!("Unreachable code!"))
                            .or_else(|_|
                                Config::default_location()
                            )?.as_path(), overwrite
                    )?;
                },
                ConfigCommand::Where => {
                    println!("{}", Config::default_location()?.display());
                }
            }
        }
        Ok(())
    })?;
    Ok(())
}
