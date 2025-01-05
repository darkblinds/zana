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
}
