use zana::circuit::{gates, QuantumCircuit};

/// Demonstrates applying multiple single and multi-qubit gates.
fn main() {
    // Create a 3-qubit quantum circuit
    let mut circuit = QuantumCircuit::new(3);

    // Add single-qubit gates
    circuit.add_gate(gates::hadamard(), vec![0]);  // Hadamard on qubit 0
    circuit.add_gate(gates::pauli_x(), vec![1]);  // Pauli-X (NOT) on qubit 1
    circuit.add_gate(gates::pauli_z(), vec![2]);  // Pauli-Z on qubit 2

    // Add a CNOT gate (control=0, target=1)
    circuit.add_gate(gates::cnot(), vec![0, 1]);

    // Simulate the circuit
    let final_state = circuit.simulate();

    // Print the final statevector
    println!("Final statevector: {:?}", final_state);
}
