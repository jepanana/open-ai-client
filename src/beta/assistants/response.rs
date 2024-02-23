use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AssistantTool, ChatModel};

/// A list of [`super::AssistantsResponse`]
pub type AssistantListResponse = Vec<AssistantsResponse>;

/// Represents an `assistant` that can call the model and use tools.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssistantsResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always assistant.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub created_at: u32,

    /// The name of the assistant. The maximum length is 256 characters.
    pub name: Option<String>,

    /// The description of the assistant. The maximum length is 512 characters.
    pub description: Option<String>,

    /// /// ID of the model to use. You can use the List [`crate::models`] API to see all of your available models,
    /// or see our [Model](https://platform.openai.com/docs/models/overview) overview for descriptions of them.
    pub model: ChatModel,

    /// The system instructions that the assistant uses. The maximum length is 32768 characters.
    pub instructions: Option<String>,

    ///A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
    /// Tools can be of types `code_interpreter`, `retrieval`, or `function`.
    pub tools: Vec<AssistantTool>,

    /// A list of [`crate::files`] IDs attached to this assistant. There can be a maximum of 20 files attached to the assistant.
    /// Files are ordered by their creation date in ascending order.
    pub file_ids: Vec<String>,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information
    /// about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::ChatModel;

    use super::*;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "asst_abc123",
          "object": "assistant",
          "created_at": 1698984975,
          "name": "Math Tutor",
          "model": "gpt-4",
          "instructions": "You are a personal math tutor. When asked a question, write and run Python code to answer the question.",
          "tools": [
            {
              "type": "code_interpreter"
            }
          ],
          "file_ids": [],
          "metadata": {}
        });

        let response: AssistantsResponse = serde_json::from_value(json).unwrap();

        let expectation = AssistantsResponse {
            id: "asst_abc123".to_string(),
            object: "assistant".to_string(),
            created_at: 1698984975,
            name: Some("Math Tutor".to_string()),
            description: None,
            model: ChatModel::GPT4,
            instructions: Some("You are a personal math tutor. When asked a question, write and run Python code to answer the question.".to_string()),
            tools: vec![AssistantTool::CodeIntepreter(Default::default())],
            file_ids: vec![],
            metadata: BTreeMap::new(),
        };

        assert_eq!(response, expectation);
    }
}
