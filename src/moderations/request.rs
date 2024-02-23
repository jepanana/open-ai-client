use serde::{Deserialize, Serialize};

use crate::common::ModerationModel;

/// Classifies if text violates OpenAI's Content Policy
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModerationRequest {
    /// The input text to classify
    pub input: String,

    /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.
    ///
    /// The default is `text-moderation-latest` which will be automatically upgraded over time. This ensures you are always using our most accurate model.
    /// If you use `text-moderation-stable`, we will provide advanced notice before updating the model.
    /// Accuracy of `text-moderation-stable` may be slightly lower than for `text-moderation-latest`.
    pub model: ModerationModel,
}

impl ModerationRequest {
    /// Creates a moderation request from input
    pub fn from_input<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }

    /// Sets models
    pub fn set_model(mut self, model: ModerationModel) -> Self {
        self.model = model;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = ModerationRequest::from_input("test");
        let request_json = serde_json::to_string(&request).unwrap();

        let json = json!({
            "input": "test",
            "model": "text-moderation-stable"
        });

        assert_eq!(request_json, json.to_string());
    }
}
