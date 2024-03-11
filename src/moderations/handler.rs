use crate::{base_client::BaseClient, common::OpenAIError};

use super::{request::CreateModerationRequest, response::CreateResponse};

const MODERATION_URL: &str = "/v1/moderations";

/// Moderation handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ModerationsHandler<'a> {
    /// Base client
    client: &'a BaseClient,
}

impl<'a> ModerationsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

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
