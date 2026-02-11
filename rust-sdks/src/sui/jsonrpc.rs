use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

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
    id: Option<u64>,
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
    #[error("invalid json-rpc batch response")]
    InvalidBatchResponse,
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
    request_id: Arc<AtomicU64>,
}

impl Client {
    pub fn new(url: impl Into<String>, network: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            network: network.into(),
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
            request_id: Arc::new(AtomicU64::new(1)),
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
        let request_id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let payload = RpcRequest {
            jsonrpc: "2.0",
            id: request_id,
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

    pub async fn dry_run_transaction_block(
        &self,
        tx_bytes_base64: &str,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_dryRunTransactionBlock",
            vec![Value::String(tx_bytes_base64.to_string())],
        )
        .await
    }

    pub async fn batch_call(
        &self,
        calls: Vec<(&str, Vec<Value>)>,
    ) -> Result<Vec<Value>, JsonRpcError> {
        if calls.is_empty() {
            return Ok(Vec::new());
        }

        #[derive(Debug, Serialize)]
        struct BatchRpcRequest<'a> {
            jsonrpc: &'static str,
            id: u64,
            method: &'a str,
            params: Vec<Value>,
        }

        let mut id_to_pos = std::collections::HashMap::new();
        let mut payload = Vec::with_capacity(calls.len());

        for (idx, (method, params)) in calls.into_iter().enumerate() {
            let id = self.request_id.fetch_add(1, Ordering::SeqCst);
            id_to_pos.insert(id, idx);
            payload.push(BatchRpcRequest {
                jsonrpc: "2.0",
                id,
                method,
                params,
            });
        }

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

        let rpc: Vec<RpcResponse> = response.json().await?;
        let mut out = vec![None; id_to_pos.len()];

        for item in rpc {
            if let Some(error) = item.error {
                return Err(JsonRpcError::Rpc {
                    code: error.code,
                    message: error.message,
                    data: error.data,
                });
            }
            let id = item.id.ok_or(JsonRpcError::InvalidBatchResponse)?;
            let pos = id_to_pos.get(&id).ok_or(JsonRpcError::InvalidBatchResponse)?;
            let result = item.result.ok_or(JsonRpcError::MissingResult)?;
            out[*pos] = Some(result);
        }

        out.into_iter()
            .map(|v| v.ok_or(JsonRpcError::InvalidBatchResponse))
            .collect()
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

    pub async fn get_all_balances(&self, owner: &str) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call("suix_getAllBalances", vec![Value::String(owner.to_string())])
            .await
    }

    pub async fn get_coins(
        &self,
        owner: &str,
        coin_type: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getCoins",
            vec![
                Value::String(owner.to_string()),
                coin_type
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(|v| Value::from(v)).unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_all_coins(
        &self,
        owner: &str,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getAllCoins",
            vec![
                Value::String(owner.to_string()),
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(|v| Value::from(v)).unwrap_or(Value::Null),
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

    pub async fn multi_get_objects(
        &self,
        object_ids: Vec<String>,
        options: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        if object_ids.iter().any(|id| !utils::is_valid_sui_object_id(id)) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_multiGetObjects",
            vec![
                Value::Array(object_ids.into_iter().map(Value::String).collect()),
                options.unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_total_supply(&self, coin_type: &str) -> Result<Value, JsonRpcError> {
        self.call("suix_getTotalSupply", vec![Value::String(coin_type.to_string())])
            .await
    }

    pub async fn get_coin_metadata(&self, coin_type: &str) -> Result<Value, JsonRpcError> {
        self.call(
            "suix_getCoinMetadata",
            vec![Value::String(coin_type.to_string())],
        )
        .await
    }

    pub async fn get_owned_objects(
        &self,
        owner: &str,
        query: Option<Value>,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getOwnedObjects",
            vec![
                Value::String(owner.to_string()),
                query.unwrap_or_else(|| serde_json::json!({})),
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn query_events(
        &self,
        query: Value,
        cursor: Option<&str>,
        limit: Option<u64>,
        descending_order: bool,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "suix_queryEvents",
            vec![
                query,
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
                Value::Bool(descending_order),
            ],
        )
        .await
    }

    pub async fn query_transaction_blocks(
        &self,
        query: Value,
        cursor: Option<&str>,
        limit: Option<u64>,
        descending_order: bool,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "suix_queryTransactionBlocks",
            vec![
                query,
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
                Value::Bool(descending_order),
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

    pub async fn get_transaction_block(
        &self,
        digest: &str,
        options: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_getTransactionBlock",
            vec![
                Value::String(digest.to_string()),
                options.unwrap_or_else(|| serde_json::json!({})),
            ],
        )
        .await
    }

    pub async fn multi_get_transaction_blocks(
        &self,
        digests: Vec<String>,
        options: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_multiGetTransactionBlocks",
            vec![
                Value::Array(digests.into_iter().map(Value::String).collect()),
                options.unwrap_or_else(|| serde_json::json!({})),
            ],
        )
        .await
    }

    pub async fn get_total_transaction_blocks(&self) -> Result<Value, JsonRpcError> {
        self.call("sui_getTotalTransactionBlocks", vec![]).await
    }

    pub async fn try_get_past_object(
        &self,
        object_id: &str,
        version: u64,
        options: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(object_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_tryGetPastObject",
            vec![
                Value::String(object_id.to_string()),
                Value::String(version.to_string()),
                options.unwrap_or_else(|| serde_json::json!({})),
            ],
        )
        .await
    }

    pub async fn try_multi_get_past_objects(
        &self,
        past_objects: Vec<Value>,
        options: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_tryMultiGetPastObjects",
            vec![
                Value::Array(past_objects),
                options.unwrap_or_else(|| serde_json::json!({})),
            ],
        )
        .await
    }

    pub async fn get_events(&self, transaction_digest: &str) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_getEvents",
            vec![Value::String(transaction_digest.to_string())],
        )
        .await
    }

    pub async fn get_latest_checkpoint_sequence_number(&self) -> Result<Value, JsonRpcError> {
        self.call("sui_getLatestCheckpointSequenceNumber", vec![])
            .await
    }

    pub async fn get_checkpoint(&self, checkpoint_id: &str) -> Result<Value, JsonRpcError> {
        self.call("sui_getCheckpoint", vec![Value::String(checkpoint_id.to_string())])
            .await
    }

    pub async fn get_checkpoints(
        &self,
        cursor: Option<&str>,
        limit: Option<u64>,
        descending_order: bool,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_getCheckpoints",
            vec![
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
                Value::Bool(descending_order),
            ],
        )
        .await
    }

    pub async fn get_protocol_config(&self, version: Option<&str>) -> Result<Value, JsonRpcError> {
        self.call(
            "sui_getProtocolConfig",
            vec![version
                .map(|v| Value::String(v.to_string()))
                .unwrap_or(Value::Null)],
        )
        .await
    }

    pub async fn get_latest_sui_system_state(&self) -> Result<Value, JsonRpcError> {
        self.call("suix_getLatestSuiSystemState", vec![]).await
    }

    pub async fn get_chain_identifier(&self) -> Result<Value, JsonRpcError> {
        self.call("sui_getChainIdentifier", vec![]).await
    }

    pub async fn get_committee_info(&self, epoch: Option<&str>) -> Result<Value, JsonRpcError> {
        self.call(
            "suix_getCommitteeInfo",
            vec![epoch
                .map(|v| Value::String(v.to_string()))
                .unwrap_or(Value::Null)],
        )
        .await
    }

    pub async fn resolve_name_service_address(
        &self,
        name: &str,
    ) -> Result<Value, JsonRpcError> {
        self.call(
            "suix_resolveNameServiceAddress",
            vec![Value::String(name.to_string())],
        )
        .await
    }

    pub async fn resolve_name_service_names(
        &self,
        address: &str,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(address) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_resolveNameServiceNames",
            vec![
                Value::String(address.to_string()),
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_validators_apy(&self) -> Result<Value, JsonRpcError> {
        self.call("suix_getValidatorsApy", vec![]).await
    }

    pub async fn get_stakes(&self, owner: &str) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(owner) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call("suix_getStakes", vec![Value::String(owner.to_string())])
            .await
    }

    pub async fn get_stakes_by_ids(&self, staked_sui_ids: Vec<String>) -> Result<Value, JsonRpcError> {
        if staked_sui_ids
            .iter()
            .any(|id| !utils::is_valid_sui_object_id(id))
        {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getStakesByIds",
            vec![Value::Array(staked_sui_ids.into_iter().map(Value::String).collect())],
        )
        .await
    }

    pub async fn get_normalized_move_modules_by_package(
        &self,
        package_id: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(package_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getNormalizedMoveModulesByPackage",
            vec![Value::String(package_id.to_string())],
        )
        .await
    }

    pub async fn get_normalized_move_module(
        &self,
        package_id: &str,
        module_name: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(package_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getNormalizedMoveModule",
            vec![
                Value::String(package_id.to_string()),
                Value::String(module_name.to_string()),
            ],
        )
        .await
    }

    pub async fn get_normalized_move_function(
        &self,
        package_id: &str,
        module_name: &str,
        function_name: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(package_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getNormalizedMoveFunction",
            vec![
                Value::String(package_id.to_string()),
                Value::String(module_name.to_string()),
                Value::String(function_name.to_string()),
            ],
        )
        .await
    }

    pub async fn get_move_function_arg_types(
        &self,
        package_id: &str,
        module_name: &str,
        function_name: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(package_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getMoveFunctionArgTypes",
            vec![
                Value::String(package_id.to_string()),
                Value::String(module_name.to_string()),
                Value::String(function_name.to_string()),
            ],
        )
        .await
    }

    pub async fn get_normalized_move_struct(
        &self,
        package_id: &str,
        module_name: &str,
        struct_name: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(package_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_getNormalizedMoveStruct",
            vec![
                Value::String(package_id.to_string()),
                Value::String(module_name.to_string()),
                Value::String(struct_name.to_string()),
            ],
        )
        .await
    }

    pub async fn get_move_function(
        &self,
        package_id: &str,
        module_name: &str,
        function_name: &str,
    ) -> Result<Value, JsonRpcError> {
        let function = self
            .get_normalized_move_function(package_id, module_name, function_name)
            .await?;
        let parameters = self
            .get_move_function_arg_types(package_id, module_name, function_name)
            .await?;
        Ok(serde_json::json!({
            "function": function,
            "parameters": parameters
        }))
    }

    pub async fn verify_zk_login_signature(
        &self,
        bytes_base64: &str,
        signature: &str,
        intent_scope: &str,
        author: &str,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_address(author) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "sui_verifyZkLoginSignature",
            vec![
                Value::String(bytes_base64.to_string()),
                Value::String(signature.to_string()),
                Value::String(intent_scope.to_string()),
                Value::String(author.to_string()),
            ],
        )
        .await
    }

    pub async fn get_dynamic_fields(
        &self,
        parent_object_id: &str,
        cursor: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(parent_object_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getDynamicFields",
            vec![
                Value::String(parent_object_id.to_string()),
                cursor
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::Null),
                limit.map(Value::from).unwrap_or(Value::Null),
            ],
        )
        .await
    }

    pub async fn get_dynamic_field_object(
        &self,
        parent_object_id: &str,
        name: Value,
    ) -> Result<Value, JsonRpcError> {
        if !utils::is_valid_sui_object_id(parent_object_id) {
            return Err(JsonRpcError::InvalidAddress);
        }
        self.call(
            "suix_getDynamicFieldObject",
            vec![Value::String(parent_object_id.to_string()), name],
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
