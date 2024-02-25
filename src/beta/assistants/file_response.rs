use serde::{Deserialize, Serialize};

/// A list of [Files](https://platform.openai.com/docs/api-reference/files) attached to an `assistant`.
pub type AssistantsFileListResponse = Vec<AssistantFileResponse>;

/// A list of [Files](https://platform.openai.com/docs/api-reference/files) attached to an `assistant`.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssistantFileResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `assistant.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant file was created.
    pub created_at: u32,

    /// The assistant ID that the file is attached to.
    pub assistant_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "file-abc123",
          "object": "assistant.file",
          "created_at": 1699055364,
          "assistant_id": "asst_abc123"
        });

        let response: AssistantFileResponse = serde_json::from_value(json).unwrap();

        let expected_response = AssistantFileResponse {
            id: "file-abc123".to_string(),
            object: "assistant.file".to_string(),
            created_at: 1699055364,
            assistant_id: "asst_abc123".to_string(),
        };

        assert_eq!(response, expected_response);
    }
}
