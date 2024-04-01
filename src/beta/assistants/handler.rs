use reqwest::Method;

use crate::{
    assistants_common::DeletionStatus, base_client::BaseClient, common::OpenAIError,
    OpenAIQueryParameters, OpenAIRequest,
};

use super::{
    AssistantFileResponse, AssistantsResponse, CreateAssistantFileRequest, CreateAssistantRequest,
    ListAssistantsFilesResponse, ListAssistantsResponse, ModifyAssistantRequest,
};

const ASSISTANTS_URL: &str = "/v1/assistants";

/// Assistants handler for OpenAI API
#[derive(Debug, Clone)]
pub struct AssistantsHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> AssistantsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Create an assistant with a model and instructions.
    pub async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_body(Method::POST, ASSISTANTS_URL.to_string(), request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Create an assistant file by attaching a [File](https://platform.openai.com/docs/api-reference/files)
    /// to an [assistant](https://platform.openai.com/docs/api-reference/assistants).
    pub async fn create_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        request: CreateAssistantFileRequest,
    ) -> Result<AssistantFileResponse, OpenAIError> {
        let url = format!("{}/{}/files", ASSISTANTS_URL, assistant_id.into());
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of assistants.
    pub async fn list_assistants(
        &self,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListAssistantsResponse, OpenAIError> {
        let openai_request = OpenAIRequest::<()>::new(Method::GET, ASSISTANTS_URL.to_string())
            .with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of assistant files.
    pub async fn list_assistants_file<S: Into<String>>(
        &self,
        assistant_id: S,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListAssistantsFilesResponse, OpenAIError> {
        let url = format!("{}/{}/files", ASSISTANTS_URL, assistant_id.into());
        let openai_request =
            OpenAIRequest::<()>::new(Method::GET, url).with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;
        Ok(response?.json().await?)
    }

    /// Retrieves an assistant.
    pub async fn retrieve_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Retrieves an assistant file.
    pub async fn retrieve_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        file_id: S,
    ) -> Result<AssistantFileResponse, OpenAIError> {
        let url = format!(
            "{}/{}/files/{}",
            ASSISTANTS_URL,
            assistant_id.into(),
            file_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Modify an assistant.
    pub async fn modify_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
        request: ModifyAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Delete an assistant.
    pub async fn delete_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<DeletionStatus, OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::DELETE, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Delete an assistant file.
    pub async fn delete_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        file_id: S,
    ) -> Result<DeletionStatus, OpenAIError> {
        let url = format!(
            "{}/{}/files/{}",
            ASSISTANTS_URL,
            assistant_id.into(),
            file_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::DELETE, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
