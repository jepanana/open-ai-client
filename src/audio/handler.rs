use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIRequest};

use super::{
    AudioResponse, CreateSpeechRequest, CreateSpeechResponse, CreateTranscriptionRequest,
    CreateTranslationRequest,
};

const AUDIO_CREATE_SPEECH_URL: &str = "v1/audio/speech";
const AUDIO_TRANSCRIPTION_URL: &str = "v1/audio/transcriptions";
const AUDIO_TRANSLATION_URL: &str = "v1/audio/translations";

/// Audio handler for OpenAI API
#[derive(Debug, Clone)]
pub struct AudioHandler<'a> {
    /// Base client
    client: &'a BaseClient,
}

impl<'a> AudioHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Generates audio from the input text.
    pub async fn create_speech(
        &self,
        request: CreateSpeechRequest,
    ) -> Result<CreateSpeechResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_body(Method::POST, AUDIO_CREATE_SPEECH_URL.to_string(), request);

        let response = self.client.send(openai_request).await;

        Ok(CreateSpeechResponse(response?.bytes().await?))
    }

    /// Transcribes audio into the input language.
    pub async fn create_transcription(
        &self,
        request: CreateTranscriptionRequest,
    ) -> Result<AudioResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_form(Method::POST, AUDIO_TRANSCRIPTION_URL.to_string(), request);

        let response = self.client.send_form(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Translates audio into English.
    pub async fn create_translations(
        &self,
        request: CreateTranslationRequest,
    ) -> Result<AudioResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_form(Method::POST, AUDIO_TRANSLATION_URL.to_string(), request);

        let response = self.client.send_form(openai_request).await;

        Ok(response?.json().await?)
    }
}
