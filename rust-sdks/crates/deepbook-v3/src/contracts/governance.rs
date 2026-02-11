use crate::config::DeepBookConfig;
use crate::encode::{encode_u64, encode_u8};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum GovernanceError {
    #[error("config error: {0}")]
    Config(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("balance manager not found: {0}")]
    BalanceManagerNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
}

pub struct GovernanceContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> GovernanceContract<'a> {
    pub fn stake(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        stake_amount: f64,
    ) -> Result<serde_json::Value, GovernanceError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| GovernanceError::PoolNotFound(e.to_string()))?;
        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key)
            .map_err(|e| GovernanceError::BalanceManagerNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;
        let stake_input = (stake_amount * crate::config::DEEP_SCALAR).round() as u64;

        let proof = tx.object(balance_manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(balance_manager.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64(stake_input));

        Ok(tx.move_call(
            &format!(
                "{}::pool::stake",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj, proof, amount_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unstake(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<serde_json::Value, GovernanceError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| GovernanceError::PoolNotFound(e.to_string()))?;
        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key)
            .map_err(|e| GovernanceError::BalanceManagerNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(balance_manager.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool::unstake",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn submit_proposal(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        taker_fee: f64,
        maker_fee: f64,
        stake_required: f64,
    ) -> Result<serde_json::Value, GovernanceError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| GovernanceError::PoolNotFound(e.to_string()))?;
        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key)
            .map_err(|e| GovernanceError::BalanceManagerNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;

        let taker_fee_input = (taker_fee * crate::config::FLOAT_SCALAR).round() as u64;
        let maker_fee_input = (maker_fee * crate::config::FLOAT_SCALAR).round() as u64;
        let stake_input = (stake_required * crate::config::DEEP_SCALAR).round() as u64;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(balance_manager.address.clone());
        let taker_fee_arg = tx.pure_bytes(&encode_u64(taker_fee_input));
        let maker_fee_arg = tx.pure_bytes(&encode_u64(maker_fee_input));
        let stake_arg = tx.pure_bytes(&encode_u64(stake_input));

        Ok(tx.move_call(
            &format!(
                "{}::pool::submit_proposal",
                self.config.package_ids.deepbook_package_id
            ),
            vec![
                pool_obj,
                manager_obj,
                taker_fee_arg,
                maker_fee_arg,
                stake_arg,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn vote(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        proposal_id: &str,
    ) -> Result<serde_json::Value, GovernanceError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| GovernanceError::PoolNotFound(e.to_string()))?;
        let balance_manager = self
            .config
            .get_balance_manager(balance_manager_key)
            .map_err(|e| GovernanceError::BalanceManagerNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| GovernanceError::CoinNotFound(e.to_string()))?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(balance_manager.address.clone());
        let proposal_id_arg = tx.pure_bytes(&encode_u64(0u64));

        Ok(tx.move_call(
            &format!(
                "{}::pool::vote",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj, proposal_id_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stake_build() {
        let mut config = DeepBookConfig::default();
        use crate::types::BalanceManager;
        let mut bms = std::collections::HashMap::new();
        bms.insert(
            "manager1".to_string(),
            BalanceManager {
                address: "0x123".to_string(),
                trade_cap: None,
            },
        );
        config.balance_managers = bms;
        let mut tx = Transaction::new();
        let result =
            GovernanceContract { config: &config }.stake(&mut tx, "DEEP_SUI", "manager1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unstake_build() {
        let mut config = DeepBookConfig::default();
        use crate::types::BalanceManager;
        let mut bms = std::collections::HashMap::new();
        bms.insert(
            "manager1".to_string(),
            BalanceManager {
                address: "0x123".to_string(),
                trade_cap: None,
            },
        );
        config.balance_managers = bms;
        let mut tx = Transaction::new();
        let result =
            GovernanceContract { config: &config }.unstake(&mut tx, "DEEP_SUI", "manager1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_proposal_build() {
        let mut config = DeepBookConfig::default();
        use crate::types::BalanceManager;
        let mut bms = std::collections::HashMap::new();
        bms.insert(
            "manager1".to_string(),
            BalanceManager {
                address: "0x123".to_string(),
                trade_cap: None,
            },
        );
        config.balance_managers = bms;
        let mut tx = Transaction::new();
        let result = GovernanceContract { config: &config }
            .submit_proposal(&mut tx, "DEEP_SUI", "manager1", 0.001, 0.001, 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vote_build() {
        let mut config = DeepBookConfig::default();
        use crate::types::BalanceManager;
        let mut bms = std::collections::HashMap::new();
        bms.insert(
            "manager1".to_string(),
            BalanceManager {
                address: "0x123".to_string(),
                trade_cap: None,
            },
        );
        config.balance_managers = bms;
        let mut tx = Transaction::new();
        let result =
            GovernanceContract { config: &config }.vote(&mut tx, "DEEP_SUI", "manager1", "0x123");
        assert!(result.is_ok());
    }
}
