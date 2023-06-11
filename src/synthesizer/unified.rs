use async_trait::async_trait;

use crate::TextOptions;

#[async_trait]
pub trait UnifiedSynthesizer {
    async fn process_ssml(&mut self, ssml: &str) -> Result<Vec<u8>, UnifiedSynthesizerError>;
    async fn process_text(
        &mut self,
        text: impl AsRef<str>,
        options: &TextOptions<'_>,
    ) -> Result<Vec<u8>, UnifiedSynthesizerError>;
}

pub struct UnifiedSynthesizerError {
    // TODO
}
