#[derive(Debug, thiserror::Error)]
pub enum StorageNodeError {
    #[error("request aborted by user")]
    UserAbort,
    #[error("connection timeout")]
    ConnectionTimeout,
    #[error("api error {status}: {message}")]
    ApiError { status: u16, message: String },
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
