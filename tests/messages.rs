use std::collections::BTreeMap;

use open_ai_client::{
    messages::{CreateMessageRequest, MessageContent, ModifyMessagesRequest},
    threads::CreateThreadRequest,
    OpenAIClient,
};

pub async fn messages_test(client: OpenAIClient) {
    let thread_id = create_thread(&client).await;
    let message_id = create_message(&client, &thread_id).await;

    list_messages(&client, &thread_id, &message_id).await;
    retrieve_message(&client, &thread_id, &message_id).await;
    modify_threads_message(&client, &thread_id, &message_id).await;
    delete_thread(&client, &thread_id).await;
}

async fn create_thread(client: &OpenAIClient) -> String {
    let request = CreateThreadRequest::empty();
    let result = client.threads().create_thread(request).await.unwrap();

    assert!(result.object == "thread");

    result.id
}

async fn create_message(client: &OpenAIClient, thread_id: &str) -> String {
    let request = CreateMessageRequest::user_message("Hello, what is AI?");

    let result = client
        .messages()
        .create_message(thread_id, request)
        .await
        .unwrap();

    assert!(result.object == "thread.message");
    assert!(result.thread_id == thread_id);

    result.id
}

async fn list_messages(client: &OpenAIClient, thread_id: &str, message_id: &str) {
    let result = client
        .messages()
        .list_messages(thread_id, None, None, None, None)
        .await
        .unwrap();

    assert!(result.data.len() > 0);
    assert!(result.data[0].id == message_id);
}

async fn retrieve_message(client: &OpenAIClient, thread_id: &str, message_id: &str) {
    let result = client
        .messages()
        .retrieve_message(thread_id, message_id)
        .await
        .unwrap();

    assert!(result.id == message_id);
    match &result.content[0] {
        MessageContent::Text(text) => {
            assert!(&text.text.value == "Hello, what is AI?");
        }
        _ => assert!(false),
    }
}

async fn modify_threads_message(client: &OpenAIClient, thread_id: &str, message_id: &str) {
    let metadata = {
        let mut map = BTreeMap::new();
        let _ = map.insert("modified".to_string(), "true".to_string());
        let _ = map.insert("user".to_string(), "abc123".to_string());
        map
    };

    let request = ModifyMessagesRequest::with_metadata(metadata);
    let result = client
        .messages()
        .modify_threads_message(thread_id, message_id, request)
        .await
        .unwrap();

    assert!(result.id == message_id);
    match &result.content[0] {
        MessageContent::Text(text) => {
            assert!(&text.text.value == "Hello, what is AI?");
        }
        _ => assert!(false),
    }
}

async fn delete_thread(client: &OpenAIClient, thread_id: &str) {
    let result = client.threads().delete_thread(thread_id).await.unwrap();

    assert!(&result.id == thread_id);
    assert!(result.deleted == true);
}
