use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use std::fs;
use base64::{encode, Engine};
use base64::engine::general_purpose;
use rand_core::{RngCore, SeedableRng};

/// Generates a new ed25519 keypair using thread_rng
pub fn generate_keypair() -> Keypair {
    // Use thread_rng to generate random bytes
    let mut rng = rand::thread_rng();
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    // Create the secret key
    let secret_key = SecretKey::from_bytes(&secret_bytes).expect("Failed to create secret key");

    // Derive the public key
    let public_key: PublicKey = (&secret_key).into();

    // Combine into a keypair
    Keypair { secret: secret_key, public: public_key }
}

/// Signs a message using the provided keypair
pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

/// Verifies a signed message using the public key and signature
pub fn verify_message(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}

/// Saves a keypair to a file
pub fn save_keypair_to_file(keypair: &Keypair, file_path: &str) -> std::io::Result<()> {
    let private_key_b64 = encode(keypair.secret.to_bytes());
    let public_key_b64 = encode(keypair.public.as_bytes());

    let content = format!("{}\n{}", private_key_b64, public_key_b64);
    fs::write(file_path, content)
}

/// Loads a keypair from a file
pub fn load_keypair_from_file(file_path: &str) -> std::io::Result<Keypair> {
    let content = fs::read_to_string(file_path)?;
    let mut lines = content.lines();

    let private_key_b64 = lines
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file format"))?;
    let public_key_b64 = lines
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file format"))?;

    let private_key_bytes = general_purpose::STANDARD
        .decode(private_key_b64)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid base64 encoding"))?;
    let public_key_bytes = general_purpose::STANDARD
        .decode(public_key_b64)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid base64 encoding"))?;

    let secret = ed25519_dalek::SecretKey::from_bytes(&private_key_bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid private key"))?;
    let public = ed25519_dalek::PublicKey::from_bytes(&public_key_bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid public key"))?;

    Ok(Keypair { secret, public })
}


/// Verifies multiple signed messages in a batch
pub fn batch_verify(public_keys: &[PublicKey], messages: &[&[u8]], signatures: &[Signature]) -> bool {
    if public_keys.len() != messages.len() || messages.len() != signatures.len() {
        return false; // Mismatched lengths
    }

    public_keys.iter()
        .zip(messages.iter())
        .zip(signatures.iter())
        .all(|((public_key, &message), signature)| verify_message(public_key, message, signature))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair();
        assert_eq!(keypair.public.as_bytes().len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let message = b"Hello, Rust!";
        let signature = sign_message(&keypair, message);

        let is_valid = verify_message(&keypair.public, message, &signature);
        assert!(is_valid, "Signature verification should succeed");
    }

    #[test]
    fn test_tampered_message() {
        let keypair = generate_keypair();
        let message = b"Original message";
        let signature = sign_message(&keypair, message);

        let tampered_message = b"Tampered message";
        let is_valid = verify_message(&keypair.public, tampered_message, &signature);
        assert!(!is_valid, "Verification should fail for tampered messages");
    }

    #[test]
    fn test_save_and_load_keypair() {
        let keypair = generate_keypair();
        let file_path = "test_keypair.txt";

        save_keypair_to_file(&keypair, file_path).expect("Failed to save keypair to file");
        let loaded_keypair = load_keypair_from_file(file_path).expect("Failed to load keypair from file");

        assert_eq!(keypair.public, loaded_keypair.public);
        remove_file(file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_invalid_file_loading() {
        let result = load_keypair_from_file("non_existent_file.txt");
        assert!(result.is_err(), "Loading non-existent file should return an error");
    }

    #[test]
    fn test_batch_verification() {
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();

        let messages: Vec<&[u8; 9]> = vec![b"Message 1", b"Message 2"];
        let signatures = vec![
            sign_message(&keypair1, &messages[0][..]), // Coerce to slice
            sign_message(&keypair2, &messages[1][..]), // Coerce to slice
        ];

        let public_keys = vec![keypair1.public, keypair2.public];
        let messages_refs: Vec<&[u8]> = messages.iter().map(|m| &m[..]).collect(); // Coerce each element to &[u8]

        let is_batch_valid = batch_verify(&public_keys, &messages_refs, &signatures);
        assert!(is_batch_valid, "Batch verification should succeed");

        let tampered_messages_refs: Vec<&[u8]> = vec![b"Message 1", b"Tampered Message"];
        let is_tampered_batch_valid = batch_verify(&public_keys, &tampered_messages_refs, &signatures);
        assert!(!is_tampered_batch_valid, "Batch verification should fail for tampered messages");
    }

}
