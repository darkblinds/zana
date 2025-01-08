use crate::agents::core::Action;

// TODO: improve these actions to, replace ludical examples to real ones from crypto and circuits
/// Predefined action: Rest
pub fn rest_action<T>() -> Action<T>
where
    T: Clone + Default + std::ops::AddAssign,
{
    Action::new(
        "rest",
        "Regains energy by resting.",
        |agent, _env| {
            if let Some(energy) = agent.state.get_mut("energy") {
                *energy += T::default() + T::default();
                println!("{} rested and gained energy!", agent.name);
            }
        },
    )
}

/// Predefined action: Explore
pub fn explore_action<T>() -> Action<T>
where
    T: Clone + Default + std::ops::SubAssign,
{
    Action::new(
        "explore",
        "Explores the environment, which might be dangerous.",
        |agent, env| {
            if let Some(danger) = env.state.get("danger") {
                if let Some(health) = agent.state.get_mut("health") {
                    *health -= danger.clone();
                    println!(
                        "{} explored a dangerous area and lost health!",
                        agent.name
                    );
                }
            }
        },
    )
}

/// Predefined action: Retreat
pub fn retreat_action<T>() -> Action<T>
where
    T: Clone + Default,
{
    Action::new(
        "retreat",
        "Retreats to a safe position.",
        |_agent, _env| {
            println!("The agent retreated to safety.");
        },
    )
}
