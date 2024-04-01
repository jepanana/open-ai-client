use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIQueryParameters, OpenAIRequest};

use super::{
    CreateMessageRequest, ListMessagesFileResponse, ListMessagesResponse, MessageResponse,
    MessagesFileResponse, ModifyMessagesRequest,
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
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of messages for a given thread.
    pub async fn list_messages<S: Into<String>>(
        &self,
        thread_id: S,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListMessagesResponse, OpenAIError> {
        let url = format!("{}/{}/messages", THREADS_URL, thread_id.into());
        let openai_request =
            OpenAIRequest::<()>::new(Method::GET, url).with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of message files.
    pub async fn list_message_files<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListMessagesFileResponse, OpenAIError> {
        let url = format!(
            "{}/{}/messages/{}/files",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        );
        let openai_request =
            OpenAIRequest::<()>::new(Method::GET, url).with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

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
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

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
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

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
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
