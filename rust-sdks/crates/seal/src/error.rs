#[derive(Debug, thiserror::Error)]
pub enum SealError {
    #[error("invalid client options: {0}")]
    InvalidClientOptions(String),
    #[error("invalid threshold: {0}")]
    InvalidThreshold(String),
    #[error("invalid ciphertext: {0}")]
    InvalidCiphertext(String),
    #[error("decryption error: {0}")]
    Decryption(String),
    #[error("key server error: {0}")]
    KeyServer(String),
    #[error("expired session key")]
    ExpiredSessionKey,
    #[error("invalid package: {0}")]
    InvalidPackage(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
