use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::crypto::SignatureScheme;

use crate::verify;

#[derive(Debug, thiserror::Error)]
pub enum MultiSigError {
    #[error("base64 decode failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid multisig: {0}")]
    Invalid(String),
    #[error("verify error: {0}")]
    Verify(#[from] verify::VerifyError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedPublicKey {
    pub scheme: SignatureScheme,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub weight: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigPublicKey {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<WeightedPublicKey>,
    pub threshold: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigEntry {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigSerialized {
    pub signatures: Vec<MultiSigEntry>,
    pub bitmap: Vec<usize>,
    pub threshold: u16,
}

impl MultiSigPublicKey {
    pub fn verify(&self, message: &[u8], serialized: &str) -> Result<bool, MultiSigError> {
        let payload: MultiSigSerialized = serde_json::from_str(serialized)?;
        if payload.threshold != self.threshold {
            return Err(MultiSigError::Invalid("threshold mismatch".to_string()));
        }

        let mut valid_weight: u32 = 0;
        let mut used = HashSet::new();
        for entry in &payload.signatures {
            if !used.insert(entry.public_key.clone()) {
                continue;
            }
            if let Some(weighted) = self.public_keys.iter().find(|pk| pk.public_key == entry.public_key) {
                use base64::Engine as _;
                let pk_bytes = base64::engine::general_purpose::STANDARD.decode(&weighted.public_key)?;
                let sig_bytes = base64::engine::general_purpose::STANDARD.decode(&entry.signature)?;
                let ok = verify::verify_signature(weighted.scheme, &pk_bytes, message, &sig_bytes)?;
                if ok {
                    valid_weight += weighted.weight as u32;
                }
            }
        }

        Ok(valid_weight >= self.threshold as u32)
    }

    pub fn to_raw_bytes(&self) -> Result<Vec<u8>, MultiSigError> {
        Ok(serde_json::to_vec(self)?)
    }
}

pub fn serialize_multisig(multisig: &MultiSigSerialized) -> Result<String, MultiSigError> {
    Ok(serde_json::to_string(multisig)?)
}

pub fn parse_multisig(data: &str) -> Result<MultiSigSerialized, MultiSigError> {
    Ok(serde_json::from_str(data)?)
}
