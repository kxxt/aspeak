mod cli;
mod error;
mod msg;
mod ssml;
mod synthesizer;
mod voice;

use std::{
    error::Error,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

use clap::Parser;
use cli::{Cli, Commands, InputArgs, OutputArgs};
use error::AspeakError;
use log::{debug, info};


use crate::ssml::interpolate_ssml;

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
    Ok(if let Some(file) = args.output {
        // todo: file already exists?
        let file = File::create(file)?;
        let mut buf_writer = BufWriter::new(file);
        Box::new(move |data| {
            buf_writer.write(data)?;
            Ok(())
        })
    } else {
        Box::new(move |data| {
            info!("Received {} bytes of data", data.len());
            Ok(())
        })
    })
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
            common_args: _,
        } => {
            let ssml = ssml
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args))?;
            let synthesizer = synthesizer::SynthesizerConfig::new(&cli.endpoint).connect()?;
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
            let synthesizer = synthesizer::SynthesizerConfig::new(&cli.endpoint).connect()?;
            let ssml = interpolate_ssml(&text_options)?;
            let callback = process_output(output_args)?;
            synthesizer.synthesize(&ssml, callback)?;
        }
        Commands::ListVoices { common_args } => {
            let url = format!("https://{}/cognitiveservices/voices/list", cli.endpoint);
            
        }
        _ => todo!(),
    }

    Ok(())
}
