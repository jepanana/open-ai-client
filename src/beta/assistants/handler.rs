use reqwest::Method;

use crate::{
    base_client::BaseClient, AssistantFileResponse, AssistantListResponse,
    AssistantsFileListResponse, AssistantsResponse, CreateAssistantFileRequest,
    CreateAssistantRequest, ModifyAssistantRequest, OpenAIError, SortingOrder,
};

const ASSISTANTS_URL: &str = "/v1/assistants";

#[derive(Debug, Clone)]
pub struct AssistantsHandler<'a> {
    pub client: &'a BaseClient,
}

impl<'a> AssistantsHandler<'a> {
    /// Create an assistant with a model and instructions.
    pub async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, ASSISTANTS_URL, Method::POST)
            .await;

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
        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of assistants.
    pub async fn list_assistants(
        &self,
        _limit: Option<u32>,
        _order: Option<SortingOrder>,
        _after: Option<String>,
        _before: Option<String>,
    ) -> Result<AssistantListResponse, OpenAIError> {
        let response = self.client.send(ASSISTANTS_URL, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of assistant files.
    pub async fn list_assistants_file<S: Into<String>>(
        &self,
        assistant_id: S,
        _limit: Option<u32>,
        _order: Option<SortingOrder>,
        _after: Option<String>,
        _before: Option<String>,
    ) -> Result<AssistantsFileListResponse, OpenAIError> {
        let url = format!("{}/{}/files", ASSISTANTS_URL, assistant_id.into());
        let response = self.client.send(&url, Method::GET).await;
        Ok(response?.json().await?)
    }

    /// Retrieves an assistant.
    pub async fn retrieve_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());
        let response = self.client.send(&url, Method::GET).await;

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

        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Modify an assistant.
    pub async fn modify_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
        request: ModifyAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());

        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Delete an assistant.
    pub async fn delete_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<(), OpenAIError> {
        let url = format!("{}/{}", ASSISTANTS_URL, assistant_id.into());

        let _ = self.client.send(&url, Method::DELETE).await?;

        Ok(())
    }

    pub async fn delete_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        file_id: S,
    ) -> Result<(), OpenAIError> {
        let url = format!(
            "{}/{}/files/{}",
            ASSISTANTS_URL,
            assistant_id.into(),
            file_id.into()
        );

        let _ = self.client.send(&url, Method::DELETE).await;

        Ok(())
    }
}