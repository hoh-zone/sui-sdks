use crate::crypto;

pub const PASSKEY_SCHEME_FLAG: u8 = 0x06;
pub const PASSKEY_PUBLIC_KEY_SIZE: usize = 33;
pub const PASSKEY_UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = 65;

pub const SECP256R1_SPKI_HEADER: [u8; 26] = [
    0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08,
    0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00,
];

#[derive(Debug, thiserror::Error)]
pub enum PasskeyError {
    #[error("invalid passkey public key length: expected 33, got {0}")]
    InvalidPublicKeyLength(usize),
    #[error("invalid DER length")]
    InvalidDerLength,
    #[error("invalid SPKI header")]
    InvalidSpkiHeader,
    #[error("invalid uncompressed key marker")]
    InvalidPointMarker,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PasskeyPublicKey {
    bytes: Vec<u8>,
}

impl PasskeyPublicKey {
    pub fn new(bytes: &[u8]) -> Result<Self, PasskeyError> {
        if bytes.len() != PASSKEY_PUBLIC_KEY_SIZE {
            return Err(PasskeyError::InvalidPublicKeyLength(bytes.len()));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn to_raw_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_sui_address(&self) -> String {
        crypto::to_sui_address(PASSKEY_SCHEME_FLAG, &self.bytes)
    }
}

pub fn parse_der_spki(der_bytes: &[u8]) -> Result<Vec<u8>, PasskeyError> {
    if der_bytes.len() != SECP256R1_SPKI_HEADER.len() + PASSKEY_UNCOMPRESSED_PUBLIC_KEY_SIZE {
        return Err(PasskeyError::InvalidDerLength);
    }
    if &der_bytes[..SECP256R1_SPKI_HEADER.len()] != SECP256R1_SPKI_HEADER.as_ref() {
        return Err(PasskeyError::InvalidSpkiHeader);
    }
    if der_bytes[SECP256R1_SPKI_HEADER.len()] != 0x04 {
        return Err(PasskeyError::InvalidPointMarker);
    }
    Ok(der_bytes[SECP256R1_SPKI_HEADER.len()..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_public_key() {
        let pk = PasskeyPublicKey::new(&[1u8; PASSKEY_PUBLIC_KEY_SIZE]).unwrap();
        assert_eq!(pk.to_raw_bytes().len(), PASSKEY_PUBLIC_KEY_SIZE);
        assert!(pk.to_sui_address().starts_with("0x"));
    }
}
