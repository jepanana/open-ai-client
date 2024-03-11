use serde::{Deserialize, Serialize};

use crate::common::TtsModel;

/// Generates audio from the input text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpeechRequest {
    /// One of the available [TTS models](crate::TtsModel): tts-1 or tts-1-hd
    pub model: TtsModel,

    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,

    ///The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`.
    /// Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech/voice-options).
    pub voice: Voice,

    /// The format to audio in. Supported formats are mp3, opus, aac, and flac.
    /// Defaults to mp3.
    pub response_format: SpeechResponseFormat,

    /// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
    pub speed: Option<f32>,
}

/// Voice options for speech generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Voice {
    /// Alloy
    Alloy,

    /// Echo
    Echo,

    /// Fable
    Fable,

    /// Onyx
    Onyx,

    /// Nova
    Nova,

    /// Shimmer
    Shimmer,
}

/// The format to audio in
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechResponseFormat {
    /// MP3
    #[default]
    Mp3,

    /// Opus
    Opus,

    /// Advanced Audio Coding
    Aac,

    /// Free Lossless Audio Codec
    Flac,
}
