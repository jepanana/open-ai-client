use serde::{Deserialize, Serialize};

use crate::ToolType;

/// A tool which OpenAI models can call
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    /// Type of the tool, only `function` is supported for now
    #[serde(rename = "type")]
    pub _type: ToolType,

    /// Function that the model will call
    pub function: Function,
}

/// A function that the model may call, as specified by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9,
    /// or contain underscores and dashes, with a maximum length of 64.
    pub name: String,

    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [guide](https://platform.openai.com/docs/guides/text-generation/function-calling) for examples,
    /// and the [JSON Schema](https://json-schema.org/understanding-json-schema/) reference for documentation about the format.
    pub parameters: serde_json::Value,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            description: None,
            parameters: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        }
    }
}

/// Controls which (if any) function is called by the model.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    /// The model will not call a function and instead generates a message.
    None,

    /// The model can pick between generating a message or calling a function.
    Auto,

    /// Forces the model to call a function.
    Object(ToolChoiceObject),
}

impl Serialize for ToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ToolChoice::None => serializer.serialize_str("none"),
            ToolChoice::Auto => serializer.serialize_str("auto"),
            ToolChoice::Object(obj) => obj.serialize(serializer),
        }
    }
}

///
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolChoiceObject {
    /// The type of the tool. Currently, only `function` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub _type: Option<String>,

    /// The function that the model will call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionChoice>,
}

/// The function choice that the model will call.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionChoice {
    /// Name of the function to call.
    pub name: String,
}
