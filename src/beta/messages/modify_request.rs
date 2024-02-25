use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Modify a message in a thread.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModifyMessagesRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_request_correctly() {
        let request = ModifyMessagesRequest {
            metadata: {
                let mut map = BTreeMap::new();
                let _ = map.insert("modified".to_string(), "true".to_string());
                let _ = map.insert("user".to_string(), "abc123".to_string());
                map
            },
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
          "metadata": {
            "modified": "true",
            "user": "abc123"
          }
        });

        assert_eq!(request_json, json.to_string());
    }
}
