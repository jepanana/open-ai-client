//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
//! Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation)

mod handler;
mod request;
mod response;

pub use self::handler::*;
pub use self::request::*;
pub use self::response::*;
