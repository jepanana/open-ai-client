use std::collections::BTreeMap;

use open_ai_client::{
    ChatModel, CreateAssistantRequest, CreateRunsRequest, CreateThreadAndRunRequest,
    CreateThreadRequest, ModifyRunsRequest, OpenAIClient, RunStatus, ThreadMessage,
};

pub async fn runs_test(client: OpenAIClient) {
    let assistant_id = creates_assistant(&client).await;
    let thread_id = create_thread(&client).await;
    let run_id = create_run(&client, &assistant_id, &thread_id).await;

    create_and_run(&client, &assistant_id).await;
    list_runs(&client, &thread_id).await;

    std::thread::sleep(std::time::Duration::from_secs(5));

    let run_step_id = list_run_steps(&client, &thread_id, &run_id).await;
    retrieve_run(&client, &thread_id, &run_id).await;
    retrieve_run_step(&client, &thread_id, &run_id, &run_step_id).await;
    modify_run(&client, &thread_id, &run_id).await;

    delete_thread(&client, &thread_id).await;
    delete_assistant(&client, &assistant_id).await;
}

async fn creates_assistant(client: &OpenAIClient) -> String {
    let mut request = CreateAssistantRequest::with_instructions(
        ChatModel::GPT3_5Turbo0125,
        "Test Assistant Instructions",
    );

    request.name = Some("Test Assistant".to_string());

    let result = client.assistants().create_assistant(request).await.unwrap();

    assert!(result.model == ChatModel::GPT3_5Turbo0125);
    assert!(result.name == Some("Test Assistant".to_string()));

    result.id
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

async fn create_run(client: &OpenAIClient, assistant_id: &str, thread_id: &str) -> String {
    let request = CreateRunsRequest::for_assistant(assistant_id.to_string());
    let result = client.runs().create_run(thread_id, request).await.unwrap();

    assert!(result.object == "thread.run");
    assert!(result.status == RunStatus::Queued);

    result.id
}

async fn create_and_run(client: &OpenAIClient, assistant_id: &str) {
    let request = CreateThreadAndRunRequest::for_thread(assistant_id, CreateThreadRequest::empty());

    let result = client.runs().create_thread_and_run(request).await.unwrap();

    assert!(result.object == "thread.run");
    assert!(result.status == RunStatus::Queued);
}

async fn list_runs(client: &OpenAIClient, thread_id: &str) {
    let result = client
        .runs()
        .list_runs(thread_id, None, None, None, None)
        .await
        .unwrap();

    assert!(result.object == "list");
    assert!(result.data.len() > 0);
    assert!(result.data.iter().any(|x| x.thread_id == thread_id));
}

async fn list_run_steps(client: &OpenAIClient, thread_id: &str, run_id: &str) -> String {
    let result = client
        .runs()
        .list_run_steps(thread_id, run_id)
        .await
        .unwrap();

    assert!(result.object == "list");
    assert!(result.data.len() > 0);

    let run_step_id = result.data.into_iter().find(|x| x.run_id == run_id);
    assert!(run_step_id.is_some());

    run_step_id.unwrap().id
}

async fn retrieve_run(client: &OpenAIClient, thread_id: &str, run_id: &str) {
    let result = client.runs().retrieve_run(thread_id, run_id).await.unwrap();

    assert!(result.id == run_id);
    assert!(result.thread_id == thread_id);
    assert!(result.status != RunStatus::Queued);
}

async fn retrieve_run_step(client: &OpenAIClient, thread_id: &str, run_id: &str, step_id: &str) {
    let result = client
        .runs()
        .retrieve_run_step(thread_id, run_id, step_id)
        .await
        .unwrap();

    assert!(result.id == step_id);
    assert!(result.run_id == run_id);
    assert!(result.object == "thread.run.step");
    assert!(result.status != RunStatus::Queued);
}

async fn modify_run(client: &OpenAIClient, thread_id: &str, run_id: &str) {
    let metadata = {
        let mut map = BTreeMap::new();
        let _ = map.insert("modified".to_string(), "true".to_string());
        let _ = map.insert("user".to_string(), "abc123".to_string());
        map
    };

    let request = ModifyRunsRequest::with_metadata(metadata);
    let result = client
        .runs()
        .modify_run(thread_id, run_id, request)
        .await
        .unwrap();

    assert!(result.id == run_id);
    assert!(result.status != RunStatus::Queued);
}

async fn delete_thread(client: &OpenAIClient, thread_id: &str) {
    let result = client.threads().delete_thread(thread_id).await.unwrap();

    assert!(&result.id == thread_id);
    assert!(result.deleted == true);
}

async fn delete_assistant(client: &OpenAIClient, assistant_id: &str) {
    let result = client
        .assistants()
        .delete_assistant(assistant_id)
        .await
        .unwrap();

    assert!(&result.id == assistant_id);
    assert!(result.deleted == true);
}
