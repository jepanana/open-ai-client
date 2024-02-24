use serde::{Deserialize, Serialize};

/// When a run has the `status: "requires_action"` and `required_action.type` is `submit_tool_outputs`,
/// this endpoint can be used to submit the outputs from the tool calls once they're all completed.
/// All outputs must be submitted in a single request.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubmitToolsRequest {
    /// A list of tools for which the outputs are being submitted.
    pub tool_outputs: Vec<ToolOutputs>,
}

/// Represents the tool outputs needed to continue a run.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ToolOutputs {
    /// The ID of the tool call in the `required_action` object within the run object the output is being submitted for.
    pub tool_call_id: Option<String>,

    /// The output of the tool call to be submitted to continue the run.
    pub output: Option<String>,
}
