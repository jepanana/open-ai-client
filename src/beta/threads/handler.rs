use reqwest::Method;

use crate::{assistants_common::DeletionStatus, base_client::BaseClient, common::OpenAIError};

use super::{CreateThreadRequest, ModifyThreadRequest, ThreadsResponse};

const THREADS_URL: &str = "/v1/threads";

/// Threads handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ThreadsHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> ThreadsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Create a thread.
    pub async fn create_thread(
        &self,
        request: CreateThreadRequest,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, THREADS_URL, Method::POST)
            .await;

        Ok(response?.json().await?)
    }

    /// Retrieves a thread.
    pub async fn retrieve_thread<S: Into<String>>(
        &self,
        thread_id: S,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let url = format!("{}/{}", THREADS_URL, thread_id.into());
        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Modifies a thread.
    pub async fn modify_thread<S: Into<String>>(
        &self,
        thread_id: S,
        request: ModifyThreadRequest,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let url = format!("{}/{}", THREADS_URL, thread_id.into());

        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Delete a thread.
    pub async fn delete_thread<S: Into<String>>(
        &self,
        thread_id: S,
    ) -> Result<DeletionStatus, OpenAIError> {
        let url = format!("{}/{}", THREADS_URL, thread_id.into());
        let response = self.client.send(&url, Method::DELETE).await;

        Ok(response?.json().await?)
    }
}
