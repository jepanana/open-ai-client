use reqwest::Client;

/// lol
#[derive(Debug, Clone)]
pub struct ModerationHandler<'a> {
    /// Test
    pub client: &'a Client,
}
