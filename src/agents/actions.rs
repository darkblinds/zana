use super::agent::Agent;
use std::collections::HashMap;

/// Parameters for actions.
pub type ActionParams = HashMap<String, String>;

/// Represents an action an agent can perform.
#[derive(Clone)]
pub struct Action {
    pub name: String,
    pub description: String,
    pub execute: fn(&mut Agent, ActionParams), // Execution logic
}

impl Action {
    pub fn new(
        name: &str,
        description: &str,
        execute: fn(&mut Agent, ActionParams),
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            execute,
        }
    }

    pub fn execute(&self, agent: &mut Agent, params: ActionParams) {
        (self.execute)(agent, params);
    }
}
