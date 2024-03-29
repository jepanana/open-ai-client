use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{assistants_common::AssistantTool, common::ChatModel};

/// Modifies an assistant.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModifyAssistantRequest {
    /// ID of the model to use. You can use the List [`crate::models`] API to see all of your available models,
    /// or see our [Model](https://platform.openai.com/docs/models/overview) overview for descriptions of them.
    pub model: Option<ChatModel>,

    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The system instructions that the assistant uses. The maximum length is 32768 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types `code_interpreter`, `retrieval`, or `function`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<AssistantTool>,

    /// A list of [`crate::files`] IDs attached to this assistant. There can be a maximum of 20 files attached to the assistant.
    /// Files are ordered by their creation date in ascending order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<String>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information
    /// about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_request_correctly() {
        let request = ModifyAssistantRequest {
            model: Some(ChatModel::GPT4),
            instructions: Some("You are an HR bot, and you have access to files to answer employee questions about company policies. Always response with info from either of the files.".into()),
            tools: vec![AssistantTool::Retrieval(Default::default())],
            file_ids: vec!["file-abc123".into(), "file-abc456".into()],
            metadata: BTreeMap::new(),
            ..Default::default()
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
          "model": "gpt-4",
          "instructions": "You are an HR bot, and you have access to files to answer employee questions about company policies. Always response with info from either of the files.",
          "tools": [{"type": "retrieval"}],
          "file_ids": ["file-abc123", "file-abc456"]
        });

        assert_eq!(request_json, json.to_string());
    }
}
