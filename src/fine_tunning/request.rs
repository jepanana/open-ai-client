use serde::{Deserialize, Serialize};

/// Creates a job that fine-tunes a specified model from a given dataset.
///
/// Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.
/// [Learn more about fine-tuning](https://platform.openai.com/docs/guides/fine-tuning)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateFineTunningJobRequest {
    /// The name of the model to fine-tune. You can select one of the
    /// [supported models](https://platform.openai.com/docs/guides/fine-tuning/what-models-can-be-fine-tuned).
    pub model: SupportedModels,

    /// The ID of an uploaded file that contains training data.
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    pub training_file: String,

    /// The hyperparameters used for the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<RequestHyperparameters>,

    /// A string of up to 18 characters that will be added to your fine-tuned model name.
    /// For example, a `suffix` of "custom-model-name" would produce a model name like `ft:gpt-3.5-turbo:openai:custom-model-name:7p4lURel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation metrics periodically during fine-tuning.
    /// These metrics can be viewed in the fine-tuning results file. The same data should not be present in both train and validation files.
    /// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

/// Parameters used in fine tunning jobs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestHyperparameters {
    /// Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<f32>,

    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<u32>,
}

/// Models supported for fine tunning
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SupportedModels {
    /// /// The latest GPT-3.5 Turbo model with improved instruction following, JSON mode, reproducible outputs, parallel function calling, and more.
    #[serde(rename = "gpt-3.5-turbo-1106")]
    GPT3_5Turbo1106,

    /// Snapshot of gpt-3.5-turbo from June 13th 2023. Will be deprecated
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GPT3_5Turbo0613,

    /// Conversational base model
    #[serde(rename = "babbage-002")]
    Babbage002,

    /// Conversational base model
    #[serde(rename = "davinci-002")]
    Davinci002,

    /// Snapshot of gpt-4 from June 13th 2023 with improved function calling support.
    #[serde(rename = "gpt-4-0613")]
    GPT4_0613,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateFineTunningJobRequest {
            model: SupportedModels::Davinci002,
            training_file: "file-abc123".to_string(),
            hyperparameters: None,
            suffix: None,
            validation_file: None,
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
          "model": "davinci-002",
          "training_file": "file-abc123",
        });

        assert_eq!(request_json, json.to_string());
    }
}
