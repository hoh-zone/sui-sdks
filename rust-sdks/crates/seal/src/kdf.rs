use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy)]
pub enum KeyPurpose {
    Dem,
    EncryptedRandomness,
}

pub fn derive_key(
    purpose: KeyPurpose,
    base_key: &[u8],
    encrypted_shares: &[Vec<u8>],
    threshold: u8,
    services: &[String],
) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(base_key);
    hasher.update([purpose as u8]);
    hasher.update([threshold]);
    for s in services {
        hasher.update(s.as_bytes());
    }
    for share in encrypted_shares {
        hasher.update(share);
    }
    hasher.finalize().to_vec()
}
