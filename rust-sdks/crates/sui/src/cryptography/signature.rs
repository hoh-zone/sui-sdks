use crate::crypto::SignatureScheme;

#[derive(Debug, Clone)]
pub enum Signature {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
    Secp256r1(Vec<u8>),
}

impl Signature {
    pub fn scheme(&self) -> SignatureScheme {
        match self {
            Signature::Ed25519(_) => SignatureScheme::Ed25519,
            Signature::Secp256k1(_) => SignatureScheme::Secp256k1,
            Signature::Secp256r1(_) => SignatureScheme::Secp256r1,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Signature::Ed25519(bytes) => bytes,
            Signature::Secp256k1(bytes) => bytes,
            Signature::Secp256r1(bytes) => bytes,
        }
    }

    pub fn from_bytes(scheme: SignatureScheme, bytes: Vec<u8>) -> Self {
        match scheme {
            SignatureScheme::Ed25519 => Signature::Ed25519(bytes),
            SignatureScheme::Secp256k1 => Signature::Secp256k1(bytes),
            SignatureScheme::Secp256r1 => Signature::Secp256r1(bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_scheme() {
        let sig = Signature::Ed25519(vec![1, 2, 3]);
        assert_eq!(sig.scheme(), SignatureScheme::Ed25519);
    }

    #[test]
    fn test_signature_bytes() {
        let sig = Signature::Ed25519(vec![1, 2, 3]);
        assert_eq!(sig.bytes(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_signature_from_bytes() {
        let sig = Signature::from_bytes(SignatureScheme::Ed25519, vec![1, 2, 3]);
        matches!(sig, Signature::Ed25519(_));
    }
}
