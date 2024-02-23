use serde::{Deserialize, Serialize};

/// Returns a list of files that belong to the user's organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesListResponse {
    /// The object type, which is always `list`.
    pub object: String,

    /// A list of document that has been uploaded to OpenAI..
    pub data: Vec<FilesResponse>,
}

/// Represents a document that has been uploaded to OpenAI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesResponse {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The size of the file, in bytes.
    pub bytes: u32,

    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at: u32,

    /// The name of the file.
    pub filename: String,

    /// The object type, which is always `file`.
    pub object: String,

    /// The intended purpose of the file. Supported values are `fine-tune`, `fine-tune-results`, `assistants`, and `assistants_output`.
    pub purpose: Purpose,
}

/// The intended purpose of the file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Purpose {
    /// The file is used for fine-tuning.
    #[serde(rename = "fine-tune")]
    FineTune,

    /// The file is used for fine-tuning results.
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,

    /// The file is used for assistants.
    #[serde(rename = "assistants")]
    Assistants,

    /// The file is used for assistants output.
    #[serde(rename = "assistants-output")]
    AssistantsOutput,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_files_list_response_correctly() {
        let json = json!({
          "data": [
            {
              "id": "file-abc123",
              "object": "file",
              "bytes": 175,
              "created_at": 1613677385,
              "filename": "train.jsonl",
              "purpose": "fine-tune"
            },
            {
              "id": "file-abc123",
              "object": "file",
              "bytes": 140,
              "created_at": 1613779121,
              "filename": "puppy.jsonl",
              "purpose": "assistants"
            }
          ],
          "object": "list"
        });

        let response: FilesListResponse = serde_json::from_value(json).unwrap();

        let expectation = FilesListResponse {
            object: "list".to_string(),
            data: vec![
                FilesResponse {
                    id: "file-abc123".to_string(),
                    object: "file".to_string(),
                    bytes: 175,
                    created_at: 1613677385,
                    filename: "train.jsonl".to_string(),
                    purpose: Purpose::FineTune,
                },
                FilesResponse {
                    id: "file-abc123".to_string(),
                    object: "file".to_string(),
                    bytes: 140,
                    created_at: 1613779121,
                    filename: "puppy.jsonl".to_string(),
                    purpose: Purpose::Assistants,
                },
            ],
        };

        assert_eq!(response, expectation);
    }
}
