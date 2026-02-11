use serde::{Deserialize, Serialize};

use crate::types::StorageConfirmation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlobMetadataResponse {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreBlobMetadataResponse {
    pub success: StoreBlobMetadataSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreBlobMetadataSuccess {
    pub code: i32,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSliverResponse {
    pub success: StoreBlobMetadataSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationResponse {
    pub success: ConfirmationSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationSuccess {
    pub code: i32,
    pub data: ConfirmationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationData {
    pub signed: StorageConfirmation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawGetBlobStatusResponse {
    pub code: i32,
    pub success: RawGetBlobStatusSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawGetBlobStatusSuccess {
    pub data: serde_json::Value,
}
