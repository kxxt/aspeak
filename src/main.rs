mod cli;

use std::{
    error::Error,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

use cli::{Cli, Commands, InputArgs, OutputArgs};

use aspeak::{
    callback_play_blocking, interpolate_ssml, AspeakError, AudioFormat, Result, SynthesizerConfig,
    Voice, ORIGIN,
};
use clap::Parser;
use color_eyre::Help;
use colored::Colorize;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use log::debug;
use phf::phf_map;
use reqwest::header::{self, HeaderMap, HeaderValue};
use strum::IntoEnumIterator;
use tokio_tungstenite::tungstenite::{error::ProtocolError, Error as TungsteniteError};

fn process_input(args: InputArgs) -> color_eyre::Result<String> {
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

fn process_output(
    args: OutputArgs,
) -> Result<(Box<dyn FnMut(Option<&[u8]>) -> Result<()>>, AudioFormat)> {
    let format = args
        .format
        .ok_or(AspeakError::ArgumentError(String::new()))
        .or_else(|_| {
            let container = args.container_format.unwrap_or_default();
            let container = container.as_ref();
            let quality = args.quality.unwrap_or_default();
            QUALITY_MAP
                .get(container)
                .unwrap()
                .get(&(quality as i8))
                .map(|x| *x)
                .ok_or(AspeakError::ArgumentError(format!(
                    "Invalid quality {} for container type {}",
                    quality, container
                )))
        })?;
    Ok(if let Some(file) = args.output {
        // todo: file already exists?
        let file = File::create(file)?;
        let mut buf_writer = BufWriter::new(file);
        (
            Box::new(move |data| {
                Ok(if let Some(data) = data {
                    buf_writer.write_all(data)?
                } else {
                    buf_writer.flush()?
                })
            }),
            format,
        )
    } else {
        (callback_play_blocking(), format)
    })
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    env_logger::builder().filter_level(cli.log_level()).init();
    debug!("Commandline args: {cli:?}");
    match cli.command.unwrap_or_default() {
        Commands::SSML {
            ssml,
            input_args,
            output_args,
        } => {
            let ssml = ssml
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args))?;
            let (callback, format) = process_output(output_args)?;
            let synthesizer = SynthesizerConfig::new((&cli.auth).try_into()?, format)
                .connect()
                .await?;
            synthesizer.synthesize(&ssml, callback).await?;
        }
        Commands::Text {
            mut text_args,
            input_args,
            output_args,
        } => {
            text_args.text = Some(
                text_args
                    .text
                    .ok_or(AspeakError::InputError)
                    .or_else(|_| process_input(input_args))?,
            );
            let (callback, format) = process_output(output_args)?;
            let synthesizer = SynthesizerConfig::new((&cli.auth).try_into()?, format)
                .connect()
                .await?;
            let ssml = interpolate_ssml((&text_args).try_into()?)?;
            let result = synthesizer.synthesize(&ssml, callback).await;
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
        Commands::ListVoices {
            ref voice,
            ref locale,
        } => {
            let url = "https://eastus.api.speech.microsoft.com/cognitiveservices/voices/list";
            let headers =
                HeaderMap::from_iter([(header::ORIGIN, HeaderValue::from_str(ORIGIN).unwrap())]);
            let client = reqwest::blocking::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap();
            let request = client.get(url).build()?;
            let voices = client.execute(request)?.json::<Vec<Voice>>()?;
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
        Commands::ListQualities => {
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
        Commands::ListFormats => {
            for format in AudioFormat::iter() {
                println!("{}", Into::<&str>::into(format));
            }
        }
    }

    Ok(())
}

type QualityMap = phf::Map<i8, AudioFormat>;

static WAV_QUALITY_MAP: QualityMap = phf_map! {
    -2i8 => AudioFormat::Riff8Khz16BitMonoPcm,
    -1i8 => AudioFormat::Riff16Khz16BitMonoPcm,
    0i8  => AudioFormat::Riff24Khz16BitMonoPcm,
    1i8  => AudioFormat::Riff24Khz16BitMonoPcm,
};

static MP3_QUALITY_MAP: QualityMap = phf_map! {
    -4i8 => AudioFormat::Audio16Khz32KBitRateMonoMp3,
    -3i8 => AudioFormat::Audio16Khz64KBitRateMonoMp3,
    -2i8 => AudioFormat::Audio16Khz128KBitRateMonoMp3,
    -1i8 => AudioFormat::Audio24Khz48KBitRateMonoMp3,
    0i8  => AudioFormat::Audio24Khz96KBitRateMonoMp3,
    1i8  => AudioFormat::Audio24Khz160KBitRateMonoMp3,
    2i8  => AudioFormat::Audio48Khz96KBitRateMonoMp3,
    3i8  => AudioFormat::Audio48Khz192KBitRateMonoMp3,
};

static OGG_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Ogg16Khz16BitMonoOpus,
    0i8  => AudioFormat::Ogg24Khz16BitMonoOpus,
    1i8  => AudioFormat::Ogg48Khz16BitMonoOpus,
};

static WEBM_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Webm16Khz16BitMonoOpus,
    0i8  => AudioFormat::Webm24Khz16BitMonoOpus,
    1i8  => AudioFormat::Webm24Khz16Bit24KbpsMonoOpus,
};

static QUALITY_MAP: phf::Map<&'static str, &'static QualityMap> = phf_map! {
    "wav" => &WAV_QUALITY_MAP,
    "mp3" => &MP3_QUALITY_MAP,
    "ogg" => &OGG_QUALITY_MAP,
    "webm" => &WEBM_QUALITY_MAP,
};
