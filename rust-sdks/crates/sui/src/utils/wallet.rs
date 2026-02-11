pub fn derive_wallet_address(keypair_bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(keypair_bytes);
    format!("0x{}", hex::encode(&digest[..32]))
}

pub fn is_valid_wallet_address(addr: &str) -> bool {
    super::address::validate_sui_address(addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_wallet_address() {
        let addr = derive_wallet_address(b"test_keypair_bytes");
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 66);
    }

    #[test]
    fn test_is_valid_wallet_address() {
        let addr = format!("0x{}", "0".repeat(64));
        assert!(is_valid_wallet_address(&addr));
        assert!(!is_valid_wallet_address(""));
    }
}
