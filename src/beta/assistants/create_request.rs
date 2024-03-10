use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AssistantTool, ChatModel};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
/// Request for create an assistant API.
pub struct CreateAssistantRequest {
    /// ID of the model to use. You can use the List [`crate::models`] API to see all of your available models,
    /// or see our [Model](https://platform.openai.com/docs/models/overview) overview for descriptions of them.
    pub model: ChatModel,

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
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl CreateAssistantRequest {
    /// Create a new assistant request.
    pub fn with_instructions<S: Into<String>>(model: ChatModel, instructions: S) -> Self {
        Self {
            model,
            instructions: Some(instructions.into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateAssistantRequest {
            model: ChatModel::GPT3_5Turbo0125,
            name: Some("Test Assistant".to_string()),
            description: Some("Test Assistant Description".to_string()),
            instructions: Some("Test Assistant Instructions".to_string()),
            tools: vec![AssistantTool::CodeIntepreter(Default::default())],
            file_ids: vec!["file-id".to_string()],
            metadata: BTreeMap::new(),
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
            "model": "gpt-3.5-turbo-0125",
            "name": "Test Assistant",
            "description": "Test Assistant Description",
            "instructions": "Test Assistant Instructions",
            "tools": [{"type": "code_interpreter"}],
            "file_ids": ["file-id"],
            "metadata": {}
        });
        assert_eq!(request_json, json.to_string());
    }
}
