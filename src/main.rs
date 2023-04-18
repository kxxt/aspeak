mod cli;
mod constants;

use std::{borrow::Cow, path::PathBuf};

use cli::{commands::Command, Cli};

use aspeak::{AspeakError, AudioFormat, SynthesizerConfig, Voice, QUALITY_MAP};
use clap::Parser;
use color_eyre::{
    eyre::{anyhow, bail},
    Help, Report,
};
use colored::Colorize;
use constants::ORIGIN;

use env_logger::WriteStyle;
use log::debug;

use reqwest::header::{HeaderMap, HeaderValue};
use strum::IntoEnumIterator;
use tokio_tungstenite::tungstenite::{error::ProtocolError, Error as TungsteniteError};

use crate::cli::{
    args::Color,
    commands::ConfigCommand,
    config::{Config, EndpointConfig},
};

const TRIAL_VOICE_LIST_URL: Option<&str> = None;

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
                let mut synthesizer = SynthesizerConfig::new(auth_options, audio_format)
                    .connect()
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
                        .ok_or(AspeakError::InputError)
                        .or_else(|_| Cli::process_input_text(&input_args).map(Cow::Owned))
                        ?;
                let audio_format = output_args.get_audio_format(config.as_ref().and_then(|c|c.output.as_ref()))?;
                let callback = Cli::process_output(output_args.output, output_args.overwrite)?;
                let auth_options = auth.to_auth_options(config.as_ref().and_then(|c|c.auth.as_ref()))?;
                let mut synthesizer = SynthesizerConfig::new(auth_options,audio_format)
                    .connect()
                    .await?;
                let options = &Cli::process_text_options(&text_args, config.as_ref().and_then(|c|c.text.as_ref()))?;
                let result = synthesizer.synthesize_text(&text, options).await;
                if let Err(e @ AspeakError::WebSocketError(TungsteniteError::Protocol(
                    ProtocolError::ResetWithoutClosingHandshake,
                ))) = result
                {
                    return Err(e).with_note(|| "This error usually indicates a poor internet connection or that the remote API terminates your service.")
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
                .or_else(|| TRIAL_VOICE_LIST_URL.map(Cow::Borrowed))
                .ok_or_else(
                    || Report::new(AspeakError::ArgumentError("No voice list API url specified!".to_string()))
                        .with_note(|| "The default voice list API that is used in aspeak v4 has been shutdown and is no longer available.")
                        .with_suggestion(|| "You can still use the list-voices command by specifying a region(authentication needed) or a custom voice list API url.")
                )?;
                let auth = auth.to_auth_options(config.as_ref().and_then(|c|c.auth.as_ref()))?;
                let mut client = reqwest::ClientBuilder::new().no_proxy(); // Disable default system proxy detection.
                if let Some(proxy) = auth.proxy {
                    client = client.proxy(reqwest::Proxy::all(&*proxy)?);
                }
                let client = client.build()?;
                let mut request = client.get(&*url);
                if let Some(key) = &auth.key {
                    request = request.header(
                        "Ocp-Apim-Subscription-Key",
                        HeaderValue::from_str(key)
                            .map_err(|e| AspeakError::ArgumentError(e.to_string()))?,
                    );
                }
                if !auth.headers.is_empty() {
                    // TODO: I don't know if this could be further optimized
                    request = request.headers(HeaderMap::from_iter(auth.headers.iter().map(Clone::clone)));
                } else if Some(url.as_ref()) == TRIAL_VOICE_LIST_URL {
                    // Trial endpoint
                    request = request.header("Origin", HeaderValue::from_str(ORIGIN).unwrap());
                }
                let request = request.build()?;
                let response = client.execute(request).await?;
                if response.status().is_client_error() {
                    bail!(anyhow!("Failed to retrieve voice list because of client side error.").with_note(|| "Maybe you are not authorized. Did you specify an auth token or a subscription key? Did the key/token expire?"))
                } else if response.status().is_server_error() {
                    bail!("Failed to retrieve voice list because of server side error.")
                }
                let voices = response.json::<Vec<Voice>>().await?;
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
