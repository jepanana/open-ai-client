use crate::assistants_common::ThreadMessage;

/// Request for creating a message.
pub type CreateMessageRequest = ThreadMessage;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ThreadMessageRole;

    use super::*;

    #[test]
    fn serializes_request_correctly() {
        let request = CreateMessageRequest {
            role: ThreadMessageRole::User,
            content: "How does AI work? Explain it in simple terms.".into(),
            ..Default::default()
        };

        let expected = json!({
          "role": "user",
          "content": "How does AI work? Explain it in simple terms."
        });

        assert_eq!(serde_json::to_value(&request).unwrap(), expected);
    }
}
