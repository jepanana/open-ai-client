use serde::{Deserialize, Serialize};

use crate::common::EmbeddingModel;

/// POST https://api.openai.com/v1/embeddings
/// Creates an embedding vector representing the input text
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request,
    /// pass an array of strings or array of token arrays. The input must not exceed the max input tokens
    /// for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less.
    /// [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.
    pub input: EmbeddingInput,

    /// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list)
    /// API to see all of your available models, or see our [Model overview](https://platform.openai.com/docs/models/overview)
    /// for descriptions of them.
    pub model: EmbeddingModel,

    /// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,

    /// The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The format to return the embeddings in.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingFormat {
    /// Embedding data encoded as a list of floats
    #[default]
    Float,

    /// Embedding data encoded as base64
    Base64,
}

/// Embedding input selection
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text input
    #[default]
    Single(String),

    /// Multiple text inputs
    Multi(Vec<String>),
}

impl EmbeddingRequest {
    /// Creates a new [`EmbeddingRequest`] with a given single input.
    pub fn from_single_input<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            input: EmbeddingInput::Single(input.into()),
            model: EmbeddingModel::TextEmbeddingAda002,
            encoding_format: Some(EncodingFormat::Float),
            ..Default::default()
        }
    }

    /// Creates a new [`EmbeddingRequest`] with multiple inputs.
    pub fn from_multiple_inputs<S>(inputs: &[String]) -> Self {
        Self {
            input: EmbeddingInput::Multi(inputs.to_vec()),
            model: EmbeddingModel::TextEmbeddingAda002,
            encoding_format: Some(EncodingFormat::Float),
            ..Default::default()
        }
    }

    /// Sets the embedding model.
    pub fn set_model(mut self, model: EmbeddingModel) -> Self {
        self.model = model;
        self
    }

    /// Sets the user.
    pub fn set_user<S>(mut self, user: S) -> Self
    where
        S: Into<String>,
    {
        self.user = Some(user.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = EmbeddingRequest {
            model: EmbeddingModel::TextEmbeddingAda002,
            input: EmbeddingInput::Single("The food was delicious and the waiter...".to_string()),
            ..Default::default()
        };

        let expected = json!({
            "model": "text-embedding-ada-002",
            "input": "The food was delicious and the waiter...",
        });

        let serialized = serde_json::to_value(request).unwrap();
        assert_eq!(serialized, expected);
    }
}
