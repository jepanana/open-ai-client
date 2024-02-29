use serde::{Deserialize, Serialize};

/// DeletionStatus is a struct that represents the status of a deletion operation.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeletionStatus {
    /// The identifier of the object that was deleted.
    pub id: String,

    /// The object type that was deleted.
    pub object: String,

    /// Whether the object was deleted.
    pub deleted: bool,
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
          "deleted": true
        });

        let response: DeletionStatus = serde_json::from_value(json).unwrap();

        let expected_response = DeletionStatus {
            id: "file-abc123".to_string(),
            object: "assistant.file".to_string(),
            deleted: true,
        };

        assert_eq!(response, expected_response);
    }
}
