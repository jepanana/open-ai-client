use serde::{Deserialize, Serialize};

use crate::common::MessageRole;

use super::ToolCall;

/// Message comprising the conversation
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatResponseChunk {
    /// The role of the author of this message
    #[serde(default)]
    pub role: Option<MessageRole>,

    /// The contents of the message
    #[serde(default)]
    pub content: Option<String>,

    /// List of tool calls
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}
