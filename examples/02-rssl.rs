use std::{env, error::Error};

use aspeak::{
    synthesizer::SynthesizerConfig, AudioFormat, AuthOptionsBuilder, RichSsmlOptionsBuilder,
    TextOptionsBuilder,
};
use rodio::{Decoder, OutputStream, Sink};
use rustyline::error::ReadlineError;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth_key = env::var("ASPEAK_AUTH_KEY")?; // Read the auth key from environment variable
    let auth = AuthOptionsBuilder::new(
        aspeak::get_websocket_endpoint_by_region("eastus"), // Use eastus endpoint, Websocket API
    )
    .key(auth_key) // Fill in the auth key
    .build();
    let config = SynthesizerConfig::new(auth, AudioFormat::Audio16Khz32KBitRateMonoMp3);
    let mut syn = config.connect_websocket().await?; // Get the synthesizer from the config
    let options = TextOptionsBuilder::new() // Adjusting text options like rate, pitch and voice
        .rate("+20%")
        .voice("zh-CN-XiaoxiaoNeural")
        .pitch("medium")
        .chain_rich_ssml_options_builder(
            RichSsmlOptionsBuilder::new().style("newscast"), // Set speech style to newscast
        )
        .build();
    println!("Welcome to the speech synthesizer RSSL (Read-Synthesize-Speak-Loop)! I will speak whatever you type in. Send EOF (Ctrl+D on Unix, Ctrl+Z on Windows) to exit.");
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut rl = rustyline::DefaultEditor::new()?;
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let data = syn.synthesize_text(line, &options).await?; // Synthesize
                let cursor = std::io::Cursor::new(data);
                let source = Decoder::new(cursor)?;
                sink.append(source);
                sink.sleep_until_end();
            }
            Err(ReadlineError::Eof) => {
                println!("Interrupted");
                break;
            }
            err => {
                err?;
            }
        }
    }
    Ok(())
}
