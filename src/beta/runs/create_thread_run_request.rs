use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AssistantTool, ChatModel, CreateThreadRequest};

/// Create a thread and run it in one request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateThreadRunRequest {
    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) to use to execute this run.
    pub assistant_id: String,

    /// Thread to create and run.
    pub thread: CreateThreadRequest,

    /// The ID of the [Model](https://platform.openai.com/docs/api-reference/models) to be used to execute this run.
    /// If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ChatModel>,

    /// Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<AssistantTool>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {

    use crate::ThreadMessage;

    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateThreadRunRequest {
            assistant_id: "asst_abc123".to_string(),
            thread: CreateThreadRequest {
                messages: vec![ThreadMessage {
                    content: "Explain deep learning to a 5 year old.".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            tools: vec![],
            metadata: BTreeMap::new(),
            ..Default::default()
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let json = json!({
          "assistant_id": "asst_abc123",
          "thread": {
            "messages": [
              {"role": "user", "content": "Explain deep learning to a 5 year old."}
            ]
          }
        });

        assert_eq!(request_json, json.to_string());
    }
}
