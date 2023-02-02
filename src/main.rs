mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
