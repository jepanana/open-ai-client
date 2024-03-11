use serde::{Deserialize, Serialize};

use crate::common::{ChatModel, TokenUsage};

use super::ChatResponseChunk;

/// Represents a chat completion response returned by model, based on the provided input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub id: String,

    /// A list of chat completion choices. Can be more than one if `n` is greater than 1
    pub choices: Vec<ChatCompletionChoice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created
    pub created: u32,

    /// The model used for the chat completion
    pub model: ChatModel,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(default)]
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`
    pub object: String,

    /// Usage statistics for the completion request
    pub usage: TokenUsage,
}

impl ChatCompletionResponse {
    /// Returns the first messages content from [`ChatCompletionChoice`]
    pub fn first_message(&self) -> Option<&String> {
        self.choices
            .first()
            .map(|choice| &choice.message)?
            .content
            .as_ref()
    }

    /// Returns the messages content from [`ChatCompletionChoice`] at the given index
    pub fn message_at_index(&self, index: usize) -> Option<&String> {
        self.choices
            .get(index)
            .map(|choice| &choice.message)?
            .content
            .as_ref()
    }
}

/// Chat completion variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    /// The index of the choice in the list of choices
    pub index: i32,

    /// A chat completion message generated by the model
    pub message: ChatResponseChunk,

    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// `tool_calls` if the model called a tool, or `function_call` (deprecated)
    /// if the model called a function.
    pub finish_reason: String,
}

#[cfg(test)]
mod tests {
    use crate::{FunctionCall, MessageRole, ToolCall, ToolType};

    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "chatcmpl-123",
          "object": "chat.completion",
          "created": 1677652288,
          "model": "gpt-3.5-turbo-0613",
          "system_fingerprint": "fp_44709d6fcb",
          "choices": [{
            "index": 0,
            "message": {
              "role": "assistant",
              "content": "\n\nHello there, how may I assist you today?",
              "tool_calls": [
                {
                  "name": "tool",
                  "type": "function",
                  "function": {
                    "name": "set_response",
                    "arguments": "Hello there, how may I assist you today?"
                  }
                }
              ],
            },
            "finish_reason": "stop"
          }],
          "usage": {
            "prompt_tokens": 9,
            "completion_tokens": 12,
            "total_tokens": 21
          }
        });

        let response: ChatCompletionResponse = serde_json::from_value(json).unwrap();

        let expectation = ChatCompletionResponse {
            id: "chatcmpl-123".to_string(),
            object: "chat.completion".to_string(),
            created: 1677652288u32,
            model: ChatModel::GPT3_5Turbo0613,
            system_fingerprint: Some("fp_44709d6fcb".to_string()),
            choices: vec![ChatCompletionChoice {
                index: 0,
                message: ChatResponseChunk {
                    role: Some(MessageRole::Assistant),
                    tool_calls: vec![ToolCall {
                        name: "tool".to_string(),
                        _type: ToolType::Function,
                        function: FunctionCall {
                            name: "set_response".to_string(),
                            arguments: "Hello there, how may I assist you today?".to_string(),
                        },
                    }],
                    content: Some("\n\nHello there, how may I assist you today?".to_string()),
                },
                finish_reason: "stop".to_string(),
            }],
            usage: TokenUsage {
                prompt_tokens: 9,
                completion_tokens: 12,
                total_tokens: 21,
            },
        };

        assert_eq!(response, expectation);
    }
}
