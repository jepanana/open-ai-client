use serde::{Deserialize, Serialize};

/// An object specifying the format that the model must output
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    /// Model output format
    pub _type: ResponseFormatType,
}

/// Model output formats.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Text format
    #[default]
    Text,

    /// JSON format
    JsonObject,
}
