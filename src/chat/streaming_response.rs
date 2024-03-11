use serde::{Deserialize, Serialize};

use super::ChatResponseChunk;

/// Represents a chat completion streaming response returned by model, based on the provided input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionStreamResponse {
    /// The id of the request.
    pub id: String,

    /// The list of choices.
    pub choices: Vec<ChatCompletionStreamChoice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: u32,

    /// The model used for the request.
    pub model: String,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    #[serde(default)]
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion.chunk `
    pub object: String,
}

impl ChatCompletionStreamResponse {
    /// Returns the first messages content from [`ChatCompletionStreamChoice`]
    pub fn first_message(&self) -> Option<&String> {
        self.choices
            .first()
            .map(|choice| &choice.delta)?
            .content
            .as_ref()
    }

    /// Returns the messages content from [`ChatCompletionStreamChoice`] at the given index
    pub fn message_at_index(&self, index: usize) -> Option<&String> {
        self.choices
            .get(index)
            .map(|choice| &choice.delta)?
            .content
            .as_ref()
    }
}

/// Chat completion streaming variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionStreamChoice {
    /// The index of the choice in the list of choices.
    pub index: i32,

    /// A chat completion delta generated by streamed model responses.
    pub delta: ChatResponseChunk,

    /// Log probability information for the choice.
    pub logprobs: Option<f32>,

    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// `tool_calls` if the model called a tool, or `function_call` (deprecated)
    /// if the model called a function.
    #[serde(default)]
    pub finish_reason: Option<String>,
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
          "model": "davinci:2020-05-03",
          "choices": [
            {
              "finish_reason": "stop",
              "index": 0,
              "delta": {
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
            }
          ],
          "object": "chat.completion.chunk"
        });

        let response: ChatCompletionStreamResponse = serde_json::from_value(json).unwrap();

        let expectation = ChatCompletionStreamResponse {
            id: "chatcmpl-123".to_string(),
            created: 1677652288,
            model: "davinci:2020-05-03".to_string(),
            choices: vec![ChatCompletionStreamChoice {
                index: 0,
                logprobs: None,
                delta: ChatResponseChunk {
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
                finish_reason: Some("stop".to_string()),
            }],
            system_fingerprint: None,
            object: "chat.completion.chunk".to_string(),
        };

        assert_eq!(response, expectation);
    }
}
