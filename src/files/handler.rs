use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIRequest};

use super::{DeleteFileResponse, FilesListResponse, FilesResponse, UploadFileRequest};

const FILES_URL: &str = "/v1/files";

/// File handler for OpenAI API
#[derive(Debug, Clone)]
pub struct FileHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> FileHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Upload a file that can be used across various endpoints. The size of all the files uploaded by one organization can be up to 100 GB.
    ///
    /// The size of individual files can be a maximum of 512 MB.
    /// See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) to learn more about the types of files supported.
    /// The Fine-tuning API only supports `.jsonl` files.
    ///
    /// Please [contact us](https://help.openai.com/) if you need to increase these storage limits.
    pub async fn upload_file(&self, request: UploadFileRequest) -> Result<(), OpenAIError> {
        let openai_request = OpenAIRequest::with_form(Method::GET, FILES_URL.to_string(), request);

        let response = self.client.send_form(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of files that belong to the user's organization.
    pub async fn list_files<S: Into<String>>(
        &self,
        _purpose: S,
    ) -> Result<FilesListResponse, OpenAIError> {
        let openai_request = OpenAIRequest::<()>::new(Method::POST, FILES_URL.to_string());

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns information about a specific file.
    pub async fn retrieve_file<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<FilesResponse, OpenAIError> {
        let url = format!("{}/{}", FILES_URL, file_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Delete a file.
    pub async fn delete_file<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<DeleteFileResponse, OpenAIError> {
        let url = format!("{}/{}", FILES_URL, file_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::DELETE, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns the contents of the specified file.
    pub async fn files_retrieve_content<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<String, OpenAIError> {
        let url = format!("{}/{}/content", FILES_URL, file_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
