use serde::{Deserialize, Serialize};

/// Deleted file response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// Deleted file id
    pub id: String,

    /// The object type, which is always `file`.
    pub object: String,

    /// Whether the file was successfully deleted.
    pub deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_files_delete_response_correctly() {
        let json = json!({
          "id": "file-abc123",
          "object": "file",
          "deleted": true
        });

        let response: DeleteFileResponse = serde_json::from_value(json).unwrap();

        let expectation = DeleteFileResponse {
            id: "file-abc123".to_string(),
            object: "file".to_string(),
            deleted: true,
        };

        assert_eq!(response, expectation);
    }
}
