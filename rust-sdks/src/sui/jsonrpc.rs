use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use super::utils;

#[derive(Debug, Clone, Serialize)]
struct RpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<JsonRpcErrorObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcErrorObject {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("invalid header name: {0}")]
    HeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error("invalid header value: {0}")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("http status error: {status}")]
    HttpStatus { status: reqwest::StatusCode },
    #[error("json-rpc error {code}: {message}")]
    Rpc {
        code: i64,
        message: String,
        data: Option<Value>,
    },
    #[error("missing json-rpc result")]
    MissingResult,
    #[error("invalid Sui address")]
    InvalidAddress,
    #[error("unknown network: {0}")]
    UnknownNetwork(String),
}

#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    network: String,
    headers: HeaderMap,
    client: reqwest::Client,
    request_id: u64,
}

impl Client {
    pub fn new(url: impl Into<String>, network: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            network: network.into(),
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
            request_id: 1,
        }
    }

    pub fn from_network(network: &str) -> Result<Self, JsonRpcError> {
        Ok(Self::new(default_jsonrpc_fullnode_url(network)?, network))
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Result<Self, JsonRpcError> {
        let header_name = HeaderName::from_str(key)?;
        let header_value = HeaderValue::from_str(value)?;
        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    pub fn network(&self) -> &str {
        &self.network
    }

    pub async fn call(&self, method: &str, params: Vec<Value>) -> Result<Value, JsonRpcError> {
        let payload = RpcRequest {
            jsonrpc: "2.0",
            id: self.request_id,
            method,
            params,
        };

        let mut req = self.client.post(&self.url).json(&payload);
        if !self.headers.is_empty() {
            req = req.headers(self.headers.clone());
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            return Err(JsonRpcError::HttpStatus {
                status: response.status(),
            });
        }

        let rpc: RpcResponse = response.json().await?;
        if let Some(error) = rpc.error {
            return Err(JsonRpcError::Rpc {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }
        rpc.result.ok_or(JsonRpcError::MissingResult)
    }

    pub async fn get_rpc_api_version(&self) -> Result<String, JsonRpcError> {
        let result = self.call("rpc.discover", vec![]).await?;
        let version = result
            .get("info")
            .and_then(|v| v.get("version"))
            .and_then(Value::as_str)
            .ok_or(JsonRpcError::MissingResult)?;
        Ok(version.to_string())
    }

    pub async fn get_balance(&self, owner: &str, coin_type: Option<&str>) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }

        self.call(
            "suix_getBalance",
            vec![
                Value::String(owner.to_string()),
                coin_type
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_object(&self, object_id: &str, options: Option<Value>) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(object_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getObject",
            vec![
                Value::String(object_id.to_string()),
                options.unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn execute_transaction_block(
        &self,
        tx_bytes_base64: &str,
        signatures: Vec<String>,
        options: Option<Value>,
        request_type: Option<&str>,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_executeTransactionBlock",
            vec![
                Value::String(tx_bytes_base64.to_string()),
                Value::Array(signatures.into_iter().map(Value::String).collect()),
                options.unwrap_or(Value::Null),
                request_type
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_reference_gas_price(&self) -> Result<String, JsonRpcError> {
        let result = self.call("suix_getReferenceGasPrice", vec![]).await?;
        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or(JsonRpcError::MissingResult)
    }
}

pub fn default_jsonrpc_fullnode_url(network: &str) -> Result<String, JsonRpcError> {
    utils::jsonrpc_fullnode_url(network)
        .map(|s| s.to_string())
        .ok_or_else(|| JsonRpcError::UnknownNetwork(network.to_string()))
}
