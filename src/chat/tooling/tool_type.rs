use serde::{Deserialize, Serialize};

/// The type of the tool. Currently, only `function` is supported.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    /// Function type
    Function,
}
