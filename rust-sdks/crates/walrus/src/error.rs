#[derive(Debug, thiserror::Error)]
pub enum WalrusClientError {
    #[error("unsupported network: {0}")]
    UnsupportedNetwork(String),
    #[error("storage node error: {0}")]
    StorageNode(String),
    #[error("upload relay error: {0}")]
    UploadRelay(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
