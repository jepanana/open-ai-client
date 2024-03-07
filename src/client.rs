use crate::{
    base_client::BaseClient, moderations::CreateRequest, AssistantFileResponse,
    AssistantListResponse, AssistantsFileListResponse, AssistantsResponse, AudioHandler,
    AudioResponse, ChatCompletionResponse, ChatCompletionStreamResponse, ChatHandler,
    CreateAssistantFileRequest, CreateAssistantRequest, CreateChatCompletionRequest,
    CreateFineTunningJobRequest, CreateImageRequest, CreateImageVariationRequest,
    CreateMessageRequest, CreateRunsRequest, CreateSpeechRequest, CreateSpeechResponse,
    CreateThreadRequest, CreateThreadRunRequest, CreateTranscriptionRequest,
    CreateTranslationRequest, EditImageRequest, EmbeddingHandler, EmbeddingRequest,
    EmbeddingResponse, FileHandler, FilesDeleteResponse, FilesListResponse, FilesResponse,
    FilesUploadRequest, FineTuningJobEventResponse, FineTuningJobListResponse,
    FineTuningJobResponse, ImageResponse, ListRunsResponse, ListRunsStepsResponse,
    MessagesFileListResponse, MessagesFileResponse, MessagesListResponse, MessagesResponse,
    ModerationModel, ModifyAssistantRequest, ModifyMessagesRequest, ModifyRunsRequest,
    ModifyThreadRequest, OpenAIError, OpenAIStream, RunsResponse, RunsStepResponse, SortingOrder,
    SubmitToolsRequest, ThreadsResponse,
};

const FINE_TUNNING_URL: &str = "/v1/fine_tuning/jobs";
const IMAGES_GENERATION_URL: &str = "/v1/images/generations";
const IMAGES_EDIT_IMAGES_URL: &str = "/v1/images/edit";
const IMAGES_VARIATIONS_URL: &str = "/v1/images/variations";
const MODEL_URL: &str = "/v1/models";

// Beta
const ASSISTANTS_URL: &str = "/v1/assistants";
const THREADS_URL: &str = "/v1/threads";

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
    pub async fn audio(&self) -> AudioHandler {
        AudioHandler {
            client: &self.client,
        }
    }

    /// Handles chat related operations
    pub async fn chat(&self) -> ChatHandler {
        ChatHandler {
            client: &self.client,
        }
    }

    /// Handles moderation related operations
    pub async fn embeddings(&self) -> EmbeddingHandler {
        EmbeddingHandler {
            client: &self.client,
        }
    }

    /// Handles file related operations
    pub async fn files(&self) -> FileHandler {
        FileHandler {
            client: &self.client,
        }
    }

    /// Calls the "/v1/images/generations" endpoint to generate an image for a given prompt
    pub async fn images_create(
        &self,
        request: CreateImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let url = self.host.join(IMAGES_GENERATION_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/images/edits" endpoint to edit an image with a given prompt
    pub async fn images_edit(
        &self,
        request: EditImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let url = self.host.join(IMAGES_EDIT_IMAGES_URL)?;
        let response = self.send_form(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/images/variations" endpoint to retrieve variations for a given image
    pub async fn images_variant(
        &self,
        request: CreateImageVariationRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let url = self.host.join(IMAGES_VARIATIONS_URL)?;
        let response = self.send_form(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/fine_tuning/jobs" endpoint to create a fine tunning job
    pub async fn create_fine_tunning_job(
        &self,
        request: CreateFineTunningJobRequest,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = self.host.join(FINE_TUNNING_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/fine_tuning/jobs" endpoint to list fine tunning jobs
    pub async fn list_fine_tunning_jobs(&self) -> Result<FineTuningJobListResponse, OpenAIError> {
        let url = self.host.join(FINE_TUNNING_URL)?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/fine_tuning/jobs/{job_id}" endpoint to retrieve fine tunning job metadata
    pub async fn retrieve_fine_tunning_job<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", FINE_TUNNING_URL, job_id.into()))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/fine_tuning/jobs/{job_id}/cancel" endpoint to cancel a fine tunning job
    pub async fn cancel_fine_tunning<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/cancel", FINE_TUNNING_URL, job_id.into()))?;
        let response = self.send(url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/fine_tuning/jobs/{job_id}/events" endpoint to retrieve fine tunning job events list
    pub async fn list_fine_tunning_job_events<S: Into<String>>(
        &self,
        job_id: S,
    ) -> Result<FineTuningJobEventResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/events", FINE_TUNNING_URL, job_id.into()))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants" endpoint to create an assistant
    pub async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = self.host.join(ASSISTANTS_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants/{assistant_id}/files" endpoint to create an assistant file
    pub async fn create_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        request: CreateAssistantFileRequest,
    ) -> Result<AssistantFileResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/files", ASSISTANTS_URL, assistant_id.into()))?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants" endpoint to list assistants
    pub async fn list_assistants(&self) -> Result<AssistantListResponse, OpenAIError> {
        let url = self.host.join(ASSISTANTS_URL)?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants/{assistant_id}/files" endpoint to list assistant files
    pub async fn list_assistants_file<S: Into<String>>(
        &self,
        assistant_id: S,
        limit: Option<u32>,
        order: Option<SortingOrder>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<AssistantsFileListResponse, OpenAIError> {
        let mut url =
            self.host
                .join(&format!("{}/{}/files", ASSISTANTS_URL, assistant_id.into()))?;

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

    /// Calls the "/v1/assistants/{assistant_id}" endpoint to retrieve an assistant
    pub async fn retrieve_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", ASSISTANTS_URL, assistant_id.into()))?;

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants/{assistant_id}/files/{file_id}" endpoint to retrieve an assistant file
    pub async fn retrieve_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        file_id: S,
    ) -> Result<AssistantFileResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/files/{}",
            ASSISTANTS_URL,
            assistant_id.into(),
            file_id.into()
        ))?;

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants/{assistant_id}" endpoint to modify an assistant
    pub async fn modify_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
        request: ModifyAssistantRequest,
    ) -> Result<AssistantsResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", ASSISTANTS_URL, assistant_id.into()))?;

        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/assistants/{assistant_id}" endpoint to delete an assistant
    pub async fn delete_assistant<S: Into<String>>(
        &self,
        assistant_id: S,
    ) -> Result<(), OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", ASSISTANTS_URL, assistant_id.into()))?;

        let _ = self.send(url, Method::DELETE).await?;

        Ok(())
    }

    /// Calls the "/v1/assistants/{assistant_id}/files/{file_id}" endpoint to delete an assistant file
    pub async fn delete_assistant_file<S: Into<String>>(
        &self,
        assistant_id: S,
        file_id: S,
    ) -> Result<(), OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/files/{}",
            ASSISTANTS_URL,
            assistant_id.into(),
            file_id.into()
        ))?;

        let _ = self.send(url, Method::DELETE).await;

        Ok(())
    }

    /// Calls the "/v1/threads" endpoint to create a thread
    pub async fn create_thread(
        &self,
        request: CreateThreadRequest,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let url = self.host.join(THREADS_URL)?;

        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}" endpoint to retrieve a thread
    pub async fn retrieve_thread<S: Into<String>>(
        &self,
        thread_id: S,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", THREADS_URL, thread_id.into()))?;

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}" endpoint to modify a thread
    pub async fn modify_thread<S: Into<String>>(
        &self,
        thread_id: S,
        request: ModifyThreadRequest,
    ) -> Result<ThreadsResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", THREADS_URL, thread_id.into()))?;

        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}" endpoint to delete a thread
    pub async fn delete_thread<S: Into<String>>(&self, thread_id: S) -> Result<(), OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", THREADS_URL, thread_id.into()))?;

        let _ = self.send(url, Method::DELETE).await?;

        Ok(())
    }

    /// Calls the "/v1/threads/{thread_id}/messages" endpoint to create a message
    pub async fn create_threads_message<S: Into<String>>(
        &self,
        thread_id: S,
        request: CreateMessageRequest,
    ) -> Result<MessagesResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/messages", THREADS_URL, thread_id.into()))?;

        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/messages endpoint to list messages in a thread
    pub async fn list_threads_messages<S: Into<String>>(
        &self,
        thread_id: S,
        limit: Option<u32>,
        order: Option<SortingOrder>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<MessagesListResponse, OpenAIError> {
        let mut url = self
            .host
            .join(&format!("{}/{}/messages", THREADS_URL, thread_id.into()))?;

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

    /// Calls the "/v1/threads/{thread_id}/messages/{message_id}/files" endpoint to retrieve files from a message
    pub async fn list_threads_message_files<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        limit: Option<u32>,
        order: Option<SortingOrder>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<MessagesFileListResponse, OpenAIError> {
        let mut url = self.host.join(&format!(
            "{}/{}/messages/{}/files",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        ))?;

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

    /// Calls the "/v1/threads/{thread_id}/messages/{message_id}" endpoint to retrieve a message
    pub async fn retrieve_threads_message<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
    ) -> Result<MessagesResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/messages/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        ))?;

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/messages/{message_id}/files/{file_id}" endpoint to retrieve a file from a message
    pub async fn retrieve_threads_message_file<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        file_id: S,
    ) -> Result<MessagesFileResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/messages/{}/files/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into(),
            file_id.into()
        ))?;

        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/threads/{thread_id}/messages/{message_id}" endpoint to modify a message
    pub async fn modify_threads_message<S: Into<String>>(
        &self,
        thread_id: S,
        message_id: S,
        request: ModifyMessagesRequest,
    ) -> Result<MessagesResponse, OpenAIError> {
        let url = self.host.join(&format!(
            "{}/{}/messages/{}",
            THREADS_URL,
            thread_id.into(),
            message_id.into()
        ))?;

        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
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
