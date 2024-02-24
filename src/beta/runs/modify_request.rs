use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Modify a run.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModifyRunsRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    pub metadata: BTreeMap<String, String>,
}
