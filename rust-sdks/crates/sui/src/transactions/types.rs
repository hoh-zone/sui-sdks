use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("serialize transaction failed: {0}")]
    Serialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GasData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Value>,
    #[serde(default, rename = "gasData")]
    pub gas_data: GasData,
    #[serde(default)]
    pub inputs: Vec<Value>,
    #[serde(default)]
    pub commands: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub data: TransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx_bytes_base64: String,
    pub signatures: Vec<String>,
}
