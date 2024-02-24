use serde::{Deserialize, Serialize};

/// A list of files attached to a `message`.
pub type MessagesFileListResponse = Vec<MessagesFileResponse>;

/// A list of files attached to a `message`.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessagesFileResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.message.file`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message file was created.
    pub created_at: u32,

    /// The ID of the [message](https://platform.openai.com/docs/api-reference/messages)
    /// that the [File](https://platform.openai.com/docs/api-reference/files) is attached to.
    pub message_id: String,
}
