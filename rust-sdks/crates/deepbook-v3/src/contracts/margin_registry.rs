use crate::config::{ConfigError, DeepBookConfig};
use crate::encode::encode_address;
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum MarginRegistryError {
    #[error("config error: {0}")]
    Config(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
}

impl From<ConfigError> for MarginRegistryError {
    fn from(e: ConfigError) -> Self {
        MarginRegistryError::Config(e.to_string())
    }
}

pub struct MarginRegistryContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginRegistryContract<'a> {
    pub fn new(config: &'a DeepBookConfig) -> Self {
        Self { config }
    }

    pub fn pool_enabled(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::pool_enabled",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![base_coin.type_tag.clone(), quote_coin.type_tag.clone()],
        ))
    }

    pub fn get_margin_pool_id(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let coin = self.config.get_coin(coin_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::get_margin_pool_id",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn get_deepbook_pool_margin_pool_ids(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::get_deepbook_pool_margin_pool_ids",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn get_margin_manager_ids(
        &self,
        tx: &mut Transaction,
        owner: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let owner_arg = tx.pure_bytes(&encode_address(owner));

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::get_margin_manager_ids",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, owner_arg],
            vec![],
        ))
    }

    pub fn base_margin_pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::base_margin_pool_id",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn quote_margin_pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::quote_margin_pool_id",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn min_withdraw_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::min_withdraw_risk_ratio",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn min_borrow_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::min_borrow_risk_ratio",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn liquidation_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::liquidation_risk_ratio",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn target_liquidation_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::target_liquidation_risk_ratio",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn user_liquidation_reward(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::user_liquidation_reward",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn pool_liquidation_reward(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::pool_liquidation_reward",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn allowed_maintainers(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::allowed_maintainers",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj],
            vec![],
        ))
    }

    pub fn allowed_pause_caps(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, MarginRegistryError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_registry::allowed_pause_caps",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj],
            vec![],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> DeepBookConfig {
        DeepBookConfig::default()
    }

    #[test]
    fn test_pool_enabled_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.pool_enabled(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_margin_pool_id_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.get_margin_pool_id(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_deepbook_pool_margin_pool_ids_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.get_deepbook_pool_margin_pool_ids(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_margin_manager_ids_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.get_margin_manager_ids(&mut tx, "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_base_margin_pool_id_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.base_margin_pool_id(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_quote_margin_pool_id_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.quote_margin_pool_id(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_withdraw_risk_ratio_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.min_withdraw_risk_ratio(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_borrow_risk_ratio_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.min_borrow_risk_ratio(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidation_risk_ratio_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.liquidation_risk_ratio(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_target_liquidation_risk_ratio_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.target_liquidation_risk_ratio(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_liquidation_reward_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.user_liquidation_reward(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_pool_liquidation_reward_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.pool_liquidation_reward(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_allowed_maintainers_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.allowed_maintainers(&mut tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_allowed_pause_caps_build() {
        let config = test_config();
        let contract = MarginRegistryContract::new(&config);
        let mut tx = Transaction::new();
        let result = contract.allowed_pause_caps(&mut tx);
        assert!(result.is_ok());
    }
}
