use reqwest::Body;
use tokio_util::codec::BytesCodec;

use crate::common::{ImageGenerationModel, OpenAIError, OpenAIFile};

use super::{ImageResponseFormat, ImageSize};

/// Creates an edited or extended image given an original image and a prompt.
#[derive(Debug)]
pub struct CreateImageEditRequest {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square. If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: OpenAIFile,

    ///A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    pub mask: Option<OpenAIFile>,

    /// The model to use for image generation. Only `dall-e-2` is supported at this time.
    pub model: ImageGenerationModel,

    /// The number of images to generate. Must be between 1 and 10.
    pub number_of_variations: Option<usize>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    pub response_format: Option<ImageResponseFormat>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    pub user: Option<String>,
}

impl TryFrom<CreateImageEditRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    fn try_from(request: CreateImageEditRequest) -> Result<Self, Self::Error> {
        let file_name = request.image.name.to_owned();
        let file_body = Body::wrap_stream(request.image.into_stream(BytesCodec::new()));

        let file_part = reqwest::multipart::Part::stream(file_body)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("prompt", request.prompt);

        if let Some(mask) = request.mask {
            let file_name = mask.name.to_owned();
            let file_body = Body::wrap_stream(mask.into_stream(BytesCodec::new()));

            let file_part = reqwest::multipart::Part::stream(file_body)
                .file_name(file_name)
                .mime_str("application/octet-stream")?;

            form = form.part("mask", file_part);
        }

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
