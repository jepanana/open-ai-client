use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ChatMessage;

/// Create a thread.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateThreadRequest {
    /// A list of [messages](https://platform.openai.com/docs/api-reference/messages) to start the thread with.
    pub messages: Vec<ChatMessage>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}