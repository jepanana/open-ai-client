//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
//! Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation)

mod request;
mod response;

pub use self::request::*;
pub use self::response::*;
