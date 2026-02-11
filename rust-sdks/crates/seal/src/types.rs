#[derive(Debug, Clone)]
pub struct KeyServerConfig {
    pub object_id: String,
    pub weight: u16,
    pub api_key_name: Option<String>,
    pub api_key: Option<String>,
    pub aggregator_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SealClientOptions {
    pub server_configs: Vec<KeyServerConfig>,
    pub verify_key_servers: Option<bool>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SealOptions {
    pub server_configs: Vec<KeyServerConfig>,
    pub verify_key_servers: Option<bool>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct EncryptOptions {
    pub kem_type: Option<crate::encrypt::KemType>,
    pub dem_type: Option<crate::encrypt::DemType>,
    pub threshold: u8,
    pub package_id: String,
    pub id: String,
    pub data: Vec<u8>,
    pub aad: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct DecryptOptions {
    pub data: Vec<u8>,
    pub session_key: crate::session_key::SessionKey,
    pub tx_bytes: Vec<u8>,
    pub check_share_consistency: Option<bool>,
    pub check_le_encoding: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FetchKeysOptions {
    pub ids: Vec<String>,
    pub tx_bytes: Vec<u8>,
    pub session_key: crate::session_key::SessionKey,
    pub threshold: u8,
}

pub type KeyCacheKey = String;
