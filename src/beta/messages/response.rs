use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::MessageRole;

/// A List of messages.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessagesListResponse {
    /// The object type, which is always `list`.
    pub object: String,

    /// A list of [`MessagesResponse`]
    pub data: Vec<MessagesResponse>,
}

/// Represents a message within a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.message`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the message was created.
    pub created_at: u32,

    /// The [thread](https://platform.openai.com/docs/api-reference/threads) ID that this message belongs to.
    pub thread_id: String,

    /// The entity that produced the message. One of `user` or `assistant`.
    pub role: MessageRole,

    /// The content of the message in array of text and/or images.
    pub content: Vec<MessageContent>,

    /// If applicable, the ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) that authored this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,

    /// If applicable, the ID of the [run](https://platform.openai.com/docs/api-reference/runs) associated with the authoring of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,

    /// A list of [file](https://platform.openai.com/docs/api-reference/files) IDs that the assistant should use. Useful for tools like retrieval and code_interpreter that can access files. A maximum of 10 files can be attached to a message.
    pub file_ids: Vec<String>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

/// The content of the [`MessageResponse`] either text and/or images.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// The content of the message is an image.
    File(FileMessageContent),
    /// The text content that is part of a message.
    Text(TextMessageContent),
}

/// The content of a file message.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMessageContent {
    /// Always `image_file`.
    #[serde(rename = "type")]
    pub _type: String,

    /// The file object with image id.
    pub image_file: FileMessageContentFile,
}

/// The file object with image id.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMessageContentFile {
    /// The [File](https://platform.openai.com/docs/api-reference/files) ID of the image in the message content.
    pub file_id: String,
}

/// Text message content.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextMessageContent {
    /// Always `text`.
    #[serde(rename = "type")]
    pub _type: String,

    /// Text message data.
    pub text: TextMessageData,
}

/// Text message data.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextMessageData {
    /// The data that makes up the text.
    pub value: String,

    /// A list of annotations for the text.
    pub annotations: Vec<TextMessageDataAnnotation>,
}

/// Text message annotation data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextMessageDataAnnotation {
    /// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message.
    FileCitation(FileCitationAnnotation),

    /// A file path within the message that points to a specific file.
    FilePath(FilePathAnnotation),
}

/// A citation within the message that points to a specific quote
/// from a specific File associated with the assistant or the message.
/// Generated when the assistant uses the "retrieval" tool to search files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCitationAnnotation {
    /// Always `file_citation`.
    #[serde(rename = "type")]
    pub _type: String,

    /// The text in the message content that needs to be replaced.
    pub text: String,

    /// The file citation data.
    pub file_citation: FileCitation,

    /// The start index of the text citation.
    pub start_index: u32,

    /// The end index of the text citation.
    pub end_index: u32,
}

/// The file citation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCitation {
    /// The ID of the specific File the citation is from.
    pub file_id: String,

    /// The specific quote in the file.
    pub quote: String,
}

/// A file path within the message that points to a specific file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePathAnnotation {
    /// Always `file_path`.
    #[serde(rename = "type")]
    pub _type: String,

    /// The text in the message content that needs to be replaced.
    pub text: String,

    /// The file path data.
    pub file_path: FilePathData,

    /// The start index of the text citation.
    pub start_index: u32,

    /// The end index of the text citation.
    pub end_index: u32,
}

/// The file path data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePathData {
    /// The ID of the file that was generated.
    pub file_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "msg_abc123",
          "object": "thread.message",
          "created_at": 1698983503,
          "thread_id": "thread_abc123",
          "role": "assistant",
          "content": [
            {
              "type": "text",
              "text": {
                "value": "Hi! How can I help you today?",
                "annotations": []
              }
            }
          ],
          "file_ids": [],
          "assistant_id": "asst_abc123",
          "run_id": "run_abc123",
        });

        let response: MessagesResponse = serde_json::from_value(json).unwrap();

        let expected_response = MessagesResponse {
            id: "msg_abc123".to_string(),
            object: "thread.message".to_string(),
            created_at: 1698983503,
            thread_id: "thread_abc123".to_string(),
            role: MessageRole::Assistant,
            content: vec![MessageContent::Text(TextMessageContent {
                _type: "text".to_string(),
                text: TextMessageData {
                    value: "Hi! How can I help you today?".to_string(),
                    ..Default::default()
                },
            })],
            assistant_id: Some("asst_abc123".to_string()),
            run_id: Some("run_abc123".to_string()),
            ..Default::default()
        };

        assert_eq!(response, expected_response);
    }
}
