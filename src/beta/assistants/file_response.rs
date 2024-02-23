use serde::{Deserialize, Serialize};

/// A list of [Files](https://platform.openai.com/docs/api-reference/files) attached to an `assistant`.
pub type AssistantsFileListResponse = Vec<AssistantFileResponse>;

/// A list of [Files](https://platform.openai.com/docs/api-reference/files) attached to an `assistant`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AssistantFileResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `assistant.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the assistant file was created.
    pub created: u32,

    /// The assistant ID that the file is attached to.
    pub assistant_id: String,
}
