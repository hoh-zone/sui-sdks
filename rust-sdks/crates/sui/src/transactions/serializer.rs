pub struct TransactionSerializer;

#[derive(Debug, thiserror::Error)]
pub enum SerializerError {
    #[error("json encode failed: {0}")]
    JsonEncode(#[from] serde_json::Error),
}

impl TransactionSerializer {
    pub fn serialize_transaction_data_v1(
        data: &crate::transactions::TransactionData,
    ) -> Result<Vec<u8>, SerializerError> {
        Ok(serde_json::to_vec(data)?)
    }

    pub fn serialize_transaction_data_v2(
        data: &crate::transactions::TransactionData,
    ) -> Result<Vec<u8>, SerializerError> {
        // Keep parity with current transaction data model (JSON-like), not BCS structs.
        Ok(serde_json::to_vec(data)?)
    }

    pub fn deserialize_transaction_data_v1(
        bytes: &[u8],
    ) -> Result<crate::transactions::TransactionData, SerializerError> {
        Ok(serde_json::from_slice(bytes)?)
    }

    pub fn deserialize_transaction_data_v2(
        bytes: &[u8],
    ) -> Result<crate::transactions::TransactionData, SerializerError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_transaction_data_v1() {
        let tx_data = crate::transactions::TransactionData {
            sender: Some("0x1".to_string()),
            ..Default::default()
        };

        let serialized = TransactionSerializer::serialize_transaction_data_v1(&tx_data);
        assert!(serialized.is_ok());
        assert!(!serialized.unwrap().is_empty());
    }

    #[test]
    fn test_serialize_deserialize_transaction_data() {
        let tx_data = crate::transactions::TransactionData {
            sender: Some("0x1".to_string()),
            ..Default::default()
        };

        let serialized = TransactionSerializer::serialize_transaction_data_v1(&tx_data).unwrap();
        let deserialized =
            TransactionSerializer::deserialize_transaction_data_v1(&serialized).unwrap();

        assert_eq!(deserialized.sender, tx_data.sender);
    }
}
