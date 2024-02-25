use serde::{Deserialize, Serialize};

/// A list of files attached to a `message`.
pub type MessagesFileListResponse = Vec<MessagesFileResponse>;

/// A list of files attached to a `message`.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessagesFileResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.message.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message file was created.
    pub created_at: u32,

    /// The ID of the [message](https://platform.openai.com/docs/api-reference/messages)
    /// that the [File](https://platform.openai.com/docs/api-reference/files) is attached to.
    pub message_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "file-abc123",
          "object": "thread.message.file",
          "created_at": 1698107661,
          "message_id": "message_QLoItBbqwyAJEzlTy4y9kOMM",
        });

        let response: MessagesFileResponse = serde_json::from_value(json).unwrap();
        let expected_response = MessagesFileResponse {
            id: "file-abc123".to_string(),
            object: "thread.message.file".to_string(),
            created_at: 1698107661,
            message_id: "message_QLoItBbqwyAJEzlTy4y9kOMM".to_string(),
        };

        assert_eq!(response, expected_response);
    }
}
