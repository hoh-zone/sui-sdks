pub fn validate_transaction_digest(digest: &[u8]) -> bool {
    digest.len() == 32
}

pub fn validate_signature(signature: &[u8]) -> bool {
    !signature.is_empty()
}

pub fn validate_public_key(pubkey: &[u8]) -> bool {
    !pubkey.is_empty() && (pubkey.len() == 32 || pubkey.len() == 33 || pubkey.len() == 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_transaction_digest() {
        assert!(validate_transaction_digest(&[0u8; 32]));
        assert!(!validate_transaction_digest(&[0u8; 31]));
        assert!(!validate_transaction_digest(&[0u8; 33]));
    }

    #[test]
    fn test_validate_signature() {
        assert!(validate_signature(&[1u8; 64]));
        assert!(!validate_signature(&[]));
    }

    #[test]
    fn test_validate_public_key() {
        assert!(validate_public_key(&[1u8; 32]));
        assert!(validate_public_key(&[1u8; 33]));
        assert!(validate_public_key(&[1u8; 64]));
        assert!(!validate_public_key(&[]));
        assert!(!validate_public_key(&[1u8; 31]));
    }
}
