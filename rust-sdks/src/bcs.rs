use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum BcsError {
    #[error("serialize failed: {0}")]
    Serialize(#[from] bcs::Error),
    #[error("base64 decode failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("hex decode failed: {0}")]
    Hex(#[from] hex::FromHexError),
}

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, BcsError> {
    Ok(bcs::to_bytes(value)?)
}

pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BcsError> {
    Ok(bcs::from_bytes(bytes)?)
}

pub fn to_base64(bytes: &[u8]) -> String {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub fn from_base64(value: &str) -> Result<Vec<u8>, BcsError> {
    use base64::Engine as _;
    Ok(base64::engine::general_purpose::STANDARD.decode(value)?)
}

pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn from_hex(value: &str) -> Result<Vec<u8>, BcsError> {
    Ok(hex::decode(value)?)
}
