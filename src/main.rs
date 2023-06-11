mod cli;

use std::{borrow::Cow, error::Error, path::PathBuf};

use cli::{commands::Command, Cli};

use aspeak::{
    synthesizer::{SynthesizerConfig, WebsocketSynthesizerError, WebsocketSynthesizerErrorKind},
    voice::{VoiceListAPIAuth, VoiceListAPIEndpoint, VoiceListAPIError, VoiceListAPIErrorKind},
    AudioFormat, Voice, QUALITY_MAP,
};
use clap::Parser;
use color_eyre::{
    eyre::{anyhow, eyre},
    Help,
};
use colored::Colorize;

use env_logger::WriteStyle;
use log::debug;

use reqwest::header::HeaderMap;
use strum::IntoEnumIterator;
use tokio_tungstenite::tungstenite::{error::ProtocolError, Error as TungsteniteError};

use crate::cli::{
    args::Color,
    commands::ConfigCommand,
    config::{Config, EndpointConfig},
};

#[derive(Debug, thiserror::Error)]
enum CliError {
    #[error("No input text/SSML.")]
    InputError,
}

fn main() -> color_eyre::eyre::Result<()> {
    let mut cli = Cli::parse();
    if cli.color == Color::Auto && std::env::var_os("NO_COLOR").is_some() {
        // Respect NO_COLOR if --color=auto
        cli.color = Color::Never;
    }
    if cli.color == Color::Never {
        colored::control::set_override(false);
    } else {
        color_eyre::install()?;
    }
    let config = cli.profile.load_profile()?;
    env_logger::builder()
        .filter_level(cli.get_log_level(config.as_ref().and_then(|c| c.verbosity)))
        .write_style(match cli.color {
            Color::Auto => WriteStyle::Auto,
            Color::Never => WriteStyle::Never,
            Color::Always => WriteStyle::Always,
        })
        .init();
    debug!("Commandline args: {cli:?}");
    debug!("Profile: {config:?}");
    let Cli { command, auth, .. } = cli;
    let auth_options = auth.to_auth_options(config.as_ref().and_then(|c| c.auth.as_ref()))?;
    debug!("Auth options: {auth_options:?}");
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
                    .ok_or(CliError::InputError)
                    .or_else(|_| Cli::process_input_text(&input_args))?;
                let audio_format = output_args.get_audio_format(config.as_ref().and_then(|c|c.output.as_ref()))?;
                let callback = Cli::process_output(output_args.output, output_args.overwrite)?;
                let mut synthesizer = SynthesizerConfig::new(auth_options, audio_format)
                    .connect_websocket()
                    .await?;
                let audio_data = synthesizer.synthesize_ssml(&ssml).await?;
                callback(&audio_data)?;
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
                        .ok_or(CliError::InputError)
                        .or_else(|_| Cli::process_input_text(&input_args).map(Cow::Owned))
                        ?;
                let audio_format = output_args.get_audio_format(config.as_ref().and_then(|c|c.output.as_ref()))?;
                let callback = Cli::process_output(output_args.output, output_args.overwrite)?;
                let mut synthesizer = SynthesizerConfig::new(auth_options,audio_format)
                    .connect_websocket()
                    .await?;
                let options = &Cli::process_text_options(&text_args, config.as_ref().and_then(|c|c.text.as_ref()))?;
                let result = synthesizer.synthesize_text(&text, options).await;
                if result.as_ref().err().and_then(
                    |e| match e {
                        WebsocketSynthesizerError {
                            kind: WebsocketSynthesizerErrorKind::Websocket,
                            ..
                        } => e.source().and_then(|err| err.downcast_ref::<tokio_tungstenite::tungstenite::Error>()).map(
                            |e| matches!(e, TungsteniteError::Protocol(ProtocolError::ResetWithoutClosingHandshake))
                        ),
                        _ => None,
                    }
                ).unwrap_or(false)
                {
                    return Err(result.err().unwrap()).with_note(|| "This error usually indicates a poor internet connection or that the remote API terminates your service.")
                        .with_suggestion(|| "Retry if you are on a poor internet connection. \
                                             If this error persists and you are using the trial service, please shorten your input.");
                } else {
                    let audio_data = result?;
                    callback(&audio_data)?;
                }
            }
            Command::ListVoices {
                ref voice,
                ref locale,
                ref url
            } => {
                // Look for --url first,
                // then look for auth.voice_list_api in profile,
                // then try to determine the url by region
                // otherwise, try to use the trial voice list url
                let url = url.as_deref().map(Cow::Borrowed).or_else(|| {
                    config.as_ref().and_then(|c| c.auth.as_ref().and_then(|a| a.voice_list_api.as_deref().map(Cow::Borrowed)))
                }).or_else(|| {
                    auth.region.as_deref().or_else(||
                        config.as_ref().and_then(
                            |c| c.auth.as_ref().and_then(
                                |a| a.endpoint_config.as_ref().and_then(
                                    |e| if let EndpointConfig::Region { ref region } =  e {
                                        Some(region.as_str())
                                    } else {
                                        None
                                    }
                                )
                            )
                        )
                    ).map(|r| Cow::Owned(format!("https://{r}.tts.speech.microsoft.com/cognitiveservices/voices/list")))
                })
                // .or_else(|| TRIAL_VOICE_LIST_URL.map(Cow::Borrowed))
                .ok_or_else(
                    || eyre!("No voice list API url specified!".to_string())
                        .with_note(|| "The default voice list API that is used in aspeak v4 has been shutdown and is no longer available.")
                        .with_suggestion(|| "You can still use the list-voices command by specifying a region(authentication needed) or a custom voice list API url.")
                )?;
                let auth = match (auth_options.key(), auth_options.token()) {
                    (_, Some(token)) => Some(VoiceListAPIAuth::AuthToken(token)),
                    (Some(key), None) => Some(VoiceListAPIAuth::SubscriptionKey(key)),
                    (None, None) => None,
                };
                let voices_result = Voice::request_available_voices_with_additional_headers(VoiceListAPIEndpoint::Url(url.as_ref()),
                    auth, auth_options.proxy(), Some(HeaderMap::from_iter(auth_options.headers().iter().map(Clone::clone)))
                ).await;
                let voices = if let Err(VoiceListAPIError {
                    kind: VoiceListAPIErrorKind::Response,
                    ..
                }) =  voices_result {
                    voices_result.with_note(|| "Maybe you are not authorized. Did you specify an auth token or a subscription key? Did the key/token expire?")?
                } else {
                    voices_result?
                };
                let voices = voices.iter();
                let locale_id = locale.as_deref();
                let voice_id = voice.as_deref();
                let voices: Box<dyn Iterator<Item = &Voice>> = {
                    if locale_id.is_some() {
                        Box::new(voices.filter(|voice| Some(voice.locale()) == locale_id))
                    } else if voice_id.is_some() {
                        Box::new(voices.filter(|voice| Some(voice.short_name()) == voice_id))
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
