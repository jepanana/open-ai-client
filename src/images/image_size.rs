use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Image sizes accepted by dalle2 and dalle3 models
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum ImageSize {
    /// 256x256
    #[serde(rename = "256x256")]
    S256x256,

    /// 512x512
    #[serde(rename = "512x512")]
    S512x512,

    /// 1024x1024
    #[serde(rename = "1024x1024")]
    #[default]
    S1024x1024,

    /// 1792x1024
    #[serde(rename = "1792x1024")]
    S1792x1024,

    /// 1024x1792
    #[serde(rename = "1024x1792")]
    S1024x1792,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageSize::S256x256 => write!(f, "256x256"),
            ImageSize::S512x512 => write!(f, "512x512"),
            ImageSize::S1024x1024 => write!(f, "1024x1024"),
            ImageSize::S1792x1024 => write!(f, "1792x1024"),
            ImageSize::S1024x1792 => write!(f, "1024x1792"),
        }
    }
}
