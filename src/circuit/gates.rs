use num_complex::Complex;

//// Gates are functions manipulating statevector and evolving it according to Schrödinger's
//// Quantum gates are represented by unitary matrices'
// applying a gate to a statevector involves matrix multiplication.

/// Represents a quantum gate.
/// It can be either a single-qubit gate or a two-qubit gate
#[derive(Debug)] // Automatically implement the Debug trait
#[derive(Clone)]
pub enum Gate {
    Single([[Complex<f64>; 2]; 2]), // Single-qubit gate (2x2 matrix)
    Two([[Complex<f64>; 4]; 4]),    // Two-qubit gate (4x4 matrix)
}

/// Returns the Hadamard gate matrix.
///
/// The Hadamard gate creates a superposition of |0> and |1> states.
/// H|0> = (|0> + |1>) / √2
/// H|1> = (|0> - |1>) / √2
pub fn hadamard() -> Gate {
    let scale = 1.0 / 2.0_f64.sqrt();
    Gate::Single([
        [Complex::new(scale, 0.0), Complex::new(scale, 0.0)],
        [Complex::new(scale, 0.0), Complex::new(-scale, 0.0)],
    ])
}


/// Returns the CNOT gate matrix.
///
/// The CNOT gate flips the target qubit if the control qubit is |1>.
pub fn cnot() -> Gate {
    let zero = Complex::new(0.0, 0.0);
    let one = Complex::new(1.0, 0.0);
    Gate::Two([
        [one, zero, zero, zero],
        [zero, one, zero, zero],
        [zero, zero, zero, one],
        [zero, zero, one, zero],
    ])
}

/// Returns the Identity gate as a `Gate::Single`.
///
/// The Identity gate leaves the state of the qubit unchanged:
/// - `|0⟩` → `|0⟩`
/// - `|1⟩` → `|1⟩`
pub fn identity_gate() -> Gate {
    Gate::Single([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ])
}


/// Returns the Pauli-X (NOT) gate as a `Gate::Single`.
///
/// This gate flips the state of a qubit:
/// - `|0⟩` → `|1⟩`
/// - `|1⟩` → `|0⟩`
pub fn pauli_x() -> Gate {
    Gate::Single([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}

/// Returns the Pauli-Z gate as a `Gate::Single`.
///
/// This gate applies a phase shift to the `|1⟩` state:
/// - `|0⟩` → `|0⟩`
/// - `|1⟩` → `-|1⟩`
pub fn pauli_z() -> Gate {
    Gate::Single([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ])
}

/// Returns the rotation gate matrix for rotation around the X-axis as a `Gate::Single`.
///
/// Rx(θ) = [[ cos(θ/2), -i*sin(θ/2) ],
///          [-i*sin(θ/2), cos(θ/2) ]]
pub fn rotation_x(theta: f64) -> Gate {
    let half_theta = theta / 2.0; // Use θ/2 for the formula
    let cos_half_theta = Complex::new(half_theta.cos(), 0.0); // Real part
    let sin_half_theta = Complex::new(0.0, -half_theta.sin()); // Imaginary part (-i)

    Gate::Single([
        [cos_half_theta, sin_half_theta],
        [sin_half_theta, cos_half_theta],
    ])
}

/// Returns the rotation gate matrix for rotation around the Y-axis as a `Gate::Single`.
///
/// Ry(θ) = [[ cos(θ/2), -sin(θ/2) ],
///          [ sin(θ/2), cos(θ/2) ]]
pub fn rotation_y(theta: f64) -> Gate {
    let half_theta = theta / 2.0; // Use θ/2
    let cos_half_theta = Complex::new(half_theta.cos(), 0.0);
    let sin_half_theta = Complex::new(half_theta.sin(), 0.0);
    Gate::Single([
        [cos_half_theta, -sin_half_theta],
        [sin_half_theta, cos_half_theta],
    ])
}

/// Returns the rotation gate matrix for rotation around the Z-axis as a `Gate::Single`.
///
/// Rz(θ) = [[ exp(-iθ), 0 ],
///          [ 0, exp(iθ) ]]
pub fn rotation_z(theta: f64) -> Gate {
    let exp_minus_i_theta = Complex::from_polar(1.0, -theta); // e^(-iθ)
    let exp_i_theta = Complex::from_polar(1.0, theta);        // e^(iθ)

    Gate::Single([
        [exp_minus_i_theta, Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), exp_i_theta],
    ])
}

/// Returns the SWAP gate matrix as a `Gate::Two`.
///
/// The SWAP gate exchanges the states of two qubits.
pub fn swap() -> Gate {
    Gate::Two([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ])
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_hadamard_gate() {
        if let Gate::Single(h) = hadamard() {
            let scale = 1.0 / 2.0_f64.sqrt();
            let expected = [
                [Complex::new(scale, 0.0), Complex::new(scale, 0.0)],
                [Complex::new(scale, 0.0), Complex::new(-scale, 0.0)],
            ];
            assert_eq!(h, expected);
        } else {
            panic!("Hadamard gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_cnot_gate() {
        if let Gate::Two(c) = cnot() {
            let zero = Complex::new(0.0, 0.0);
            let one = Complex::new(1.0, 0.0);
            let expected = [
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, zero, one],
                [zero, zero, one, zero],
            ];
            assert_eq!(c, expected);
        } else {
            panic!("CNOT gate did not return a Two-qubit gate");
        }
    }

    #[test]
    fn test_identity_gate() {
        if let Gate::Single(identity) = identity_gate() {
            let expected = [
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ];
            assert_eq!(identity, expected, "Identity gate matrix mismatch");
        } else {
            panic!("Identity gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_pauli_x() {
        if let Gate::Single(pauli_x) = pauli_x() {
            assert_eq!(pauli_x[0][1], Complex::new(1.0, 0.0));
            assert_eq!(pauli_x[1][0], Complex::new(1.0, 0.0));
            assert_eq!(pauli_x[0][0], Complex::new(0.0, 0.0));
            assert_eq!(pauli_x[1][1], Complex::new(0.0, 0.0));
        } else {
            panic!("Pauli-X gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_pauli_z() {
        if let Gate::Single(pauli_z) = pauli_z() {
            assert_eq!(pauli_z[0][0], Complex::new(1.0, 0.0));
            assert_eq!(pauli_z[1][1], Complex::new(-1.0, 0.0));
            assert_eq!(pauli_z[0][1], Complex::new(0.0, 0.0));
            assert_eq!(pauli_z[1][0], Complex::new(0.0, 0.0));
        } else {
            panic!("Pauli-Z gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_rotation_x_gate() {
        let theta = PI / 2.0; // θ = π/2
        if let Gate::Single(rx) = rotation_x(theta) {
            let half_theta = theta / 2.0;
            let expected = [
                [Complex::new(half_theta.cos(), 0.0), Complex::new(0.0, -half_theta.sin())],
                [Complex::new(0.0, -half_theta.sin()), Complex::new(half_theta.cos(), 0.0)],
            ];
            assert_eq!(rx, expected, "Rotation X gate matrix mismatch");
        } else {
            panic!("Rotation X gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_rotation_y_gate() {
        let theta = PI / 2.0; // θ = π/2
        if let Gate::Single(ry) = rotation_y(theta) {
            let half_theta = theta / 2.0;
            let expected = [
                [Complex::new(half_theta.cos(), 0.0), Complex::new(-half_theta.sin(), 0.0)],
                [Complex::new(half_theta.sin(), 0.0), Complex::new(half_theta.cos(), 0.0)],
            ];
            assert_eq!(ry, expected, "Rotation Y gate matrix mismatch");
        } else {
            panic!("Rotation Y gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_rotation_z_gate() {
        let theta = PI / 2.0; // θ = π/2
        if let Gate::Single(rz) = rotation_z(theta) {
            let expected = [
                [Complex::from_polar(1.0, -theta), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::from_polar(1.0, theta)],
            ];
            assert_eq!(rz, expected, "Rotation Z gate matrix mismatch");
        } else {
            panic!("Rotation Z gate did not return a Single-qubit gate");
        }
    }

    #[test]
    fn test_swap_gate() {
        if let Gate::Two(swap) = swap() {
            let expected = [
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ];
            assert_eq!(swap, expected, "SWAP gate matrix mismatch");
        } else {
            panic!("SWAP gate did not return a Two-qubit gate");
        }
    }

}
