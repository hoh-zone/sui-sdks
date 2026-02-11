use crate::encode::encode_u64;
use serde::{Deserialize, Serialize};
use sui::transactions::Transaction;

const PYTH_PACKAGE_ID: &str = "0xabf837e98c26087cba0883c0a7a28326b1fa3c5e1e2c5abdb486f9e8f594c837";

const STALE_PRICE_THRESHOLD_SEC: u64 = 60;

const CURRENT_UNIX_TIME: u64 = 1740000000u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub price: i64,
    pub conf: u64,
    pub expo: i64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFeed {
    pub price_identifier: PriceIdentifier,
    pub price: Price,
    pub ema_price: Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceIdentifier {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub attestation_time: u64,
    pub arrival_time: u64,
    pub price_feed: PriceFeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPriceAttestation {
    pub header: Header,
    pub attestation_count: u64,
    pub price_infos: Vec<PriceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub magic: u64,
    pub version_major: u64,
    pub version_minor: u64,
    pub header_size: u64,
    pub payload_id: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormholeVAAVerificationReceipt {
    pub payload: Vec<u8>,
    pub digest: Vec<u8>,
    pub sequence: u64,
}

#[derive(Debug, Clone)]
pub struct UpdatePriceData {
    pub price_feed_id: Vec<u8>,
    pub price: i64,
    pub conf: u64,
    pub expo: i64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct PriceUpdateParams {
    pub updates: Vec<UpdatePriceData>,
}

#[derive(Debug, thiserror::Error)]
pub enum PythError {
    #[error("invalid price feed")]
    InvalidPriceFeed,
    #[error("price expired")]
    PriceExpired,
    #[error("invalid VAA")]
    InvalidVAA,
    #[error("governance error: {0}")]
    Governance(String),
}

pub struct PythOracleContract;

impl PythOracleContract {
    pub fn get_price(
        tx: &mut Transaction,
        state_id: &str,
        price_feed_id: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let price_feed_arg = tx.pure_bytes(price_feed_id);
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!("{}::pyth::get_price", PYTH_PACKAGE_ID),
            vec![state_obj, price_feed_arg, clock_obj],
            vec![],
        ))
    }

    pub fn get_price_unsafe(
        tx: &mut Transaction,
        state_id: &str,
        price_feed_id: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let price_feed_arg = tx.pure_bytes(price_feed_id);

        Ok(tx.move_call(
            &format!("{}::pyth::get_price_unsafe", PYTH_PACKAGE_ID),
            vec![state_obj, price_feed_arg],
            vec![],
        ))
    }

    pub fn get_price_no_older_than(
        tx: &mut Transaction,
        state_id: &str,
        price_feed_id: &[u8],
        max_age_secs: u64,
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let price_feed_arg = tx.pure_bytes(price_feed_id);
        let max_age_arg = tx.pure_bytes(&encode_u64(max_age_secs));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!("{}::pyth::get_price_no_older_than", PYTH_PACKAGE_ID),
            vec![state_obj, price_feed_arg, max_age_arg, clock_obj],
            vec![],
        ))
    }

    pub fn price_feed_exists(
        tx: &mut Transaction,
        state_id: &str,
        price_feed_id: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let price_feed_arg = tx.pure_bytes(price_feed_id);

        Ok(tx.move_call(
            &format!("{}::pyth::price_feed_exists", PYTH_PACKAGE_ID),
            vec![state_obj, price_feed_arg],
            vec![],
        ))
    }

    pub fn update_single_price_feed(
        tx: &mut Transaction,
        state_id: &str,
        vaa: &[u8],
        price_feed_id: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let price_feed_arg = tx.pure_bytes(price_feed_id);
        let vaa_arg = tx.pure_bytes(vaa);
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!("{}::pyth::update_single_price_feed", PYTH_PACKAGE_ID),
            vec![state_obj, vaa_arg, price_feed_arg, clock_obj],
            vec![],
        ))
    }

    pub fn create_authenticated_price_info(
        tx: &mut Transaction,
        state_id: &str,
        price_attestation: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let attestation_arg = tx.pure_bytes(price_attestation);
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!("{}::pyth::create_authenticated_price_info", PYTH_PACKAGE_ID),
            vec![state_obj, attestation_arg, clock_obj],
            vec![],
        ))
    }

    pub fn get_stale_price_threshold_secs(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());

        Ok(tx.move_call(
            &format!("{}::pyth::get_stale_price_threshold_secs", PYTH_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_total_update_fee(
        tx: &mut Transaction,
        state_id: &str,
        price_feed_ids: Vec<Vec<u8>>,
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let count_arg = tx.pure_bytes(&encode_u64(price_feed_ids.len() as u64));

        Ok(tx.move_call(
            &format!("{}::pyth::get_total_update_fee", PYTH_PACKAGE_ID),
            vec![state_obj, count_arg],
            vec![],
        ))
    }

    pub fn set_governance_data_source(
        tx: &mut Transaction,
        state_id: &str,
        upgrade_cap: &str,
        governance_data_source: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let upgrade_cap_obj = tx.object(upgrade_cap.to_string());
        let gov_source_arg = tx.pure_bytes(governance_data_source);

        Ok(tx.move_call(
            &format!(
                "{}::governance::set_governance_data_source",
                PYTH_PACKAGE_ID
            ),
            vec![state_obj, upgrade_cap_obj, gov_source_arg],
            vec![],
        ))
    }

    pub fn verify_vaa(
        tx: &mut Transaction,
        state_id: &str,
        vaa: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let vaa_arg = tx.pure_bytes(vaa);

        Ok(tx.move_call(
            &format!("{}::governance::verify_vaa", PYTH_PACKAGE_ID),
            vec![state_obj, vaa_arg],
            vec![],
        ))
    }

    pub fn execute_governance_instruction(
        tx: &mut Transaction,
        state_id: &str,
        vaa_receipt: &str,
        governance_instruction: &[u8],
    ) -> Result<serde_json::Value, PythError> {
        let state_obj = tx.object(state_id.to_string());
        let vaa_receipt_obj = tx.object(vaa_receipt.to_string());
        let instruction_arg = tx.pure_bytes(governance_instruction);

        Ok(tx.move_call(
            &format!(
                "{}::governance::execute_governance_instruction",
                PYTH_PACKAGE_ID
            ),
            vec![state_obj, vaa_receipt_obj, instruction_arg],
            vec![],
        ))
    }

    pub fn parse_price_from_bytecode(bytecode: &[u8]) -> Result<Price, PythError> {
        if bytecode.len() < 32 {
            return Err(PythError::InvalidPriceFeed);
        }

        let price = i64::from_le_bytes(bytecode[0..8].try_into().unwrap());
        let conf = u64::from_le_bytes(bytecode[8..16].try_into().unwrap());
        let expo = i64::from_le_bytes(bytecode[16..24].try_into().unwrap());
        let timestamp = u64::from_le_bytes(bytecode[24..32].try_into().unwrap());

        Ok(Price {
            price,
            conf,
            expo,
            timestamp,
        })
    }

    pub fn validate_price(price: &Price, max_age_secs: u64) -> Result<(), PythError> {
        if price.conf > price.price.abs() as u64 * 2 {
            return Err(PythError::InvalidPriceFeed);
        }
        if price.expo > 20 || price.expo < -20 {
            return Err(PythError::InvalidPriceFeed);
        }

        let current_time = self::CURRENT_UNIX_TIME;

        if current_time.saturating_sub(price.timestamp) > max_age_secs {
            return Err(PythError::PriceExpired);
        }

        Ok(())
    }
}

impl Default for Price {
    fn default() -> Self {
        Self {
            price: 0,
            conf: 0,
            expo: 0,
            timestamp: 0,
        }
    }
}

impl Default for PriceFeed {
    fn default() -> Self {
        Self {
            price_identifier: PriceIdentifier { value: vec![] },
            price: Price::default(),
            ema_price: Price::default(),
        }
    }
}

impl Default for PriceInfo {
    fn default() -> Self {
        Self {
            attestation_time: 0,
            arrival_time: 0,
            price_feed: PriceFeed::default(),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            magic: 0,
            version_major: 0,
            version_minor: 0,
            header_size: 0,
            payload_id: 0,
        }
    }
}

impl Default for BatchPriceAttestation {
    fn default() -> Self {
        Self {
            header: Header::default(),
            attestation_count: 0,
            price_infos: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_price_build() {
        let mut tx = Transaction::new();
        let price_feed_id = vec![1, 2, 3, 4];
        let result = PythOracleContract::get_price(&mut tx, "state_id", &price_feed_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_price_unsafe_build() {
        let mut tx = Transaction::new();
        let price_feed_id = vec![1, 2, 3, 4];
        let result = PythOracleContract::get_price_unsafe(&mut tx, "state_id", &price_feed_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_price_no_older_than_build() {
        let mut tx = Transaction::new();
        let price_feed_id = vec![1, 2, 3, 4];
        let result =
            PythOracleContract::get_price_no_older_than(&mut tx, "state_id", &price_feed_id, 60);
        assert!(result.is_ok());
    }

    #[test]
    fn test_price_feed_exists_build() {
        let mut tx = Transaction::new();
        let price_feed_id = vec![1, 2, 3, 4];
        let result = PythOracleContract::price_feed_exists(&mut tx, "state_id", &price_feed_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_single_price_feed_build() {
        let mut tx = Transaction::new();
        let price_feed_id = vec![1, 2, 3, 4];
        let vaa = vec![5, 6, 7, 8];
        let result =
            PythOracleContract::update_single_price_feed(&mut tx, "state_id", &vaa, &price_feed_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_authenticated_price_info_build() {
        let mut tx = Transaction::new();
        let attestation = vec![1, 2, 3, 4];
        let result =
            PythOracleContract::create_authenticated_price_info(&mut tx, "state_id", &attestation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_stale_price_threshold_secs_build() {
        let mut tx = Transaction::new();
        let result = PythOracleContract::get_stale_price_threshold_secs(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_total_update_fee_build() {
        let mut tx = Transaction::new();
        let feed_ids = vec![vec![1, 2], vec![3, 4]];
        let result = PythOracleContract::get_total_update_fee(&mut tx, "state_id", feed_ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_governance_data_source_build() {
        let mut tx = Transaction::new();
        let gov_source = vec![1, 2, 3, 4];
        let result = PythOracleContract::set_governance_data_source(
            &mut tx,
            "state_id",
            "cap_id",
            &gov_source,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_vaa_build() {
        let mut tx = Transaction::new();
        let vaa = vec![1, 2, 3, 4];
        let result = PythOracleContract::verify_vaa(&mut tx, "state_id", &vaa);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_governance_instruction_build() {
        let mut tx = Transaction::new();
        let instruction = vec![1, 2, 3, 4];
        let result = PythOracleContract::execute_governance_instruction(
            &mut tx,
            "state_id",
            "vaa_receipt",
            &instruction,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_price_from_bytecode() {
        let bytecode = [
            0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x40, 0x20, 0x0A, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let result = PythOracleContract::parse_price_from_bytecode(&bytecode);
        assert!(result.is_ok());
        let price = result.unwrap();
        assert_eq!(price.price, 4096);
        assert_eq!(price.conf, 1280);
    }

    #[test]
    fn test_validate_price_valid() {
        let price = Price {
            price: 100_000_000,
            conf: 10_000,
            expo: 8,
            timestamp: CURRENT_UNIX_TIME.saturating_sub(10),
        };
        let result = PythOracleContract::validate_price(&price, 60);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_price_expired() {
        let price = Price {
            price: 100_000_000,
            conf: 10_000,
            expo: 8,
            timestamp: CURRENT_UNIX_TIME.saturating_sub(100),
        };
        let result = PythOracleContract::validate_price(&price, 60);
        assert!(matches!(result, Err(PythError::PriceExpired)));
    }

    #[test]
    fn test_validate_price_high_confidence() {
        let price = Price {
            price: 100_000_000,
            conf: 250_000_000,
            expo: 8,
            timestamp: CURRENT_UNIX_TIME.saturating_sub(10),
        };
        let result = PythOracleContract::validate_price(&price, 60);
        assert!(matches!(result, Err(PythError::InvalidPriceFeed)));
    }

    #[test]
    fn test_validate_price_invalid_expo() {
        let price = Price {
            price: 100_000_000,
            conf: 10_000,
            expo: 25,
            timestamp: CURRENT_UNIX_TIME.saturating_sub(10),
        };
        let result = PythOracleContract::validate_price(&price, 60);
        assert!(matches!(result, Err(PythError::InvalidPriceFeed)));
    }

    #[test]
    fn test_price_defaults() {
        let price = Price::default();
        assert_eq!(price.price, 0);
        assert_eq!(price.conf, 0);
    }

    #[test]
    fn test_price_feed_defaults() {
        let feed = PriceFeed::default();
        assert_eq!(feed.price.price, 0);
    }

    #[test]
    fn test_price_info_defaults() {
        let info = PriceInfo::default();
        assert_eq!(info.attestation_time, 0);
    }
}
