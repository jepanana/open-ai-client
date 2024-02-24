use serde::{Deserialize, Serialize};

/// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant.
/// Tools can be of types `code_interpreter`, `retrieval`, or `function`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantTool {
    /// Code Interpreter tool
    CodeIntepreter(CodeInterpreterTool),
    /// Retrieval tool
    Retrival(RetrievalTool),

    /// Function tool
    Function(FunctionTool),
}

/// Code interpreting tool for assistant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterTool {
    /// The type of tool. Must be `code_interpreter`.
    #[serde(rename = "type")]
    pub _type: String,
}

impl Default for CodeInterpreterTool {
    fn default() -> Self {
        Self {
            _type: "code_interpreter".to_string(),
        }
    }
}

/// Retrieval tool for assistant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetrievalTool {
    /// The type of tool. Must be `retrieval`.
    #[serde(rename = "type")]
    pub _type: String,
}

impl Default for RetrievalTool {
    fn default() -> Self {
        Self {
            _type: "retrieval".to_string(),
        }
    }
}

/// Function tool for assistant
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionTool {
    /// The type of tool. Must be `function`.
    #[serde(rename = "type")]
    pub _type: String,

    /// The function object.
    pub function: FunctionObject,
}

/// Function object for assistant
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionObject {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,

    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [guide](https://platform.openai.com/docs/guides/text-generation/function-calling) for examples,
    /// and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
    ///
    /// Omitting `parameters` defines a function with an empty parameter list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
