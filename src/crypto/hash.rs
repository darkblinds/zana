use sha2::{Digest, Sha256};
use blake2::{Blake2b512, Digest};
use hmac::{Hmac, Mac};

/// Computes the SHA-256 hash of the given input data.
///
/// # Arguments
/// - `data`: The input data to hash.
///
/// # Returns
/// A vector containing the hash bytes.
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

use sha2::{Sha512};
use blake2::{Blake2b512, Digest};

/// Computes the SHA-512 hash of the given input data.
pub fn sha512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Computes the Blake2b512 hash of the given input data.
pub fn blake2b512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

type HmacSha256 = Hmac<Sha256>;

/// Computes the HMAC of the given data using the provided key.
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key initialization failed");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"zana quantum-ai";
        let hash = sha256(data);
        let expected = hex::decode("1d9da67616c2e2d49dbe81cd155dd63c5e6a3d786285e0cdcb2645686af122f8").unwrap();
        assert_eq!(hash, expected);
    }
}
