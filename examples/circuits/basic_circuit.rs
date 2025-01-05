use zana::circuit::{gates, QuantumCircuit};

/// Demonstrates a basic quantum circuit with a Hadamard and CNOT gate.
fn main() {
    // Create a 2-qubit quantum circuit
    let mut circuit = QuantumCircuit::new(2);

    // Add a Hadamard gate to qubit 0
    circuit.add_gate(gates::hadamard(), vec![0]);

    // Add a CNOT gate (control=0, target=1)
    circuit.add_gate(gates::cnot(), vec![0, 1]);

    // Simulate the circuit
    let final_state = circuit.simulate();

    // Print the final statevector
    println!("Final statevector: {:?}", final_state);
}
