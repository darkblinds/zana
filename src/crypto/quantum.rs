//! Quantum Cryptography Utilities
//!
//! This module provides tools and simulations for quantum cryptographic concepts,
//! including quantum key distribution (BB84), quantum random number generation (QRNG),
//! and basic qubit operations.

use rand::{thread_rng, Rng};
use std::collections::HashMap;

/// Represents the state of a single qubit.
///
/// Qubits can exist in the computational basis (|0⟩, |1⟩) or the diagonal basis (|+⟩, |−⟩).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QubitState {
    /// Represents the |0⟩ state in the computational basis.
    Zero,
    /// Represents the |1⟩ state in the computational basis.
    One,
    /// Represents the |+⟩ state in the diagonal basis.
    Plus,
    /// Represents the |−⟩ state in the diagonal basis.
    Minus,
}

/// Applies a Hadamard gate to a qubit, toggling between the computational and diagonal bases.
///
/// # Arguments
///
/// * `qubit` - The input qubit state.
///
/// # Returns
///
/// The new qubit state after applying the Hadamard gate.
///
/// # Examples
///
/// ```
/// use quantum_crypto::QubitState;
/// use quantum_crypto::apply_hadamard;
///
/// let qubit = QubitState::Zero;
/// let new_state = apply_hadamard(qubit);
/// assert_eq!(new_state, QubitState::Plus);
/// ```
pub fn apply_hadamard(qubit: QubitState) -> QubitState {
    match qubit {
        QubitState::Zero => QubitState::Plus,
        QubitState::One => QubitState::Minus,
        QubitState::Plus => QubitState::Zero,
        QubitState::Minus => QubitState::One,
    }
}

/// Generates a single quantum random bit using simulated quantum principles.
///
/// # Returns
///
/// A random bit (0 or 1).
///
/// # Examples
///
/// ```
/// use quantum_crypto::generate_quantum_random_bit;
///
/// let bit = generate_quantum_random_bit();
/// assert!(bit == 0 || bit == 1);
/// ```
pub fn generate_quantum_random_bit() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(0..2) // Simulates a random quantum measurement
}

/// Simulates the BB84 Quantum Key Distribution (QKD) protocol.
///
/// This simulation generates random bits for Alice and Bob, with a public reconciliation of their bases.
///
/// # Returns
///
/// A tuple containing:
/// - Alice's raw bits.
/// - Bob's raw bits.
///
/// # Examples
///
/// ```
/// use quantum_crypto::bb84_simulation;
///
/// let (alice_bits, bob_bits) = bb84_simulation();
/// assert_eq!(alice_bits.len(), bob_bits.len());
/// ```
pub fn bb84_simulation() -> (Vec<u8>, Vec<u8>) {
    let mut rng = thread_rng();

    // Step 1: Generate random bits and random bases for Alice
    let alice_bits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..2)).collect();
    let alice_bases: Vec<u8> = (0..10).map(|_| rng.gen_range(0..2)).collect();

    // Step 2: Bob chooses random bases
    let bob_bases: Vec<u8> = (0..10).map(|_| rng.gen_range(0..2)).collect();

    // Step 3: Alice and Bob share their bases publicly
    let mut bob_bits = vec![];
    for (bit, (alice_base, bob_base)) in alice_bits.iter().zip(alice_bases.iter().zip(bob_bases.iter())) {
        if alice_base == bob_base {
            bob_bits.push(*bit);
        } else {
            bob_bits.push(generate_quantum_random_bit());
        }
    }

    (alice_bits, bob_bits)
}

/// Verifies the similarity of Alice's and Bob's keys in the BB84 protocol.
///
/// # Arguments
///
/// * `alice_bits` - Alice's bits from the BB84 simulation.
/// * `bob_bits` - Bob's bits from the BB84 simulation.
///
/// # Returns
///
/// The number of matching bits between Alice and Bob.
///
/// # Examples
///
/// ```
/// use quantum_crypto::{bb84_simulation, verify_bb84_keys};
///
/// let (alice_bits, bob_bits) = bb84_simulation();
/// let matches = verify_bb84_keys(&alice_bits, &bob_bits);
/// assert!(matches <= alice_bits.len());
/// ```
pub fn verify_bb84_keys(alice_bits: &[u8], bob_bits: &[u8]) -> usize {
    alice_bits.iter().zip(bob_bits.iter()).filter(|(a, b)| a == b).count()
}

/// Simulates an eavesdropper (Eve) in the BB84 protocol.
///
/// Eve intercepts and measures the qubits before they reach Bob, using random bases for measurement.
///
/// # Returns
///
/// A tuple containing:
/// - Alice's bits.
/// - Bob's bits.
/// - Eve's measured bits.
///
/// # Examples
///
/// ```
/// use quantum_crypto::simulate_eavesdropping;
///
/// let (alice_bits, bob_bits, eve_bits) = simulate_eavesdropping();
/// assert_eq!(alice_bits.len(), bob_bits.len());
/// assert_eq!(bob_bits.len(), eve_bits.len());
/// ```
pub fn simulate_eavesdropping() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let (alice_bits, bob_bits) = bb84_simulation();
    let mut rng = thread_rng();
    let eve_bases: Vec<u8> = (0..10).map(|_| rng.gen_range(0..2)).collect();
    let mut eve_bits = vec![];

    // Eve measures the bits before they reach Bob
    for ((bit, alice_base), eve_base) in alice_bits.iter().zip(eve_bases.iter()).zip(bob_bits.iter()) {
        if alice_base == eve_base {
            eve_bits.push(*bit);
        } else {
            eve_bits.push(generate_quantum_random_bit());
        }
    }


    (alice_bits, bob_bits, eve_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_gate() {
        assert_eq!(apply_hadamard(QubitState::Zero), QubitState::Plus);
        assert_eq!(apply_hadamard(QubitState::Plus), QubitState::Zero);
    }

    #[test]
    fn test_generate_quantum_random_bit() {
        let bit = generate_quantum_random_bit();
        assert!(bit == 0 || bit == 1);
    }

    #[test]
    fn test_bb84_simulation() {
        let (alice_bits, bob_bits) = bb84_simulation();
        assert_eq!(alice_bits.len(), bob_bits.len());
    }

    #[test]
    fn test_verify_bb84_keys() {
        let (alice_bits, bob_bits) = bb84_simulation();
        let matches = verify_bb84_keys(&alice_bits, &bob_bits);
        assert!(matches <= alice_bits.len());
    }

    #[test]
    fn test_simulate_eavesdropping() {
        let (alice_bits, bob_bits, eve_bits) = simulate_eavesdropping();
        assert_eq!(alice_bits.len(), bob_bits.len());
        assert_eq!(bob_bits.len(), eve_bits.len());
    }
}
