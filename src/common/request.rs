use reqwest::{multipart::Form, Method};
use serde::Serialize;

use crate::{OpenAIError, OpenAIQueryParameters};

pub(crate) trait JsonTrait: Serialize + std::fmt::Debug {}
impl<T: Serialize + std::fmt::Debug> JsonTrait for T {}

pub(crate) trait FormTrait: TryInto<Form, Error = OpenAIError> + std::fmt::Debug {}
impl<T: TryInto<Form, Error = OpenAIError> + std::fmt::Debug> FormTrait for T {}

#[derive(Debug)]
pub(crate) struct OpenAIRequest<T> {
    pub(crate) body: Option<T>,
    pub(crate) query_parameters: OpenAIQueryParameters,
    pub(crate) url: String,
    pub(crate) method: Method,
}

impl<T> OpenAIRequest<T> {
    pub(crate) fn new(method: Method, url: String) -> Self {
        OpenAIRequest {
            body: None,
            query_parameters: Default::default(),
            url,
            method,
        }
    }

    pub(crate) fn with_query_parameters(mut self, query_parameters: OpenAIQueryParameters) -> Self {
        self.query_parameters = query_parameters;
        self
    }
}

impl<T: JsonTrait> OpenAIRequest<T> {
    pub(crate) fn with_body(method: Method, url: String, body: T) -> Self {
        OpenAIRequest {
            body: Some(body),
            query_parameters: Default::default(),
            url,
            method,
        }
    }
}

impl<T: FormTrait> OpenAIRequest<T> {
    pub(crate) fn with_form(method: Method, url: String, body: T) -> Self {
        OpenAIRequest {
            body: Some(body),
            query_parameters: Default::default(),
            url,
            method,
        }
    }
}
