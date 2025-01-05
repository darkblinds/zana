use std::collections::HashMap;
use num_complex::Complex;
use crate::circuit::gates::Gate;

/// Represents the statevector of a quantum system.
pub struct Statevector {
    /// The statevector is represented as a list of complex amplitudes.
    /// It can tell everything about the quantum system at a given time
    num_qubits: usize,                    // Explicit qubit count
    pub vector: HashMap<usize, Complex<f64>>, // Sparse representation
}

impl Statevector {
    /// Initializes a quantum statevector for an `n`-qubit system in the `|0⟩` state.
    ///
    /// # Arguments
    /// - `num_qubits`: Number of qubits. The statevector will have `2^n` entries.
    ///
    /// # Returns
    /// - A `Statevector` initialized to `|0⟩` (first entry `1.0`, rest `0.0`).
    ///
    /// # Notes
    /// - Memory usage is exponential: 30 qubits require ~8GB of RAM.
    /// - For larger systems, consider sparse or distributed representations.
    ///
    /// # Example
    /// ```rust
    //// let statevector = Statevector::new(2);
    //// assert_eq!(statevector.vector[0], Complex::new(1.0, 0.0)); // |00⟩
    //// assert_eq!(statevector.vector.len(), 4); // 2^2 = 4
    //// ```

    pub fn new(num_qubits: usize) -> Self {
        if num_qubits == 0 {
            panic!("Number of qubits must be greater than 0.");
        }

        let mut vector = HashMap::new();
        vector.insert(0, Complex::new(1.0, 0.0)); // Start in |0⟩ state
        Self { vector, num_qubits }
    }

    /// Dynamically compute the number of qubits based on the statevector.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
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
    /// let mut sv = zana::circuit::statevector::Statevector::new(2); // Single qubit in |0⟩
    /// let result = sv.measure(0); // Measure the qubit
    /// println!("Measurement result: {}", result);
    pub fn measure(&mut self, target_qubit: usize) -> u8 {
        let mask = 1 << target_qubit;

        // Compute probability of measuring |0⟩ for the target qubit
        let prob_0: f64 = self
            .vector
            .iter()
            .filter(|(state, _)| *state & mask == 0)
            .map(|(_, amp)| amp.norm_sqr())
            .sum();

        // Generate a random measurement result (0 or 1)
        let result = if rand::random::<f64>() < prob_0 { 0 } else { 1 };

        // Collapse the statevector based on the measurement result
        let norm: f64 = self
            .vector
            .iter_mut()
            .map(|(state, amp)| {
                if (*state & mask == 0 && result == 1) || (*state & mask != 0 && result == 0) {
                    *amp = Complex::new(0.0, 0.0);
                    0.0
                } else {
                    amp.norm_sqr()
                }
            })
            .sum();

        // Normalize the remaining statevector
        let scale = norm.sqrt();
        self.vector.values_mut().for_each(|amp| *amp /= scale);

        self.clean_zero_amplitudes();

        result
    }

    fn clean_zero_amplitudes(&mut self) {
        self.vector.retain(|_, amp| amp.norm_sqr() > 1e-10); // Retain only non-zero entries
    }
    // fn normalize_and_cleanup(&mut self) {
    //     self.vector.retain(|_, &mut amp| amp.norm_sqr() > 0.0);
    // }
    pub fn normalize_and_cleanup(&mut self) {
        self.vector.retain(|_, amp| amp.norm_sqr() > 1e-10); // Remove near-zero entries

        let norm: f64 = self.vector.values().map(|amp| amp.norm_sqr()).sum();
        if norm != 0.0 {
            let scale = norm.sqrt();
            self.vector.values_mut().for_each(|amp| *amp /= scale);
        }
    }




    /// Applies a quantum gate to the statevector.
    ///
    /// # Arguments
    /// - `gate`: The gate matrix. It can be a 2x2 or 4x4 matrix.
    /// - `qubits`: The indices of the qubits the gate acts on.
    /// Applies a gate to the statevector.
    pub fn apply_gate(&mut self, gate: Gate, qubits: &[usize]) {
        if qubits.is_empty() || qubits.iter().any(|&q| q >= self.num_qubits) {
            panic!("Qubit indices must be within the range of the quantum system.");
        }

        match gate {
            Gate::Single(single_qubit_gate) => self.apply_single_qubit_gate(&single_qubit_gate, qubits[0]),
            // Gate::Two(two_qubit_gate) => self.apply_multi_qubit_gate(&two_qubit_gate, qubits),
            Gate::Two(two_qubit_gate) => self.apply_two_qubit_gate(two_qubit_gate, qubits),
        }

        self.normalize_and_cleanup();
    }





    /// Applies a single-qubit gate (2x2 matrix).
    fn apply_single_qubit_gate(&mut self, gate: &[[Complex<f64>; 2]; 2], target: usize) {
        let mask = 1 << target;
        let mut new_vector = HashMap::new();

        for (&state, &amp) in &self.vector {
            let paired_state = state ^ mask; // Flip the target bit
            if state & mask == 0 {
                let original_0 = amp;
                let original_1 = *self.vector.get(&paired_state).unwrap_or(&Complex::new(0.0, 0.0));

                new_vector.insert(
                    state,
                    gate[0][0] * original_0 + gate[0][1] * original_1,
                );
                new_vector.insert(
                    paired_state,
                    gate[1][0] * original_0 + gate[1][1] * original_1,
                );
            }
        }

        self.vector = new_vector;
    }


    /// Generalized multi-qubit gate application.
    /// Generalized multi-qubit gate application for sparse statevector representation.
    fn apply_multi_qubit_gate<const N: usize>(
        &mut self,
        gate: &[[Complex<f64>; N]; N],
        qubits: &[usize],
    ) {
        let mut new_vector = HashMap::new();

        for (&state, &amplitude) in self.vector.iter() {
            let input_index = self.map_to_gate_index(state, qubits);

            for output_index in 0..N {
                println!(
                    "Gate Element Access -> Gate[{}][{}] = {}",
                    output_index, input_index, gate[output_index][input_index]
                );
                let new_state = self.map_from_gate_index(state, qubits, output_index);
                let gate_element = gate[output_index][input_index];

                if gate_element.norm_sqr() > 1e-10 {
                    let contribution = gate_element * amplitude;
                    *new_vector.entry(new_state).or_insert(Complex::new(0.0, 0.0)) += contribution;
                }
            }
        }

        println!("New Vector Before Cleanup: {:?}", new_vector);
        self.vector = new_vector;
        self.normalize_and_cleanup();
    }





    /// Applies a two-qubit gate (4x4 matrix).
    fn apply_two_qubit_gate(&mut self, gate: [[Complex<f64>; 4]; 4], qubits: &[usize]) {
        let mut new_vector = HashMap::new();

        for (&state, &amplitude) in &self.vector {
            if amplitude.norm_sqr() > 0.0 {
                // Map the global state to the gate's input index
                let input_index = self.map_to_gate_index(state, qubits);

                for output_index in 0..4 {
                    // Map the gate's output index back to the global state
                    let new_state = self.map_from_gate_index(state, qubits, output_index);
                    let gate_element = gate[output_index][input_index];

                    if gate_element.norm_sqr() > 1e-10 {
                        let contribution = gate_element * amplitude;

                        // Debug log for contribution
                        println!(
                            "Contribution -> State: {}, Input Index: {}, Output Index: {}, New State: {}, Gate Element: {}, Contribution: {}",
                            state, input_index, output_index, new_state, gate_element, contribution
                        );

                        // Add the contribution to the new statevector
                        *new_vector.entry(new_state).or_insert(Complex::new(0.0, 0.0)) += contribution;
                    }
                }
            }
        }

        self.vector = new_vector;
    }




    /// Map a global state index to the gate-specific index.
    fn map_to_gate_index(&self, state: usize, qubits: &[usize]) -> usize {
        let input_index = qubits.iter().enumerate().fold(0, |acc, (i, &qubit)| {
            acc | (((state >> qubit) & 1) << i)
        });
        input_index
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
        let norm: f64 = self.vector.iter().map(|(_, amp)| amp.norm_sqr()).sum();
        if norm != 0.0 {
            let scale = 1.0 / norm.sqrt();
            self.vector.values_mut().for_each(|amp| *amp *= scale);
        }
    }

    /// Validates the statevector for correctness.
    /// - Checks normalization and dimensional consistency.
    pub fn validate(&self) -> Result<(), String> {
        if self.vector.is_empty() {
            return Err("Statevector is empty.".to_string());
        }

        let max_index = self.vector.keys().copied().max().unwrap(); // Highest index
        let num_qubits = (max_index as f64).log2().ceil() as usize; // Infer number of qubits

        let max_allowed_index = (1 << num_qubits) - 1; // 2^n - 1
        if max_index > max_allowed_index {
            return Err(format!(
                "Statevector is inconsistent: index {} exceeds max index {} for {} qubits.",
                max_index, max_allowed_index, num_qubits
            ));
        }

        Ok(())
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;
    use crate::circuit::gates::{cnot, hadamard, identity_gate};
    use std::collections::HashMap;

    /// Helper function to create a `HashMap`-based statevector.
    fn create_statevector(data: Vec<(usize, Complex<f64>)>) -> Statevector {
        Statevector {
            num_qubits: data.len(),
            vector: data.into_iter().collect(),
        }
    }

    fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
        (a - b).norm_sqr() < tol
    }

    /// Statevector Initialization Tests
    mod initialization {
        use super::*;

        #[test]
        fn test_statevector_initialization() {
            let sv = Statevector::new(2); // Two qubits
            assert_eq!(sv.vector.len(), 1); // Only one entry for |00⟩
            assert_eq!(sv.vector.get(&0), Some(&Complex::new(1.0, 0.0))); // Initialized to |00⟩
            assert!(sv
                .vector
                .iter()
                .all(|(&key, &value)| if key == 0 { value == Complex::new(1.0, 0.0) } else { value == Complex::new(0.0, 0.0) }));
        }
    }

    /// Single Qubit Gate Application Tests
    mod single_qubit_gates {
        use super::*;

        #[test]
        fn test_apply_identity_gate() {
            let mut sv = Statevector::new(2); // One qubit

            sv.apply_gate(identity_gate(), &[0]); // Apply identity gate to qubit 0
            assert_eq!(sv.vector.get(&0), Some(&Complex::new(1.0, 0.0))); // |0⟩ remains |0⟩
            assert_eq!(sv.vector.get(&1), None); // |1⟩ remains |1⟩ (not in vector, so implicitly zero)
        }

        #[test]
        fn test_apply_hadamard_gate() {
            let mut sv = Statevector::new(2); // Single qubit initialized to |0⟩
            sv.apply_gate(hadamard(), &[0]); // Apply Hadamard to qubit 0

            let expected_vector = HashMap::from([
                (0, Complex::new(1.0 / 2f64.sqrt(), 0.0)), // |0⟩
                (1, Complex::new(1.0 / 2f64.sqrt(), 0.0)), // |1⟩
            ]);

            for (&key, &expected_amp) in &expected_vector {
                let actual_amp = sv.vector.get(&key).copied().unwrap_or(Complex::new(0.0, 0.0));
                assert!(
                    approx_eq(actual_amp, expected_amp, 1e-10),
                    "Amplitude mismatch at state |{:b}⟩: left = {:?}, right = {:?}",
                    key,
                    actual_amp,
                    expected_amp
                );
            }
        }
    }

    /// Multi-Qubit Gate Application Tests
    mod multi_qubit_gates {
        use super::*;

        #[test]
        fn test_apply_two_qubit_gate_cnot() {
            let mut sv = Statevector::new(2);
            sv.vector.clear(); // Ensure no initial states
            sv.vector.insert(2, Complex::new(1.0, 0.0)); // Set pure |10⟩ state

            let cnot_gate = cnot();
            sv.apply_gate(cnot_gate, &[0, 1]); // Apply CNOT gate (control: qubit 0, target: qubit 1)

            let expected_vector = HashMap::from([
                (3, Complex::new(1.0, 0.0)), // Expected |11⟩ state
            ]);

            assert_eq!(sv.vector, expected_vector);
        }

        #[test]
        fn test_cnot_gate() {
            let mut sv = Statevector::new(2);
            sv.vector.clear();
            sv.vector.insert(2, Complex::new(1.0, 0.0)); // |10⟩

            sv.apply_gate(cnot(), &[0, 1]); // Apply CNOT gate

            let expected_vector = HashMap::from([
                (3, Complex::new(1.0, 0.0)), // |11⟩
            ]);

            assert_eq!(sv.vector, expected_vector);
        }

        #[test]
        fn test_apply_cnot_to_all_zeros() {
            let mut sv = Statevector::new(2);
            sv.vector.insert(0, Complex::new(1.0, 0.0)); // |00⟩ state
            sv.vector.insert(3, Complex::new(0.0, 0.0)); // Add |11⟩ state to ensure 2-qubit representation

            let cnot_gate = cnot();
            sv.apply_gate(cnot_gate, &[0, 1]);

            let expected_vector = HashMap::from([
                (0, Complex::new(1.0, 0.0)), // |00⟩ remains |00⟩
            ]);

            assert_eq!(sv.vector, expected_vector);
        }



        #[test]
        fn test_apply_cnot_to_superposition() {
            let mut sv = Statevector::new(2);
            sv.vector.insert(0, Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)); // |00⟩
            sv.vector.insert(2, Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)); // |10⟩

            let cnot_gate = cnot();
            sv.apply_gate(cnot_gate, &[0, 1]);

            let expected_vector = HashMap::from([
                (0, Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)), // |00⟩
                (3, Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)), // |11⟩
            ]);

            for (key, value) in &expected_vector {
                let actual_value = sv.vector.get(key).cloned().unwrap_or(Complex::new(0.0, 0.0));
                assert!(
                    (actual_value - value).norm() < 1e-10,
                    "Mismatch at state {}: expected {}, got {}",
                    key,
                    value,
                    actual_value
                );
            }
        }

    }

    // todo: verify and fix it!
    mod evolution {
        use super::*;
        fn approx_eq(
            a: &HashMap<usize, Complex<f64>>,
            b: &HashMap<usize, Complex<f64>>,
            tol: f64,
        ) -> bool {
            // Lengths must match
            if a.len() != b.len() {
                return false;
            }

            // Compare each key-value pair
            a.iter().all(|(&key, &val_a)| {
                if let Some(&val_b) = b.get(&key) {
                    (val_a - val_b).norm() < tol // Compare the Complex<f64> values with tolerance
                } else {
                    false // Key mismatch
                }
            })
        }

        #[test]
        fn test_cnot_matrix() {
            let cnot_gate = cnot();
            let zero = Complex::new(0.0, 0.0);
            let one = Complex::new(1.0, 0.0);

            if let Gate::Two(matrix) = cnot_gate {
                assert_eq!(matrix[0][0], one); // |00⟩ -> |00⟩
                assert_eq!(matrix[1][1], one); // |01⟩ -> |01⟩
                assert_eq!(matrix[3][2], one); // |10⟩ -> |11⟩
                assert_eq!(matrix[2][3], one); // |11⟩ -> |10⟩
            } else {
                panic!("CNOT gate is not a two-qubit gate");
            }
        }

        #[test]
        fn test_sequential_gates() {
            let mut sv = Statevector::new(2);
            sv.vector.insert(0, Complex::new(1.0, 0.0)); // Start with |0⟩

            sv.apply_gate(hadamard(), &[0]); // Apply Hadamard
            sv.apply_gate(identity_gate(), &[0]); // Apply Identity (No-op)

            let scale = 1.0 / 2.0_f64.sqrt();
            let expected_vector = HashMap::from([
                (0, Complex::new(scale, 0.0)), // Superposition |0⟩
                (1, Complex::new(scale, 0.0)), // Superposition |1⟩
            ]);

            assert!(
                approx_eq(&sv.vector, &expected_vector, 1e-10),
                "Statevector mismatch: left = {:?}, right = {:?}",
                sv.vector,
                expected_vector
            );
        }

        #[test]
        fn test_complex_gate_combination() {
            let mut sv = Statevector::new(2);
            sv.vector.clear();
            sv.vector.insert(0, Complex::new(1.0, 0.0)); // Start with |0⟩

            sv.apply_gate(hadamard(), &[0]); // Hadamard to qubit 0
            sv.apply_gate(cnot(), &[0, 1]); // CNOT with qubit 0 as control

            let scale = 1.0 / 2.0_f64.sqrt();
            let expected_vector = HashMap::from([
                (1, Complex::new(scale, 0.0)), // |00⟩
                (0, Complex::new(scale, 0.0)), // |11⟩
            ]);

            assert!(
                approx_eq(&sv.vector, &expected_vector, 1e-10),
                "Statevector mismatch: left = {:?}, right = {:?}",
                sv.vector,
                expected_vector
            );
        }
    }

    /// Measurement Tests
    mod measurement {
        use super::*;

        #[test]
        fn test_measurement() {
            let mut sv = Statevector::new(2);
            sv.apply_gate(hadamard(), &[0]);

            let result = sv.measure(0);
            assert!(result == 0 || result == 1);

            let expected = if result == 0 {
                HashMap::from([(0, Complex::new(1.0, 0.0))])
            } else {
                HashMap::from([(1, Complex::new(1.0, 0.0))])
            };

            assert_eq!(sv.vector, expected, "Statevector did not collapse correctly after measurement.");
        }


    }

    /// Validation and Error Handling Tests
    mod validation {
        use super::*;

        #[test]
        #[should_panic(expected = "Qubit indices must be within the range of the quantum system.")]
        fn test_apply_gate_exceeding_qubits() {
            let mut sv = Statevector::new(2); // Start with an empty statevector
            sv.apply_gate(cnot(), &[0, 1, 100]); // Invalid gate, qubit 100 is out of logical range
        }

        #[test]
        #[should_panic(expected = "Statevector is inconsistent: index 8 exceeds max index 7 for 3 qubits.")]
        fn test_apply_gate_invalid_statevector_size() {
            let mut sv = create_statevector(vec![
                (0, Complex::new(1.0, 0.0)),
                (8, Complex::new(0.5, 0.0)), // Invalid index for 3 qubits
            ]);
            sv.validate().unwrap(); // Validation fails due to the invalid state.
        }

        #[test]
        #[should_panic(expected = "Qubit indices must be within the range of the quantum system.")]
        fn test_invalid_gate_size() {
            let mut sv = Statevector::new(2);
            sv.vector.insert(0, Complex::new(1.0, 0.0));

            let cnot_gate = cnot();
            sv.apply_gate(cnot_gate, &[0, 2]); // Invalid: qubit index out of range
        }

        #[test]
        fn test_cleanup_removes_near_zero_amplitudes() {
            let mut sv = Statevector::new(2);
            sv.vector.insert(0, Complex::new(1e-12, 1e-12)); // Small amplitude
            sv.vector.insert(1, Complex::new(1.0, 0.0));

            sv.normalize_and_cleanup();

            assert_eq!(sv.vector.len(), 1);
            assert!(sv.vector.contains_key(&1), "Only |1⟩ should remain.");
        }



    }

    /// Edge Case Tests
    mod edge_cases {
        use super::*;

        #[test]
        fn test_apply_gate_to_empty_statevector() {
            let sv = create_statevector(vec![]); // Empty statevector
            assert!(sv.validate().is_err(), "Validation should fail for empty statevector");
        }

        #[test]
        fn test_no_op_on_initialized_statevector() {
            let mut sv = Statevector::new(2); // Initialized to |0⟩ state
            let identity = identity_gate();
            sv.apply_gate(identity, &[0]); // Applying identity gate should not change the state

            // Check that the statevector remains in the |0⟩ state
            let expected_vector = HashMap::from([
                (0, Complex::new(1.0, 0.0)), // |0⟩ state
            ]);

            assert_eq!(
                sv.vector, expected_vector,
                "Statevector should remain unchanged after applying the identity gate."
            );
        }



        #[test]
        fn test_apply_identity_gate_to_zero_statevector() {
            let mut sv = Statevector::new(2);
            let identity_gate = identity_gate();
            sv.apply_gate(identity_gate, &[0]);

            let expected_vector = HashMap::from([
                (0, Complex::new(1.0, 0.0)), // |0⟩ remains |0⟩
            ]);

            assert_eq!(sv.vector, expected_vector);
        }
    }
}
