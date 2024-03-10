use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Request to modify a thread.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModifyThreadRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl ModifyThreadRequest {
    /// Creates a new instance of the request.
    pub fn with_metadata(metadata: BTreeMap<String, String>) -> Self {
        Self { metadata }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = ModifyThreadRequest {
            metadata: {
                let mut map = BTreeMap::new();
                let _ = map.insert("modified".to_string(), "true".to_string());
                let _ = map.insert("user".to_string(), "abc123".to_string());
                map
            },
        };

        let expected_json = json!({
          "metadata": {
            "modified": "true",
            "user": "abc123"
          }
        });

        let serialized_request = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized_request, expected_json);
    }
}
