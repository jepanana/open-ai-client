use futures_util::StreamExt;
use reqwest_eventsource::Event;

use super::OpenAIError;

/// OpenAIStream is a wrapper around [`tokio::sync::mpsc::UnboundedReceiver<Result<T, OpenAIError>>`] that
/// provides a convenient way to read values from stream.
#[derive(Debug)]
pub struct OpenAIStream<T> {
    rx: tokio::sync::mpsc::UnboundedReceiver<Result<T, OpenAIError>>,
}

impl<T> OpenAIStream<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug + Send + Sync + 'static,
{
    /// Creates a new receiver for data from the stream.
    pub async fn new(event_source: reqwest_eventsource::EventSource) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let handle = tokio::spawn(async move { handle_event_source(event_source, tx).await });
        drop(handle);

        Self { rx }
    }

    /// Reads values from the stream.
    pub fn read_values(&mut self) -> Result<T, OpenAIError> {
        self.rx.try_recv()?
    }
}

async fn handle_event_source<T>(
    mut event_source: reqwest_eventsource::EventSource,
    tx: tokio::sync::mpsc::UnboundedSender<Result<T, OpenAIError>>,
) where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    while let Some(es) = event_source.next().await {
        match es {
            Err(e) => {
                if let Err(_e) = tx.send(Err(OpenAIError::StreamError(e.to_string()))) {
                    break;
                }
            }
            Ok(event) => match event {
                Event::Open => continue,
                Event::Message(message) => {
                    if message.data == "[DONE]" {
                        break;
                    }
                    let event_data = serde_json::from_str::<T>(&message.data).map_err(Into::into);

                    if let Err(_e) = tx.send(event_data) {
                        break;
                    }
                }
            },
        }
    }

    event_source.close();
}
