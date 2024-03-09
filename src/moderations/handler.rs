use crate::{base_client::BaseClient, OpenAIError};

use super::{request::CreateModerationRequest, response::CreateResponse};

const MODERATION_URL: &str = "/v1/moderations";

/// Moderation handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ModerationHandler<'a> {
    /// Base client
    pub client: &'a BaseClient,
}

impl<'a> ModerationHandler<'a> {
    /// Classifies if text is potentially harmful.
    pub async fn create_moderation(
        &self,
        request: CreateModerationRequest,
    ) -> Result<CreateResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, MODERATION_URL, reqwest::Method::POST)
            .await;

        Ok(response?.json().await?)
    }
}
