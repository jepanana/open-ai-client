use open_ai_client::{ModerationRequest, OpenAIClient};

pub async fn moderations_test(client: OpenAIClient) {
    let request = ModerationRequest {
        input: "This is a sexually explicit text. A nude person".to_string(),
        ..Default::default()
    };

    let result = client.moderation(request).await.unwrap();

    assert!(!result.results.is_empty());
    assert!(result.results.first().unwrap().flagged);
    assert!(result.results.first().unwrap().categories.sexual);
}
