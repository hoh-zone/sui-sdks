use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const NONCE_LENGTH: usize = 27;
pub const MAX_HEADER_LEN_B64: usize = 248;
pub const MAX_PADDED_UNSIGNED_JWT_LEN: usize = 64 * 25;

#[derive(Debug, thiserror::Error)]
pub enum ZkLoginError {
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid jwt")]
    InvalidJwt,
    #[error("jwt header too long")]
    HeaderTooLong,
    #[error("jwt too long")]
    JwtTooLong,
    #[error("base64 decode failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("json decode failed: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeZkLoginAddressOptions {
    pub iss: Option<String>,
    pub aud: Option<String>,
    #[serde(rename = "userSalt")]
    pub user_salt: String,
    pub jwt: Option<String>,
    #[serde(rename = "legacyAddress")]
    pub legacy_address: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkLoginSignatureExtended {
    pub inputs: Value,
    #[serde(rename = "maxEpoch")]
    pub max_epoch: u64,
    #[serde(rename = "userSignature")]
    pub user_signature: String,
}

pub fn generate_randomness() -> [u8; 16] {
    rand::random::<[u8; 16]>()
}

pub fn generate_nonce(
    public_key: &[u8],
    max_epoch: u64,
    randomness: &[u8],
) -> Result<String, ZkLoginError> {
    if public_key.is_empty() || randomness.is_empty() {
        return Err(ZkLoginError::InvalidInput);
    }

    let mut bytes = Vec::with_capacity(public_key.len() + 4 + randomness.len());
    bytes.extend_from_slice(public_key);
    bytes.extend_from_slice(&[
        (max_epoch & 0xff) as u8,
        ((max_epoch >> 8) & 0xff) as u8,
        ((max_epoch >> 16) & 0xff) as u8,
        ((max_epoch >> 24) & 0xff) as u8,
    ]);
    bytes.extend_from_slice(randomness);

    use base64::Engine as _;
    let mut nonce = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    if nonce.len() > NONCE_LENGTH {
        nonce.truncate(NONCE_LENGTH);
    }
    Ok(nonce)
}

pub fn length_checks(jwt: &str) -> Result<(), ZkLoginError> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() < 2 {
        return Err(ZkLoginError::InvalidJwt);
    }
    if parts[0].len() > MAX_HEADER_LEN_B64 {
        return Err(ZkLoginError::HeaderTooLong);
    }
    if parts[0].len() + parts[1].len() > MAX_PADDED_UNSIGNED_JWT_LEN {
        return Err(ZkLoginError::JwtTooLong);
    }
    Ok(())
}

pub fn jwt_decode(token: &str, header: bool) -> Result<Value, ZkLoginError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return Err(ZkLoginError::InvalidJwt);
    }

    let idx = if header { 0 } else { 1 };
    let payload = parts[idx];
    let padded = format!(
        "{}{}",
        payload,
        "=".repeat((4usize.wrapping_sub(payload.len() % 4)) % 4)
    );
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::URL_SAFE.decode(padded)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn decode_jwt(jwt: &str) -> Result<serde_json::Map<String, Value>, ZkLoginError> {
    let value = jwt_decode(jwt, false)?;
    match value {
        Value::Object(map) => Ok(map),
        _ => Err(ZkLoginError::InvalidJwt),
    }
}

pub fn compute_zklogin_address_from_seed(seed: &[u8], iss: &str, aud: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(iss.as_bytes());
    hasher.update(aud.as_bytes());
    format!("0x{}", hex::encode(hasher.finalize()))
}

pub fn jwt_to_address(jwt: &str, user_salt: &str, _legacy_address: bool) -> Result<String, ZkLoginError> {
    length_checks(jwt)?;
    let mut hasher = Sha256::new();
    hasher.update(jwt.as_bytes());
    hasher.update(b":");
    hasher.update(user_salt.as_bytes());
    Ok(format!("0x{}", hex::encode(hasher.finalize())))
}

pub fn compute_zklogin_address(opts: ComputeZkLoginAddressOptions) -> Result<String, ZkLoginError> {
    if let Some(jwt) = opts.jwt {
        jwt_to_address(&jwt, &opts.user_salt, opts.legacy_address.unwrap_or(false))
    } else {
        Ok(compute_zklogin_address_from_seed(
            opts.user_salt.as_bytes(),
            opts.iss.as_deref().unwrap_or_default(),
            opts.aud.as_deref().unwrap_or_default(),
        ))
    }
}

pub fn get_zklogin_signature(input: &ZkLoginSignatureExtended) -> Result<String, ZkLoginError> {
    let bytes = serde_json::to_vec(input)?;
    use base64::Engine as _;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

pub fn parse_zklogin_signature(signature: &str) -> Result<ZkLoginSignatureExtended, ZkLoginError> {
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::STANDARD.decode(signature)?;
    Ok(serde_json::from_slice(&bytes)?)
}
