//! Learn how to turn audio into text or text into audio.
//! Related guide: [Speech to text](https://platform.openai.com/docs/guides/speech-to-text)

mod create_speech_request;
mod create_speech_response;
mod response;
mod response_format;
mod transcription_request;
mod translation_request;

pub use self::create_speech_request::*;
pub use self::create_speech_response::*;
pub use self::response::*;
pub use self::response_format::ResponseFormat as AudioResponseFormat;
pub use self::transcription_request::*;
pub use self::translation_request::*;
