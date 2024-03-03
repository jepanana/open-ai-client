use serde::{Deserialize, Serialize};

/// Lists the currently available models, and provides basic information about each one such as the owner and availability.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListResponse {
    /// The object type, which is always "list".
    pub object: String,

    /// A list of model data.
    pub data: Vec<ModelObjectResponse>,
}

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelObjectResponse {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The object type, which is always "model".
    pub object: String,

    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u32,

    /// The organization that owns the model.
    pub owned_by: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "object": "list",
          "data": [
            {
              "id": "model-id-0",
              "object": "model",
              "created": 1686935002,
              "owned_by": "organization-owner"
            },
            {
              "id": "model-id-1",
              "object": "model",
              "created": 1686935002,
              "owned_by": "organization-owner",
            },
            {
              "id": "model-id-2",
              "object": "model",
              "created": 1686935002,
              "owned_by": "openai"
            },
          ],
        });

        let response: ListResponse = serde_json::from_value(json).unwrap();

        let expectation = ListResponse {
            object: "list".into(),
            data: vec![
                ModelObjectResponse {
                    id: "model-id-0".into(),
                    object: "model".into(),
                    created: 1686935002,
                    owned_by: "organization-owner".into(),
                },
                ModelObjectResponse {
                    id: "model-id-1".into(),
                    object: "model".into(),
                    created: 1686935002,
                    owned_by: "organization-owner".into(),
                },
                ModelObjectResponse {
                    id: "model-id-2".into(),
                    object: "model".into(),
                    created: 1686935002,
                    owned_by: "openai".into(),
                },
            ],
        };

        assert_eq!(response, expectation)
    }
}
