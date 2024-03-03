use reqwest::{Client, Method, Response};
use reqwest_eventsource::{EventSource, RequestBuilderExt};
use serde::Serialize;
use url::Url;

use crate::OpenAIError;

/// A wrapper for request client
#[derive(Debug, Clone)]
pub struct BaseClient {
    /// The request client
    client: Client,
}

impl BaseClient {
    /// Create a new client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send a an API request without a body
    pub async fn send(&self, url: Url, method: Method) -> Result<Response, OpenAIError> {
        let response = self.client.request(method, url).send().await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    /// Send a an API request with a body
    pub async fn send_body<Q>(
        &self,
        request: Q,
        url: Url,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    /// Send a an API request with a form
    pub async fn send_form<Q>(
        &self,
        request: Q,
        url: Url,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: TryInto<reqwest::multipart::Form, Error = OpenAIError> + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .multipart(request.try_into()?)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    /// Send a body to create an event stream
    pub async fn create_stream<Q>(
        &self,
        request: Q,
        url: Url,
        method: Method,
    ) -> Result<EventSource, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let response = self
            .client
            .request(method, url)
            .json(&request)
            .eventsource()?;

        Ok(response)
    }
}
