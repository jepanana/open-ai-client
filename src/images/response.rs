use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{download_file, validate_dir, validate_file_name, OpenAIError};

/// Represents the url or the content of an image generated by the OpenAI API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImageResponse {
    /// The Unix timestamp (in seconds) for when the image was created.
    pub created: u32,

    /// Image response data
    pub data: Vec<ImageResponse>,
}

/// Image response format
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageResponse {
    /// The URL of the generated image, if `response_format` is `url` (default).
    Url(UrlResponse),

    /// The base64-encoded JSON of the generated image, if `response_format` is `b64_json`.
    B64Json(String),
}

/// URL response format
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlResponse {
    /// The URL of the image.
    pub url: String,

    /// The prompt that was used to generate the image, if there was any revision to the prompt.
    pub revised_prompt: Option<String>,
}

impl UrlResponse {
    /// Downloads and saves the image to the specified directory. If no file extension is provided,
    /// it will default to `.png`.
    pub async fn save<P>(&self, dir: P, file_name: String) -> Result<(), OpenAIError>
    where
        P: AsRef<Path>,
    {
        validate_dir(dir.as_ref())?;
        validate_file_name(&file_name)?;

        let downloaded_file = download_file(self.url.as_str()).await?;

        let mut file_path = dir.as_ref().join(file_name);

        if file_path.extension().is_none() {
            file_path = file_path.join(".png");
        }

        File::create(file_path)
            .await?
            .write_all(&downloaded_file)
            .await?;

        Ok(())
    }
}

/// Base64 JSON response format
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Base64JsonResponse {
    /// The base64-encoded JSON of the generated image.
    pub b64_json: String,

    /// The prompt that was used to generate the image, if there was any revision to the prompt.
    pub revised_prompt: Option<String>,
}

impl Base64JsonResponse {
    /// Saves the image to the specified directory. If no file extension is provided,
    /// it will default to `.png`.
    pub async fn save<P>(&self, dir: P, file_name: String) -> Result<(), OpenAIError>
    where
        P: AsRef<Path>,
    {
        validate_dir(dir.as_ref())?;
        validate_file_name(&file_name)?;

        let mut file_path = dir.as_ref().join(file_name);

        if file_path.extension().is_none() {
            file_path = file_path.join(".png");
        }

        let bytes = general_purpose::STANDARD.decode(&self.b64_json)?;

        File::create(file_path).await?.write_all(&bytes).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "created": 1629789140,
          "data": [
            {
              "url": "https://cdn.openai.com/ada-covid-19/1629789140_1.png",
              "revised_prompt": "test prompt"
            }
          ]
        });

        let response: CreateImageResponse = serde_json::from_value(json).unwrap();

        let expectation = CreateImageResponse {
            created: 1629789140,
            data: vec![ImageResponse::Url(UrlResponse {
                url: "https://cdn.openai.com/ada-covid-19/1629789140_1.png".to_string(),
                revised_prompt: Some("test prompt".to_string()),
            })],
        };

        assert_eq!(response, expectation);
    }
}