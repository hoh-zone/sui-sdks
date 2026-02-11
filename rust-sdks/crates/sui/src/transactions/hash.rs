use sha2::{Digest, Sha256};

pub fn transaction_digest(tx_data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(tx_data);
    let mut result = [0u8; 32];
    result.copy_from_slice(&digest);
    result
}

pub fn transaction_message(tx_data: &[u8]) -> Vec<u8> {
    tx_data.to_vec()
}

pub fn hash_with_intent(intent: [u8; 3], message: &[u8]) -> [u8; 32] {
    let mut result = Vec::with_capacity(3 + message.len());
    result.extend_from_slice(&intent);
    result.extend_from_slice(message);
    let digest = Sha256::digest(result);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&digest);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_digest() {
        let tx_data = b"test transaction data";
        let digest = transaction_digest(tx_data);
        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn test_transaction_message() {
        let tx_data = b"test transaction data";
        let message = transaction_message(tx_data);
        assert_eq!(message, tx_data.to_vec());
    }

    #[test]
    fn test_hash_with_intent() {
        let intent = [0, 0, 0];
        let message = b"test message";
        let hash = hash_with_intent(intent, message);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_transaction_digest_deterministic() {
        let tx_data = b"deterministic test data";
        let digest1 = transaction_digest(tx_data);
        let digest2 = transaction_digest(tx_data);
        assert_eq!(digest1, digest2);
    }
}
