use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIQueryParameters, OpenAIRequest};

use super::{
    CreateFineTunningJobRequest, FineTuningJobResponse, ListFineTuningJobResponse,
    ListFineTunningJobEventResponse,
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
        let openai_request =
            OpenAIRequest::with_body(Method::POST, FINE_TUNNING_URL.to_string(), request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// List your organization's fine-tuning jobs
    pub async fn list_fine_tunning_jobs<S: Into<String>>(
        &self,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListFineTuningJobResponse, OpenAIError> {
        let openai_request = OpenAIRequest::<()>::new(Method::GET, FINE_TUNNING_URL.to_string())
            .with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Get status updates for a fine-tuning job.
    pub async fn list_fine_tunning_job_events<S: Into<String>>(
        &self,
        job_id: S,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListFineTunningJobEventResponse, OpenAIError> {
        let url = format!("{}/{}/events", FINE_TUNNING_URL, job_id.into());
        let openai_request =
            OpenAIRequest::<()>::new(Method::GET, url).with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Get info about a fine-tuning job.
    pub async fn retrieve_fine_tunning_job<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = format!("{}/{}", FINE_TUNNING_URL, job_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel_fine_tunning<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = format!("{}/{}/cancel", FINE_TUNNING_URL, job_id.into());
        let openai_request = OpenAIRequest::<()>::new(Method::POST, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
