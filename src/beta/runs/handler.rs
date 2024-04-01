use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIQueryParameters, OpenAIRequest};

use super::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRunsResponse, ListRunsStepsResponse,
    ModifyRunRequest, RunsResponse, RunsStepResponse, SubmitToolsRequest,
};

const THREADS_URL: &str = "/v1/threads";

/// Runs handler for OpenAI API
#[derive(Debug, Clone)]
pub struct RunsHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> RunsHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Create a run.
    pub async fn create_run<S: Into<String>>(
        &self,
        thread_id: S,
        request: CreateRunRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!("{}/{}/runs", THREADS_URL, thread_id.into());
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Create a thread and run it in one request.
    pub async fn create_thread_and_run(
        &self,
        request: CreateThreadAndRunRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!("{}/runs", THREADS_URL);
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of runs belonging to a thread.
    pub async fn list_runs<S: Into<String>>(
        &self,
        thread_id: S,
        parameters: OpenAIQueryParameters,
    ) -> Result<ListRunsResponse, OpenAIError> {
        let url = format!("{}/{}/runs", THREADS_URL, thread_id.into());
        let openai_request =
            OpenAIRequest::<()>::new(Method::GET, url).with_query_parameters(parameters);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of run steps belonging to a run.
    pub async fn list_run_steps<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<ListRunsStepsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}/steps",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Retrieves a run.
    pub async fn retrieve_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Retrieves a run step.
    pub async fn retrieve_run_step<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        step_id: S,
    ) -> Result<RunsStepResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}/steps/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into(),
            step_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::GET, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Modifies a run.
    pub async fn modify_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        request: ModifyRunRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// When a run has the `status: "requires_action"` and `required_action.type` is `submit_tool_outputs`,
    /// this endpoint can be used to submit the outputs from the tool calls once they're all completed.
    /// All outputs must be submitted in a single request.
    pub async fn submit_tool_outputs_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        request: SubmitToolsRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}/submit_tool_outputs",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let openai_request = OpenAIRequest::with_body(Method::POST, url, request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Cancels a run that is `in_progress`.
    pub async fn cancel_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}/cancel",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let openai_request = OpenAIRequest::<()>::new(Method::POST, url);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }
}
