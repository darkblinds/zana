use std::collections::HashMap;

/// Represents an AGI Agent's memory.
pub struct Memory {
    pub long_term: HashMap<String, String>, // Long-term knowledge
    pub short_term: HashMap<String, String>, // Short-term observations
}

impl Memory {
    pub fn new() -> Self {
        Self {
            long_term: HashMap::new(),
            short_term: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: &str, value: &str) {
        self.long_term.insert(key.to_string(), value.to_string());
    }

    pub fn recall(&self, key: &str) -> Option<&String> {
        self.long_term.get(key)
    }
}
