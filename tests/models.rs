use open_ai_client::OpenAIClient;

pub async fn models_test(client: OpenAIClient) {
    let models = client.models_list().await.unwrap();

    assert!(!models.data.is_empty());
}
