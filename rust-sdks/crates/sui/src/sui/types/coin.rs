use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoinMetadata {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
}

impl CoinMetadata {
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        Self {
            decimals,
            name,
            symbol,
            description: None,
            icon_url: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coin {
    pub coin_type: String,
    pub balance: u64,
    pub object_ref: ObjectReference,
    pub previous_transaction: TransactionDigest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinBalance {
    pub coin_type: String,
    pub total_balance: u64,
    pub locked_balance: u64,
}

impl CoinBalance {
    pub fn available_amount(&self) -> u64 {
        self.total_balance.saturating_sub(self.locked_balance)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObjectReference {
    pub object_id: ObjectDigest,
    pub version: u64,
    pub digest: String,
}

impl ObjectReference {
    pub fn new(object_id: ObjectDigest, version: u64, digest: String) -> Self {
        Self {
            object_id,
            version,
            digest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TransactionDigest(pub String);

impl TransactionDigest {
    pub fn new(digest: String) -> Self {
        Self(digest)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TransactionDigest {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TransactionDigest {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObjectDigest(pub String);

impl ObjectDigest {
    pub fn new(digest: String) -> Self {
        Self(digest)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_metadata() {
        let metadata = CoinMetadata::new("SUI".to_string(), "SUI".to_string(), 9);
        assert_eq!(metadata.decimals, 9);
        assert_eq!(metadata.name, "SUI");
        assert_eq!(metadata.symbol, "SUI");
        assert!(metadata.description.is_none());
    }

    #[test]
    fn test_object_reference() {
        let obj_ref = ObjectReference::new(
            ObjectDigest::new("0x123".to_string()),
            1,
            "digest".to_string(),
        );
        assert_eq!(obj_ref.version, 1);
        assert_eq!(obj_ref.object_id.as_str(), "0x123");
    }

    #[test]
    fn test_transaction_digest() {
        let digest = TransactionDigest::new("0xabc123".to_string());
        assert_eq!(digest.as_str(), "0xabc123");
        assert_eq!(digest.to_string(), "0xabc123");
    }

    #[test]
    fn test_coin_balance_available() {
        let balance = CoinBalance {
            coin_type: "0x2::sui::SUI".to_string(),
            total_balance: 1000,
            locked_balance: 200,
        };
        assert_eq!(balance.available_amount(), 800);
    }

    #[test]
    fn test_coin_balance_all_locked() {
        let balance = CoinBalance {
            coin_type: "0x2::sui::SUI".to_string(),
            total_balance: 1000,
            locked_balance: 1000,
        };
        assert_eq!(balance.available_amount(), 0);
    }

    #[test]
    fn test_transaction_digest_from() {
        let digest: TransactionDigest = "0x456".into();
        assert_eq!(digest.as_str(), "0x456");

        let digest2: TransactionDigest = String::from("0x789").into();
        assert_eq!(digest2.as_str(), "0x789");
    }

    #[test]
    fn test_object_digest_display() {
        let digest = ObjectDigest::new("0xtest".to_string());
        assert_eq!(digest.to_string(), "0xtest");
    }
}
