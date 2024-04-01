use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIRequest};

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
    pub async fn list_models(&self) -> Result<ListResponse, OpenAIError> {
        let openai_request = OpenAIRequest::<()>::new(Method::GET, MODEL_URL.to_string());

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    pub async fn retrieve_model<S: Into<String>>(
        &self,
        model: S,
    ) -> Result<ModelObjectResponse, OpenAIError> {
        let url = format!("{}/{}", MODEL_URL, model.into());
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.
    pub async fn delete_fine_tunned_model<S: Into<String>>(
        &self,
        model: S,
    ) -> Result<(), OpenAIError> {
        let url = format!("{}/{}", MODEL_URL, model.into());
        let openai_request = OpenAIRequest::<()>::new(Method::DELETE, url);

        let _ = self.client.send(openai_request).await;

        Ok(())
    }
}
