//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//! Related guide: [Embeddings](https://platform.openai.com/docs/guides/embeddings)

mod handler;
mod request;
mod response;

pub use self::handler::*;
pub use self::request::*;
pub use self::response::*;
