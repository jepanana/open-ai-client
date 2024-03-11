use reqwest::Method;

use crate::{
    base_client::BaseClient,
    common::{OpenAIError, SortingOrder},
};

use super::{
    CreateMessageRequest, MessageResponse, MessagesFileListResponse, MessagesFileResponse,
    MessagesListResponse, ModifyMessagesRequest,
};

const THREADS_URL: &str = "/v1/threads";

/// Messages handler for OpenAI API
#[derive(Debug, Clone)]
pub struct MessagesHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> MessagesHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Create a message.
    pub async fn create_message<S: Into<String>>(
        &self,
        thread_id: S,
        request: CreateMessageRequest,
    ) -> Result<MessageResponse, OpenAIError> {
        let url = format!("{}/{}/messages", THREADS_URL, thread_id.into());

        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of messages for a given thread.
    pub async fn list_messages<S: Into<String>>(
        &self,
        thread_id: S,
        _limit: Option<u32>,
        _order: Option<SortingOrder>,
        _after: Option<String>,
        _before: Option<String>,
    ) -> Result<MessagesListResponse, OpenAIError> {
        let url = format!("{}/{}/messages", THREADS_URL, thread_id.into());

        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of message files.
    pub async fn list_message_files<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        _limit: Option<u32>,
        _order: Option<SortingOrder>,
        _after: Option<String>,
        _before: Option<String>,
    ) -> Result<MessagesFileListResponse, OpenAIError> {
        let url = format!(
            "{}/{}/messages/{}/files",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        );

        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Retrieve a message.
    pub async fn retrieve_message<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
    ) -> Result<MessageResponse, OpenAIError> {
        let url = format!(
            "{}/{}/messages/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        );

        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Retrieves a message file.
    pub async fn retrieve_message_file<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        file_id: S,
    ) -> Result<MessagesFileResponse, OpenAIError> {
        let url = format!(
            "{}/{}/messages/{}/files/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into(),
            file_id.into()
        );

        let response = self.client.send(&url, Method::GET).await;
        Ok(response?.json().await?)
    }

    /// Modifies a message.
    pub async fn modify_threads_message<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        request: ModifyMessagesRequest,
    ) -> Result<MessageResponse, OpenAIError> {
        let url = format!(
            "{}/{}/messages/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        );

        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }
}
