use reqwest_eventsource::CannotCloneRequestError;

/// OpenAI errors
#[derive(Debug)]
pub enum OpenAIError {
    /// Client error
    Client(reqwest::Error),

    /// Decoding error
    Decoder(base64::DecodeError),

    /// OpenAI exception
    Exception(String),

    /// Input/Output error
    IO(std::io::Error),

    /// Serde error
    Serde(serde_json::Error),

    /// Stream request error
    StreamRequest(CannotCloneRequestError),

    /// Stream error
    StreamError(String),

    /// Stream receive error
    StreamReceiveError(tokio::sync::mpsc::error::TryRecvError),
}

impl std::fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serde(error) => error.fmt(f),
            Self::Client(error) => error.fmt(f),
            Self::Decoder(error) => error.fmt(f),
            Self::Exception(message) => message.fmt(f),
            Self::IO(error) => error.fmt(f),
            Self::StreamRequest(error) => error.fmt(f),
            Self::StreamError(message) => message.fmt(f),
            Self::StreamReceiveError(error) => error.fmt(f),
        }
    }
}

impl From<reqwest::Error> for OpenAIError {
    fn from(value: reqwest::Error) -> Self {
        Self::Client(value)
    }
}

impl From<base64::DecodeError> for OpenAIError {
    fn from(value: base64::DecodeError) -> Self {
        Self::Decoder(value)
    }
}

impl From<url::ParseError> for OpenAIError {
    fn from(value: url::ParseError) -> Self {
        Self::Exception(value.to_string())
    }
}

impl From<std::io::Error> for OpenAIError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<CannotCloneRequestError> for OpenAIError {
    fn from(value: CannotCloneRequestError) -> Self {
        Self::StreamRequest(value)
    }
}

impl From<serde_json::Error> for OpenAIError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<tokio::sync::mpsc::error::TryRecvError> for OpenAIError {
    fn from(value: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Self::StreamReceiveError(value)
    }
}

impl std::error::Error for OpenAIError {}
