#[derive(Debug, thiserror::Error)]
pub enum BatchJsonError {
    #[error("json error: {0}")]
    Json(serde_json::Error),
    #[error("jsonrpc error: {0}")]
    JsonRpc(#[from] JsonRpcError),
}

impl From<serde_json::Error> for BatchJsonError {
    fn from(e: serde_json::Error) -> Self {
        BatchJsonError::Json(e)
    }
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::jsonrpc::JsonRpcError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    #[allow(private_interfaces)]
    Single(RpcRequest),
    #[allow(private_interfaces)]
    Batch(Vec<RpcRequest>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchRequest {
    #[allow(private_interfaces)]
    pub requests: Vec<RpcRequest>,
}

impl BatchRequest {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    pub fn add(&mut self, method: &str, params: Vec<Value>) -> &mut Self {
        let id = self.requests.len() as u64 + 1;
        self.requests.push(RpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        });
        self
    }

    pub fn with_request(mut self, method: &str, params: Vec<Value>) -> Self {
        self.add(method, params);
        self
    }

    pub fn len(&self) -> usize {
        self.requests.len()
    }

    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    pub fn build(&self) -> Result<Value, BatchJsonError> {
        if self.requests.is_empty() {
            return Ok(Value::Array(vec![]));
        }
        serde_json::to_value(&self.requests).map_err(Into::into)
    }

    pub fn requests(&self) -> &[RpcRequest] {
        &self.requests
    }
}

impl Default for BatchRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BatchResponse {
    Single(Option<RpcResponse>),
    Batch(Vec<Option<RpcResponse>>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcResponse {
    id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<JsonRpcErrorObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcErrorObject {
    pub code: i64,
    #[allow(dead_code)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    pub data: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum BatchError {
    #[error("empty batch request")]
    EmptyRequest,
    #[error("batch request limit exceeded: {0} > {1}")]
    LimitExceeded(usize, usize),
    #[error("invalid response format")]
    InvalidResponse,
    #[error("jsonrpc error: {0}")]
    JsonRpc(#[from] JsonRpcError),
    #[error("mixed single/batch response")]
    MixedResponse,
    #[error("response count mismatch: expected {expected}, got {actual}")]
    CountMismatch { expected: usize, actual: usize },
}

#[derive(Debug, Clone)]
pub struct BatchOptions {
    pub max_requests: usize,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self { max_requests: 100 }
    }
}

#[derive(Debug, Clone)]
pub struct BatchResponseItem {
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<JsonRpcErrorObject>,
}

impl BatchRequest {
    #[allow(private_interfaces)]
    pub fn validate(&self, opts: &BatchOptions) -> Result<(), BatchError> {
        if self.requests.is_empty() {
            return Err(BatchError::EmptyRequest);
        }
        if self.requests.len() > opts.max_requests {
            return Err(BatchError::LimitExceeded(
                self.requests.len(),
                opts.max_requests,
            ));
        }
        Ok(())
    }
}

pub fn parse_batch_response(value: Value) -> Result<Vec<BatchResponseItem>, BatchError> {
    if let Value::Array(items) = value {
        let mut results = Vec::new();
        for item in items {
            if let Ok(resp) = serde_json::from_value::<RpcResponse>(item) {
                results.push(BatchResponseItem {
                    id: resp.id.unwrap_or(0),
                    result: resp.result,
                    error: resp.error,
                });
            } else {
                return Err(BatchError::InvalidResponse);
            }
        }
        Ok(results)
    } else {
        Err(BatchError::InvalidResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_request_new() {
        let batch = BatchRequest::new();
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
    }

    #[test]
    fn test_batch_request_add() {
        let mut batch = BatchRequest::new();
        batch.add("test_method", vec![]);

        assert!(!batch.is_empty());
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_request_with_request() {
        let batch = BatchRequest::new()
            .with_request("method1", vec![])
            .with_request("method2", vec![]);

        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_batch_request_build() {
        let mut batch = BatchRequest::new();
        batch.add("test_method", vec![Value::String("param".to_string())]);

        let result = batch.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_request_build_empty() {
        let batch = BatchRequest::new();
        let result = batch.build();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn test_batch_request_validate_success() {
        let mut batch = BatchRequest::new();
        batch.add("test_method", vec![]);

        let opts = BatchOptions::default();
        let result = batch.validate(&opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_request_validate_empty() {
        let batch = BatchRequest::new();
        let opts = BatchOptions::default();

        let result = batch.validate(&opts);
        assert!(matches!(result, Err(BatchError::EmptyRequest)));
    }

    #[test]
    fn test_batch_request_validate_limit_exceeded() {
        let mut batch = BatchRequest::new();
        for _ in 0..101 {
            batch.add("test_method", vec![]);
        }

        let opts = BatchOptions::default();
        let result = batch.validate(&opts);
        assert!(matches!(result, Err(BatchError::LimitExceeded(..))));
    }

    #[test]
    fn test_batch_options_default() {
        let opts = BatchOptions::default();
        assert_eq!(opts.max_requests, 100);
    }

    #[test]
    fn test_parse_batch_response() {
        let response_value = serde_json::json!([
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {"value": "test"}
            },
            {
                "jsonrpc": "2.0",
                "id": 2,
                "result": {"value": "test2"}
            }
        ]);

        let result = parse_batch_response(response_value);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_parse_batch_response_invalid() {
        let response_value = serde_json::json!("invalid");

        let result = parse_batch_response(response_value);
        assert!(matches!(result, Err(BatchError::InvalidResponse)));
    }

    #[test]
    fn test_parse_batch_response_with_error() {
        let response_value = serde_json::json!([
            {
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32600,
                    "message": "Invalid Request"
                }
            }
        ]);

        let result = parse_batch_response(response_value);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items[0].error.as_ref().unwrap().code, -32600);
    }
}
