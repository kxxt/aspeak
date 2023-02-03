mod cli;

use std::{
    error::Error,
    fs::File,
    io::{self, BufWriter, Cursor, Read, Write},
};

use cli::{Cli, Commands, InputArgs, OutputArgs};

use aspeak::{interpolate_ssml, AspeakError, Result, SynthesizerConfig, Voice, ORIGIN};
use clap::Parser;
use log::{debug, info};
use reqwest::header::{self, HeaderMap, HeaderValue};
use rodio::{Decoder, OutputStream, Sink};

fn process_input(args: InputArgs) -> Result<String> {
    let mut s = String::new();
    // todo: encoding
    if let Some(file) = args.file {
        File::open(&file)?.read_to_string(&mut s)?;
    } else {
        io::stdin().read_to_string(&mut s)?;
    }
    Ok(s)
}

fn process_output(args: OutputArgs) -> Result<Box<dyn FnMut(Option<&[u8]>) -> Result<()>>> {
    Ok(if let Some(file) = args.output {
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
        let mut buffer = Vec::new();
        Box::new(move |data| {
            if let Some(data) = data {
                buffer.extend_from_slice(data);
            } else {
                info!("Playing audio... ({} bytes)", buffer.len());
                let (_stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle).unwrap();
                let cursor = Cursor::new(Vec::from(&buffer[..]));
                let source = Decoder::new(cursor)?;
                sink.append(source);
                sink.sleep_until_end();
            }
            Ok(())
        })
    })
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Commandline args: {cli:?}");
    // TODO: fix empty command case
    match cli.command.unwrap() {
        Commands::SSML {
            ssml,
            input_args,
            output_args,
        } => {
            let ssml = ssml
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args))?;
            let synthesizer =
                SynthesizerConfig::new(&cli.endpoint, output_args.format.unwrap()).connect()?; // todo
            let callback = process_output(output_args)?;
            synthesizer.synthesize(&ssml, callback)?;
        }
        Commands::Text {
            mut text_options,
            input_args,
            output_args,
        } => {
            text_options.text = Some(
                text_options
                    .text
                    .ok_or(AspeakError::InputError)
                    .or_else(|_| process_input(input_args))?,
            );
            let synthesizer =
                SynthesizerConfig::new(&cli.endpoint, output_args.format.unwrap()).connect()?;
            let ssml = interpolate_ssml(&text_options)?;
            let callback = process_output(output_args)?;
            synthesizer.synthesize(&ssml, callback)?;
        }
        Commands::ListVoices {
            ref voice,
            ref locale,
        } => {
            let url = format!("https://{}/cognitiveservices/voices/list", cli.endpoint);
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
        _ => todo!(),
    }

    Ok(())
}
