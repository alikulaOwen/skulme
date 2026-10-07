use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Inference failure: {0}")]
    InferenceError(String),

    #[error("Model {0} not loaded")]
    ModelNotLoaded(String),

    #[error("Network failure: {0}")]
    NetworkError(String),
}

/// Pluggable interface for vector embedding generation namespaced by model_id
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Return the model identifier (e.g. "fastembed/bge-small-en-v1.5" or "openai/text-embedding-3-small")
    fn model_id(&self) -> &'static str;

    /// Return the vector dimensionality (e.g. 384 or 1536)
    fn dimension(&self) -> usize;

    /// Generate an embedding vector for a single text snippet
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;

    /// Generate embeddings for a batch of text snippets
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut results = Vec::with_capacity(texts.len());
        for text in texts {
            results.push(self.embed(text).await?);
        }
        Ok(results)
    }
}
