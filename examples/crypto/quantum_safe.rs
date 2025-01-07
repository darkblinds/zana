use zana::crypto::quantum::{QubitState, apply_hadamard, generate_quantum_random_bit, bb84_simulation, verify_bb84_keys, simulate_eavesdropping};

fn main() {
    // Demonstrate the Hadamard gate
    println!("--- Hadamard Gate Demonstration ---");
    let states = [QubitState::Zero, QubitState::One, QubitState::Plus, QubitState::Minus];
    for state in states.iter() {
        let new_state = apply_hadamard(*state);
        println!("Initial State: {:?} -> New State: {:?}", state, new_state);
    }

    // Generate multiple quantum random bits
    println!("\n--- Quantum Random Bit Generator ---");
    let random_bits: Vec<u8> = (0..20).map(|_| generate_quantum_random_bit()).collect();
    println!("Generated Random Bits: {:?}", random_bits);

    // Initialize stats for summary
    let mut total_alice_bob_matches = 0;
    let mut total_alice_eve_matches = 0;
    let mut total_bits = 0;

    // Run the BB84 simulation 4 times
    println!("\n--- BB84 Quantum Key Distribution (4 Runs) ---");
    for i in 1..=4 {
        println!("\n** Run {} **", i);

        let (alice_bits, bob_bits) = bb84_simulation();
        println!("Alice's Bits: {:?}", alice_bits);
        println!("Bob's Bits:   {:?}", bob_bits);

        let matches = verify_bb84_keys(&alice_bits, &bob_bits);
        println!("Matching Bits: {}/{}", matches, alice_bits.len());

        // Simulate eavesdropping
        let (alice_bits, bob_bits, eve_bits) = simulate_eavesdropping();
        println!("\nEavesdropping Simulation (Run {}):", i);
        println!("Alice's Bits: {:?}", alice_bits);
        println!("Bob's Bits:   {:?}", bob_bits);
        println!("Eve's Bits:   {:?}", eve_bits);

        // Analyze how much eavesdropping affects the matching keys
        let alice_bob_matches = verify_bb84_keys(&alice_bits, &bob_bits);
        let alice_eve_matches = verify_bb84_keys(&alice_bits, &eve_bits);

        println!("Impact of Eavesdropping:");
        println!("Matching Bits (Alice <-> Bob): {}/{}", alice_bob_matches, alice_bits.len());
        println!("Matching Bits (Alice <-> Eve): {}/{}", alice_eve_matches, alice_bits.len());

        // Update total stats
        total_alice_bob_matches += alice_bob_matches;
        total_alice_eve_matches += alice_eve_matches;
        total_bits += alice_bits.len();
    }

    // Print final stats summary
    println!("\n--- Total Stats Across All Runs ---");
    println!("Total Bits Processed: {}", total_bits);
    println!(
        "Total Matching Bits (Alice <-> Bob): {} ({:.2}%)",
        total_alice_bob_matches,
        (total_alice_bob_matches as f64 / total_bits as f64) * 100.0
    );
    println!(
        "Total Matching Bits (Alice <-> Eve): {} ({:.2}%)",
        total_alice_eve_matches,
        (total_alice_eve_matches as f64 / total_bits as f64) * 100.0
    );
}
