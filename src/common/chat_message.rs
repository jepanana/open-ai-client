use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Message comprising the conversation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The role of the messages author. One of `system`, `user`, `assistant`, or `function`
    pub role: MessageRole,

    /// The contents of the message. `content` is required for all messages,
    /// and may be null for assistant messages with function calls.
    pub content: String,

    /// The name of the author of this message. `name` is required if role is `function`,
    /// and it should be the name of the function whose response is in the `content`.
    /// May contain a-z, A-Z, 0-9, and underscores, with a maximum length of 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    /// The name and arguments of a function that should be called, as generated by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function_call: Option<String>,
}

/// The role of the author of this message.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    /// System
    System,

    /// User
    #[default]
    User,

    /// Assistant
    Assistant,
}

impl Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
