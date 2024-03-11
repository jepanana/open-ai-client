use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError};

use super::{
    CreateFineTunningJobRequest, FineTuningJobEventResponse, FineTuningJobListResponse,
    FineTuningJobResponse,
};

const FINE_TUNNING_URL: &str = "/v1/fine_tuning/jobs";

/// Fine-tuning handler for OpenAI API
#[derive(Debug, Clone)]
pub struct FineTuningHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> FineTuningHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Creates a fine-tuning job which begins the process of creating a new model from a given dataset.
    /// Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.
    /// [Learn more about fine-tuning](https://platform.openai.com/docs/guides/fine-tuning)
    pub async fn create_fine_tunning_job(
        &self,
        request: CreateFineTunningJobRequest,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let response = self
            .client
            .send_body(request, FINE_TUNNING_URL, Method::POST)
            .await;

        Ok(response?.json().await?)
    }

    /// List your organization's fine-tuning jobs
    pub async fn list_fine_tunning_jobs(&self) -> Result<FineTuningJobListResponse, OpenAIError> {
        let response = self.client.send(FINE_TUNNING_URL, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Get status updates for a fine-tuning job.
    pub async fn list_fine_tunning_job_events<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobEventResponse, OpenAIError> {
        let url = format!("{}/{}/events", FINE_TUNNING_URL, job_id.into());
        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Get info about a fine-tuning job.
    pub async fn retrieve_fine_tunning_job<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = format!("{}/{}", FINE_TUNNING_URL, job_id.into());
        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel_fine_tunning<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = format!("{}/{}/cancel", FINE_TUNNING_URL, job_id.into());
        let response = self.client.send(&url, Method::POST).await;

        Ok(response?.json().await?)
    }
}
