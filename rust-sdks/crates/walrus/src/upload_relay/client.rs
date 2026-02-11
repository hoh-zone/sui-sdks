use std::time::Duration;

use base64::Engine;

use crate::types::{ProtocolMessageCertificate, WriteBlobToUploadRelayOptions};

#[derive(Debug, thiserror::Error)]
pub enum UploadRelayError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct UploadRelayClient {
    host: String,
    timeout_ms: u64,
    http: reqwest::Client,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TipConfig {
    pub address: String,
    pub kind: serde_json::Value,
}

impl UploadRelayClient {
    pub fn new(host: String, timeout_ms: Option<u64>) -> Self {
        Self {
            host,
            timeout_ms: timeout_ms.unwrap_or(30_000),
            http: reqwest::Client::new(),
        }
    }

    pub async fn tip_config(&self) -> Result<Option<TipConfig>, UploadRelayError> {
        let url = format!("{}/v1/tip-config", self.host.trim_end_matches('/'));
        let value: serde_json::Value = self
            .http
            .get(url)
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        if value.as_str() == Some("no_tip") {
            return Ok(None);
        }
        let send_tip = value.get("send_tip").cloned().unwrap_or_default();
        let address = send_tip
            .get("address")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let kind = send_tip.get("kind").cloned().unwrap_or_default();
        Ok(Some(TipConfig { address, kind }))
    }

    pub async fn write_blob(
        &self,
        options: WriteBlobToUploadRelayOptions,
    ) -> Result<(String, ProtocolMessageCertificate), UploadRelayError> {
        let mut query = vec![("blob_id".to_string(), options.blob_id.clone())];

        if options.requires_tip {
            let nonce_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(options.nonce);
            query.push(("nonce".to_string(), nonce_b64));
            query.push(("tx_id".to_string(), options.tx_digest));
        }
        if options.deletable {
            query.push((
                "deletable_blob_object".to_string(),
                options.blob_object_id,
            ));
        }
        if let Some(encoding_type) = options.encoding_type {
            query.push(("encoding_type".to_string(), encoding_type));
        }

        let query = query
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/v1/blob-upload-relay?{}",
            self.host.trim_end_matches('/'),
            query
        );

        let data: serde_json::Value = self
            .http
            .post(url)
            .timeout(Duration::from_millis(self.timeout_ms))
            .body(options.blob)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let cert = data
            .get("confirmation_certificate")
            .cloned()
            .unwrap_or_default();
        let signers = cert
            .get("signers")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_u64().map(|i| i as u16))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let serialized_message = cert
            .get("serialized_message")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_u64().map(|i| i as u8))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let signature = cert
            .get("signature")
            .and_then(|v| v.as_str())
            .map(|s| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s).unwrap_or_default())
            .unwrap_or_default();

        Ok((
            options.blob_id,
            ProtocolMessageCertificate {
                signers,
                serialized_message,
                signature,
            },
        ))
    }
}
