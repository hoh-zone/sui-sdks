use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SignatureScheme {
    Ed25519,
    Secp256k1,
    Secp256r1,
}

impl SignatureScheme {
    pub fn flag(self) -> u8 {
        match self {
            SignatureScheme::Ed25519 => 0x00,
            SignatureScheme::Secp256k1 => 0x01,
            SignatureScheme::Secp256r1 => 0x02,
        }
    }

    pub fn from_flag(flag: u8) -> Option<Self> {
        match flag {
            0x00 => Some(SignatureScheme::Ed25519),
            0x01 => Some(SignatureScheme::Secp256k1),
            0x02 => Some(SignatureScheme::Secp256r1),
            _ => None,
        }
    }
}

pub fn to_sui_public_key(flag: u8, public_key_bytes: &[u8]) -> String {
    let mut sui_bytes = Vec::with_capacity(public_key_bytes.len() + 1);
    sui_bytes.push(flag);
    sui_bytes.extend_from_slice(public_key_bytes);
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(sui_bytes)
}

pub fn to_sui_address(flag: u8, public_key_bytes: &[u8]) -> String {
    let mut sui_bytes = Vec::with_capacity(public_key_bytes.len() + 1);
    sui_bytes.push(flag);
    sui_bytes.extend_from_slice(public_key_bytes);

    let digest = Sha256::digest(sui_bytes);
    format!("0x{}", hex::encode(&digest[..32]))
}

pub fn message_with_intent(intent: [u8; 3], message: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(3 + message.len());
    result.extend_from_slice(&intent);
    result.extend_from_slice(message);
    result
}

pub fn hash_with_intent(intent: [u8; 3], message: &[u8]) -> [u8; 32] {
    let payload = message_with_intent(intent, message);
    Sha256::digest(payload).into()
}

pub mod fastcrypto_support {
    pub use fastcrypto;

    pub fn crate_name() -> &'static str {
        "fastcrypto"
    }
}
