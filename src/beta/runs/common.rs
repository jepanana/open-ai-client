use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the status of a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    /// The run is queued and waiting to be executed.
    Queued,

    /// The run is in progress.
    InProgress,

    /// The run requires action to continue.
    RequiresAction,

    /// The run is in the process of being cancelled.
    Cancelling,

    /// The run was cancelled.
    Cancelled,

    /// The run failed.
    Failed,

    /// The run completed successfully.
    Completed,

    /// The run expired.
    Expired,
}

impl Display for RunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents an error that occurred during the execution of a run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunError {
    /// One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`.
    pub code: RunErrorCode,

    /// A human-readable description of the error.
    pub message: String,
}

/// Represents the error code for a run error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunErrorCode {
    /// A server error occurred.
    ServerError,

    /// The rate limit was exceeded.
    RateLimitExceeded,

    /// The prompt provided was invalid.
    InvalidPrompt,
}
