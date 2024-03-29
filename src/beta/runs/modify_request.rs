use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Request to modify a run.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModifyRunRequest {
    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    pub metadata: BTreeMap<String, String>,
}

impl ModifyRunRequest {
    /// Creates a new instance of the request.
    pub fn with_metadata(metadata: BTreeMap<String, String>) -> Self {
        Self { metadata }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = ModifyRunRequest {
            metadata: {
                let mut map = BTreeMap::new();
                let _ = map.insert("user_id".to_string(), "user_abc123".to_string());
                map
            },
        };

        let request_json = serde_json::to_string(&request).unwrap();

        let json = json!({
          "metadata": {
            "user_id": "user_abc123"
          }
        });

        assert_eq!(request_json, json.to_string());
    }
}
