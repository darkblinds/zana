use std::collections::HashMap;

/// Represents an action that can be performed by an agent.
pub struct Action<T> {
    pub name: String,
    pub description: String,
    pub effect: fn(&mut Agent<T>, &mut Environment<T>), // Effect of the action
}

impl<T> Action<T> {
    /// Creates a new action with a name, description, and effect function.
    pub fn new(
        name: &str,
        description: &str,
        effect: fn(&mut Agent<T>, &mut Environment<T>),
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            effect,
        }
    }
}

/// Represents a generic AI agent.
pub struct Agent<T> {
    pub name: String,
    pub state: HashMap<String, T>, // Agent's internal state
    pub actions: Vec<Action<T>>,  // Actions the agent can perform
    pub policy: fn(&Agent<T>, &Environment<T>) -> String, // Decision-making policy
}

impl<T> Agent<T>
where
    T: Clone + Default,
{
    /// Creates a new agent with a name, state, actions, and a policy.
    pub fn new(
        name: &str,
        state: HashMap<String, T>,
        actions: Vec<Action<T>>,
        policy: fn(&Agent<T>, &Environment<T>) -> String,
    ) -> Self {
        Self {
            name: name.to_string(),
            state,
            actions,
            policy,
        }
    }

    /// Executes the agent's decision-making policy to determine the next action.
    pub fn decide(&self, environment: &Environment<T>) -> Option<&Action<T>> {
        let action_name = (self.policy)(self, environment);
        self.actions.iter().find(|action| action.name == action_name)
    }

    /// Performs the selected action.
    pub fn perform_action(&mut self, action: &Action<T>, environment: &mut Environment<T>) {
        (action.effect)(self, environment);
        println!("{} performed action: {}", self.name, action.name);
    }
}

/// Represents a generic environment for agents to operate in.
pub struct Environment<T> {
    pub state: HashMap<String, T>, // Environment's state
}

impl<T> Environment<T>
where
    T: Clone + Default,
{
    /// Creates a new environment with an initial state.
    pub fn new(state: HashMap<String, T>) -> Self {
        Self { state }
    }

    /// Updates the state of the environment.
    pub fn update_state(&mut self, key: &str, value: T) {
        self.state.insert(key.to_string(), value);
    }
}
