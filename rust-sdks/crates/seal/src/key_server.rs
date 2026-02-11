use base64::Engine;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;

use crate::error::SealError;

pub const CLIENT_SDK_TYPE: &str = "rust";
pub const CLIENT_SDK_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchedKey {
    pub full_id: String,
    pub key: Vec<u8>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyServerServiceInfo {
    pub service_id: String,
    pub pop: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SealApiError {
    #[error("invalid PTB")]
    InvalidPtb,
    #[error("invalid package")]
    InvalidPackage,
    #[error("no access")]
    NoAccess,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid session signature")]
    InvalidSessionSignature,
    #[error("invalid certificate")]
    InvalidCertificate,
    #[error("invalid SDK version")]
    InvalidSdkVersion,
    #[error("invalid SDK type")]
    InvalidSdkType,
    #[error("deprecated SDK version")]
    DeprecatedSdkVersion,
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("invalid MVR name")]
    InvalidMvrName,
    #[error("invalid service id")]
    InvalidServiceId,
    #[error("unsupported package id")]
    UnsupportedPackageId,
    #[error("internal failure")]
    Failure,
    #[error("general error: {0}")]
    General(String),
}

pub async fn assert_response(response: reqwest::Response) -> Result<reqwest::Response, SealError> {
    if response.status().is_success() {
        return Ok(response);
    }

    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        let code = value
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("General");
        let msg = value
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        let mapped = match code {
            "InvalidPTB" => SealApiError::InvalidPtb,
            "InvalidPackage" => SealApiError::InvalidPackage,
            "NoAccess" => SealApiError::NoAccess,
            "InvalidSignature" => SealApiError::InvalidSignature,
            "InvalidSessionSignature" => SealApiError::InvalidSessionSignature,
            "InvalidCertificate" => SealApiError::InvalidCertificate,
            "InvalidSDKVersion" => SealApiError::InvalidSdkVersion,
            "InvalidSDKType" => SealApiError::InvalidSdkType,
            "DeprecatedSDKVersion" => SealApiError::DeprecatedSdkVersion,
            "InvalidParameter" => SealApiError::InvalidParameter,
            "InvalidMVRName" => SealApiError::InvalidMvrName,
            "InvalidServiceId" => SealApiError::InvalidServiceId,
            "UnsupportedPackageId" => SealApiError::UnsupportedPackageId,
            "Failure" => SealApiError::Failure,
            _ => SealApiError::General(msg.to_string()),
        };
        return Err(SealError::KeyServer(format!("{} (status {})", mapped, status)));
    }

    Err(SealError::KeyServer(format!(
        "http status {}: {}",
        status, text
    )))
}

pub async fn verify_key_server(
    url: &str,
    object_id: &str,
    timeout_ms: u64,
    api_key_name: Option<&str>,
    api_key: Option<&str>,
) -> Result<bool, SealError> {
    let mut headers = HeaderMap::new();
    headers.insert("Client-Sdk-Type", CLIENT_SDK_TYPE.parse().unwrap());
    headers.insert("Client-Sdk-Version", CLIENT_SDK_VERSION.parse().unwrap());
    headers.insert("Request-Id", uuid::Uuid::new_v4().to_string().parse().unwrap());

    if let (Some(name), Some(value)) = (api_key_name, api_key) {
        if let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(header_value) = value.parse() {
                headers.insert(header_name, header_value);
            }
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/v1/service?service_id={}",
            url.trim_end_matches('/'),
            object_id
        ))
        .headers(headers)
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .send()
        .await?;

    let response = assert_response(response).await?;
    let info: KeyServerServiceInfo = response.json().await?;
    Ok(info.service_id == object_id)
}

pub async fn fetch_keys_for_all_ids(
    url: &str,
    request_signature: &str,
    tx_bytes: &[u8],
    enc_key_pk: &[u8],
    enc_verification_key: &[u8],
    timeout_ms: u64,
    api_key_name: Option<&str>,
    api_key: Option<&str>,
) -> Result<Vec<FetchedKey>, SealError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let mut headers = HeaderMap::new();
    headers.insert("Request-Id", request_id.parse().unwrap());
    headers.insert("Client-Sdk-Type", CLIENT_SDK_TYPE.parse().unwrap());
    headers.insert("Client-Sdk-Version", CLIENT_SDK_VERSION.parse().unwrap());
    if let (Some(name), Some(value)) = (api_key_name, api_key) {
        if let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(header_value) = value.parse() {
                headers.insert(header_name, header_value);
            }
        }
    }

    let body = serde_json::json!({
        "ptb": base64::engine::general_purpose::STANDARD.encode(tx_bytes),
        "enc_key": base64::engine::general_purpose::STANDARD.encode(enc_key_pk),
        "enc_verification_key": base64::engine::general_purpose::STANDARD.encode(enc_verification_key),
        "request_signature": request_signature,
    });

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/v1/fetch_key", url.trim_end_matches('/')))
        .headers(headers)
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .json(&body)
        .send()
        .await?;

    let response = assert_response(response).await?;
    let value: serde_json::Value = response.json().await?;

    let items = value
        .get("keys")
        .or_else(|| value.get("success").and_then(|s| s.get("data")).and_then(|d| d.get("keys")))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let full_id = item
            .get("full_id")
            .or_else(|| item.get("fullId"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let key_b64 = item
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let key = base64::engine::general_purpose::STANDARD
            .decode(key_b64)
            .unwrap_or_default();
        out.push(FetchedKey { full_id, key });
    }

    Ok(out)
}
