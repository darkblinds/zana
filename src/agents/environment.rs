use std::collections::HashMap;

/// Represents the environment the agent operates in.
pub struct Environment {
    pub state: HashMap<String, String>, // Environmental data
}

impl Environment {
    pub fn new(state: HashMap<String, String>) -> Self {
        Self { state }
    }

    pub fn update(&mut self, key: &str, value: &str) {
        self.state.insert(key.to_string(), value.to_string());
    }
}
