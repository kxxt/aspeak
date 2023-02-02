mod cli;
mod error;
mod msg;
mod synthesizer;

use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Cursor, Write},
};

use clap::Parser;
use cli::Cli;
use log::{debug, info};
use rodio::{Decoder, OutputStream, Source};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Commandline args: {cli:?}");
    let synthesizer = synthesizer::SynthesizerConfig::new(&cli.endpoint).connect()?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = File::create("debug.mp3")?;
    let mut buf_writer = BufWriter::new(file);
    synthesizer.synthesize(|data| {
        // let cursor = Cursor::new(Vec::from(data));
        // let source = Decoder::new_mp3(cursor)?;
        // stream_handle.play_raw(source.convert_samples())?;
        buf_writer.write(data)?;
        // buf_writer.write(b"\r\n")?;
        Ok(())
    })?;
    Ok(())
}
