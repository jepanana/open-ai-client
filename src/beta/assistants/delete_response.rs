use serde::{Deserialize, Serialize};

/// Delete assistant response.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeleteAssistantResponse {
    /// The identifier of the assistant.
    pub id: String,

    /// The object type, which is always starts with  `assistant.*`.
    pub object: String,

    /// Whether the assistant was deleted.
    pub deleted: bool,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
            "id": "assistant-id",
            "object": "assistant.deleted",
            "deleted": true
        });

        let response: DeleteAssistantResponse = serde_json::from_value(json).unwrap();

        let expected = DeleteAssistantResponse {
            id: "assistant-id".to_string(),
            object: "assistant.deleted".to_string(),
            deleted: true,
        };

        assert_eq!(response, expectation);
    }
}
