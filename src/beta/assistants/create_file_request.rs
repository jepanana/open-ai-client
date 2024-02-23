use serde::{Deserialize, Serialize};

/// Create an assistant file by attaching a [File](https://platform.openai.com/docs/api-reference/files)
/// to an [assistant](https://platform.openai.com/docs/api-reference/assistants).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateAssistantFileRequest {
    /// A [File](https://platform.openai.com/docs/api-reference/files) ID (with `purpose="assistants"`)
    /// that the assistant should use. Useful for tools like `retrieval` and `code_interpreter` that can access files.
    pub file_id: String,
}
