use reqwest::{Method, Response};
use reqwest_eventsource::{EventSource, RequestBuilderExt};
use serde::Serialize;

use crate::{
    moderations::{ModerationRequest, ModerationResponse},
    AssistantFileResponse, AssistantListResponse, AssistantsFileListResponse, AssistantsResponse,
    AudioResponse, AudioTranscriptionRequest, AudioTranslationRequest, ChatCompletionRequest,
    ChatCompletionResponse, ChatCompletionStreamResponse, CreateAssistantFileRequest,
    CreateAssistantRequest, CreateFineTunningJobRequest, CreateImageRequest,
    CreateImageVariationRequest, CreateMessageRequest, CreateRunsRequest, CreateSpeechRequest,
    CreateSpeechResponse, CreateThreadRequest, CreateThreadRunRequest, EditImageRequest,
    EmbeddingRequest, EmbeddingResponse, FilesDeleteResponse, FilesListResponse, FilesResponse,
    FilesUploadRequest, FineTuningJobEventResponse, FineTuningJobListResponse,
    FineTuningJobResponse, ImageResponse, ListRunsResponse, MessagesFileListResponse,
    MessagesFileResponse, MessagesListResponse, MessagesResponse, ModelDataResponse,
    ModelsListResponse, ModifyAssistantRequest, ModifyMessagesRequest, ModifyThreadRequest,
    OpenAIError, OpenAIStream, RunsResponse, SortingOrder, ThreadsResponse,
};

const AUDIO_CREATE_SPEECH_URL: &str = "v1/audio/speech";
const AUDIO_TRANSCRIPTION_URL: &str = "v1/audio/transcriptions";
const AUDIO_TRANSLATION_URL: &str = "v1/audio/translations";
const CHAT_COMPLETION_URL: &str = "/v1/chat/completions";
const EMBEDDING_URL: &str = "/v1/embeddings";
const FINE_TUNNING_URL: &str = "/v1/fine_tuning/jobs";
const FILES_URL: &str = "/v1/files";
const IMAGES_GENERATION_URL: &str = "/v1/images/generations";
const IMAGES_EDIT_IMAGES_URL: &str = "/v1/images/edit";
const IMAGES_VARIATIONS_URL: &str = "/v1/images/variations";
const MODERATION_URL: &str = "/v1/moderations";
const MODEL_URL: &str = "/v1/models";

// Beta
const ASSISTANTS_URL: &str = "/v1/assistants";
const THREADS_URL: &str = "/v1/threads";

/// OpenAI client
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: reqwest::Client,
    host: reqwest::Url,
}

impl OpenAIClient {
    /// Create a new OpenAI client from reqwest client and host
    pub fn new(client: reqwest::Client, host: reqwest::Url) -> Self {
        Self { client, host }
    }

    /// Calls the "/v1/chat/completions" endpoint to create chat completions
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenAIError> {
        let url = self.host.join(CHAT_COMPLETION_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/chat/completions" endpoint to create chat completions streaming
    pub async fn chat_completion_streaming(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<OpenAIStream<ChatCompletionStreamResponse>, OpenAIError> {
        let url = self.host.join(CHAT_COMPLETION_URL)?;
        let response = self.create_stream(request, url, Method::POST).await?;

        Ok(OpenAIStream::new(response).await)
    }

    /// Calls the "/v1/embeddings" endpoint to create embeddings for the given text
    pub async fn embedding(
        &self,
        request: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, OpenAIError> {
        let url = self.host.join(EMBEDDING_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/audio/speech" endpoint to create audio speech
    pub async fn create_speech(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let url = self.host.join(AUDIO_CREATE_SPEECH_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(CreateSpeechResponse(response?.bytes().await?))
    }

    /// Calls the "/v1/audio/transcriptions" endpoint to create audio transcriptions
    pub async fn audio_transcriptions(
        &self,
        request: AudioTranscriptionRequest,
    ) -> Result<AudioResponse, OpenAIError> {
        let url = self.host.join(AUDIO_TRANSCRIPTION_URL)?;
        let response = self.send_form(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/audio/translations" endpoint to translate given audio
    pub async fn audio_translations(
        &self,
        request: AudioTranslationRequest,
    ) -> Result<AudioResponse, OpenAIError> {
        let url = self.host.join(AUDIO_TRANSLATION_URL)?;
        let response = self.send_form(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/moderations" endpoint to see if a text is safe for use
    pub async fn moderation(
        &self,
        request: ModerationRequest,
    ) -> Result<ModerationResponse, OpenAIError> {
        let url = self.host.join(MODERATION_URL)?;
        let response = self.send_body(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/models" endpoint to list OpenAI and company saved models
    pub async fn models_list(&self) -> Result<ModelsListResponse, OpenAIError> {
        let url = self.host.join(MODEL_URL)?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/models/{model}" endpoint to get metadata of a model
    pub async fn model<S: Into<String>>(&self, model: S) -> Result<ModelDataResponse, OpenAIError> {
        let url = self.host.join(&format!("{}/{}", MODEL_URL, model.into()))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/models/{model}" endpoint to delete a model
    pub async fn model_delete<S: Into<String>>(&self, model: S) -> Result<(), OpenAIError> {
        let url = self.host.join(&format!("{}/{}", MODEL_URL, model.into()))?;
        let _ = self.send(url, Method::DELETE).await;

        Ok(())
    }

    /// Calls the "/v1/files" endpoint to list all the uploaded files
    pub async fn files_list(&self) -> Result<FilesListResponse, OpenAIError> {
        let url = self.host.join(FILES_URL)?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/files" endpoint to upload a file
    pub async fn files_upload(&self, request: FilesUploadRequest) -> Result<(), OpenAIError> {
        let url = self.host.join(FILES_URL)?;
        let response = self.send_form(request, url, Method::POST).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/files/{file_id}" endpoint to delete a file
    pub async fn files_delete<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<FilesDeleteResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", FILES_URL, file_id.into()))?;
        let response = self.send(url, Method::DELETE).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/files/{file_id}" endpoint to retrieve file metadata
    pub async fn files_retrieve<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<FilesResponse, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}", FILES_URL, file_id.into()))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
    }

    /// Calls the "/v1/files/{file_id}/content" endpoint to retrieve file contents
    pub async fn files_retrieve_content<S: Into<String>>(
        &self,
        file_id: S,
    ) -> Result<String, OpenAIError> {
        let url = self
            .host
            .join(&format!("{}/{}/content", FILES_URL, file_id.into()))?;
        let response = self.send(url, Method::GET).await;

        Ok(response?.json().await?)
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

    pub async fn list_runs<S: Into<String>>(
        &self,
        thread_id: S,
        limit: Option<u32>,
        order: Option<SortingOrder>,
        after: Option<String>,
        before: Option<String>,
    ) -> ListRunsResponse {
        todo!()
    }

    async fn send(&self, url: reqwest::Url, method: Method) -> Result<Response, OpenAIError> {
        let response = self.client.request(method, url).send().await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    async fn send_body<Q>(
        &self,
        request: Q,
        url: reqwest::Url,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    async fn send_form<Q>(
        &self,
        request: Q,
        url: reqwest::Url,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: TryInto<reqwest::multipart::Form, Error = OpenAIError> + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .multipart(request.try_into()?)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    async fn create_stream<Q>(
        &self,
        request: Q,
        url: reqwest::Url,
        method: Method,
    ) -> Result<EventSource, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .json(&request)
            .eventsource()?;

        Ok(response)
    }
}
