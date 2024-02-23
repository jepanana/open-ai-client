//! Manage fine-tuning jobs to tailor a model to your specific training data.
//! Related guide: [Fine-tune models](https://platform.openai.com/docs/guides/fine-tuning)

mod job_event_response;
mod job_list_response;
mod request;
mod status;

pub use self::job_event_response::*;
pub use self::job_list_response::*;
pub use self::request::*;
pub use self::status::*;
