use reqwest::Method;

use crate::{base_client::BaseClient, common::OpenAIError, OpenAIRequest};

use super::{
    CreateImageEditRequest, CreateImageRequest, CreateImageVariationRequest, ImageResponse,
};

const IMAGES_GENERATION_URL: &str = "/v1/images/generations";
const IMAGES_EDIT_IMAGES_URL: &str = "/v1/images/edit";
const IMAGES_VARIATIONS_URL: &str = "/v1/images/variations";

/// Images handler for OpenAI API
#[derive(Debug, Clone)]
pub struct ImagesHandler<'a> {
    client: &'a BaseClient,
}

impl<'a> ImagesHandler<'a> {
    pub(crate) fn new(client: &'a BaseClient) -> Self {
        Self { client }
    }

    /// Creates an image given a prompt.
    pub async fn create_image_request(
        &self,
        request: CreateImageRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_body(Method::POST, IMAGES_GENERATION_URL.to_string(), request);

        let response = self.client.send(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Creates an edited or extended image given an original image and a prompt.
    pub async fn create_image_edit(
        &self,
        request: CreateImageEditRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_form(Method::POST, IMAGES_EDIT_IMAGES_URL.to_string(), request);

        let response = self.client.send_form(openai_request).await;

        Ok(response?.json().await?)
    }

    /// Creates a variation of a given image.
    pub async fn create_image_variation(
        &self,
        request: CreateImageVariationRequest,
    ) -> Result<ImageResponse, OpenAIError> {
        let openai_request =
            OpenAIRequest::with_form(Method::POST, IMAGES_VARIATIONS_URL.to_string(), request);

        let response = self.client.send_form(openai_request).await;

        Ok(response?.json().await?)
    }
}
