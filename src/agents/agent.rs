use super::memory::Memory;
use super::environment::Environment;
use super::actions::{Action, ActionParams};
use super::model_provider::ModelProvider;
use std::collections::HashMap;

/// Represents an AGI Agent.
pub struct Agent {
    pub name: String,
    pub memory: Memory,
    pub actions: HashMap<String, Action>,
    pub model_provider: Option<ModelProvider>, // Optional AI model provider
}

impl Agent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            memory: Memory::new(),
            actions: HashMap::new(),
            model_provider: None,
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.insert(action.name.clone(), action);
    }

    pub fn set_model_provider(&mut self, provider: ModelProvider) {
        self.model_provider = Some(provider);
    }

    pub async fn decide(&self, environment: &Environment) -> Option<String> {
        if let Some(provider) = &self.model_provider {
            provider
                .decide(&self.memory.short_term, &environment.state)
                .await
                .ok()
        } else {
            None
        }
    }

    pub fn execute_action(&mut self, action_name: &str, params: ActionParams) {
        if let Some(action) = self.actions.get(action_name).cloned() {
            action.execute(self, params);
        } else {
            println!("Action '{}' not found.", action_name);
        }
    }
}
