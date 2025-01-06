use sha2::Sha256;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use pbkdf2::{pbkdf2_hmac, password_hash::SaltString};
use rand_core::OsRng;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}};

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
    let cipher = Aes256Gcm::new(Key::from_slice(key));
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
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).expect("decryption failure")
}

/// Encrypts data using ChaCha20-Poly1305.
pub fn chacha_encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher.encrypt(Nonce::from_slice(nonce), plaintext).expect("encryption failure")
}

/// Decrypts data using ChaCha20-Poly1305.
pub fn chacha_decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).expect("decryption failure")
}

/// Derives a 256-bit key from a password using PBKDF2.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}

/// Generates a random salt for key derivation.
pub fn generate_salt() -> Vec<u8> {
    let salt = SaltString::generate(&mut OsRng);
    salt.as_bytes().to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm() {
        let key = [0u8; 32]; // Example key
        let nonce = [0u8; 12]; // Example nonce
        let plaintext = b"zana quantum-ai";

        let ciphertext = encrypt(&key, &nonce, plaintext);
        let decrypted = decrypt(&key, &nonce, &ciphertext);

        assert_eq!(decrypted, plaintext);
    }
}
