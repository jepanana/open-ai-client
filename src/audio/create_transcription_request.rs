use reqwest::Body;
use tokio_util::codec::BytesCodec;

use crate::{
    common::{AudioModel, OpenAIFile},
    AudioResponseFormat, OpenAIError,
};

/// Transcribes audio into the input language
#[derive(Debug)]
pub struct CreateTranscriptionRequest {
    /// The audio file object (not file name) to transcribe, in one of these formats:
    /// flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: OpenAIFile,

    /// ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.
    pub model: AudioModel,

    /// The language of the input audio. Supplying the input language in
    /// (ISO-639-1)[https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes]
    /// format will improve accuracy and latency
    pub language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio segment.
    /// The (prompt)[https://platform.openai.com/docs/guides/speech-to-text/prompting]
    /// should match the audio language
    pub prompt: Option<String>,

    /// The format of the audio output, in one of these options: json, text, srt, verbose_json, or vtt.
    pub response_format: AudioResponseFormat,

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic. If set to 0,
    /// the model will use (log probability)[https://en.wikipedia.org/wiki/Log_probability]
    /// to automatically increase the temperature until certain thresholds are hit
    pub temperature: Option<f32>,
}

impl TryFrom<CreateTranscriptionRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    fn try_from(request: CreateTranscriptionRequest) -> Result<Self, Self::Error> {
        let file_name = request.file.name.to_owned();
        let file_body = Body::wrap_stream(request.file.into_stream(BytesCodec::new()));

        let file_part = reqwest::multipart::Part::stream(file_body)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("model", request.model.to_string())
            .text("response_format", request.response_format.to_string());

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        if let Some(language) = request.language {
            form = form.text("language", language);
        }

        Ok(form)
    }
}
