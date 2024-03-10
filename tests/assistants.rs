use open_ai_client::{ChatModel, CreateAssistantRequest, ModifyAssistantRequest, OpenAIClient};

pub async fn assistants_test(client: OpenAIClient) {
    let assistant_id = creates_assistant(&client).await;

    list_assistants(&client, &assistant_id).await;
    retrieve_assistant(&client, &assistant_id).await;
    modify_assistant(&client, &assistant_id).await;
    delete_assistant(&client, &assistant_id).await
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

async fn list_assistants(client: &OpenAIClient, assistant_id: &str) {
    let result = client
        .assistants()
        .list_assistants(None, None, None, None)
        .await
        .unwrap();

    assert!(result.data.len() > 0);
    assert!(result.data[0].id == assistant_id);
}

async fn retrieve_assistant(client: &OpenAIClient, assistant_id: &str) {
    let result = client
        .assistants()
        .retrieve_assistant(assistant_id)
        .await
        .unwrap();

    assert!(result.name == Some("Test Assistant".to_string()));
}

async fn modify_assistant(client: &OpenAIClient, assistant_id: &str) {
    let request = ModifyAssistantRequest {
        model: Some(ChatModel::GPT3_5Turbo0125),
        name: Some("Modified Test Assistant".to_string()),
        ..Default::default()
    };

    let result = client
        .assistants()
        .modify_assistant(assistant_id, request)
        .await
        .unwrap();

    assert!(result.name == Some("Modified Test Assistant".to_string()));
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
