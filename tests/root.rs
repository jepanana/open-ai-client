use std::time::Duration;

use open_ai_client::{ClientBuilder, OpenAIClient};

mod assistants;
mod audio;
mod chat;
mod embeddings;
mod messages;
mod models;
mod moderations;
mod threads;

fn create_client() -> OpenAIClient {
    // let open_ai_token = std::env::var("OPEN_AI_TOKEN").expect("OPEN_AI_TOKEN not set");
    let open_ai_token = "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";

    ClientBuilder::new(&open_ai_token)
        .timeout(Duration::from_secs(20))
        .enable_beta()
        .build()
        .unwrap()
}

#[tokio::test]
async fn audio_create_speech() {
    let client = create_client();
    audio::audio_create_speech_test(client).await;
}

#[tokio::test]
async fn audio_transcriptions() {
    let client = create_client();
    audio::audio_transcriptions_test(client).await;
}

#[tokio::test]
async fn audio_translations() {
    let client = create_client();
    audio::audio_translations_test(client).await;
}

#[tokio::test]
async fn chat_completion() {
    let client = create_client();
    chat::chat_completion_test(client).await;
}

// Not sure how to fix this yet, but the test works if one awaits it
// #[tokio::test]
// async fn chat_completion_streaming() {
//     let client = create_client();
//     chat_completion::chat_completion_streaming_test(client).await;
// }

#[tokio::test]
async fn embeddings() {
    let client = create_client();
    embeddings::embeddings_test(client).await;
}

#[tokio::test]
async fn models() {
    let client = create_client();
    models::models_test(client).await;
}

#[tokio::test]
async fn moderation() {
    let client = create_client();
    moderations::moderations_test(client).await;
}

#[tokio::test]
async fn assistant() {
    let client = create_client();
    assistants::assistants_test(client).await;
}

#[tokio::test]
async fn threads() {
    let client = create_client();
    threads::threads_test(client).await;
}

#[tokio::test]
async fn messages() {
    let client = create_client();
    messages::messages_test(client).await;
}
