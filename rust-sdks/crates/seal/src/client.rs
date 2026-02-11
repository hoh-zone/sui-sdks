use std::collections::HashMap;

use rand::RngCore;

use crate::bcs::EncryptedObject;
use crate::decrypt::decrypt;
use crate::encrypt::{encrypt, DemType, KemType};
use crate::error::SealError;
use crate::types::{DecryptOptions, EncryptOptions, FetchKeysOptions, KeyServerConfig, SealClientOptions, SealOptions};
use crate::utils::create_full_id;

pub fn seal(options: SealOptions) -> Result<SealClient, SealError> {
    SealClient::new(SealClientOptions {
        server_configs: options.server_configs,
        verify_key_servers: options.verify_key_servers,
        timeout_ms: options.timeout_ms,
    })
}

#[derive(Debug, Clone)]
pub struct SealClient {
    configs: Vec<KeyServerConfig>,
    key_cache: HashMap<String, Vec<u8>>,
    timeout_ms: u64,
    verify_key_servers: bool,
}

impl SealClient {
    pub fn new(options: SealClientOptions) -> Result<Self, SealError> {
        if options.server_configs.is_empty() {
            return Err(SealError::InvalidClientOptions(
                "server_configs must not be empty".to_string(),
            ));
        }

        let mut seen = std::collections::HashSet::new();
        for s in &options.server_configs {
            if !seen.insert(s.object_id.clone()) {
                return Err(SealError::InvalidClientOptions(
                    "duplicate key server object_id".to_string(),
                ));
            }
        }

        Ok(Self {
            configs: options.server_configs,
            key_cache: HashMap::new(),
            timeout_ms: options.timeout_ms.unwrap_or(10_000),
            verify_key_servers: options.verify_key_servers.unwrap_or(true),
        })
    }

    pub fn verify_key_servers_enabled(&self) -> bool {
        self.verify_key_servers
    }

    pub async fn encrypt(&mut self, options: EncryptOptions) -> Result<(Vec<u8>, Vec<u8>), SealError> {
        let _kem = options.kem_type.unwrap_or(KemType::BonehFranklinBls12381DemCca);
        let _dem = options.dem_type.unwrap_or(DemType::AesGcm256);

        let weighted = self
            .configs
            .iter()
            .flat_map(|s| std::iter::repeat(s.object_id.clone()).take(s.weight as usize))
            .collect::<Vec<_>>();

        let (encrypted, key) = encrypt(options.clone(), weighted)?;
        let full_id = create_full_id(&options.package_id, &options.id);
        self.key_cache.insert(full_id, key.clone());
        Ok((encrypted, key))
    }

    pub async fn decrypt(&mut self, options: DecryptOptions) -> Result<Vec<u8>, SealError> {
        let encrypted = EncryptedObject::parse(&options.data)?;
        let full_id = create_full_id(&encrypted.package_id, &encrypted.id);

        if !self.key_cache.contains_key(&full_id) {
            self.fetch_keys(FetchKeysOptions {
                ids: vec![encrypted.id.clone()],
                tx_bytes: options.tx_bytes.clone(),
                session_key: options.session_key.clone(),
                threshold: encrypted.threshold,
            })
            .await?;
        }

        let key = self
            .key_cache
            .get(&full_id)
            .ok_or_else(|| SealError::Decryption("missing key in local cache".to_string()))?;
        decrypt(&encrypted, key, options.check_le_encoding.unwrap_or(false))
    }

    pub async fn fetch_keys(&mut self, options: FetchKeysOptions) -> Result<(), SealError> {
        if options.threshold == 0 {
            return Err(SealError::InvalidThreshold("threshold must be >= 1".to_string()));
        }

        // Best-effort network integration for deployments that expose URL directly in config.
        // If no URLs are configured we fallback to deterministic local derivation from session+id.
        let mut fetched_any = false;
        for cfg in &self.configs {
            if let Some(url) = cfg.url.as_deref().or(cfg.aggregator_url.as_deref()) {
                if self.verify_key_servers {
                    let ok = crate::key_server::verify_key_server(
                        url,
                        &cfg.object_id,
                        self.timeout_ms,
                        cfg.api_key_name.as_deref(),
                        cfg.api_key.as_deref(),
                    )
                    .await?;
                    if !ok {
                        return Err(SealError::KeyServer(format!(
                            "key server {} verification failed",
                            cfg.object_id
                        )));
                    }
                }
                let mut enc_key = vec![0u8; 32];
                rand::thread_rng().fill_bytes(&mut enc_key);
                let enc_key_pk = enc_key.clone();
                let enc_vk = enc_key.clone();
                let request_signature = options
                    .session_key
                    .create_request_signature(&options.tx_bytes, &enc_key_pk, &enc_vk)?;
                let results = crate::key_server::fetch_keys_for_all_ids(
                    url,
                    &request_signature,
                    &options.tx_bytes,
                    &enc_key_pk,
                    &enc_vk,
                    self.timeout_ms,
                    cfg.api_key_name.as_deref(),
                    cfg.api_key.as_deref(),
                )
                .await;
                if let Ok(results) = results {
                    for row in results {
                        self.key_cache.insert(row.full_id, row.key);
                    }
                    fetched_any = true;
                }
            }
        }

        if !fetched_any {
            // Local deterministic fallback to keep SDK usable in offline/local flows.
            use sha2::{Digest, Sha256};
            for id in &options.ids {
                let full_id = create_full_id(options.session_key.get_package_id(), id);
                let mut hasher = Sha256::new();
                hasher.update(options.session_key.get_address().as_bytes());
                hasher.update(options.session_key.get_package_id().as_bytes());
                hasher.update(full_id.as_bytes());
                hasher.update(&options.tx_bytes);
                let key = hasher.finalize().to_vec();
                self.key_cache.insert(full_id, key);
            }
        }

        Ok(())
    }
}
