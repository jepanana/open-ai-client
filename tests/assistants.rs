pub async fn assistants_test(client: OpenAIClient) {
    let request = CreateAssistantRequest {
        model: AssistantModel::Curie,
        messages: vec![AssistantMessage {
            role: AssistantRole::User,
            content: "What is the meaning of life?".to_string(),
        }],
        max_tokens: Some(50),
        temperature: Some(0.7),
        stop: Some(vec!["\n".to_string()]),
        response_format: AssistantResponseFormat::Json,
    };

    let result = client.assistants().create_assistant(request).await.unwrap();

    assert!(!result.choices.is_empty());
}
