mod basic_circuit;
mod medium_circuit;
mod complex_circuit;

use zana::circuit::{gates, QuantumCircuit};

/// Demonstrates the usage of the `QuantumCircuit` with single and multi-qubit gates.
fn main() {
    // Create a quantum circuit with 2 qubits
    let mut circuit = QuantumCircuit::new(2);

    // Add a single-qubit gate (Hadamard on qubit 0)
    circuit.add_gate(gates::hadamard(), vec![0]);

    // Add a two-qubit gate (CNOT with control=0, target=1)
    circuit.add_gate(gates::cnot(), vec![0, 1]);

    // Print the gates in the circuit
    println!("Circuit Gates:");
    for (gate, qubits) in circuit.gates.iter() {
        println!("Gate: {:?}, Qubits: {:?}", gate, qubits);
    }

    // Additional Examples
    apply_multiple_single_qubit_gates();
    add_swap_gate_example();
}

/// Demonstrates applying multiple single-qubit gates to the circuit.
fn apply_multiple_single_qubit_gates() {
    let mut circuit = QuantumCircuit::new(3);

    // Apply a Hadamard gate to qubit 0
    circuit.add_gate(gates::hadamard(), vec![0]);

    // Apply a Pauli-X gate (NOT gate) to qubit 1
    circuit.add_gate(gates::pauli_x(), vec![1]);

    // Apply a Pauli-Z gate to qubit 2
    circuit.add_gate(gates::pauli_z(), vec![2]);

    println!("\nCircuit with multiple single-qubit gates:");
    for (gate, qubits) in circuit.gates.iter() {
        println!("Gate: {:?}, Qubits: {:?}", gate, qubits);
    }
}

/// Demonstrates adding a SWAP gate to the circuit.
fn add_swap_gate_example() {
    let mut circuit = QuantumCircuit::new(2);

    // Add the SWAP gate to the circuit (qubits 0 and 1)
    circuit.add_gate(gates::swap(), vec![0, 1]);

    println!("\nCircuit with SWAP gate:");
    for (gate, qubits) in circuit.gates.iter() {
        println!("Gate: {:?}, Qubits: {:?}", gate, qubits);
    }
}
