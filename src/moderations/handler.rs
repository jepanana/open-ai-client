use crate::{base_client::BaseClient, CreateRequest, OpenAIError, Response};

const MODERATION_URL: &str = "/v1/moderations";

/// Moderation handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ModerationHandler<'a> {
    /// Base client
    pub client: &'a BaseClient,
}

impl<'a> ModerationHandler<'a> {
    /// Calls the "/v1/moderations" endpoint to see if a text is safe for use
    pub async fn create(&self, request: CreateRequest) -> Result<Response, OpenAIError> {
        let response = self
            .client
            .send_body(request, MODERATION_URL, reqwest::Method::POST)
            .await;

        Ok(response?.json().await?)
    }
}
