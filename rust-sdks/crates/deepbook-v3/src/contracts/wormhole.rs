use crate::encode::encode_u64;
use serde::{Deserialize, Serialize};
use sui::transactions::Transaction;

const WORMHOLE_PACKAGE_ID: &str =
    "0xf47329f4344f3bf0f8e436e2f7b485466cff300f12a166563995d3888c296a94";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormholeState {
    pub id: String,
    pub governance_chain: u16,
    pub governance_contract: ExternalAddress,
    pub guardian_set_index: u32,
    pub guardian_sets: Vec<GuardianSet>,
    pub guardian_set_seconds_to_live: u32,
    pub consumed_vaas: ConsumedVAAs,
    pub fee_collector: FeeCollector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAddress {
    pub value: Bytes32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bytes32 {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardianSet {
    pub index: u32,
    pub keys: Vec<String>,
    pub expiration_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumedVAAs {
    pub hashes: Vec<Bytes32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCollector {
    pub fee_amount: u64,
    pub balance: Balance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VAA {
    pub version: u8,
    pub guardian_set_index: u32,
    pub timestamp: u64,
    pub nonce: u32,
    pub emitter_chain: u16,
    pub emitter_address: String,
    pub sequence: u64,
    pub consistency_level: u8,
    pub payload: Vec<u8>,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub guardian_index: u8,
    pub signature: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum WormholeError {
    #[error("invalid VAA")]
    InvalidVAA,
    #[error("guardian set not found")]
    GuardianSetNotFound,
    #[error("VAA expired")]
    VAAExpired,
    #[error("insufficient signatures")]
    InsufficientSignatures,
}

pub struct WormholeContract;

impl WormholeContract {
    pub fn get_state(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!("{}::state::borrow_state", WORMHOLE_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_governance_chain(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!("{}::state::get_governance_chain", WORMHOLE_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_guardian_set_index(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!("{}::state::get_guardian_set_index", WORMHOLE_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_guardian_set_seconds_to_live(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::state::get_guardian_set_seconds_to_live",
                WORMHOLE_PACKAGE_ID
            ),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_fee_collector(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::fee_collector::borrow_fee_collector",
                WORMHOLE_PACKAGE_ID
            ),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn get_fee_amount(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!("{}::fee_collector::get_fee_amount", WORMHOLE_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn set_fee_amount(
        tx: &mut Transaction,
        state_id: &str,
        upgrade_cap: &str,
        new_fee: u64,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        let upgrade_cap_obj = tx.object(upgrade_cap.to_string());
        let fee_arg = tx.pure_bytes(&encode_u64(new_fee));

        Ok(tx.move_call(
            &format!("{}::fee_collector::set_fee_amount", WORMHOLE_PACKAGE_ID),
            vec![state_obj, upgrade_cap_obj, fee_arg],
            vec![],
        ))
    }

    pub fn consume_fee(
        tx: &mut Transaction,
        state_id: &str,
        upgrade_cap: &str,
        amount: u64,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        let upgrade_cap_obj = tx.object(upgrade_cap.to_string());
        let amount_arg = tx.pure_bytes(&encode_u64(amount));

        Ok(tx.move_call(
            &format!("{}::fee_collector::consume_fee", WORMHOLE_PACKAGE_ID),
            vec![state_obj, upgrade_cap_obj, amount_arg],
            vec![],
        ))
    }

    pub fn is_vaa_consumed(
        tx: &mut Transaction,
        state_id: &str,
        vaa_hash: &[u8],
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        let hash_arg = tx.pure_bytes(vaa_hash);

        Ok(tx.move_call(
            &format!("{}::consumed_vaas::is_vaa_consumed", WORMHOLE_PACKAGE_ID),
            vec![state_obj, hash_arg],
            vec![],
        ))
    }

    pub fn consume_vaa(
        tx: &mut Transaction,
        state_id: &str,
        upgrade_cap: &str,
        vaa_hash: &[u8],
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        let upgrade_cap_obj = tx.object(upgrade_cap.to_string());
        let hash_arg = tx.pure_bytes(vaa_hash);

        Ok(tx.move_call(
            &format!("{}::consumed_vaas::consume_vaa", WORMHOLE_PACKAGE_ID),
            vec![state_obj, upgrade_cap_obj, hash_arg],
            vec![],
        ))
    }

    pub fn get_upgrade_cap(
        tx: &mut Transaction,
        state_id: &str,
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        Ok(tx.move_call(
            &format!("{}::state::get_upgrade_cap", WORMHOLE_PACKAGE_ID),
            vec![state_obj],
            vec![],
        ))
    }

    pub fn verify_and_execute_vaa(
        tx: &mut Transaction,
        state_id: &str,
        vaa: &[u8],
    ) -> Result<serde_json::Value, WormholeError> {
        let state_obj = tx.object(state_id.to_string());
        let vaa_arg = tx.pure_bytes(vaa);

        Ok(tx.move_call(
            &format!("{}::state::verify_and_execute_vaa", WORMHOLE_PACKAGE_ID),
            vec![state_obj, vaa_arg],
            vec![],
        ))
    }
}

impl Default for WormholeState {
    fn default() -> Self {
        Self {
            id: String::new(),
            governance_chain: 1,
            governance_contract: ExternalAddress {
                value: Bytes32 {
                    data: vec![0u8; 32],
                },
            },
            guardian_set_index: 0,
            guardian_sets: vec![],
            guardian_set_seconds_to_live: 86400,
            consumed_vaas: ConsumedVAAs { hashes: vec![] },
            fee_collector: FeeCollector {
                fee_amount: 0,
                balance: Balance { value: vec![] },
            },
        }
    }
}

impl Default for ExternalAddress {
    fn default() -> Self {
        Self {
            value: Bytes32 {
                data: vec![0u8; 32],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wormhole_state_default() {
        let state = WormholeState::default();
        assert_eq!(state.governance_chain, 1);
        assert_eq!(state.guardian_set_index, 0);
    }

    #[test]
    fn test_external_address_default() {
        let addr = ExternalAddress::default();
        assert_eq!(addr.value.data.len(), 32);
    }

    #[test]
    fn test_bytes32_default() {
        let bytes = Bytes32 {
            data: vec![0u8; 32],
        };
        assert_eq!(bytes.data.len(), 32);
    }

    #[test]
    fn test_get_state_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::get_state(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_governance_chain_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::get_governance_chain(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_guardian_set_index_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::get_guardian_set_index(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_fee_collector_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::get_fee_collector(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_fee_amount_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::get_fee_amount(&mut tx, "state_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_fee_amount_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::set_fee_amount(&mut tx, "state_id", "cap_id", 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_consume_fee_build() {
        let mut tx = Transaction::new();
        let result = WormholeContract::consume_fee(&mut tx, "state_id", "cap_id", 50);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_vaa_consumed_build() {
        let mut tx = Transaction::new();
        let hash = vec![0u8; 32];
        let result = WormholeContract::is_vaa_consumed(&mut tx, "state_id", &hash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_consume_vaa_build() {
        let mut tx = Transaction::new();
        let hash = vec![0u8; 32];
        let result = WormholeContract::consume_vaa(&mut tx, "state_id", "cap_id", &hash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_and_execute_vaa_build() {
        let mut tx = Transaction::new();
        let vaa = vec![1, 2, 3];
        let result = WormholeContract::verify_and_execute_vaa(&mut tx, "state_id", &vaa);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vaa_serialization() {
        let vaa = VAA {
            version: 1,
            guardian_set_index: 0,
            timestamp: 12345,
            nonce: 0,
            emitter_chain: 2,
            emitter_address: "0x123".to_string(),
            sequence: 1,
            consistency_level: 1,
            payload: vec![],
            signatures: vec![],
        };
        assert_eq!(vaa.version, 1);
        assert_eq!(vaa.emitter_chain, 2);
    }
}
