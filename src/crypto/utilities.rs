pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn from_hex(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).expect("Invalid hex string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let data = b"zana quantum-ai";
        let hex_str = to_hex(data);
        let bytes = from_hex(&hex_str);

        assert_eq!(bytes, data, "Hex conversion failed");
    }
}
