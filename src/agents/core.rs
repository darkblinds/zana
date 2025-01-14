use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Represents an AI model that communicates with an external API.
pub struct AIModel {
    pub api_url: String,
    pub client: Client,
}

impl AIModel {
    /// Creates a new AIModel with the specified API URL.
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            client: Client::new(),
        }
    }

    /// Sends the agent's state and environment to the API and retrieves the next action.
    pub async fn decide(
        &self,
        agent_state: &HashMap<String, String>,
        environment: &Environment<String>,
    ) -> Result<String, reqwest::Error> {
        let payload = json!({
            "agent_state": agent_state,
            "environment_state": environment.state,
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

/// Represents a generic environment for agents to operate in.
pub struct Environment<T> {
    pub state: HashMap<String, T>,
}

impl<T> Environment<T> {
    pub fn new(state: HashMap<String, T>) -> Self {
        Self { state }
    }
}
