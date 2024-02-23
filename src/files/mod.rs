//! Files are used to upload documents that can be used with features like
//! [Assistants](https://platform.openai.com/docs/api-reference/assistants) and
//! [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
mod delete_response;
mod list_response;
mod request;

pub use self::delete_response::*;
pub use self::list_response::*;
pub use self::request::*;
