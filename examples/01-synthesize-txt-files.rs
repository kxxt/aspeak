use std::{env, error::Error, path::PathBuf};

use futures_util::{stream::FuturesUnordered, StreamExt};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use aspeak::{
    synthesizer::{RestSynthesizer, SynthesizerConfig},
    AudioFormat, AuthOptionsBuilder, TextOptions, TextOptionsBuilder,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth_key = env::var("ASPEAK_AUTH_KEY")?; // Read the auth key from environment variable
    let auth = AuthOptionsBuilder::new(
        aspeak::get_rest_endpoint_by_region("eastus"), // Use eastus endpoint, RESTful API
    )
    .key(auth_key) // Fill in the auth key
    .build();
    let config = SynthesizerConfig::new(
        auth,
        AudioFormat::Audio16Khz32KBitRateMonoMp3, // Let's use mp3 format!
    );
    let syn = config.rest_synthesizer()?; // Get the synthesizer from the config
    let options = TextOptionsBuilder::new() // Adjusting text options like rate, pitch and voice
        .rate("fast")
        .voice("en-US-JennyNeural")
        .pitch("low")
        .build();
    assert!(std::env::args().len() < 3); // Throw an error if we got extra arguments
    let dir = PathBuf::from(
        // Get the path from the first argument
        env::args().nth(1).unwrap_or_else(|| ".".to_string()), // Use CWD if path is not provided
    );
    // Get a list of all txt files (*.txt)
    let txt_paths = std::fs::read_dir(dir)?.filter_map(|r| {
        r.and_then(|ent| {
            let path = ent.path();
            let file_type = ent.file_type()?;
            Ok((path.extension() == Some("txt".as_ref())
                && (file_type.is_file() || file_type.is_symlink()))
            .then_some(path))
        })
        .transpose()
    });
    // Process txt files concurrently
    let mut tasks = txt_paths
        .map(|path| path.map(|path| process_file(path, &syn, &options)))
        .collect::<Result<FuturesUnordered<_>, _>>()?;
    while let Some(next) = tasks.next().await {
        // Examine the results, stop if we encounter an error.
        next?;
    }
    Ok(())
}

async fn process_file<'a>(
    mut path: PathBuf,
    syn: &RestSynthesizer,
    options: &TextOptions<'a>,
) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&path).await?; // Read the text file
    path.set_extension("mp3"); // Change the extension to mp3
    let data = syn.synthesize_text(text, options).await?; // Synthesize
    File::create(path).await?.write_all(&data).await?; // Save the synthesized audio
    Ok(())
}
