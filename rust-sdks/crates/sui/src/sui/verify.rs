use fastcrypto::ed25519::{Ed25519PublicKey, Ed25519Signature};
use fastcrypto::secp256k1::{Secp256k1PublicKey, Secp256k1Signature};
use fastcrypto::secp256r1::{Secp256r1PublicKey, Secp256r1Signature};
use fastcrypto::traits::{ToFromBytes, VerifyingKey};

use crate::crypto::{self, SignatureScheme};

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("unsupported signature scheme")]
    UnsupportedScheme,
    #[error("invalid public key bytes")]
    InvalidPublicKey,
    #[error("invalid signature bytes")]
    InvalidSignature,
}

pub fn verify_signature(
    scheme: SignatureScheme,
    public_key_bytes: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, VerifyError> {
    match scheme {
        SignatureScheme::Ed25519 => {
            let pk = Ed25519PublicKey::from_bytes(public_key_bytes).map_err(|_| VerifyError::InvalidPublicKey)?;
            let sig = Ed25519Signature::from_bytes(signature).map_err(|_| VerifyError::InvalidSignature)?;
            Ok(pk.verify(message, &sig).is_ok())
        }
        SignatureScheme::Secp256k1 => {
            let pk = Secp256k1PublicKey::from_bytes(public_key_bytes).map_err(|_| VerifyError::InvalidPublicKey)?;
            let sig = Secp256k1Signature::from_bytes(signature).map_err(|_| VerifyError::InvalidSignature)?;
            Ok(pk.verify(message, &sig).is_ok())
        }
        SignatureScheme::Secp256r1 => {
            let pk = Secp256r1PublicKey::from_bytes(public_key_bytes).map_err(|_| VerifyError::InvalidPublicKey)?;
            let sig = Secp256r1Signature::from_bytes(signature).map_err(|_| VerifyError::InvalidSignature)?;
            Ok(pk.verify(message, &sig).is_ok())
        }
    }
}

pub fn verify_personal_message(
    scheme: SignatureScheme,
    public_key_bytes: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, VerifyError> {
    let msg = crypto::message_with_intent([3, 0, 0], message);
    verify_signature(scheme, public_key_bytes, &msg, signature)
}

pub fn verify_transaction(
    scheme: SignatureScheme,
    public_key_bytes: &[u8],
    tx_bytes: &[u8],
    signature: &[u8],
) -> Result<bool, VerifyError> {
    let msg = crypto::message_with_intent([0, 0, 0], tx_bytes);
    verify_signature(scheme, public_key_bytes, &msg, signature)
}
