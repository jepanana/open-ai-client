use crate::{
    audio::AudioHandler,
    base_client::BaseClient,
    beta::{
        assistants::AssistantsHandler, messages::MessagesHandler, runs::RunsHandler,
        threads::ThreadsHandler,
    },
    chat::ChatHandler,
    embeddings::EmbeddingsHandler,
    files::FileHandler,
    fine_tunning::FineTuningHandler,
    images::ImagesHandler,
    models::ModelsHandler,
    moderations::ModerationsHandler,
};

/// OpenAI client
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: BaseClient,
}

impl OpenAIClient {
    /// Create a new OpenAI client from reqwest client and host
    pub(crate) fn new(client: BaseClient) -> Self {
        Self { client }
    }

    /// Handles audio related operations
    #[cfg(feature = "audio")]
    pub fn audio(&self) -> AudioHandler {
        AudioHandler::new(&self.client)
    }

    /// Handles chat related operations
    #[cfg(feature = "chat")]
    pub fn chat(&self) -> ChatHandler {
        ChatHandler::new(&self.client)
    }

    /// Handles moderation related operations
    #[cfg(feature = "embeddings")]
    pub fn embeddings(&self) -> EmbeddingsHandler {
        EmbeddingsHandler::new(&self.client)
    }

    /// Handles file related operations
    #[cfg(feature = "files")]
    pub fn files(&self) -> FileHandler {
        FileHandler::new(&self.client)
    }

    /// Handles fine-tuning related operations
    #[cfg(feature = "fine_tunning")]
    pub fn fine_tunning(&self) -> FineTuningHandler {
        FineTuningHandler::new(&self.client)
    }

    /// Handles images related operations
    #[cfg(feature = "images")]
    pub fn images(&self) -> ImagesHandler {
        ImagesHandler::new(&self.client)
    }

    /// Handles models related operations
    #[cfg(feature = "models")]
    pub fn models(&self) -> ModelsHandler {
        ModelsHandler::new(&self.client)
    }

    /// Handles moderation related operations
    #[cfg(feature = "moderations")]
    pub fn moderation(&self) -> ModerationsHandler {
        ModerationsHandler::new(&self.client)
    }

    /// Handles assistants related operations
    #[cfg(feature = "assistants")]
    pub fn assistants(&self) -> AssistantsHandler {
        AssistantsHandler::new(&self.client)
    }

    /// Handles threads related operations
    #[cfg(feature = "messages")]
    pub fn threads(&self) -> ThreadsHandler {
        ThreadsHandler::new(&self.client)
    }

    /// Handles messages related operations
    #[cfg(feature = "runs")]
    pub fn messages(&self) -> MessagesHandler {
        MessagesHandler::new(&self.client)
    }

    /// Handles runs related operations
    #[cfg(feature = "threads")]
    pub fn runs(&self) -> RunsHandler {
        RunsHandler::new(&self.client)
    }
}
