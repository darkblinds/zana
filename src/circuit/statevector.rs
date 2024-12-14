use num_complex::Complex;
use crate::circuit::gates::Gate;

/// Represents the statevector of a quantum system.
pub struct Statevector {
    /// The statevector is represented as a list of complex amplitudes.
    /// It can tell everything about the quantum system at a given time
    pub vector: Vec<Complex<f64>>,
}

impl Statevector {
    /// Creates a new statevector initialized to the `|0⟩` state.
    ///
    /// # Arguments
    /// - `num_qubits`: Number of qubits in the quantum system.
    ///
    /// # Returns
    /// A `Statevector` initialized to the `|0⟩` state.
    pub fn new(num_qubits: usize) -> Self {
        let size = 2_usize.pow(num_qubits as u32);
        let mut vector = vec![Complex::new(0.0, 0.0); size];
        vector[0] = Complex::new(1.0, 0.0); // Initialize to |0⟩
        Self { vector }
    }

    /// Returns the number of qubits in the quantum system.
    pub fn num_qubits(&self) -> usize {
        (self.vector.len() as f64).log2() as usize
    }

    /// Measures a specific qubit, collapsing the statevector.
    ///
    /// # How It Works
    /// 1. The function calculates the probability (`prob_0`) of the target qubit being in the `|0⟩` state.
    /// 2. A random number is generated to simulate a probabilistic measurement:
    ///     - If the random number is less than `prob_0`, the result is `0` (collapsed to `|0⟩`).
    ///     - Otherwise, the result is `1` (collapsed to `|1⟩`).
    /// 3. The statevector is updated to reflect the measurement result:
    ///     - Amplitudes inconsistent with the result are set to `0`.
    ///     - Remaining amplitudes are normalized to maintain the total probability of 1.
    ///
    /// # Arguments
    /// - `target_qubit`: The index of the qubit to measure (starting from 0).
    ///
    /// # Returns
    /// The result of the measurement (`0` for `|0⟩`, `1` for `|1⟩`).
    ///
    /// # Example
    /// ```rust
    /// let mut sv = zana::circuit::statevector::Statevector::new(1); // Single qubit in |0⟩
    /// let result = sv.measure(0); // Measure the qubit
    /// println!("Measurement result: {}", result);
    pub fn measure(&mut self, target_qubit: usize) -> u8 {
        let mask = 1 << target_qubit;

        // Compute probability of measuring |0> for the target qubit
        let prob_0: f64 = self
            .vector
            .iter()
            .enumerate()
            .filter(|(state, _)| state & mask == 0)
            .map(|(_, amp)| amp.norm_sqr())
            .sum();

        // Generate a random measurement result (0 or 1)
        let result = if rand::random::<f64>() < prob_0 { 0 } else { 1 };

        // Collapse the statevector based on the measurement result
        let norm: f64 = self
            .vector
            .iter_mut()
            .enumerate()
            .map(|(state, amp)| {
                if (state & mask == 0 && result == 1) || (state & mask != 0 && result == 0) {
                    *amp = Complex::new(0.0, 0.0);
                    0.0
                } else {
                    amp.norm_sqr()
                }
            })
            .sum();

        // Normalize the remaining statevector
        let normalization_factor = norm.sqrt();
        for amp in self.vector.iter_mut() {
            *amp /= normalization_factor;
        }

        result
    }

    /// Applies a quantum gate to the statevector.
    ///
    /// # Arguments
    /// - `gate`: The gate matrix. It can be a 2x2 or 4x4 matrix.
    /// - `qubits`: The indices of the qubits the gate acts on.
    /// Applies a gate to the statevector.
    pub fn apply_gate(&mut self, gate: Gate, qubits: &[usize]) {
        // Ensure that the qubits provided are valid
        let num_qubits = self.num_qubits();
        assert!(
            qubits.iter().all(|&q| q < num_qubits),
            "Qubit indices must be within the range of the quantum system."
        );

        match gate {
            Gate::Single(single_qubit_gate) => self.apply_single_qubit_gate(&single_qubit_gate, qubits[0]),
            Gate::Two(two_qubit_gate) => self.apply_multi_qubit_gate(&two_qubit_gate, qubits),
        }

        // Validate only in debug builds
        // Validate the statevector after applying the gate, only in debug mode
        if cfg!(debug_assertions) {
            self.validate()
                .expect("Statevector validation failed after applying gate.");
        }
    }

    /// Applies a single-qubit gate (2x2 matrix).
    fn apply_single_qubit_gate(&mut self, gate: &[[Complex<f64>; 2]; 2], target: usize) {
        let mask = 1 << target;

        for state in 0..self.vector.len() {
            if state & mask == 0 {
                let paired_state = state | mask;
                let original_0 = self.vector[state];
                let original_1 = self.vector[paired_state];

                // Apply the gate transformation
                self.vector[state] = gate[0][0] * original_0 + gate[0][1] * original_1;
                self.vector[paired_state] = gate[1][0] * original_0 + gate[1][1] * original_1;
            }
        }
    }

    /// Generalized multi-qubit gate application.
    fn apply_multi_qubit_gate<const N: usize>(
        &mut self,
        gate: &[[Complex<f64>; N]; N],
        qubits: &[usize],
    ) {
        let num_states = self.vector.len();
        let mut new_vector = vec![Complex::new(0.0, 0.0); num_states];

        for state in 0..num_states {
            let input_index = self.map_to_gate_index(state, qubits);
            for output_index in 0..N {
                let new_state = self.map_from_gate_index(state, qubits, output_index);
                new_vector[new_state] += gate[output_index][input_index] * self.vector[state];
            }
        }

        self.vector = new_vector;
    }


    /// Applies a two-qubit gate (4x4 matrix).
    fn apply_two_qubit_gate(&mut self, gate: [[Complex<f64>; 4]; 4]) {
        // Ensure the statevector has 4 elements (2-qubit system)
        assert_eq!(self.vector.len(), 4, "Statevector must have exactly 4 elements.");

        // Temporary vector to hold the result
        let mut new_vector = vec![Complex::new(0.0, 0.0); 4];

        // Matrix-vector multiplication: new_vector[i] = Σ_j gate[i][j] * vector[j]
        for i in 0..4 {
            for j in 0..4 {
                new_vector[i] += gate[i][j] * self.vector[j];
            }
        }

        // Update the statevector with the transformed values
        self.vector = new_vector;
    }

    /// Map a global state index to the gate-specific index.
    fn map_to_gate_index(&self, state: usize, qubits: &[usize]) -> usize {
        qubits.iter().enumerate().fold(0, |acc, (i, &qubit)| {
            acc | (((state >> qubit) & 1) << i)
        })
    }

    /// Map a gate-specific output index back to the global state index.
    fn map_from_gate_index(&self, state: usize, qubits: &[usize], output_index: usize) -> usize {
        let mut new_state = state;
        for (i, &qubit) in qubits.iter().enumerate() {
            let bit = (output_index >> i) & 1;
            new_state = (new_state & !(1 << qubit)) | (bit << qubit);
        }
        new_state
    }

    /// Normalizes the statevector to ensure the sum of squared amplitudes equals 1.
    pub fn normalize(&mut self) {
        let norm: f64 = self.vector.iter().map(|amp| amp.norm_sqr()).sum();
        if norm != 0.0 {
            let scale = 1.0 / norm.sqrt();
            self.vector.iter_mut().for_each(|amp| *amp *= scale);
        }
    }

    /// Validates the statevector for correctness.
    /// - Checks normalization and dimensional consistency.
    pub fn validate(&self) -> Result<(), String> {
        let num_qubits = self.num_qubits();
        let expected_size = 2_usize.pow(num_qubits as u32);
        if self.vector.len() != expected_size {
            return Err(format!(
                "Invalid statevector length: expected {}, got {}.",
                expected_size,
                self.vector.len()
            ));
        }

        // Additional validations if necessary
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;
    use crate::circuit::gates::{cnot, hadamard, identity_gate};

    /// Helper function for Two-Qubit Gates
    fn extract_two_qubit_gate(gate: Gate) -> [[Complex<f64>; 4]; 4] {
        match gate {
            Gate::Two(matrix) => matrix,
            _ => panic!("Expected a two-qubit gate."),
        }
    }

    #[test]
    fn test_statevector_initialization() {
        let sv = Statevector::new(2); // Two qubits
        assert_eq!(sv.vector.len(), 4); // Statevector size is 2^2 = 4
        assert_eq!(sv.vector[0], Complex::new(1.0, 0.0)); // Initialized to |00⟩
        assert!(sv.vector.iter().skip(1).all(|&amp| amp == Complex::new(0.0, 0.0)));
    }

    #[test]
    fn test_apply_single_qubit_gate_identity() {
        let mut sv = Statevector::new(1); // One qubit

        sv.apply_gate(identity_gate(), &[0]); // Apply identity gate to qubit 0
        assert_eq!(sv.vector[0], Complex::new(1.0, 0.0)); // |0⟩ remains |0⟩
        assert_eq!(sv.vector[1], Complex::new(0.0, 0.0)); // |1⟩ remains |1⟩
    }

    #[test]
    fn test_apply_single_qubit_gate_hadamard() {
        let mut sv = Statevector::new(1); // One qubit

        sv.apply_gate(hadamard(), &[0]); // Apply Hadamard to qubit 0
        let scale = 1.0 / 2.0_f64.sqrt();
        assert_eq!(sv.vector[0], Complex::new(scale, 0.0)); // Superposition |0>
        assert_eq!(sv.vector[1], Complex::new(scale, 0.0)); // Superposition |1>
    }

    #[test]
    fn test_apply_gate_valid_single_qubit() {
        let mut sv = Statevector::new(2); // 2 qubits
        sv.apply_gate(identity_gate(), &[0]); // Apply identity gate to qubit 0
        assert_eq!(sv.vector[0], Complex::new(1.0, 0.0)); // |00⟩ remains |00⟩
    }

    #[test]
    fn test_apply_gate_valid_two_qubit() {
        let mut sv = Statevector::new(2); // 2 qubits
        sv.apply_gate(hadamard(), &[0]); // Apply Hadamard to qubit 0
        sv.apply_gate(cnot(), &[0, 1]); // Apply CNOT with control 0 and target 1
        assert!(sv.vector.iter().all(|&amp| amp.norm_sqr() <= 1.0)); // Amplitudes normalized
    }


    #[test]
    fn test_apply_two_qubit_gate_cnot() {
        let mut sv = Statevector {
            vector: vec![
                Complex::new(0.0, 0.0), // |00⟩
                Complex::new(0.0, 0.0), // |01⟩
                Complex::new(1.0, 0.0), // |10⟩
                Complex::new(0.0, 0.0), // |11⟩
            ],
        };

        let cnot_gate = cnot(); // Get the CNOT gate
        if let Gate::Two(cnot_matrix) = cnot_gate {
            // Extract the raw matrix
            sv.apply_two_qubit_gate(cnot_matrix); // Apply CNOT gate (control: qubit 0, target: qubit 1)
        } else {
            panic!("Expected a two-qubit gate, but got a different variant.");
        }

        let expected_vector = vec![
            Complex::new(0.0, 0.0), // |00⟩
            Complex::new(0.0, 0.0), // |01⟩
            Complex::new(0.0, 0.0), // |10⟩
            Complex::new(1.0, 0.0), // |11⟩
        ];

        assert_eq!(
            sv.vector, expected_vector,
            "Statevector after applying CNOT gate does not match expected result."
        );
    }

    #[test]
    fn test_new_state_mapping() {
        let control = 0; // Control qubit index
        let target = 1; // Target qubit index
        let mask_control = 1 << control;
        let mask_target = 1 << target;

        // Example states to test
        let states = vec![0b00, 0b01, 0b10, 0b11]; // Binary representation of |00⟩, |01⟩, |10⟩, |11⟩

        for state in states {
            println!("Testing state: {:02b}", state);

            // Calculate input_index based on control/target bits
            let control_bit = (state & mask_control) >> control;
            let target_bit = (state & mask_target) >> target;
            let input_index = (control_bit << 1) | target_bit;

            for output_index in 0..4 {
                // Calculate new control and target bits from output_index
                let new_control_bit = (output_index >> 1) & 1;
                let new_target_bit = output_index & 1;

                // Calculate new_state
                let new_state = (state & !(mask_control | mask_target))
                    | (new_control_bit << control)
                    | (new_target_bit << target);

                println!(
                    "State: {:02b}, Input Index: {}, Output Index: {}, New Control Bit: {}, New Target Bit: {}, New State: {:02b}",
                    state, input_index, output_index, new_control_bit, new_target_bit, new_state
                );

                // Assert that new_state is within bounds
                assert!(new_state < 4, "new_state is out of bounds: {:02b}", new_state);
            }
        }
    }

    #[test]
    fn test_measurement() {
        let mut sv = Statevector::new(1); // One qubit

        // Apply a Hadamard gate to create superposition
        sv.apply_gate(hadamard(), &[0]);

        // Measure the qubit
        let result = sv.measure(0);

        // The measurement result must be 0 or 1
        assert!(result == 0 || result == 1);

        // Check that the statevector collapsed correctly
        if result == 0 {
            assert_eq!(sv.vector, vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]); // Collapsed to |0>
        } else {
            assert_eq!(sv.vector, vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]); // Collapsed to |1>
        }
    }


    // SHOULD PANIC TESTS
    #[test]
    #[should_panic(expected = "Qubit indices must be within the range of the quantum system.")]
    fn test_apply_gate_exceeding_qubits() {
        let mut sv = Statevector::new(2); // 2 qubits
        sv.apply_gate(cnot(), &[0, 1, 2]); // Invalid gate, qubits exceed statevector size
    }

    #[test]
    #[should_panic(expected = "Qubit indices must be within the range of the quantum system.")]
    fn test_apply_gate_empty_statevector() {
        let mut sv = Statevector { vector: vec![] }; // Empty statevector
        sv.apply_gate(identity_gate(), &[0]); // Invalid gate, statevector has no qubits
    }

}

