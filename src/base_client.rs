use reqwest::{Client, Method, Response};
use reqwest_eventsource::{EventSource, RequestBuilderExt};
use serde::Serialize;
use url::Url;

use crate::OpenAIError;

/// A wrapper for request client
#[derive(Debug, Clone)]
pub(crate) struct BaseClient {
    /// The request client
    client: Client,

    /// The host of the API
    host: Url,
}

impl BaseClient {
    /// Create a new client
    pub(crate) fn new(client: Client, host: Url) -> Self {
        Self { client, host }
    }

    /// Send a an API request without a body
    pub(crate) async fn send(&self, path: &str, method: Method) -> Result<Response, OpenAIError> {
        let url = self.host.join(path)?;
        let response = self.client.request(method, url).send().await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    /// Send a an API request with a body
    pub(crate) async fn send_body<Q>(
        &self,
        request: Q,
        path: &str,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let url = self.host.join(path)?;

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
    pub(crate) async fn send_form<Q>(
        &self,
        request: Q,
        path: &str,
        method: Method,
    ) -> Result<Response, OpenAIError>
    where
        Q: TryInto<reqwest::multipart::Form, Error = OpenAIError> + std::fmt::Debug,
    {
        let url = self.host.join(path)?;

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
    pub(crate) async fn create_stream<Q>(
        &self,
        request: Q,
        path: &str,
        method: Method,
    ) -> Result<EventSource, OpenAIError>
    where
        Q: Serialize + std::fmt::Debug,
    {
        let url = self.host.join(path)?;

        let response = self
            .client
            .request(method, url)
            .json(&request)
            .eventsource()?;

        Ok(response)
    }
}
