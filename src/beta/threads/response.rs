use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Represents a thread that contains [messages](https://platform.openai.com/docs/api-reference/messages).
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThreadsResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: i64,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "thread-abc123",
          "object": "thread",
          "created_at": 1699055364,
          "metadata": {}
        });

        let response: ThreadsResponse = serde_json::from_value(json).unwrap();

        let expected_response = ThreadsResponse {
            id: "thread-abc123".to_string(),
            object: "thread".to_string(),
            created_at: 1699055364,
            ..Default::default()
        };

        assert_eq!(response, expected_response);
    }
}
