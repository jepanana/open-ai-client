use reqwest::Method;

use crate::{
    base_client::BaseClient, CreateRunsRequest, CreateThreadAndRunRequest, ListRunsResponse,
    ListRunsStepsResponse, ModifyRunsRequest, OpenAIError, RunsResponse, RunsStepResponse,
    SortingOrder, SubmitToolsRequest,
};

const ASSISTANTS_URL: &str = "/v1/assistants";
const THREADS_URL: &str = "/v1/threads";

#[derive(Debug, Clone)]
pub struct RunsHandler<'a> {
    pub client: &'a BaseClient,
}

impl<'a> RunsHandler<'a> {
    /// Create a run.
    pub async fn create_run<S: Into<String>>(
        &self,
        request: CreateRunsRequest,
        thread_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!("{}/{}/runs", THREADS_URL, thread_id.into());
        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Create a thread and run it in one request.
    pub async fn create_thread_and_run(
        &self,
        request: CreateThreadAndRunRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!("{}/runs", ASSISTANTS_URL);
        let response = self.client.send_body(request, &url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Returns a list of runs belonging to a thread.
    pub async fn list_runs<S: Into<String>>(
        &self,
        thread_id: S,
        _limit: Option<u32>,
        _order: Option<SortingOrder>,
        _after: Option<String>,
        _before: Option<String>,
    ) -> Result<ListRunsResponse, OpenAIError> {
        let url = format!("{}/{}/runs", THREADS_URL, thread_id.into());
        let response = self.client.send(&url, Method::GET).await;

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
        let response = self.client.send(&url, Method::GET).await;

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
        let response = self.client.send(&url, Method::GET).await;

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
        let response = self.client.send(&url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Modifies a run.
    pub async fn modify_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        request: ModifyRunsRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = format!(
            "{}/{}/runs/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        );
        let response = self.client.send_body(request, &url, Method::POST).await;

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
        let response = self.client.send_body(request, &url, Method::POST).await;

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
        let response = self.client.send(&url, Method::POST).await;

        Ok(response?.json().await?)
    }
}
