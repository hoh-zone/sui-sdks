use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn generate_secret_key() -> Vec<u8> {
    let mut sk = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut sk);
    sk
}

pub fn to_public_key(secret_key: &[u8]) -> Vec<u8> {
    Sha256::digest(secret_key).to_vec()
}

pub fn to_verification_key(secret_key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    hasher.update(b"verify");
    hasher.finalize().to_vec()
}

pub fn elgamal_decrypt(ciphertext: &[u8], secret_key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    let key = hasher.finalize();
    ciphertext
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}
