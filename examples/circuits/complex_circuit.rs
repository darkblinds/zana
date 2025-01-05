use std::{env, thread};
use std::time::Duration;
use zana::circuit::{gates, QuantumCircuit};

/// Demonstrates a complex quantum circuit with multiple qubits and entanglement.
/// Takes a command-line argument to control the output:
/// - `raw`: Runs the simulation and outputs the raw statevector.
/// - `visual`: Visualizes the circuit.
/// - `both`: Runs the simulation and visualizes the circuit.
fn main() {
    // Parse the command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example complex_circuit -- <result>");
        eprintln!("result: raw, visual, or both");
        return;
    }
    let result = &args[1];

    // Create an 8-qubit quantum circuit
    let mut circuit = QuantumCircuit::new(8);

    // Create superposition on all qubits
    for i in 0..8 {
        circuit.add_gate(gates::hadamard(), vec![i]);
    }

    // Apply CNOT gates to create entanglement
    for i in 0..7 {
        circuit.add_gate(gates::cnot(), vec![i, i + 1]);
    }

    // Execute based on the `result` flag
    match result.as_str() {
        "raw" => {
            let final_state = circuit.simulate();
            println!("Final statevector: {:?}", final_state);
        }
        "visual" => {
            println!("Visualizing Circuit:");
            circuit.visualize();
        }
        "both" => {
            // Run simulation first
            let final_state = circuit.simulate();
            println!("RAW Final statevector: {:?}", final_state);

            // Delay before visualization
            println!("Now running Visual Circuit...");
            thread::sleep(Duration::from_secs(3));
            circuit.visualize();
        }
        _ => {
            eprintln!("Invalid result argument. Use 'raw', 'visual', or 'both'.");
        }
    }
}