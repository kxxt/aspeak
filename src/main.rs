mod cli;
mod error;
mod msg;
mod synthesizer;

use std::error::Error;

use clap::Parser;
use cli::Cli;
use log::{debug, info};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Commandline args: {cli:?}");
    let synthesizer = synthesizer::SynthesizerConfig::new(&cli.endpoint).connect()?;
    synthesizer.synthesize(|data| info!("Received data!"))?;
    Ok(())
}
