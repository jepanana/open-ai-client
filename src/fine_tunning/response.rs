use serde::{Deserialize, Serialize};

use super::Status;

/// List your organization's fine-tuning jobs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobListResponse {
    /// The object type, which is always "list".
    pub object: String,

    /// A list of fine-tuning jobs.
    pub data: Vec<FineTuningJobResponse>,
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobResponse {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: u32,

    /// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
    pub error: Option<FineTunningError>,

    /// The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running.
    pub fine_tuned_model: Option<String>,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running.
    pub finished_at: Option<u32>,

    /// The hyperparameters used for the fine-tuning job.
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    pub hyperparameters: Option<ResponseHyperParameters>,

    /// The base model that is being fine-tuned.
    pub model: String,

    /// The object type, which is always "fine_tuning.job".
    pub object: String,

    /// The organization that owns the fine-tuning job.
    pub organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    /// You can retrieve the results with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub result_files: Vec<String>,

    /// The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`.
    pub status: Status,

    /// The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running.
    pub trained_tokens: Option<u32>,

    /// The file ID used for training. You can retrieve the training data with the
    /// [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub training_file: String,

    /// The file ID used for validation. You can retrieve the validation results with the
    /// [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub validation_file: Option<String>,
}

/// Information about why the fine-tuning job failed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTunningError {
    /// A machine-readable error code.
    pub code: String,

    /// A human-readable error message.
    pub message: String,

    /// The parameter that was invalid, usually `training_file` or `validation_file`.
    /// This field will be null if the failure was not parameter-specific.
    pub param: String,
}

/// The hyperparameters used for the fine-tuning job
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseHyperParameters {
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    /// "auto" decides the optimal number of epochs based on the size of the dataset. If setting the number manually,
    /// we support any number between 1 and 50 epochs.
    pub n_epochs: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_fine_tunning_job_response_correctly() {
        let json = json!({
          "object": "fine_tuning.job",
          "id": "ftjob-abc123",
          "model": "davinci-002",
          "created_at": 1692661014,
          "finished_at": 1692661190,
          "fine_tuned_model": "ft:davinci-002:my-org:custom_suffix:7q8mpxmy",
          "organization_id": "org-123",
          "result_files": [
              "file-abc123"
          ],
          "status": "succeeded",
          "validation_file": null,
          "training_file": "file-abc123",
          "hyperparameters": {
              "n_epochs": 4,
          },
          "trained_tokens": 5768
        });

        let response: FineTuningJobResponse = serde_json::from_value(json).unwrap();

        let expectation = FineTuningJobResponse {
            object: "fine_tuning.job".into(),
            id: "ftjob-abc123".into(),
            model: "davinci-002".into(),
            created_at: 1692661014,
            finished_at: Some(1692661190),
            fine_tuned_model: Some("ft:davinci-002:my-org:custom_suffix:7q8mpxmy".into()),
            organization_id: "org-123".into(),
            result_files: vec!["file-abc123".into()],
            status: Status::Succeeded,
            validation_file: None,
            training_file: "file-abc123".into(),
            hyperparameters: Some(ResponseHyperParameters { n_epochs: 4 }),
            trained_tokens: Some(5768),
            error: None,
        };

        assert_eq!(response, expectation);
    }
}
