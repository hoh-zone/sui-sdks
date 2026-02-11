use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub data: TransactionData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionData {
    pub sender: Option<String>,
    pub expiration: Option<Value>,
    #[serde(default, rename = "gasData")]
    pub gas_data: GasData,
    #[serde(default)]
    pub inputs: Vec<Value>,
    #[serde(default)]
    pub commands: Vec<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GasData {
    pub owner: Option<String>,
    pub price: Option<String>,
    pub budget: Option<String>,
    pub payment: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx_bytes_base64: String,
    pub signatures: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EpochId(pub u64);

impl EpochId {
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectId(pub String);

impl ObjectId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_data_default() {
        let tx_data = TransactionData::default();
        assert!(tx_data.sender.is_none());
        assert!(tx_data.inputs.is_empty());
        assert!(tx_data.commands.is_empty());
    }

    #[test]
    fn test_gas_data_default() {
        let gas = GasData::default();
        assert!(gas.owner.is_none());
        assert!(gas.price.is_none());
        assert!(gas.budget.is_none());
    }

    #[test]
    fn test_transaction_default() {
        let tx = Transaction::default();
        assert!(tx.data.sender.is_none());
    }

    #[test]
    fn test_epoch_id() {
        let epoch = EpochId::new(100);
        assert_eq!(epoch.as_u64(), 100);
    }

    #[test]
    fn test_epoch_id_ord() {
        assert!(EpochId(10) < EpochId(20));
        assert!(EpochId(30) > EpochId(10));
    }

    #[test]
    fn test_object_id() {
        let id = ObjectId::new("0x123".to_string());
        assert_eq!(id.as_str(), "0x123");
    }

    #[test]
    fn test_signed_transaction() {
        let signed = SignedTransaction {
            tx_bytes_base64: "base64data".to_string(),
            signatures: vec!["signature".to_string()],
        };
        assert_eq!(signed.tx_bytes_base64, "base64data");
        assert_eq!(signed.signatures.len(), 1);
    }
}
