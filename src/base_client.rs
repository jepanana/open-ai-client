use reqwest::{Client, Response};
use reqwest_eventsource::{EventSource, RequestBuilderExt};
use serde::Serialize;
use url::Url;

use crate::{common::OpenAIError, OpenAIRequest};

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
    pub(crate) async fn send<T>(&self, request: OpenAIRequest<T>) -> Result<Response, OpenAIError>
    where
        T: Serialize + std::fmt::Debug,
    {
        let url = self.host.join(&request.url)?;
        let query_parameters = request.query_parameters.to_query();
        let mut request_builder = self.client.request(request.method, url);

        if let Some(body) = request.body {
            request_builder = request_builder.json(&body);
        }

        if !query_parameters.is_empty() {
            request_builder = request_builder.query(&query_parameters);
        }

        let response = request_builder.send().await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            warn!(error = %error, "OpenAI responded with an error");

            return Err(OpenAIError::Exception(error));
        }

        Ok(response)
    }

    /// Send a an API request with a form
    pub(crate) async fn send_form<T>(
        &self,
        request: OpenAIRequest<T>,
    ) -> Result<Response, OpenAIError>
    where
        T: TryInto<reqwest::multipart::Form, Error = OpenAIError> + std::fmt::Debug,
    {
        let url = self.host.join(&request.url)?;
        let form = request
            .body
            .ok_or_else(|| OpenAIError::Exception("No body provided".to_string()))?
            .try_into()?;

        let response = self
            .client
            .request(request.method.clone(), url)
            .multipart(form)
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
    pub(crate) async fn create_stream<T>(
        &self,
        request: OpenAIRequest<T>,
    ) -> Result<EventSource, OpenAIError>
    where
        T: Serialize + std::fmt::Debug,
    {
        let url = self.host.join(&request.url)?;
        let query_parameters = request.query_parameters.to_query();
        let mut request_builder = self.client.request(request.method, url);

        if let Some(body) = request.body {
            request_builder = request_builder.json(&body);
        }

        if !query_parameters.is_empty() {
            request_builder = request_builder.query(&query_parameters);
        }

        let response = request_builder.eventsource()?;

        Ok(response)
    }
}
