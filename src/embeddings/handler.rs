use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIRequest};

use super::{CreateEmbeddingsRequest, EmbeddingResponse};

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
        request: CreateEmbeddingsRequest,
    ) -> Result<EmbeddingResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_body(Method::POST, EMBEDDING_URL.to_string(), request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
