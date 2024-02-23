use std::{collections::BTreeMap, time::Duration};

use reqwest::header::HeaderName;

use crate::OpenAIClient;

/// A builder for [`Client`].
#[derive(Debug)]
pub struct ClientBuilder {
    host: String,

    timeout: Duration,

    headers: BTreeMap<String, String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            headers: BTreeMap::new(),
            timeout: Duration::from_secs(10),
            host: "https://api.openai.com".into(),
        }
    }
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new(open_api_token: &str) -> Self {
        let mut headers = BTreeMap::new();
        let _ = headers.insert("Authorization".into(), format!("Bearer {}", open_api_token));
        let _ = headers.insert("OpenAI-Beta".into(), "assistants=v1".into());

        Self {
            headers,
            ..Default::default()
        }
    }

    /// Sets OpenAI model host
    pub fn host<S>(mut self, host: S) -> Self
    where
        S: Into<String>,
    {
        self.host = host.into();
        self
    }

    /// Adds organization ID to the client.
    pub fn organization<S>(mut self, organization: S) -> Self
    where
        S: Into<String>,
    {
        let _ = self
            .headers
            .insert("OpenAI-Organization".into(), organization.into());

        self
    }

    /// Extends clients headers
    pub fn headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.headers.extend(headers);

        self
    }

    /// Sets request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;

        self
    }

    /// Builds OpenAI [`Client`].
    pub fn build(self) -> Result<OpenAIClient, anyhow::Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        for (header, value) in self.headers {
            let _ = headers.insert(
                HeaderName::from_bytes(header.as_bytes())?,
                reqwest::header::HeaderValue::from_str(&value)?,
            );
        }

        let reqwest_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(self.timeout)
            .tcp_nodelay(true)
            .trust_dns(true)
            .no_proxy()
            .build()?;

        let host = reqwest::Url::parse(&self.host)?;

        Ok(OpenAIClient::new(reqwest_client, host))
    }
}
