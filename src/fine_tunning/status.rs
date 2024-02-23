use serde::{Deserialize, Serialize};

/// The status of fine-tuning job
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Validating files
    ValidatingFiles,

    /// Queued
    Queued,

    /// Running
    Running,

    /// Succeeded
    Succeeded,

    /// Failed
    Failed,

    /// Cancelled
    Cancelled,
}
