use serde::{Deserialize, Serialize};

/// Usage statistics for the completion request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the prompt.
    #[serde(default)]
    pub prompt_tokens: i32,

    /// Number of tokens in the generated completion.
    #[serde(default)]
    pub completion_tokens: i32,

    /// Total number of tokens used in the request (prompt + completion).
    #[serde(default)]
    pub total_tokens: i32,
}
