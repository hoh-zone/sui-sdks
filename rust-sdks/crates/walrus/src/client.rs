use crate::constants::{MAINNET_WALRUS_PACKAGE_CONFIG, TESTNET_WALRUS_PACKAGE_CONFIG};
use crate::error::WalrusClientError;
use crate::storage_node::client::{StorageNodeClient, StorageNodeClientOptions};
use crate::storage_node::error::StorageNodeError;
use crate::storage_node::types::{ConfirmationResponse, GetBlobMetadataResponse, StoreBlobMetadataResponse, StoreSliverResponse};
use crate::types::{BlobStatus, WalrusClientConfig, WalrusOptions, WriteBlobToUploadRelayOptions};
use crate::upload_relay::client::UploadRelayClient;

#[derive(Debug, Clone)]
pub struct WalrusClient {
    storage_node_client: StorageNodeClient,
    upload_relay_client: Option<UploadRelayClient>,
    pub package_config: crate::types::WalrusPackageConfig,
}

pub fn walrus(options: WalrusOptions) -> Result<WalrusClient, WalrusClientError> {
    WalrusClient::new(WalrusClientConfig {
        network: options.network,
        package_config: options.package_config,
        storage_node_timeout_ms: options.storage_node_timeout_ms,
        upload_relay: options.upload_relay,
    })
}

impl WalrusClient {
    pub fn new(config: WalrusClientConfig) -> Result<Self, WalrusClientError> {
        let package_config = if let Some(cfg) = config.package_config {
            cfg
        } else {
            match config.network.as_deref() {
                Some("mainnet") => MAINNET_WALRUS_PACKAGE_CONFIG.clone(),
                Some("testnet") | None => TESTNET_WALRUS_PACKAGE_CONFIG.clone(),
                Some(other) => return Err(WalrusClientError::UnsupportedNetwork(other.to_string())),
            }
        };

        let storage_node_client = StorageNodeClient::new(StorageNodeClientOptions {
            timeout_ms: config.storage_node_timeout_ms.unwrap_or(30_000),
        });

        let upload_relay_client = config
            .upload_relay
            .map(|cfg| UploadRelayClient::new(cfg.host, cfg.timeout_ms));

        Ok(Self {
            storage_node_client,
            upload_relay_client,
            package_config,
        })
    }

    pub fn storage_node(&self) -> &StorageNodeClient {
        &self.storage_node_client
    }

    pub fn upload_relay(&self) -> Option<&UploadRelayClient> {
        self.upload_relay_client.as_ref()
    }

    pub async fn get_blob_metadata(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<GetBlobMetadataResponse, StorageNodeError> {
        self.storage_node_client.get_blob_metadata(node_url, blob_id).await
    }

    pub async fn get_blob_status(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<BlobStatus, StorageNodeError> {
        self.storage_node_client.get_blob_status(node_url, blob_id).await
    }

    pub async fn store_blob_metadata(
        &self,
        node_url: &str,
        blob_id: &str,
        metadata: &[u8],
    ) -> Result<StoreBlobMetadataResponse, StorageNodeError> {
        self.storage_node_client
            .store_blob_metadata(node_url, blob_id, metadata, None)
            .await
    }

    pub async fn get_sliver(
        &self,
        node_url: &str,
        blob_id: &str,
        sliver_pair_index: u64,
        sliver_type: &str,
    ) -> Result<Vec<u8>, StorageNodeError> {
        self.storage_node_client
            .get_sliver(node_url, blob_id, sliver_pair_index, sliver_type)
            .await
    }

    pub async fn store_sliver(
        &self,
        node_url: &str,
        blob_id: &str,
        sliver_pair_index: u64,
        sliver_type: &str,
        sliver: &[u8],
    ) -> Result<StoreSliverResponse, StorageNodeError> {
        self.storage_node_client
            .store_sliver(node_url, blob_id, sliver_pair_index, sliver_type, sliver)
            .await
    }

    pub async fn get_deletable_blob_confirmation(
        &self,
        node_url: &str,
        blob_id: &str,
        object_id: &str,
    ) -> Result<ConfirmationResponse, StorageNodeError> {
        self.storage_node_client
            .get_deletable_blob_confirmation(node_url, blob_id, object_id)
            .await
    }

    pub async fn get_permanent_blob_confirmation(
        &self,
        node_url: &str,
        blob_id: &str,
    ) -> Result<ConfirmationResponse, StorageNodeError> {
        self.storage_node_client
            .get_permanent_blob_confirmation(node_url, blob_id)
            .await
    }

    pub async fn write_blob_to_upload_relay(
        &self,
        options: WriteBlobToUploadRelayOptions,
    ) -> Result<(String, crate::types::ProtocolMessageCertificate), WalrusClientError> {
        let relay = self
            .upload_relay_client
            .as_ref()
            .ok_or_else(|| WalrusClientError::UploadRelay("upload relay not configured".to_string()))?;
        relay
            .write_blob(options)
            .await
            .map_err(|e| WalrusClientError::UploadRelay(e.to_string()))
    }
}
