use sha2::{Digest, Sha256, Sha512};
use blake2::Blake2b512;
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

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        hex::decode(hex).expect("Failed to decode hex string")
    }

    fn bytes_to_hex(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }

    #[test]
    fn test_sha256() {
        let data = b"zana quantum-ai";
        let hash = sha256(data);
        println!("SHA256 computed: {}", bytes_to_hex(&hash));
        let expected = hex_to_bytes("91cdb2a80db3fab915f8dabffd5cd128ac931aea6437e4cba13d2a4329128768");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_sha512() {
        let data = b"zana quantum-ai";
        let hash = sha512(data);
        println!("SHA512 computed: {}", bytes_to_hex(&hash));
        let expected = hex_to_bytes("922e82ceab84aef4ac8851c60e1c564cf7c977e50452cd10004d04b8dcba6969f507c7328b7ba7bb3b8480cf9c49f48d99a08d8dbc569ce3d0985324bf51ed69");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_blake2b512() {
        let data = b"zana quantum-ai";
        let hash = blake2b512(data);
        println!("Blake2b512 computed: {}", bytes_to_hex(&hash));
        let expected = hex_to_bytes("10f7e3149efbe202ae38cee32087b939bea73490e6eaef4ffbc5af6b43b5c81b1615fe3fd891f7a540d32a08ee31405cb65e6ff8ec9e94941af1acc20fe874e8");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"my-secret-key";
        let data = b"zana quantum-ai";
        let hmac = hmac_sha256(key, data);
        println!("HMAC-SHA256 computed: {}", bytes_to_hex(&hmac));
        let expected = hex_to_bytes("64fe202dc9bb9d43dfff7a0a982b2ce3ff2f20293cc34775698432eaf16d4f42");
        assert_eq!(hmac, expected);
    }
}
