use reqwest::Body;
use tokio_util::codec::BytesCodec;

use crate::{
    common::OpenAIFile, ImageGenerationModel, ImageResponseFormat, ImageSize, OpenAIError,
};

/// Creates a variation of a given image.
#[derive(Debug)]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
    pub image: OpenAIFile,

    /// The model to use for image generation.
    pub model: ImageGenerationModel,

    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    pub number_of_variations: Option<usize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<ImageSize>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    pub user: Option<String>,
}

impl TryFrom<CreateImageVariationRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    fn try_from(request: CreateImageVariationRequest) -> Result<Self, Self::Error> {
        let file_name = request.image.name.to_owned();
        let file_body = Body::wrap_stream(request.image.into_stream(BytesCodec::new()));

        let file_part = reqwest::multipart::Part::stream(file_body)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let mut form = reqwest::multipart::Form::new().part("file", file_part);

        if let Some(number_of_variations) = request.number_of_variations {
            form = form.text("n", number_of_variations.to_string());
        }

        if let Some(size) = request.size {
            form = form.text("size", size.to_string());
        }

        if let Some(response_format) = request.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(user) = request.user {
            form = form.text("user", user);
        }

        Ok(form)
    }
}
