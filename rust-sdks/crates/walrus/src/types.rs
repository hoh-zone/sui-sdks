use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalrusPackageConfig {
    pub system_object_id: String,
    pub staking_pool_id: String,
}

#[derive(Debug, Clone)]
pub struct WalrusClientConfig {
    pub network: Option<String>,
    pub package_config: Option<WalrusPackageConfig>,
    pub storage_node_timeout_ms: Option<u64>,
    pub upload_relay: Option<UploadRelayConfig>,
}

#[derive(Debug, Clone)]
pub struct WalrusOptions {
    pub network: Option<String>,
    pub package_config: Option<WalrusPackageConfig>,
    pub storage_node_timeout_ms: Option<u64>,
    pub upload_relay: Option<UploadRelayConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRelayConfig {
    pub host: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfirmation {
    pub serialized_message: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEvent {
    pub event_seq: String,
    pub tx_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletableCounts {
    pub count_deletable_total: u64,
    pub count_deletable_certified: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BlobStatus {
    #[serde(rename = "nonexistent")]
    Nonexistent,
    #[serde(rename = "invalid")]
    Invalid { event: StatusEvent },
    #[serde(rename = "permanent")]
    Permanent {
        deletable_counts: DeletableCounts,
        end_epoch: u64,
        is_certified: bool,
        status_event: StatusEvent,
        initial_certified_epoch: Option<u64>,
    },
    #[serde(rename = "deletable")]
    Deletable {
        deletable_counts: DeletableCounts,
        initial_certified_epoch: Option<u64>,
    },
}

#[derive(Debug, Clone)]
pub struct ReadBlobOptions {
    pub blob_id: String,
}

#[derive(Debug, Clone)]
pub struct WriteBlobToUploadRelayOptions {
    pub blob_id: String,
    pub nonce: Vec<u8>,
    pub tx_digest: String,
    pub blob: Vec<u8>,
    pub blob_object_id: String,
    pub deletable: bool,
    pub requires_tip: bool,
    pub encoding_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessageCertificate {
    pub signers: Vec<u16>,
    pub serialized_message: Vec<u8>,
    pub signature: Vec<u8>,
}
