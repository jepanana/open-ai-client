use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Modify a message in a thread.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModifyMessagesRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    pub metadata: BTreeMap<String, String>,
}
