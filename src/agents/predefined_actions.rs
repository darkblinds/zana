use crate::agents::actions::{Action, ActionParams};
use crate::agents::agent::Agent;
use crate::agents::environment::Environment;
use std::collections::HashMap;

/// Action: Learn something and save it in memory.
pub fn learn_action() -> Action {
    Action::new(
        "learn",
        "Learn and store knowledge in the agent's memory.",
        |agent, params| {
            if let Some(concept) = params.get("concept") {
                agent.memory.store("knowledge", concept);
                println!("{} learned about: {}", agent.name, concept);
            } else {
                println!("{} failed to learn due to missing 'concept' parameter.", agent.name);
            }
        },
    )
}

/// Action: Forget something by removing it from memory.
pub fn forget_action() -> Action {
    Action::new(
        "forget",
        "Forget and remove knowledge from memory.",
        |agent, params| {
            if let Some(concept) = params.get("concept") {
                agent.memory.long_term.remove(concept);
                println!("{} forgot: {}", agent.name, concept);
            } else {
                println!(
                    "{} failed to forget due to missing 'concept' parameter.",
                    agent.name
                );
            }
        },
    )
}

/// Action: Send a message to a specified channel with a given tone.
pub fn send_message_action() -> Action {
    Action::new(
        "send_message",
        "Send a message to a specific channel.",
        |agent, params| {
            let channel = params.get("channel").unwrap_or(&"general".to_string()).clone();
            let tone = params.get("tone").unwrap_or(&"neutral".to_string()).clone();
            let message = params
                .get("message")
                .unwrap_or(&"Hello, world!".to_string())
                .clone();

            println!(
                "{} sent a message to '{}': [{}] {}",
                agent.name, channel, tone, message
            );
            agent.memory.store("last_message", &message);
        },
    )
}

/// Action: Perform a cryptographic operation.
pub fn cryptography_action() -> Action {
    Action::new(
        "cryptography",
        "Perform a cryptographic operation using specified parameters.",
        |_agent, params| {
            let crypto_type = params.get("crypto_type").unwrap_or(&"sign".to_string()).clone();
            let keys = params.get("keys").unwrap_or(&"default_keys".to_string()).clone();
            let owner = params.get("owner").unwrap_or(&"anonymous".to_string()).clone();

            println!(
                "Performed '{}' cryptographic operation with keys '{}' for owner '{}'.",
                crypto_type, keys, owner
            );
        },
    )
}

/// Action: Gather resources from the environment.
pub fn gather_resources_action() -> Action {
    Action::new(
        "gather_resources",
        "Gather resources from the environment.",
        |_agent, params| {
            let resource = params.get("resource").unwrap_or(&"unknown".to_string()).clone();
            let quantity = params
                .get("quantity")
                .unwrap_or(&"1".to_string())
                .parse::<u32>()
                .unwrap_or(1);

            println!(
                "Gathered {} of resource '{}' from the environment.",
                quantity, resource
            );
        },
    )
}

/// Action: Analyze environmental data.
pub fn analyze_environment_action() -> Action {
    Action::new(
        "analyze_environment",
        "Analyze the current environment and log findings.",
        |_agent, params| {
            if let Some(environment) = params.get("environment_state") {
                println!("Analyzing environment: {}", environment);
            } else {
                println!("No environment data provided for analysis.");
            }
        },
    )
}

/// Action: Collaborate with another agent.
pub fn collaborate_action() -> Action {
    Action::new(
        "collaborate",
        "Collaborate with another agent on a task.",
        |_agent, params| {
            let partner = params.get("partner").unwrap_or(&"unknown".to_string()).clone();
            let task = params.get("task").unwrap_or(&"unspecified".to_string()).clone();

            println!("Collaborated with '{}' on task '{}'.", partner, task);
        },
    )
}

/// Action: Train a skill.
pub fn train_skill_action() -> Action {
    Action::new(
        "train_skill",
        "Train and improve a skill over time.",
        |agent, params| {
            if let Some(skill) = params.get("skill") {
                agent.memory.store("current_training", skill);
                println!("{} is training to improve skill: {}", agent.name, skill);
            } else {
                println!("{} failed to train due to missing 'skill' parameter.", agent.name);
            }
        },
    )
}
