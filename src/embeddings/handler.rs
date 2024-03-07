use reqwest::Method;

use crate::{base_client::BaseClient, EmbeddingRequest, EmbeddingResponse, OpenAIError};

const EMBEDDING_URL: &str = "/v1/embeddings";

/// Embedding handler for OpenAI API
#[derive(Debug, Clone)]
pub struct EmbeddingHandler<'a> {
    pub client: &'a BaseClient,
}

impl<'a> EmbeddingHandler<'a> {
    /// Creates an embedding vector representing the input text.
    pub async fn create_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, EMBEDDING_URL, Method::POST)
            .await;

        Ok(response?.json().await?)
    }
}
