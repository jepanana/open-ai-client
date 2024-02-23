use serde::{Deserialize, Serialize};

/// The response from the audio transcribe and translate endpoint
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioResponse {
    /// The text that was used to generate the audio
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_response_correctly() {
        let audio_response = AudioResponse {
            text: "Hello, world!".to_string(),
        };

        assert_eq!(audio_response.text, "Hello, world!");
    }
}
