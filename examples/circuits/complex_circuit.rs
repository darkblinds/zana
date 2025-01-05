use zana::circuit::{gates, QuantumCircuit};

fn main() {
    // Create an 8-qubit circuit
    let mut circuit = QuantumCircuit::new(8);

    // Create superposition on all qubits
    for i in 0..8 {
        circuit.add_gate(gates::hadamard(), vec![i]);
    }

    // Apply CNOT gates to create entanglement
    for i in 0..7 {
        circuit.add_gate(gates::cnot(), vec![i, i + 1]);
    }

    // Simulate the circuit
    let final_state = circuit.simulate();

    // Print the final statevector
    println!("Final statevector: {:?}", final_state);
}
