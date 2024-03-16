use serde::{Deserialize, Serialize};

/// Get status updates for a fine-tuning job.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFineTunningJobEventResponse {
    /// The object type, which is always "list".
    pub object: String,

    /// A list of fine-tuning job events.
    pub data: Vec<FineTuningJobEventResponse>,
}

/// Fine tunning job event message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobEventResponse {
    /// Id  
    pub id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job event was created.
    pub created_at: u32,

    /// Level
    pub level: String,

    /// Message
    pub message: String,

    /// The object type, which is always "fine_tuning.job.event".
    pub object: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_fine_tunning_job_event_response_correctly() {
        let json = json!({
          "object": "event",
          "id": "ftevent-abc123",
          "created_at": 1677610602,
          "level": "info",
          "message": "Created fine-tuning job"
        });

        let response: FineTuningJobEventResponse = serde_json::from_value(json).unwrap();

        let expectation = FineTuningJobEventResponse {
            object: "event".into(),
            id: "ftevent-abc123".into(),
            created_at: 1677610602,
            level: "info".into(),
            message: "Created fine-tuning job".into(),
        };

        assert_eq!(response, expectation)
    }
}
