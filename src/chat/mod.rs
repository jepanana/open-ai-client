//! Given a list of messages comprising a conversation, the model will return a response.
//! Related guide: [Chat completions](https://platform.openai.com/docs/api-reference/chat)

mod handler;
mod request;
mod response;
mod response_chunk;
mod response_format;
mod streaming_response;
mod tooling;

pub use self::handler::*;
pub use self::request::*;
pub use self::response::*;
pub use self::response_chunk::*;
pub use self::response_format::ResponseFormat as ChatResponseFormat;
pub use self::response_format::ResponseFormatType as ChatResponseFormatType;
pub use self::streaming_response::*;
pub use self::tooling::*;
