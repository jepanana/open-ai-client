use serde::{Deserialize, Serialize};

use crate::common::EmbeddingModel;

/// POST https://api.openai.com/v1/embeddings
/// Creates an embedding vector representing the input text
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request,
    /// pass an array of strings or array of token arrays. Each input must not exceed the max input tokens for the model
    /// (8191 tokens for `text-embedding-ada-002`) and cannot be an empty string
    pub input: EmbeddingInput,

    /// ID of the model to use. You can use the (List models API)[`crate::models`]
    /// to see all of your available models, or see our (Model overview)[https://platform.openai.com/docs/models/overview]
    /// for descriptions of them.
    pub model: EmbeddingModel,

    /// The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The format to return the embeddings in.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingFormat {
    /// Embedding data encoded as a list of floats
    Float,

    /// Embedding data encoded as base64
    Base64,
}

/// Embedding input selection
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text input
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
            user: None,
        }
    }

    /// Creates a new [`EmbeddingRequest`] with multiple inputs.
    pub fn from_multiple_inputs<S>(inputs: &[String]) -> Self {
        Self {
            input: EmbeddingInput::Multi(inputs.to_vec()),
            model: EmbeddingModel::TextEmbeddingAda002,
            encoding_format: Some(EncodingFormat::Float),
            user: None,
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
            encoding_format: None,
            user: None,
        };

        let expected = json!({
            "model": "text-embedding-ada-002",
            "input": "The food was delicious and the waiter...",
        });

        let serialized = serde_json::to_value(request).unwrap();
        assert_eq!(serialized, expected);
    }
}
