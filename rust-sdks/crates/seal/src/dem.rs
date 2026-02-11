use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use sha2::{Digest, Sha256};

use crate::error::SealError;

pub fn aes_gcm_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, SealError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| SealError::Decryption(e.to_string()))?;
    cipher
        .encrypt(
            Nonce::from_slice(nonce),
            aes_gcm::aead::Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|e| SealError::Decryption(e.to_string()))
}

pub fn aes_gcm_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>, SealError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| SealError::Decryption(e.to_string()))?;
    cipher
        .decrypt(
            Nonce::from_slice(nonce),
            aes_gcm::aead::Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|e| SealError::Decryption(e.to_string()))
}

pub fn hmac_ctr_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Vec<u8> {
    xor_keystream(key, nonce, plaintext)
}

pub fn hmac_ctr_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    xor_keystream(key, nonce, ciphertext)
}

fn xor_keystream(key: &[u8], nonce: &[u8], input: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; input.len()];
    let mut offset = 0usize;
    let mut counter: u64 = 0;
    while offset < input.len() {
        let mut hasher = Sha256::new();
        hasher.update(key);
        hasher.update(nonce);
        hasher.update(counter.to_le_bytes());
        let block = hasher.finalize();
        let take = usize::min(32, input.len() - offset);
        for i in 0..take {
            out[offset + i] = input[offset + i] ^ block[i];
        }
        offset += take;
        counter += 1;
    }
    out
}
