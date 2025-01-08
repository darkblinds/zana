use rand::Rng;
use std::time::Instant;

/// Generates a random integer in the given range.
pub fn random_in_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Logs an action taken by the agent.
pub fn log_action(agent_name: &str, action: &str) {
    println!("[LOG] Agent '{}' performed action: {}", agent_name, action);
}

/// Measures the execution time of a given function.
pub fn measure_execution<F: FnOnce()>(label: &str, func: F) {
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    println!("[TIMER] {} executed in {:?}", label, duration);
}
