use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Audio model options
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AudioModel {
    #[default]
    /// Whisper is an automatic speech recognition (ASR) system trained on 680,000 hours of
    /// multilingual and multitask supervised data collected from the web
    Whisper1,
}

impl Display for AudioModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioModel::Whisper1 => write!(f, "whisper-1"),
        }
    }
}

/// Chat model options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ChatModel {
    /// <b>Updated GPT 3.5 Turbo</b>
    ///
    // The latest GPT-3.5 Turbo model with higher accuracy at responding in requested formats and a fix for a bug which caused a text encoding issue for non-English language function calls. Returns a maximum of 4,096 output tokens.
    /// See [new embedding models and api updates](https://openai.com/blog/new-embedding-models-and-api-updates)
    ///
    /// Context window - 16,385 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-0125")]
    #[default]
    GPT3_5Turbo0125,

    /// Currently points to gpt-3.5-turbo-0613. Will point to gpt-3.5-turbo-1106 starting Dec 11, 2023.
    /// See [continuous model upgrades](https://platform.openai.com/docs/models/continuous-model-upgrades)
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3_5Turbo,

    /// Currently points to gpt-3.5-turbo-0613. Will point to gpt-3.5-turbo-1106 starting Dec 11, 2023.
    /// See [continuous model upgrades](https://platform.openai.com/docs/models/continuous-model-upgrades)
    ///
    /// Context window - 16,385 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-16k")]
    GPT3_5Turbo16k,

    /// Similar capabilities as text-davinci-003 but compatible with legacy Completions endpoint and not Chat Completions.
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    GPT3_5TurboInstinct,

    /// Snapshot of gpt-3.5-turbo from June 13th 2023. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on June 13, 2024.
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GPT3_5Turbo0613,

    /// Snapshot of gpt-3.5-16k-turbo from June 13th 2023. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on June 13, 2024.
    ///
    /// Context window - 16,385 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    GPT3_5Turbo16k0613,

    /// Snapshot of gpt-3.5-turbo from March 1st 2023. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on June 13, 2024.
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-3.5-turbo-0301")]
    GPT3_5Turbo0301,

    /// <b>GPT-4 Turbo</b>
    ///
    /// The latest GPT-4 model with improved instruction following, JSON mode, reproducible outputs, parallel function calling, and more.
    /// Returns a maximum of 4,096 output tokens. This preview model is not yet suited for production traffic.
    /// [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday)
    ///
    /// Context window - 128,000 tokens
    /// Training data - Up to Apr 2023
    #[serde(rename = "gpt-4-1106-preview")]
    GPT4_1106Preview,

    /// <b>GPT-4 Turbo with vision</b>
    ///
    /// Ability to understand images, in addition to all other GPT-4 Turbo capabilities.
    /// Returns a maximum of 4,096 output tokens. This is a preview model version and not suited yet for production traffic.
    /// [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday)
    ///
    /// Context window - 128,000 tokens
    /// Training data - Up to Apr 2023
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,

    /// Currently points to gpt-4-0613
    /// See [continuous model upgrades](https://platform.openai.com/docs/models/continuous-model-upgrades)  
    ///
    /// Context window - 8,192 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4")]
    GPT4,

    /// Currently points to gpt-4-32k-0613.
    /// See (continuous model upgrades)[https://platform.openai.com/docs/models/continuous-model-upgrades]
    ///
    /// Context window - 32,768 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4-32k")]
    GPT4_32k,

    /// Snapshot of gpt-4 from June 13th 2023 with improved function calling support.
    ///
    /// Context window - 8,192 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4-0613")]
    GPT4_0613,

    /// Snapshot of gpt-4-32k from June 13th 2023 with improved function calling support.
    ///
    /// Context window - 32,768 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4-32k-0613")]
    GPT4_32k0613,

    /// Snapshot of gpt-4 from March 14th 2023 with function calling support. This model version will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on June 13, 2024.
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4-0314")]
    GPT4_0314,

    /// Snapshot of gpt-4-32k from March 14th 2023 with function calling support. This model version will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on June 13, 2024.
    ///
    /// Context window - 4,096 tokens
    /// Training data - Up to Sep 2021
    #[serde(rename = "gpt-4-32k-0314")]
    GPT4_32k0314,
}

/// Embedding model options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    #[serde(rename = "text-embedding-ada-002")]
    #[serde(alias = "text-embedding-ada-002-v2")]
    #[default]
    /// Designed to replace the previous 16 first-generation embedding models at a fraction of the cost.
    ///
    /// Context window - 8,191 tokens
    /// Trainning data - Up to Jun 2021
    TextEmbeddingAda002,

    /// Can do language tasks with better quality and consistency than the curie, babbage, or ada models. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on Jan 4 2024.
    ///
    /// Context window - 4,096 tokens
    /// Trainning data - Up to Jun 2021
    #[serde(rename = "text-davinci-003")]
    TextDavinci003,

    /// Similar capabilities to text-davinci-003 but trained with supervised fine-tuning instead of reinforcement learning. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on Jan 4 2024.
    ///
    /// Context window - 4,096 tokens
    /// Trainning data - Up to Jun 2021
    #[serde(rename = "text-davinci-002")]
    TextDavinci002,

    /// Optimized for code-completion tasks. Will be
    /// [deprecated](https://platform.openai.com/docs/deprecations) on Jan 4 2024.
    ///
    /// Context window - 8,001 tokens
    /// Trainning data - Up to Jun 2021
    #[serde(rename = "code-davinci-002")]
    CodeDavinci002,
}

/// Image generation model options
///
/// DALL·E is a AI system that can create realistic images and art from a description in natural language.
/// DALL·E 3 currently supports the ability, given a prompt, to create a new image with a specific size.
/// DALL·E 2 also support the ability to edit an existing image, or create variations of a user provided image.
///
/// [DALL·E 3](https://openai.com/dall-e-3) is available through our
/// [Images API](https://platform.openai.com/docs/guides/images/introduction) along with
/// [DALL·E 2](https://openai.com/blog/dall-e-api-now-available-in-public-beta). You can try DALL·E 3 through
/// [ChatGPT Plus](https://chat.openai.com/).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ImageGenerationModel {
    /// The latest DALL·E model released in Nov 2023. [Learn more](https://openai.com/blog/new-models-and-developer-products-announced-at-devday).
    #[serde(rename = "dall-e-3")]
    Dalle3,

    /// The previous DALL·E model released in Nov 2022. The 2nd iteration of DALL·E with more realistic, accurate, and 4x greater resolution images than the original model.
    #[serde(rename = "dall-e-2")]
    #[default]
    Dalle2,
}

/// Moderation model options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ModerationModel {
    /// Most capable moderation model. Accuracy will be slightly higher than the stable model.
    ///
    /// Context window - 32,768 tokens
    #[serde(rename = "text-moderation-stable")]
    #[default]
    TextModerationLatest,

    /// Almost as capable as the latest model, but slightly older.
    ///
    /// Context window - 32,768 tokens
    #[serde(rename = "text-moderation-latest")]
    TextModerationStable,
}

/// TTS is an AI model that converts text to natural sounding spoken text.
/// We offer two different model variates, `tts-1` is optimized for real time text to speech use cases
/// and `tts-1-hd` is optimized for quality. These models can be used with the
/// [Speech endpoint in the Audio API](https://platform.openai.com/docs/guides/text-to-speech).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TtsModel {
    /// The latest text to speech model, optimized for speed.
    #[serde(rename = "tts-1")]
    Tts1,

    /// The latest text to speech model, optimized for quality.
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
}
