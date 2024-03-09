use crate::{
    base_client::BaseClient, AssistantsHandler, AudioHandler, ChatHandler, EmbeddingHandler,
    FileHandler, FineTuningHandler, ImagesHandler, MessagesHandler, RunsHandler, ThreadsHandler,
};

/// OpenAI client
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: BaseClient,
}

impl OpenAIClient {
    /// Create a new OpenAI client from reqwest client and host
    pub fn new(client: BaseClient) -> Self {
        Self { client }
    }

    /// Handles audio related operations
    pub fn audio(&self) -> AudioHandler {
        AudioHandler {
            client: &self.client,
        }
    }

    /// Handles chat related operations
    pub fn chat(&self) -> ChatHandler {
        ChatHandler {
            client: &self.client,
        }
    }

    /// Handles moderation related operations
    pub fn embeddings(&self) -> EmbeddingHandler {
        EmbeddingHandler {
            client: &self.client,
        }
    }

    /// Handles file related operations
    pub fn files(&self) -> FileHandler {
        FileHandler {
            client: &self.client,
        }
    }

    pub fn fine_tunning(&self) -> FineTuningHandler {
        FineTuningHandler {
            client: &self.client,
        }
    }

    pub fn images(&self) -> ImagesHandler {
        ImagesHandler {
            client: &self.client,
        }
    }

    pub fn assistants(&self) -> AssistantsHandler {
        AssistantsHandler {
            client: &self.client,
        }
    }

    pub fn threads(&self) -> ThreadsHandler {
        ThreadsHandler {
            client: &self.client,
        }
    }

    pub fn messages(&self) -> MessagesHandler {
        MessagesHandler {
            client: &self.client,
        }
    }

    pub fn runs(&self) -> RunsHandler {
        RunsHandler {
            client: &self.client,
        }
    }
}
