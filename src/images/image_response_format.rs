use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Image response format.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    /// Returns a URL to the image.
    Url,

    /// Returns a base64-encoded JSON object containing the image.
    B64Json,
}

impl Display for ImageResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageResponseFormat::Url => write!(f, "url"),
            ImageResponseFormat::B64Json => write!(f, "b64_json"),
        }
    }
}
