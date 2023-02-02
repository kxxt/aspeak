mod cli;
mod error;
mod msg;
mod synthesizer;

use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Stdin, Write},
    path::Path,
};

use clap::Parser;
use cli::{Cli, Commands, InputArgs, OutputArgs};
use error::AspeakError;
use log::{debug, info};
use rodio::{Decoder, OutputStream, Source};

fn process_input(args: InputArgs) -> Result<String, AspeakError> {
    let mut s = String::new();
    // todo: encoding
    if let Some(file) = args.file {
        File::open(&file)?.read_to_string(&mut s)?;
    } else {
        io::stdin().read_to_string(&mut s)?;
    }
    Ok(s)
}

fn process_output(
    args: OutputArgs,
) -> Result<Box<dyn FnMut(&[u8]) -> Result<(), AspeakError>>, AspeakError> {
    todo!();
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Commandline args: {cli:?}");
    // TODO: fix empty command case
    match cli.command.unwrap() {
        Commands::SSML {
            ssml,
            input_args,
            output_args,
            common_args,
        } => {
            let ssml = ssml
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args));
        }
        Commands::Text {
            text,
            pitch,
            rate,
            style,
            role,
            style_degree,
            input_args,
            output_args,
            common_args,
        } => {
            let text = text
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args));
        }
        _ => todo!(),
    }
    let synthesizer = synthesizer::SynthesizerConfig::new(&cli.endpoint).connect()?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = File::create("debug.mp3")?;
    let mut buf_writer = BufWriter::new(file);
    synthesizer.synthesize("", |data| {
        // let cursor = Cursor::new(Vec::from(data));
        // let source = Decoder::new_mp3(cursor)?;
        // stream_handle.play_raw(source.convert_samples())?;
        buf_writer.write(data)?;
        // buf_writer.write(b"\r\n")?;
        Ok(())
    })?;
    Ok(())
}
