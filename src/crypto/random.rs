use rand::Rng;

/// Generates a random 256-bit key.
pub fn generate_random_key() -> [u8; 32] {
    rand::thread_rng().gen()
}

/// Generates a random salt of the given length.
///
/// # Arguments
/// - `len`: The desired length of the salt.
///
/// # Returns
/// A vector containing the generated random salt.
pub fn generate_random_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0u8; len];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_key() {
        let key1 = generate_random_key();
        let key2 = generate_random_key();

        assert_ne!(key1, key2, "Keys should be random and unique");
        assert_eq!(key1.len(), 32, "Key length should be 256 bits");
    }

    #[test]
    fn test_generate_random_salt() {
        let salt1 = generate_random_salt(16);
        let salt2 = generate_random_salt(16);

        assert_ne!(salt1, salt2, "Salts should be random and unique");
        assert_eq!(salt1.len(), 16, "Salt length should match the given length");
        assert_eq!(salt2.len(), 16, "Salt length should match the given length");
    }
}
