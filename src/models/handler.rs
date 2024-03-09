use crate::{base_client::BaseClient, OpenAIError};

use super::{ListResponse, ModelObjectResponse};

const MODEL_URL: &str = "/v1/models";

/// Models handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ModelsHandler<'a> {
    /// Base client
    client: &'a BaseClient,
}

impl<'a> ModelsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub async fn list(&self) -> Result<ListResponse, OpenAIError> {
        let response = self.client.send(MODEL_URL, reqwest::Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub async fn retrieve<S: Into<String>>(
        &self,
        model: S,
    ) -> Result<ModelObjectResponse, OpenAIError> {
        let path = format!("{}/{}", MODEL_URL, model.into());
        let response = self.client.send(&path, reqwest::Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.
    pub async fn delete<S: Into<String>>(&self, model: S) -> Result<(), OpenAIError> {
        let path = format!("{}/{}", MODEL_URL, model.into());
        let _ = self.client.send(&path, reqwest::Method::DELETE).await;

        Ok(())
    }
}
