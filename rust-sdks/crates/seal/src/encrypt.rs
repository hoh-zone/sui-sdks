use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::bcs::EncryptedObject;
use crate::dem::{aes_gcm_encrypt, hmac_ctr_encrypt};
use crate::error::SealError;
use crate::types::EncryptOptions;
use crate::utils::{create_full_id, MAX_U8};

#[derive(Debug, Clone, Copy)]
pub enum KemType {
    BonehFranklinBls12381DemCca = 0,
}

#[derive(Debug, Clone, Copy)]
pub enum DemType {
    AesGcm256 = 0,
    Hmac256Ctr = 1,
}

pub fn encrypt(
    options: EncryptOptions,
    weighted_services: Vec<String>,
) -> Result<(Vec<u8>, Vec<u8>), SealError> {
    let threshold = options.threshold;
    if threshold == 0 || threshold >= MAX_U8 {
        return Err(SealError::InvalidThreshold(format!(
            "invalid threshold {}",
            threshold
        )));
    }
    if weighted_services.len() < threshold as usize {
        return Err(SealError::InvalidThreshold(format!(
            "threshold {} exceeds services {}",
            threshold,
            weighted_services.len()
        )));
    }

    let mut base_key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut base_key);

    let full_id = create_full_id(&options.package_id, &options.id);
    let mut kdf = Sha256::new();
    kdf.update(&base_key);
    kdf.update(full_id.as_bytes());
    let dem_key = kdf.finalize().to_vec();

    let mut nonce = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    let aad = options.aad.clone().unwrap_or_default();
    let dem_type = options.dem_type.unwrap_or(DemType::AesGcm256);
    let ciphertext = match dem_type {
        DemType::AesGcm256 => aes_gcm_encrypt(&dem_key[..32], &nonce, &options.data, &aad)?,
        DemType::Hmac256Ctr => hmac_ctr_encrypt(&dem_key[..32], &nonce, &options.data),
    };

    let services = weighted_services
        .iter()
        .enumerate()
        .map(|(i, s)| (s.clone(), i as u8 + 1))
        .collect::<Vec<_>>();

    let encrypted = EncryptedObject {
        version: 0,
        package_id: options.package_id,
        id: options.id,
        services,
        threshold,
        dem_type: dem_type as u8,
        nonce,
        ciphertext,
        aad,
    };

    Ok((encrypted.to_bytes()?, dem_key[..32].to_vec()))
}
