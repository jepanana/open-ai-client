use reqwest::Body;
use tokio_util::codec::BytesCodec;

use crate::{common::OpenAIFile, OpenAIError};

/// Upload a file that can be used across various endpoints. The size of all the files uploaded by one organization can be up to 100 GB.
///
/// The size of individual files can be a maximum of 512 MB.
/// See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) to learn more about the types of files supported.
/// The Fine-tuning API only supports `.jsonl` files.
///
/// Please [contact us](https://help.openai.com/) if you need to increase these storage limits.
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
