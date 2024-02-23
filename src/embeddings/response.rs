use serde::{Deserialize, Serialize};

use crate::common::{EmbeddingModel, Usage};

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// The object type, which should always be "list"
    pub object: String,

    /// Embedding data
    pub data: Vec<EmbeddingData>,

    /// The model used for the embedding creation
    pub model: EmbeddingModel,

    /// Usage statistics for the completion request
    pub usage: Usage,
}

/// Represents embedding data returned by embedding endpoint.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingData {
    /// The index of the embedding in the list of embeddings
    pub index: u32,

    /// The object type, which is always "embedding"
    pub object: String,

    /// The embedding vector, which is a list of floats. The length of vector depends
    /// on the model as listed in the (embedding guide)[https://platform.openai.com/docs/guides/embeddings]
    pub embedding: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "object": "list",
          "data": [
            {
              "object": "embedding",
              "embedding": [
                0.0023064255,
                -0.009327292,
                -0.0028842222,
              ],
              "index": 0
            }
          ],
          "model": "text-embedding-ada-002",
          "usage": {
            "prompt_tokens": 8,
            "total_tokens": 8
          }
        });

        let response: EmbeddingResponse = serde_json::from_value(json).unwrap();

        let expectation = EmbeddingResponse {
            object: "list".into(),
            data: vec![EmbeddingData {
                object: "embedding".into(),
                embedding: vec![0.0023064255, -0.009327292, -0.0028842222],
                index: 0,
            }],
            model: EmbeddingModel::TextEmbeddingAda002,
            usage: Usage {
                prompt_tokens: 8,
                completion_tokens: 0,
                total_tokens: 8,
            },
        };

        assert_eq!(response, expectation);
    }
}
