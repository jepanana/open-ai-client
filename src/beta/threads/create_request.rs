use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ThreadMessage;

/// Request for creating a thread.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateThreadRequest {
    /// A list of [messages](https://platform.openai.com/docs/api-reference/messages) to start the thread with.
    pub messages: Vec<ThreadMessage>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl CreateThreadRequest {
    /// Creates a new instance of the request with the provided messages.
    pub fn with_messages(messages: Vec<ThreadMessage>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ThreadMessageRole;

    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateThreadRequest {
            messages: vec![
                ThreadMessage {
                    role: ThreadMessageRole::User,
                    content: "Hello, what is AI?".to_string(),
                    file_ids: vec!["file-abc123".to_string()],
                    ..Default::default()
                },
                ThreadMessage {
                    role: ThreadMessageRole::User,
                    content: "How does AI work? Explain it in simple terms.".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let expected_json = json!({
          "messages": [{
            "role": "user",
            "content": "Hello, what is AI?",
            "file_ids": ["file-abc123"]
          }, {
            "role": "user",
            "content": "How does AI work? Explain it in simple terms."
          }]
        });

        let serialized_request = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized_request, expected_json);
    }
}
