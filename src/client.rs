use crate::{
    base_client::BaseClient, AssistantsHandler, AudioHandler, ChatHandler, EmbeddingHandler,
    FileHandler, FineTuningHandler, ImagesHandler, MessagesHandler, ModelsHandler,
    ModerationsHandler, RunsHandler, ThreadsHandler,
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
    pub fn audio(&self) -> AudioHandler {
        AudioHandler::new(&self.client)
    }

    /// Handles chat related operations
    pub fn chat(&self) -> ChatHandler {
        ChatHandler::new(&self.client)
    }

    /// Handles moderation related operations
    pub fn embeddings(&self) -> EmbeddingHandler {
        EmbeddingHandler::new(&self.client)
    }

    /// Handles file related operations
    pub fn files(&self) -> FileHandler {
        FileHandler::new(&self.client)
    }

    /// Handles fine-tuning related operations
    pub fn fine_tunning(&self) -> FineTuningHandler {
        FineTuningHandler::new(&self.client)
    }

    /// Handles images related operations
    pub fn images(&self) -> ImagesHandler {
        ImagesHandler::new(&self.client)
    }

    /// Handles models related operations
    pub fn models(&self) -> ModelsHandler {
        ModelsHandler::new(&self.client)
    }

    /// Handles moderation related operations
    pub fn moderation(&self) -> ModerationsHandler {
        ModerationsHandler::new(&self.client)
    }

    /// Handles assistants related operations
    pub fn assistants(&self) -> AssistantsHandler {
        AssistantsHandler::new(&self.client)
    }

    /// Handles threads related operations
    pub fn threads(&self) -> ThreadsHandler {
        ThreadsHandler::new(&self.client)
    }

    /// Handles messages related operations
    pub fn messages(&self) -> MessagesHandler {
        MessagesHandler::new(&self.client)
    }

    /// Handles runs related operations
    pub fn runs(&self) -> RunsHandler {
        RunsHandler::new(&self.client)
    }
}
