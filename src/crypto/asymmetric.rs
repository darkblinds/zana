use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme, Pkcs1v15Encrypt};
use rand::rngs::OsRng;

/// Generates an RSA key pair (private and public keys).
///
/// # Returns
/// A tuple containing the private key and public key.
pub fn generate_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// Encrypts data using the RSA public key and PKCS1 v1.5 padding.
///
/// # Arguments
/// - `public_key`: The RSA public key.
/// - `plaintext`: The data to encrypt.
///
/// # Returns
/// The encrypted data (ciphertext).
pub fn rsa_encrypt(public_key: &RsaPublicKey, plaintext: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, plaintext)
        .expect("Failed to encrypt")
}

/// Decrypts data using the RSA private key and PKCS1 v1.5 padding.
///
/// # Arguments
/// - `private_key`: The RSA private key.
/// - `ciphertext`: The encrypted data to decrypt.
///
/// # Returns
/// The decrypted data (plaintext).
pub fn rsa_decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    private_key
        .decrypt(Pkcs1v15Encrypt, ciphertext)
        .expect("Failed to decrypt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_encryption_decryption() {
        let (private_key, public_key) = generate_rsa_keys();
        let message = b"zana quantum-ai";

        let ciphertext = rsa_encrypt(&public_key, message);
        let decrypted = rsa_decrypt(&private_key, &ciphertext);

        assert_eq!(decrypted, message, "Decrypted message does not match original");
    }
}