use rand::Rng;
use crate::agents::core::{Agent, Environment};

/// Random decision-making policy: Chooses an action at random.
pub fn random_policy<T>(agent: &Agent<T>, _environment: &Environment<T>) -> String {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..agent.actions.len());
    agent.actions[idx].name.clone()
}

/// Greedy policy: Always chooses the action with the highest immediate benefit (scoring-based).
pub fn greedy_policy<T>(agent: &Agent<T>, environment: &Environment<T>) -> String
where
    T: PartialOrd + Default,
{
    let mut best_action = "idle".to_string();
    let mut best_score = T::default();

    for action in &agent.actions {
        let score = environment
            .state
            .get(&format!("{}_value", action.name))
            .cloned()
            .unwrap_or(T::default());

        if score > best_score {
            best_score = score;
            best_action = action.name.clone();
        }
    }

    best_action
}

/// Rule-based policy: Uses predefined rules to choose an action.
pub fn rule_based_policy<T>(agent: &Agent<T>, environment: &Environment<T>) -> String
where
    T: PartialOrd + Default,
{
    if let Some(threat_level) = environment.state.get("threat") {
        if *threat_level > T::default() {
            return "retreat".to_string();
        }
    }

    if let Some(resources) = environment.state.get("resources") {
        if *resources > T::default() {
            return "gather".to_string();
        }
    }

    "idle".to_string()
}
