use crate::{
    base_client::BaseClient, AssistantFileResponse, AssistantListResponse,
    AssistantsFileListResponse, AssistantsHandler, AssistantsResponse, AudioHandler, ChatHandler,
    CreateAssistantFileRequest, CreateAssistantRequest, CreateEditImageRequest,
    CreateFineTunningJobRequest, CreateImageRequest, CreateImageVariationRequest,
    CreateMessageRequest, CreateRunsRequest, CreateThreadRequest, CreateThreadRunRequest,
    EmbeddingHandler, FileHandler, FineTuningHandler, FineTuningJobEventResponse,
    FineTuningJobListResponse, FineTuningJobResponse, ImageResponse, ImagesHandler,
    ListRunsResponse, ListRunsStepsResponse, MessagesFileListResponse, MessagesFileResponse,
    MessagesHandler, MessagesListResponse, MessagesResponse, ModifyAssistantRequest,
    ModifyMessagesRequest, ModifyRunsRequest, ModifyThreadRequest, OpenAIError, RunsResponse,
    RunsStepResponse, SortingOrder, SubmitToolsRequest, ThreadsHandler, ThreadsResponse,
};

/// OpenAI client
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: BaseClient,
}

impl OpenAIClient {
    /// Create a new OpenAI client from reqwest client and host
    pub fn new(client: BaseClient) -> Self {
        Self { client }
    }

    /// Handles audio related operations
    pub fn audio(&self) -> AudioHandler {
        AudioHandler {
            client: &self.client,
        }
    }

    /// Handles chat related operations
    pub fn chat(&self) -> ChatHandler {
        ChatHandler {
            client: &self.client,
        }
    }

    /// Handles moderation related operations
    pub fn embeddings(&self) -> EmbeddingHandler {
        EmbeddingHandler {
            client: &self.client,
        }
    }

    /// Handles file related operations
    pub fn files(&self) -> FileHandler {
        FileHandler {
            client: &self.client,
        }
    }

    pub fn fine_tunning(&self) -> FineTuningHandler {
        FineTuningHandler {
            client: &self.client,
        }
    }

    pub fn images(&self) -> ImagesHandler {
        ImagesHandler {
            client: &self.client,
        }
    }

    pub fn assistants(&self) -> AssistantsHandler {
        AssistantsHandler {
            client: &self.client,
        }
    }

    pub fn threads(&self) -> ThreadsHandler {
        ThreadsHandler {
            client: &self.client,
        }
    }

    pub fn messages(&self) -> MessagesHandler {
        MessagesHandler {
            client: &self.client,
        }
    }

    /// Calls the "/v1/threads/{thread_id}/runs" endpoint to create a run
    pub async fn create_run<S: Into<String>>(
        &self,
        request: CreateRunsRequest,
        thread_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/runs", THREADS_URL, thread_id.into()))?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/runs" endpoint to create a thread and run it
    pub async fn create_thread_run(
        &self,
        request: CreateThreadRunRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self.host.join(&format!("{}/runs", ASSISTANTS_URL,))?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs" endpoint to list runs in a thread
    pub async fn list_runs<S: Into<String>>(
        &self,
        thread_id: S,
        limit: Option<u32>,
        order: Option<SortingOrder>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRunsResponse, OpenAIError> {
        let mut url = self
            .host
            .join(&format!("{}/{}/runs", THREADS_URL, thread_id.into()))?;

        if let Some(limit) = limit {
            let _ = url
                .query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(order) = order {
            let _ = url
                .query_pairs_mut()
                .append_pair("order", &order.to_string());
        }

        if let Some(after) = after {
            let _ = url.query_pairs_mut().append_pair("after", &after);
        }

        if let Some(before) = before {
            let _ = url.query_pairs_mut().append_pair("before", &before);
        }

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}/steps" endpoint to retrieve a run
    pub async fn list_run_steps<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<ListRunsStepsResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}/steps",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        ))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}" endpoint to retrieve a run
    pub async fn retrieve_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        ))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}/steps/{step_id}" endpoint to retrieve a run step
    pub async fn retrieve_run_step<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        step_id: S,
    ) -> Result<RunsStepResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}/steps/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into(),
            step_id.into()
        ))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}" endpoint to modify a run
    pub async fn modify_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        request: ModifyRunsRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        ))?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}/submit_tool_outputs" endpoint to submit tool outputs
    pub async fn submit_tool_outputs_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
        request: SubmitToolsRequest,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}/submit_tool_outputs",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        ))?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/runs/{run_id}/cancel" endpoint to cancel a run
    pub async fn cancel_run<S: Into<String>>(
        &self,
        thread_id: S,
        run_id: S,
    ) -> Result<RunsResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/runs/{}/cancel",
            THREADS_URL,
            thread_id.into(),
            run_id.into()
        ))?;
        let response = self.send(url, Method::POST).await;

        Ok(response?.json().await?)
    }
}
