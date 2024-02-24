use open_ai_client::{EmbeddingInput, EmbeddingModel, EmbeddingRequest, OpenAIClient};

pub async fn embeddings_test(client: OpenAIClient) {
    let request = EmbeddingRequest {
        model: EmbeddingModel::TextEmbeddingAda002,
        input: EmbeddingInput::Single("The food was delicious.".to_string()),
        encoding_format: None,
        user: None,
    };

    let result = client.embedding(request).await.unwrap();

    assert!(!result.data.is_empty());
    assert_eq!(result.data.first().unwrap().embedding.len(), 1536);
}