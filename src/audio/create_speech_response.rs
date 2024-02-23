use std::path::Path;

use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::bytes::Bytes;

use crate::{validate_dir, validate_file_name, OpenAIError};

/// The response from the audio create speech endpoint
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSpeechResponse(pub Bytes);

impl CreateSpeechResponse {
    /// Saves the image to the specified directory. If no file extension is provided,
    /// it will default to `.mp3`.
    pub async fn save<P>(&self, dir: P, file_name: String) -> Result<(), OpenAIError>
    where
        P: AsRef<Path>,
    {
        validate_dir(dir.as_ref())?;
        validate_file_name(&file_name)?;

        let mut file_path = dir.as_ref().join(file_name);

        if file_path.extension().is_none() {
            file_path = file_path.join(".mp3");
        }

        File::create(file_path).await?.write_all(&self.0).await?;
        Ok(())
    }

    /// Returns the bytes of the response
    pub fn bytes(&self) -> &Bytes {
        &self.0
    }
}
