use std::time::Duration;

use reqwest::header::HeaderMap;

use crate::types::{BlobStatus, DeletableCounts, StatusEvent};

use super::error::StorageNodeError;
use super::types::{
    ConfirmationResponse, GetBlobMetadataResponse, RawGetBlobStatusResponse,
    StoreBlobMetadataResponse, StoreSliverResponse,
};
use super::utils::merge_headers;

#[derive(Debug, Clone)]
pub struct StorageNodeClientOptions {
    pub timeout_ms: u64,
}

impl Default for StorageNodeClientOptions {
    fn default() -> Self {
        Self { timeout_ms: 30_000 }
    }
}

#[derive(Debug, Clone)]
pub struct StorageNodeClient {
    http: reqwest::Client,
    timeout_ms: u64,
}

impl StorageNodeClient {
    pub fn new(options: StorageNodeClientOptions) -> Self {
        Self {
            http: reqwest::Client::new(),
            timeout_ms: options.timeout_ms,
        }
    }

    pub async fn get_blob_metadata(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<GetBlobMetadataResponse, StorageNodeError> {
        let url = format!("{}/v1/blobs/{}/metadata", node_url.trim_end_matches('/'), blob_id);
        let bytes = self
            .http
            .get(url)
            .header("Accept", "application/octet-stream")
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(GetBlobMetadataResponse { bytes: bytes.to_vec() })
    }

    pub async fn get_blob_status(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<BlobStatus, StorageNodeError> {
        let url = format!("{}/v1/blobs/{}/status", node_url.trim_end_matches('/'), blob_id);
        let value: RawGetBlobStatusResponse = self
            .http
            .get(url)
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        parse_blob_status(value.success.data)
    }

    pub async fn store_blob_metadata(
        &self,
        node_url: &str,
        blob_id: &str,
        metadata: &[u8],
        headers: Option<&HeaderMap>,
    ) -> Result<StoreBlobMetadataResponse, StorageNodeError> {
        let url = format!("{}/v1/blobs/{}/metadata", node_url.trim_end_matches('/'), blob_id);
        let response = self
            .http
            .put(url)
            .headers(merge_headers(&[("Content-Type", "application/octet-stream")], headers))
            .body(metadata.to_vec())
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_sliver(
        &self,
        node_url: &str,
        blob_id: &str,
        sliver_pair_index: u64,
        sliver_type: &str,
    ) -> Result<Vec<u8>, StorageNodeError> {
        let url = format!(
            "{}/v1/blobs/{}/slivers/{}/{}",
            node_url.trim_end_matches('/'),
            blob_id,
            sliver_pair_index,
            sliver_type
        );
        let bytes = self
            .http
            .get(url)
            .header("Accept", "application/octet-stream")
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(bytes.to_vec())
    }

    pub async fn store_sliver(
        &self,
        node_url: &str,
        blob_id: &str,
        sliver_pair_index: u64,
        sliver_type: &str,
        sliver: &[u8],
    ) -> Result<StoreSliverResponse, StorageNodeError> {
        let url = format!(
            "{}/v1/blobs/{}/slivers/{}/{}",
            node_url.trim_end_matches('/'),
            blob_id,
            sliver_pair_index,
            sliver_type
        );
        let response = self
            .http
            .put(url)
            .header("Content-Type", "application/octet-stream")
            .body(sliver.to_vec())
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_deletable_blob_confirmation(
        &self,
        node_url: &str,
        blob_id: &str,
        object_id: &str,
    ) -> Result<ConfirmationResponse, StorageNodeError> {
        let url = format!(
            "{}/v1/blobs/{}/confirmation/deletable/{}",
            node_url.trim_end_matches('/'),
            blob_id,
            object_id
        );
        let response = self
            .http
            .get(url)
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_permanent_blob_confirmation(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<ConfirmationResponse, StorageNodeError> {
        let url = format!(
            "{}/v1/blobs/{}/confirmation/permanent",
            node_url.trim_end_matches('/'),
            blob_id
        );
        let response = self
            .http
            .get(url)
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }
}

fn parse_blob_status(v: serde_json::Value) -> Result<BlobStatus, StorageNodeError> {
    if let Some(s) = v.as_str() {
        if s == "nonexistent" {
            return Ok(BlobStatus::Nonexistent);
        }
    }

    if let Some(invalid) = v.get("invalid") {
        let event = invalid
            .get("event")
            .cloned()
            .ok_or_else(|| StorageNodeError::ApiError {
                status: 500,
                message: "invalid status event missing".to_string(),
            })?;
        let event: StatusEvent = serde_json::from_value(event)?;
        return Ok(BlobStatus::Invalid { event });
    }

    if let Some(permanent) = v.get("permanent") {
        let deletable_counts: DeletableCounts = serde_json::from_value(
            permanent
                .get("deletableCounts")
                .cloned()
                .unwrap_or(serde_json::json!({"count_deletable_total":0,"count_deletable_certified":0})),
        )?;
        let status_event: StatusEvent = serde_json::from_value(
            permanent
                .get("statusEvent")
                .cloned()
                .unwrap_or(serde_json::json!({"event_seq":"0","tx_digest":""})),
        )?;
        return Ok(BlobStatus::Permanent {
            deletable_counts,
            end_epoch: permanent.get("endEpoch").and_then(|v| v.as_u64()).unwrap_or(0),
            is_certified: permanent
                .get("isCertified")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            status_event,
            initial_certified_epoch: permanent.get("initialCertifiedEpoch").and_then(|v| v.as_u64()),
        });
    }

    if let Some(deletable) = v.get("deletable") {
        let deletable_counts: DeletableCounts = serde_json::from_value(
            deletable
                .get("deletableCounts")
                .cloned()
                .unwrap_or(serde_json::json!({"count_deletable_total":0,"count_deletable_certified":0})),
        )?;
        return Ok(BlobStatus::Deletable {
            deletable_counts,
            initial_certified_epoch: deletable.get("initialCertifiedEpoch").and_then(|v| v.as_u64()),
        });
    }

    Err(StorageNodeError::ApiError {
        status: 500,
        message: "unknown blob status".to_string(),
    })
}
