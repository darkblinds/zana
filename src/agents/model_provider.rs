use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Interface for external AI models.
pub struct ModelProvider {
    pub api_url: String,
    pub client: Client,
}

impl ModelProvider {
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn decide(
        &self,
        memory: &HashMap<String, String>,
        environment: &HashMap<String, String>,
    ) -> Result<String, reqwest::Error> {
        let payload = json!({
            "memory": memory,
            "environment": environment,
        });

        let response: Value = self
            .client
            .post(&self.api_url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(response["action"].as_str().unwrap_or("idle").to_string())
    }
}
