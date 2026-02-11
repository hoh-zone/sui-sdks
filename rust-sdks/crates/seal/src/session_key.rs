use base64::Engine;

use crate::error::SealError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportedSessionKey {
    pub address: String,
    pub package_id: String,
    pub mvr_name: Option<String>,
    pub creation_time_ms: u64,
    pub ttl_min: u32,
    pub session_private_key_b64: String,
}

pub struct SessionKey {
    address: String,
    package_id: String,
    mvr_name: Option<String>,
    creation_time_ms: u64,
    ttl_min: u32,
    keypair: sui::keypairs::ed25519::Keypair,
}

impl core::fmt::Debug for SessionKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SessionKey")
            .field("address", &self.address)
            .field("package_id", &self.package_id)
            .field("mvr_name", &self.mvr_name)
            .field("creation_time_ms", &self.creation_time_ms)
            .field("ttl_min", &self.ttl_min)
            .finish()
    }
}

impl Clone for SessionKey {
    fn clone(&self) -> Self {
        let sui_sk = self.keypair.to_sui_private_key();
        let keypair = sui::keypairs::ed25519::Keypair::from_sui_private_key(&sui_sk)
            .expect("valid sui private key");
        Self {
            address: self.address.clone(),
            package_id: self.package_id.clone(),
            mvr_name: self.mvr_name.clone(),
            creation_time_ms: self.creation_time_ms,
            ttl_min: self.ttl_min,
            keypair,
        }
    }
}

impl SessionKey {
    pub fn create(address: String, package_id: String, mvr_name: Option<String>, ttl_min: u32) -> Result<Self, SealError> {
        if ttl_min == 0 || ttl_min > 30 {
            return Err(SealError::InvalidClientOptions("ttl_min must be in [1, 30]".to_string()));
        }
        Ok(Self {
            address,
            package_id,
            mvr_name,
            creation_time_ms: current_ms(),
            ttl_min,
            keypair: sui::keypairs::ed25519::Keypair::generate(),
        })
    }

    pub fn is_expired(&self) -> bool {
        self.creation_time_ms + self.ttl_min as u64 * 60_000 < current_ms()
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_package_id(&self) -> &str {
        &self.package_id
    }

    pub fn get_package_name(&self) -> String {
        self.mvr_name.clone().unwrap_or_else(|| self.package_id.clone())
    }

    pub fn get_personal_message(&self) -> Vec<u8> {
        let pk = base64::engine::general_purpose::STANDARD.encode(self.keypair.public_key_bytes());
        format!(
            "Accessing keys of package {} for {} mins, session key {}",
            self.get_package_name(),
            self.ttl_min,
            pk
        )
        .into_bytes()
    }

    pub fn create_request_signature(&self, tx_bytes: &[u8], enc_key_pk: &[u8], enc_vk: &[u8]) -> Result<String, SealError> {
        if self.is_expired() {
            return Err(SealError::ExpiredSessionKey);
        }
        let mut msg = Vec::new();
        msg.extend_from_slice(tx_bytes);
        msg.extend_from_slice(enc_key_pk);
        msg.extend_from_slice(enc_vk);
        let sig = self.keypair.sign(&msg);
        Ok(base64::engine::general_purpose::STANDARD.encode(sig))
    }

    pub fn export(&self) -> ExportedSessionKey {
        let mut sk = [0u8; 32];
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(self.keypair.to_sui_private_key())
            .unwrap_or_default();
        if bytes.len() >= 33 {
            sk.copy_from_slice(&bytes[1..33]);
        }
        ExportedSessionKey {
            address: self.address.clone(),
            package_id: self.package_id.clone(),
            mvr_name: self.mvr_name.clone(),
            creation_time_ms: self.creation_time_ms,
            ttl_min: self.ttl_min,
            session_private_key_b64: base64::engine::general_purpose::STANDARD.encode(sk),
        }
    }

    pub fn import(data: ExportedSessionKey) -> Result<Self, SealError> {
        let raw = base64::engine::general_purpose::STANDARD
            .decode(data.session_private_key_b64)
            .map_err(|e| SealError::InvalidClientOptions(e.to_string()))?;
        let keypair = sui::keypairs::ed25519::Keypair::from_secret_key(&raw)
            .map_err(|e| SealError::InvalidClientOptions(e.to_string()))?;
        Ok(Self {
            address: data.address,
            package_id: data.package_id,
            mvr_name: data.mvr_name,
            creation_time_ms: data.creation_time_ms,
            ttl_min: data.ttl_min,
            keypair,
        })
    }
}

fn current_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
