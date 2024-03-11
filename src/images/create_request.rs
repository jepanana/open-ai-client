use serde::{Deserialize, Serialize};

use crate::common::ImageGenerationModel;

use super::{ImageResponseFormat, ImageSize};

/// Creates an image given a prompt.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    pub prompt: String,

    /// The model to use for image generation.
    pub model: ImageGenerationModel,

    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub number_of_variations: Option<usize>,

    /// The quality of the image that will be generated. `hd` creates images with finer details and greater consistency across the image.
    /// This param is only supported for `dall-e-3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ImageResponseFormat>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
    /// Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// The style of the generated images. Must be one of vivid or natural. Vivid causes the model to lean towards generating hyper-real and dramatic images.
    /// Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for dall-e-3.
    pub style: ImageStyle,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The style of the generated image.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageStyle {
    /// Causes the model to lean towards generating hyper-real and dramatic images.
    #[default]
    Vivid,

    /// Causes the model to produce more natural, less hyper-real looking images.
    Natural,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateImageRequest {
            prompt: "A painting of a dragon".to_string(),
            number_of_variations: Some(3),
            response_format: Some(ImageResponseFormat::Url),
            size: Some(ImageSize::S512x512),
            user: Some("user-123".to_string()),
            ..Default::default()
        };

        let request_json = serde_json::to_string(&request).unwrap();
        let json = json!({
          "prompt": "A painting of a dragon",
          "model": "dall-e-2",
          "n": 3,
          "response_format": "url",
          "size": "512x512",
          "style": "vivid",
          "user": "user-123",
        });

        assert_eq!(json.to_string(), request_json);
    }
}
