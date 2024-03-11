use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError};

use super::{EmbeddingRequest, EmbeddingResponse};

const EMBEDDING_URL: &str = "/v1/embeddings";

/// Embedding handler for OpenAI API
#[derive(Debug, Clone)]
pub struct EmbeddingsHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> EmbeddingsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

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
