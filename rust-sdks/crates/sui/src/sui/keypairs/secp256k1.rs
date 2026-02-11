use fastcrypto::secp256k1::{
    Secp256k1KeyPair, Secp256k1PrivateKey, Secp256k1Signature, SECP256K1_PRIVATE_KEY_LENGTH,
};
use fastcrypto::traits::{KeyPair, Signer, ToFromBytes, VerifyingKey};

use crate::crypto;

const SECP256K1_SCHEME_FLAG: u8 = 0x01;

#[derive(Debug, thiserror::Error)]
pub enum KeypairError {
    #[error("invalid private key length: expected {expected}, got {actual}")]
    InvalidPrivateKeyLength { expected: usize, actual: usize },
    #[error("base64 decode failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid signature bytes")]
    InvalidSignature,
    #[error("invalid private key bytes")]
    InvalidPrivateKey,
    #[error("invalid Sui private key scheme flag: expected 0x01, got 0x{0:02x}")]
    InvalidScheme(u8),
}

pub struct Keypair {
    inner: Secp256k1KeyPair,
}

impl Keypair {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let inner = Secp256k1KeyPair::generate(&mut rng);
        Self { inner }
    }

    pub fn from_secret_key(secret_key: &[u8]) -> Result<Self, KeypairError> {
        if secret_key.len() != SECP256K1_PRIVATE_KEY_LENGTH {
            return Err(KeypairError::InvalidPrivateKeyLength {
                expected: SECP256K1_PRIVATE_KEY_LENGTH,
                actual: secret_key.len(),
            });
        }
        let private = Secp256k1PrivateKey::from_bytes(secret_key).map_err(|_| KeypairError::InvalidPrivateKey)?;
        Ok(Self {
            inner: private.into(),
        })
    }

    pub fn from_sui_private_key(encoded: &str) -> Result<Self, KeypairError> {
        use base64::Engine as _;
        let bytes = base64::engine::general_purpose::STANDARD.decode(encoded)?;
        if bytes.is_empty() {
            return Err(KeypairError::InvalidPrivateKey);
        }
        if bytes[0] != SECP256K1_SCHEME_FLAG {
            return Err(KeypairError::InvalidScheme(bytes[0]));
        }
        Self::from_secret_key(&bytes[1..])
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.inner.sign(message).as_ref().to_vec()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, KeypairError> {
        let sig = Secp256k1Signature::from_bytes(signature).map_err(|_| KeypairError::InvalidSignature)?;
        Ok(self.inner.public().verify(message, &sig).is_ok())
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.inner.public().as_ref().to_vec()
    }

    pub fn to_sui_address(&self) -> String {
        crypto::to_sui_address(SECP256K1_SCHEME_FLAG, self.inner.public().as_ref())
    }

    pub fn to_sui_private_key(&self) -> String {
        let mut bytes = Vec::with_capacity(1 + SECP256K1_PRIVATE_KEY_LENGTH);
        bytes.push(SECP256K1_SCHEME_FLAG);
        bytes.extend_from_slice(self.inner.as_ref());
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }
}
