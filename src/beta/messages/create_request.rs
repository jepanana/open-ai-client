use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::MessageRole;

/// Create a message in a thread.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// The role of the entity that is creating the message. Currently only `user` is supported.
    pub role: MessageRole,

    /// The content of the message.
    pub content: String,

    /// A list of [File](https://platform.openai.com/docs/api-reference/files) IDs
    /// that the message should use. There can be a maximum of 10 files attached to a message.
    /// Useful for tools like `retrieval` and `code_interpreter` that can access and use files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<String>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateMessageRequest {
            role: MessageRole::User,
            content: "How does AI work? Explain it in simple terms.".into(),
            ..Default::default()
        };

        let expected = json!({
          "role": "user",
          "content": "How does AI work? Explain it in simple terms."
        });

        assert_eq!(serde_json::to_value(&request).unwrap(), expected);
    }
}
