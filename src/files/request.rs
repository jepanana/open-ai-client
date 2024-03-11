use reqwest::Body;
use tokio_util::codec::BytesCodec;

use crate::common::{OpenAIError, OpenAIFile};

/// Request to upload a file to the OpenAI API.
#[derive(Debug)]
pub struct FilesUploadRequest {
    /// The File object (not file name) to be uploaded.
    pub file: OpenAIFile,

    /// The intended purpose of the uploaded file.
    ///
    /// Use "fine-tune" for [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
    /// and "assistants" for [Assistants](https://platform.openai.com/docs/api-reference/assistants)
    /// and [Messages](https://platform.openai.com/docs/api-reference/messages).
    /// This allows us to validate the format of the uploaded file is correct for fine-tuning.
    pub purpose: String,
}

impl TryFrom<FilesUploadRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    fn try_from(request: FilesUploadRequest) -> Result<Self, Self::Error> {
        let file_name = request.file.name.to_owned();
        let file_body = Body::wrap_stream(request.file.into_stream(BytesCodec::new()));

        let file_part = reqwest::multipart::Part::stream(file_body)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("purpose", request.purpose);

        Ok(form)
    }
}
