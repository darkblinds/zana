//! Post-Quantum Cryptography Utilities
//!
//! This module provides cryptographic functions that are resistant to attacks by quantum computers.
//! It includes implementations of lattice-based cryptography and hash-based cryptography.

use rand::Rng;
use sha2::{Sha256, Digest};

/// Parameters for the Learning With Errors (LWE) key exchange.
const MODULUS: u32 = 65536; // Prime modulus for lattice operations
const SECRET_BOUND: u32 = 100; // Range of secret coefficients
const ERROR_BOUND: u32 = 50; // Range of error coefficients
const VECTOR_SIZE: usize = 10; // Dimension of the lattice vectors

/// Generates a random lattice vector of given size within the modulus.
fn generate_random_vector(size: usize, modulus: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..modulus)).collect()
}

/// Generates a secret vector used in the Learning With Errors (LWE) key exchange.
fn generate_secret_vector(size: usize, bound: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..bound as i32) - (bound as i32) / 2).collect()
}

/// LWE Key Exchange: Generates public and secret keys for Alice.
///
/// # Returns
/// A tuple containing Alice's public vector and secret vector.
pub fn lwe_generate_keypair() -> (Vec<u32>, Vec<i32>) {
    let public_vector = generate_random_vector(VECTOR_SIZE, MODULUS);
    let secret_vector = generate_secret_vector(VECTOR_SIZE, SECRET_BOUND);
    (public_vector, secret_vector)
}

/// LWE Key Exchange: Generates the shared secret.
///
/// # Arguments
/// - `public_vector` - The public vector from Alice.
/// - `secret_vector` - The secret vector from Bob.
///
/// # Returns
/// The shared secret vector.
pub fn lwe_generate_shared_secret(public_vector: &[u32], secret_vector: &[i32]) -> u32 {
    let mut rng = rand::thread_rng();
    let error: i32 = rng.gen_range(0..ERROR_BOUND as i32) - (ERROR_BOUND as i32) / 2;

    let dot_product: i32 = public_vector
        .iter()
        .zip(secret_vector.iter())
        .map(|(p, s)| *p as i32 * s)
        .sum();

    ((dot_product + error).rem_euclid(MODULUS as i32)) as u32
}


/// Lamport Signature Scheme: Generates private and public keys.
///
/// # Returns
/// A tuple containing the private key and public key.
pub fn lamport_generate_keypair() -> (Vec<[Vec<u8>; 2]>, Vec<[Vec<u8>; 2]>) {
    let mut rng = rand::thread_rng();
    let private_key: Vec<[Vec<u8>; 2]> = (0..256)
        .map(|_| {
            [
                (0..32).map(|_| rng.gen::<u8>()).collect(),
                (0..32).map(|_| rng.gen::<u8>()).collect(),
            ]
        })
        .collect();

    let public_key: Vec<[Vec<u8>; 2]> = private_key
        .iter()
        .map(|pair| {
            [
                Sha256::digest(&pair[0]).to_vec(),
                Sha256::digest(&pair[1]).to_vec(),
            ]
        })
        .collect();

    (private_key, public_key)
}


/// Lamport Signature Scheme: Signs a message.
///
/// # Arguments
/// - `message` - The message to be signed.
/// - `private_key` - The private key for signing.
///
/// # Returns
/// The signature.
pub fn lamport_sign(message: &[u8], private_key: &Vec<[Vec<u8>; 2]>) -> Vec<Vec<u8>> {
    let hash = Sha256::digest(message); // Compute the hash of the message
    let signature: Vec<Vec<u8>> = hash
        .iter()
        .enumerate()
        .map(|(i, bit)| private_key[i][(*bit & 1) as usize].clone()) // Extract the correct Vec<u8>
        .collect();

    signature
}



/// Lamport Signature Scheme: Verifies a signature.
///
/// # Arguments
/// - `message` - The original message.
/// - `signature` - The signature to verify.
/// - `public_key` - The public key.
///
/// # Returns
/// `true` if the signature is valid, `false` otherwise.
pub fn lamport_verify(
    message: &[u8],
    signature: &[Vec<u8>],
    public_key: &[Vec<[Vec<u8>; 2]>],
) -> bool {
    // Calculate the SHA-256 hash of the message
    let hash = Sha256::digest(message);

    // Iterate over the hash bytes and compare with public key
    let verification_result = hash.iter().enumerate().all(|(i, bit)| {
        let public_key_vec = &public_key[i][0];

        if let Some(public_key_byte) = public_key_vec.get(*bit as usize) {
            let signature_byte = signature[i][*bit as usize];
            signature_byte == public_key_byte[0]
        } else {
            false // Handle the case where the public key byte is not found
        }
    });

    verification_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_vector() {
        let vector = generate_random_vector(10, 65536);
        assert_eq!(vector.len(), 10);
        assert!(vector.iter().all(|&v| v < 65536));
    }

    #[test]
    fn test_generate_secret_vector() {
        let vector = generate_secret_vector(10, 100);
        assert_eq!(vector.len(), 10);
        assert!(vector.iter().all(|&v| v >= -50 && v < 50));
    }

    #[test]
    fn test_lwe_generate_keypair() {
        let (public_vector, secret_vector) = lwe_generate_keypair();
        assert_eq!(public_vector.len(), VECTOR_SIZE);
        assert!(public_vector.iter().all(|&v| v < MODULUS));
        assert_eq!(secret_vector.len(), VECTOR_SIZE);
        assert!(secret_vector.iter().all(|&v| v >= -(SECRET_BOUND as i32) / 2 && v < (SECRET_BOUND as i32) / 2));
    }

    #[test]
    fn test_lwe_generate_shared_secret() {
        let (alice_public, alice_secret) = lwe_generate_keypair();
        let (_, bob_secret) = lwe_generate_keypair();
        let shared_secret_alice = lwe_generate_shared_secret(&alice_public, &bob_secret);
        let shared_secret_bob = lwe_generate_shared_secret(&alice_public, &bob_secret);
        // Assert shared secrets fall within valid range (modulus)
        assert!(shared_secret_alice < MODULUS);
        assert!(shared_secret_bob < MODULUS);
    }

    #[test]
    fn test_lamport_generate_keypair() {
        let (private_key, public_key) = lamport_generate_keypair();
        assert_eq!(private_key.len(), 256);
        assert_eq!(public_key.len(), 256);

        for i in 0..256 {
            assert_eq!(private_key[i].len(), 2);
            assert_eq!(public_key[i].len(), 2);
            assert_eq!(private_key[i][0].len(), 32);
            assert_eq!(private_key[i][1].len(), 32);
            assert_eq!(public_key[i][0].len(), 32);
            assert_eq!(public_key[i][1].len(), 32);
        }
    }

    #[test]
    fn test_lamport_sign() {
        // Generate keypair
        let (private_key, public_key) = lamport_generate_keypair();

        // Define test message
        let message = b"Test message";

        // Sign the message
        let signature = lamport_sign(message, &private_key); // Pass the Vec directly

        // Verify signature length
        assert_eq!(signature.len(), 32, "Signature length mismatch");

        // Hash the message
        let hash = Sha256::digest(message);

        // Validate signature against private key
        for (i, &hash_byte) in hash.iter().enumerate() {
            let bit = (hash_byte & 1) as usize;
            assert_eq!(signature[i], private_key[i][bit], "Mismatch at index {}", i);
        }
    }


}

