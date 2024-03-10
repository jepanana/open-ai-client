use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AssistantTool, RunError, RunStatus, TokenUsage};

/// A list of [`RunsResponse`] objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRunsStepsResponse {
    /// Object type, which is always `list`.
    pub object: String,

    /// A list of [`RunsStepResponse`] objects.
    pub data: Vec<RunsStepResponse>,
}

/// Represents an execution run on a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsStepResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.run.step`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the run was created.
    pub created_at: u32,

    /// The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) that was executed on as a part of this run.
    pub thread_id: String,

    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for execution of this run.
    pub assistant_id: String,

    /// The ID of the [run](https://platform.openai.com/docs/api-reference/runs) that this run step is a part of.
    pub run_id: String,

    /// The type of run step, which can be either `message_creation` or `tool_calls`.
    #[serde(rename = "type")]
    pub _type: StepType,

    /// The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`.
    pub status: RunStatus,

    /// The details of the run step.
    pub step_details: StepDetails,

    /// The last error associated with this run. Will be `null` if there are no errors.
    pub last_error: Option<RunError>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub expired_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub canceled_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run failed.
    pub failed_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run was completed.
    pub completed_at: Option<u32>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,

    /// Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Represents the details of a run step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepDetails {
    /// The run step is a message creation.
    MessageCreation(MessageCreationDetails),
    /// The run step is a tool call.
    ToolCalls(ToolCallsDetails),
}

/// Represents the details of a message creation run step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageCreationDetails {
    /// Always `message_creation`.
    #[serde(rename = "type")]
    pub _type: String,

    /// The details of the message creation.
    pub message_creation: MessageCreation,
}

/// Represents the details of a tool call run step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageCreation {
    /// The identifier of the message that was created.
    pub message_id: String,
}

/// Represents the details of a tool call run step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallsDetails {
    /// Always `tool_calls`.
    #[serde(rename = "type")]
    pub _type: String,

    /// An array of tool calls the run step was involved in.
    /// These can be associated with one of three types of tools: `code_interpreter`, `retrieval`, or `function`.
    pub tool_calls: Vec<AssistantTool>,
}

/// Represents the type of run step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    /// The run step is a message creation.
    MessageCreation,

    /// The run step is a tool call.
    ToolCalls,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "step_abc123",
          "object": "thread.run.step",
          "created_at": 1699063291,
          "run_id": "run_abc123",
          "assistant_id": "asst_abc123",
          "thread_id": "thread_abc123",
          "type": "message_creation",
          "status": "completed",
          "cancelled_at": null,
          "completed_at": 1699063291,
          "expired_at": null,
          "failed_at": null,
          "last_error": null,
          "step_details": {
            "type": "message_creation",
            "message_creation": {
              "message_id": "msg_abc123"
            }
          },
          "usage": {
            "prompt_tokens": 123,
            "completion_tokens": 456,
            "total_tokens": 579
          }
        });

        let response: RunsStepResponse = serde_json::from_value(json).unwrap();

        let expected_response = RunsStepResponse {
            id: "step_abc123".to_string(),
            object: "thread.run.step".to_string(),
            created_at: 1699063291,
            run_id: "run_abc123".to_string(),
            assistant_id: "asst_abc123".to_string(),
            thread_id: "thread_abc123".to_string(),
            _type: StepType::MessageCreation,
            status: RunStatus::Completed,
            canceled_at: None,
            completed_at: Some(1699063291),
            expired_at: None,
            failed_at: None,
            last_error: None,
            step_details: StepDetails::MessageCreation(MessageCreationDetails {
                _type: "message_creation".to_string(),
                message_creation: MessageCreation {
                    message_id: "msg_abc123".to_string(),
                },
            }),
            metadata: Default::default(),
            usage: Some(TokenUsage {
                prompt_tokens: 123,
                completion_tokens: 456,
                total_tokens: 579,
            }),
        };

        assert_eq!(response, expected_response);
    }
}
