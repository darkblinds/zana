use std::{env, thread};
use std::time::Duration;
use zana::circuit::{gates, QuantumCircuit};

/// Demonstrates applying multiple single and multi-qubit gates.
/// Takes a command-line argument to control the output:
/// - `raw`: Runs the simulation and outputs the raw statevector.
/// - `visual`: Visualizes the circuit.
/// - `heatmap-terminal`: Displays the heatmap of the state probabilities in the terminal.
/// - `heatmap-file`: Saves the heatmap of the state probabilities to a file.
/// - `both`: Runs the simulation and visualizes the circuit.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example medium_circuit -- <result>");
        eprintln!("result: raw, visual, heatmap-terminal, heatmap-file, or both");
        return Ok(());
    }
    let result = &args[1];

    // Create a 3-qubit quantum circuit
    let mut circuit = QuantumCircuit::new(3);

    // Add single-qubit gates
    circuit.add_gate(gates::hadamard(), vec![0]); // Hadamard on qubit 0
    circuit.add_gate(gates::pauli_x(), vec![1]); // Pauli-X (NOT) on qubit 1
    circuit.add_gate(gates::pauli_z(), vec![2]); // Pauli-Z on qubit 2

    // Add a CNOT gate (control=0, target=1)
    circuit.add_gate(gates::cnot(), vec![0, 1]);

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
        "heatmap-terminal" => {
            println!("Generating Heatmap in terminal...");
            circuit.visualize_heatmap(None)?;
            println!("Heatmap displayed in terminal.");
        }
        "heatmap-file" => {
            println!("Generating Heatmap...");
            circuit.visualize_heatmap(Some("examples/circuits/medium_circuit_heatmap.png"))?;
            println!("Heatmap saved to 'examples/circuits/medium_circuit_heatmap.png'.");
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
            eprintln!("Invalid result argument. Use 'raw', 'visual', 'heatmap-terminal', 'heatmap-file', or 'both'.");
        }
    }

    Ok(())
}
