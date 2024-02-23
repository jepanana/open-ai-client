use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Represents a thread that contains [messages](https://platform.openai.com/docs/api-reference/messages).
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThreadsResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at: i64,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    pub metadata: BTreeMap<String, String>,
}
