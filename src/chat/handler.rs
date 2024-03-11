use reqwest::Method;

use crate::{
    base_client::BaseClient,
    common::{OpenAIError, OpenAIStream},
};

use super::{ChatCompletionResponse, ChatCompletionStreamResponse, CreateChatCompletionRequest};

const CHAT_COMPLETION_URL: &str = "/v1/chat/completions";

/// Chat handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ChatHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> ChatHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Creates a model response for the given chat conversation.
    pub async fn create_chat_completion(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, CHAT_COMPLETION_URL, Method::POST)
            .await;

        Ok(response?.json().await?)
    }

    /// Creates a model response for the given chat conversation and returns a streaming response.
    pub async fn create_chat_completion_streaming(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<OpenAIStream<ChatCompletionStreamResponse>, OpenAIError> {
        let response = self
            .client
            .create_stream(request, CHAT_COMPLETION_URL, Method::POST)
            .await?;

        Ok(OpenAIStream::new(response).await)
    }
}
