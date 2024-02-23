use std::{fs, path::Path};

use tokio::fs::File;
use tokio_util::codec::{Decoder, FramedRead};

use crate::OpenAIError;

/// A structure representing a file that can be uploaded to OpenAI
#[derive(Debug)]
pub struct OpenAIFile {
    /// The name of the file
    pub name: String,

    /// File handle
    pub file: File,
}

impl OpenAIFile {
    /// Creates an OpenAI file from a path
    pub async fn from_path<P>(path: P) -> Result<Self, OpenAIError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let name = path
            .file_name()
            .ok_or_else(|| {
                OpenAIError::IO(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid path: {}", path.display()),
                ))
            })?
            .to_str()
            .ok_or_else(|| {
                OpenAIError::IO(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not stringify filename",
                ))
            })?
            .to_string();

        let file = File::open(path).await?;
        Ok(Self { name, file })
    }

    /// Converts the file into a stream
    pub fn into_stream<D>(self, decoder: D) -> FramedRead<File, D>
    where
        D: Decoder,
    {
        FramedRead::new(self.file, decoder)
    }
}

/// Dowloads a file from the web and saves it
pub async fn download_file(url: &str) -> Result<tokio_util::bytes::Bytes, OpenAIError> {
    let mut response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(OpenAIError::Exception(format!(
            "Could not download file from url - {}",
            url
        )));
    }

    response.chunk().await?.ok_or_else(|| {
        OpenAIError::Exception(format!("Could not download file from url - {}", url))
    })
}

/// Validates if a file name is not empty
pub fn validate_file_name(file_name: &str) -> Result<(), OpenAIError> {
    if file_name.is_empty() {
        return Err(OpenAIError::IO(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File name cannot be empty",
        )));
    }

    Ok(())
}

/// Validates if a directory is valid:
/// - If it exists
/// - If it's a directory
/// - If it's not read-only
pub fn validate_dir<P>(dir: P) -> Result<(), OpenAIError>
where
    P: AsRef<Path>,
{
    let metadata = fs::metadata(dir.as_ref())?;

    if !dir.as_ref().exists() {
        fs::create_dir_all(dir.as_ref())?;
    }

    if !metadata.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid path: {}", dir.as_ref().display()),
        )
        .into());
    }

    if metadata.permissions().readonly() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Path is read-only: {}", dir.as_ref().display()),
        )
        .into());
    }

    Ok(())
}
