use open_ai_client::{
    embeddings::{CreateEmbeddingsRequest, EmbeddingInput},
    EmbeddingModel, OpenAIClient,
};

pub async fn embeddings_test(client: OpenAIClient) {
    let request = CreateEmbeddingsRequest {
        model: EmbeddingModel::TextEmbeddingAda002,
        input: EmbeddingInput::Single("The food was delicious.".to_string()),
        ..Default::default()
    };

    let result = client
        .embeddings()
        .create_embeddings(request)
        .await
        .unwrap();

    assert!(!result.data.is_empty());
    assert_eq!(result.data.first().unwrap().embedding.len(), 1536);
}
