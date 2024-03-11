use std::collections::BTreeMap;

use open_ai_client::{CreateThreadRequest, ModifyThreadRequest, OpenAIClient, ThreadMessage};

pub async fn threads_test(client: OpenAIClient) {
    let thread_id = create_thread(&client).await;

    retrieve_thread(&client, &thread_id).await;
    modify_thread(&client, &thread_id).await;
    delete_thread(&client, &thread_id).await
}

async fn create_thread(client: &OpenAIClient) -> String {
    let messages = vec![
        ThreadMessage::user_message("Hello, what is AI?"),
        ThreadMessage::user_message("How does AI work? Explain it in simple terms."),
    ];

    let request = CreateThreadRequest::from_messages(messages);
    let result = client.threads().create_thread(request).await.unwrap();

    assert!(result.object == "thread");

    result.id
}

async fn retrieve_thread(client: &OpenAIClient, thread_id: &str) {
    let result = client.threads().retrieve_thread(thread_id).await.unwrap();

    assert!(result.object == "thread");
}

async fn modify_thread(client: &OpenAIClient, thread_id: &str) {
    let metadata = {
        let mut map = BTreeMap::new();
        let _ = map.insert("modified".to_string(), "true".to_string());
        let _ = map.insert("user".to_string(), "abc123".to_string());
        map
    };

    let request = ModifyThreadRequest::with_metadata(metadata);
    let result = client
        .threads()
        .modify_thread(thread_id, request)
        .await
        .unwrap();

    assert!(result.object == "thread");
}

async fn delete_thread(client: &OpenAIClient, thread_id: &str) {
    let result = client.threads().delete_thread(thread_id).await.unwrap();

    assert!(&result.id == thread_id);
    assert!(result.deleted == true);
}
