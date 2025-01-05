pub mod gates;        // Expose gates.rs
pub mod statevector;  // Expose statevector.rs

use crate::circuit::gates::Gate;
use crate::circuit::statevector::Statevector;

/// Represents a quantum circuit.
///
/// A quantum circuit consists of a set number of qubits and a sequence of gate operations.
/// Single-qubit gates and multi-qubit gates are stored with their associated qubits for clarity.
pub struct QuantumCircuit {
    /// The number of qubits in the circuit.
    pub qubits: usize,

    /// A sequence of gates applied to the circuit, stored as `(gate, qubits)`.
    pub gates: Vec<(Gate, Vec<usize>)>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with the specified number of qubits.
    ///
    /// # Arguments
    /// - `qubits`: The number of qubits in the circuit.
    pub fn new(qubits: usize) -> Self {
        Self {
            qubits,
            gates: Vec::new(),
        }
    }

    /// Adds a gate to the circuit.
    ///
    /// The function dynamically determines whether the gate is single-qubit or multi-qubit
    /// based on the `Gate` enum and the number of qubits provided.
    ///
    /// # Arguments
    /// - `gate`: The gate to add (e.g., `Gate::Single` or `Gate::Two`).
    /// - `qubits`: The indices of the qubits the gate acts on.
    ///
    /// # Panics
    /// - If any qubit index is out of bounds.
    /// - If the gate size does not match the number of qubits specified.
    pub fn add_gate(&mut self, gate: Gate, qubits: Vec<usize>) {
        // Validate qubit indices
        for &qubit in &qubits {
            assert!(
                qubit < self.qubits,
                "Qubit index {} is out of bounds for a circuit with {} qubits.",
                qubit,
                self.qubits
            );
        }

        // Validate gate size
        match (&gate, qubits.len()) {
            (Gate::Single(_), 1) => (),
            (Gate::Two(_), 2) => (),
            _ => panic!("Invalid gate or mismatched qubits for gate type."),
        }

        self.gates.push((gate, qubits));
    }

    /// Simulates the quantum circuit and returns the final statevector.
    ///
    /// # Returns
    /// - A `Statevector` representing the quantum system's state after all gates have been applied.
    ///
    /// # Panics
    /// - If the circuit contains invalid gates or qubit indices.
    pub fn simulate(&self) -> Statevector {
        let mut statevector = Statevector::new(self.qubits);

        for (gate, qubits) in &self.gates {
            statevector.apply_gate(gate.clone(), qubits.as_slice()); // Clone the gate
        }

        statevector
    }

    /// Visualizes the quantum circuit as a text-based diagram.
    ///
    /// # How It Works
    /// - Single-qubit gates are represented by their symbols (e.g., `H` for Hadamard).
    /// - Multi-qubit gates use `●` for control qubits and `⊕` for target qubits.
    /// - The visualization includes all qubits and the sequence of gates applied to them.
    ///
    /// # Example Output
    /// For a circuit with a Hadamard on Q0 and a CNOT (control: Q0, target: Q1):
    /// ```
    /// Q0: ───H───●─────
    ///             │
    /// Q1: ───────⊕─────
    /// ```
    ///
    /// # Example Usage
    /// ```
    /// use zana::circuit::{gates, QuantumCircuit};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.add_gate(gates::hadamard(), vec![0]);
    /// circuit.add_gate(gates::cnot(), vec![0, 1]);
    ///
    /// circuit.visualize();
    /// ```
    pub fn visualize(&self) {
        let mut layers: Vec<String> = vec![String::new(); self.qubits];

        for (gate, qubits) in &self.gates {
            match gate {
                Gate::Single(_) => {
                    let qubit = qubits[0];
                    layers[qubit].push_str("──H──"); // Replace "H" for specific gates
                }
                Gate::Two(_) => {
                    let control = qubits[0];
                    let target = qubits[1];
                    for (i, layer) in layers.iter_mut().enumerate() {
                        if i == control {
                            layer.push_str("──●──");
                        } else if i == target {
                            layer.push_str("──⊕──");
                        } else {
                            layer.push_str("─────");
                        }
                    }
                }
            }
        }

        for (i, layer) in layers.iter().enumerate() {
            println!("Q{}: {}", i, layer);
        }
    }
}
