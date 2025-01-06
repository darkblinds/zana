use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use rand_core::RngCore;
use sha2::{Sha256, Digest};

/// Encrypts the given plaintext using AES-256-GCM.
///
/// # Arguments
/// - `key`: A 256-bit key.
/// - `nonce`: A unique 96-bit nonce.
/// - `plaintext`: The data to encrypt.
///
/// # Returns
/// The ciphertext.
pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher.encrypt(Nonce::from_slice(nonce), plaintext).expect("encryption failure")
}

/// Decrypts the given ciphertext using AES-256-GCM.
///
/// # Arguments
/// - `key`: A 256-bit key.
/// - `nonce`: A unique 96-bit nonce.
/// - `ciphertext`: The encrypted data.
///
/// # Returns
/// The plaintext.
pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).expect("decryption failure")
}

/// Generates a random 256-bit key for encryption.
///
/// # Returns
/// A random 256-bit key.
pub fn generate_random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Generates a random 96-bit nonce for encryption.
///
/// # Returns
/// A random 96-bit nonce.
pub fn generate_random_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Derives a key from a password using SHA-256.
///
/// # Arguments
/// - `password`: The password to derive the key from.
///
/// # Returns
/// A 256-bit key derived from the password.
pub fn derive_key_from_password(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}

/// Verifies if two sets of data are equal in constant time.
///
/// # Arguments
/// - `data1`: First data to compare.
/// - `data2`: Second data to compare.
///
/// # Returns
/// `true` if both data are equal, otherwise `false`.
pub fn constant_time_compare(data1: &[u8], data2: &[u8]) -> bool {
    use subtle::ConstantTimeEq;
    data1.ct_eq(data2).unwrap_u8() == 1
}

/// Hashes data using SHA-256.
///
/// # Arguments
/// - `data`: The data to hash.
///
/// # Returns
/// The hash of the data.
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result[..32]);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_random_key();
        let nonce = generate_random_nonce();
        let plaintext = b"Hello, world!";

        let ciphertext = encrypt(&key, &nonce, plaintext);
        let decrypted = decrypt(&key, &nonce, &ciphertext);

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_derive_key_from_password() {
        let password = "securepassword";
        let key1 = derive_key_from_password(password);
        let key2 = derive_key_from_password(password);

        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_constant_time_compare() {
        let data1 = b"securedata";
        let data2 = b"securedata";
        let data3 = b"differentdata";

        assert!(constant_time_compare(data1, data2));
        assert!(!constant_time_compare(data1, data3));
    }

    #[test]
    fn test_hash_sha256() {
        let data = b"important data";
        let hash1 = hash_sha256(data);
        let hash2 = hash_sha256(data);

        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_generate_random_key() {
        let key1 = generate_random_key();
        let key2 = generate_random_key();

        assert_ne!(key1, key2); // Keys should be random and unique
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_generate_random_nonce() {
        let nonce1 = generate_random_nonce();
        let nonce2 = generate_random_nonce();

        assert_ne!(nonce1, nonce2); // Nonces should be random and unique
        assert_eq!(nonce1.len(), 12);
    }
}
