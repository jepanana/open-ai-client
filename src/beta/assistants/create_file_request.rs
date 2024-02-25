use serde::{Deserialize, Serialize};

/// Create an assistant file by attaching a [File](https://platform.openai.com/docs/api-reference/files)
/// to an [assistant](https://platform.openai.com/docs/api-reference/assistants).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateAssistantFileRequest {
    /// A [File](https://platform.openai.com/docs/api-reference/files) ID (with `purpose="assistants"`)
    /// that the assistant should use. Useful for tools like `retrieval` and `code_interpreter` that can access files.
    pub file_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateAssistantFileRequest {
            file_id: "file-abc123".to_string(),
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
          "file_id": "file-abc123"
        });

        assert_eq!(request_json, json.to_string());
    }
}
