use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{RunError, RunStatus, TokenUsage};

/// A list of [`RunsResponse`] objects.
pub type ListRunsStepsResponse = Vec<RunsStepResponse>;

/// Represents an execution run on a [thread](https://platform.openai.com/docs/api-reference/threads).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub step_details: String,

    /// The last error associated with this run. Will be `null` if there are no errors.
    pub last_error: RunError,

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
    pub metadata: BTreeMap<String, String>,

    /// Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.).
    pub usage: TokenUsage,
}

/// Represents the type of run step.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    /// The run step is a message creation.
    MessageCreation,

    /// The run step is a tool call.
    ToolCalls,
}
