use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AssistantTool, ChatModel, RunError, RunStatus, TokenUsage};

/// A list of runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRunsResponse {
    /// Object type, which is always `list`.
    pub object: String,

    /// A list of [`RunsResponse`] objects.
    pub data: Vec<RunsResponse>,
}

/// Represents an execution run on a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsResponse {
    /// The identifier, which can be referenced in API endpoints.
    pub id: String,

    /// The object type, which is always `thread.run`.
    pub object: String,

    /// The Unix timestamp (in seconds) for when the run was created.
    pub created_at: u32,

    /// The ID of the [thread](https://platform.openai.com/docs/api-reference/threads) that was executed on as a part of this run.
    pub thread_id: String,

    /// The ID of the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for execution of this run.
    pub assistant_id: String,

    /// The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, or `expired`.
    pub status: RunStatus,

    /// Details on the action required to continue the run. Will be `null` if no action is required.
    pub required_action: Option<RunsAction>,

    /// The last error associated with this run. Will be `null` if there are no errors.
    pub last_error: Option<RunError>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub expires_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run was started.
    pub started_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub canceled_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run failed.
    pub failed_at: Option<u32>,

    /// The Unix timestamp (in seconds) for when the run was completed.
    pub completed_at: Option<u32>,

    /// The model that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub model: ChatModel,

    /// The instructions that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub instructions: Option<String>,

    /// The list of tools that the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub tools: Vec<AssistantTool>,

    /// The list of [File](https://platform.openai.com/docs/api-reference/files) IDs the [assistant](https://platform.openai.com/docs/api-reference/assistants) used for this run.
    pub file_ids: Vec<String>,

    /// Set of 16 key-value pairs that can be attached to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,

    /// Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Represents an action that can be taken to continue a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsAction {
    /// For now, this is always `submit_tool_outputs`.
    #[serde(rename = "type")]
    pub _type: String,

    /// Details on the tool outputs needed for this run to continue.
    pub submit_tool_outputs: Vec<RunsActionToolOutputs>,
}

/// Represents the tool outputs needed to continue a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsActionToolOutputs {
    /// A list of the relevant tool calls.
    pub tools_calls: Vec<RunsActionToolCall>,
}

/// Represents a tool call that the output is required for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsActionToolCall {
    /// The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](https://platform.openai.com/docs/api-reference/runs/submitToolOutputs) endpoint.
    pub id: String,

    /// The type of tool call the output is required for. For now, this is always `function`.
    pub _type: String,

    /// The function definition.
    pub function: RunsActionToolCallFunction,
}

/// Represents the function definition for a tool call that the output is required for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsActionToolCallFunction {
    /// The name of the function.
    pub name: String,

    /// The arguments that the model expects you to pass to the function.
    pub arguments: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "run_abc123",
          "object": "thread.run",
          "created_at": 1698107661,
          "assistant_id": "asst_abc123",
          "thread_id": "thread_abc123",
          "status": "completed",
          "started_at": 1699073476,
          "expires_at": null,
          "cancelled_at": null,
          "failed_at": null,
          "completed_at": 1699073498,
          "last_error": null,
          "model": "gpt-4",
          "instructions": null,
          "tools": [{"type": "code_interpreter"}],
          "file_ids": [],
          "metadata": {},
          "usage": {
            "prompt_tokens": 123,
            "completion_tokens": 456,
            "total_tokens": 579
          }
        });

        let response: RunsResponse = serde_json::from_value(json).unwrap();

        let expected_response = RunsResponse {
            id: "run_abc123".to_string(),
            object: "thread.run".to_string(),
            created_at: 1698107661,
            assistant_id: "asst_abc123".to_string(),
            thread_id: "thread_abc123".to_string(),
            status: RunStatus::Completed,
            started_at: Some(1699073476),
            expires_at: None,
            canceled_at: None,
            failed_at: None,
            completed_at: Some(1699073498),
            last_error: None,
            model: ChatModel::GPT4,
            instructions: None,
            tools: vec![AssistantTool::CodeIntepreter(Default::default())],
            file_ids: vec![],
            metadata: BTreeMap::new(),
            usage: Some(TokenUsage {
                prompt_tokens: 123,
                completion_tokens: 456,
                total_tokens: 579,
            }),
            required_action: None,
        };

        assert_eq!(response, expected_response);
    }
}
