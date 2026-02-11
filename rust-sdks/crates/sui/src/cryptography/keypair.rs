use crate::crypto::SignatureScheme;

pub enum Keypair {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
    Secp256r1(Vec<u8>),
}

impl Keypair {
    pub fn scheme(&self) -> SignatureScheme {
        match self {
            Keypair::Ed25519(_) => SignatureScheme::Ed25519,
            Keypair::Secp256k1(_) => SignatureScheme::Secp256k1,
            Keypair::Secp256r1(_) => SignatureScheme::Secp256r1,
        }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        match self {
            Keypair::Ed25519(bytes) => bytes.clone(),
            Keypair::Secp256k1(bytes) => bytes.clone(),
            Keypair::Secp256r1(bytes) => bytes.clone(),
        }
    }

    pub fn sign(&self, _message: &[u8]) -> Vec<u8> {
        Vec::new()
    }
}
