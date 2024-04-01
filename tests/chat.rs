use std::{thread, time::Duration};

use open_ai_client::{
    chat::{ChatRequestMessage, CreateChatCompletionRequest},
    ChatModel, MessageRole, OpenAIClient,
};

pub async fn chat_completion_test(client: OpenAIClient) {
    let request = CreateChatCompletionRequest {
        model: ChatModel::GPT3_5Turbo.to_string(),
        messages: vec![
            ChatRequestMessage {
                role: MessageRole::System,
                content: "You are a helpful assistant, that response only with a yes or a no"
                    .to_string(),
                ..Default::default()
            },
            ChatRequestMessage {
                role: MessageRole::User,
                content: "Hi, is Washington DC the capital of USA".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let result = client.chat().create_chat_completion(request).await.unwrap();

    assert!(!result.choices.is_empty());
    assert_eq!(result.first_message().unwrap(), "Yes.");
}

pub async fn _chat_completion_streaming_test(client: OpenAIClient) {
    let request = CreateChatCompletionRequest {
        model: ChatModel::GPT3_5Turbo.to_string(),
        stream: Some(true),
        messages: vec![
            ChatRequestMessage {
                role: MessageRole::System,
                content: "You are a helpful assistant, that response only with a yes or a no"
                    .to_string(),
                ..Default::default()
            },
            ChatRequestMessage {
                role: MessageRole::User,
                content: "Hi, is Washington DC the capital of USA".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let mut stream = client
        .chat()
        .create_chat_completion_streaming(request)
        .await
        .unwrap();

    let mut messages = vec![];

    for _ in 0..3 {
        if let Ok(result) = stream.read_values() {
            if let Some(msg) = result.first_message() {
                messages.push(msg.clone())
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    assert!(!messages.is_empty());
    assert!(messages.join("").contains("Yes"))
}
