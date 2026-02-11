use crate::bcs::EncryptedObject;
use crate::dem::{aes_gcm_decrypt, hmac_ctr_decrypt};
use crate::error::SealError;

pub fn decrypt(encrypted: &EncryptedObject, key: &[u8], check_le_encoding: bool) -> Result<Vec<u8>, SealError> {
    let _ = check_le_encoding;
    match encrypted.dem_type {
        0 => aes_gcm_decrypt(key, &encrypted.nonce, &encrypted.ciphertext, &encrypted.aad),
        1 => Ok(hmac_ctr_decrypt(key, &encrypted.nonce, &encrypted.ciphertext)),
        other => Err(SealError::InvalidCiphertext(format!("unsupported dem type {}", other))),
    }
}
