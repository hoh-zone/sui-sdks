use crate::config::{ConfigError, DeepBookConfig};
use crate::encode::{encode_option_u64, encode_u64};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum MarginLiquidationsError {
    #[error("config error: {0}")]
    Config(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
}

impl From<ConfigError> for MarginLiquidationsError {
    fn from(e: ConfigError) -> Self {
        MarginLiquidationsError::Config(e.to_string())
    }
}

pub struct MarginLiquidationsContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginLiquidationsContract<'a> {
    pub fn create_liquidation_vault(
        &self,
        tx: &mut Transaction,
        admin_cap: &str,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let admin_cap_obj = tx.object(admin_cap.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::create_liquidation_vault",
                self.config.package_ids.liquidation_package_id
            ),
            vec![admin_cap_obj],
            vec![],
        ))
    }

    pub fn deposit(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        admin_cap: &str,
        coin_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let coin = self.config.get_coin(coin_key)?;
        let vault_obj = tx.object(vault_id.to_string());
        let admin_cap_obj = tx.object(admin_cap.to_string());
        let coin_obj = tx.object(coin.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::deposit",
                self.config.package_ids.liquidation_package_id
            ),
            vec![vault_obj, admin_cap_obj, coin_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn withdraw(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        admin_cap: &str,
        coin_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let coin = self.config.get_coin(coin_key)?;
        let vault_obj = tx.object(vault_id.to_string());
        let admin_cap_obj = tx.object(admin_cap.to_string());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * coin.scalar as f64).round() as u64));

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::withdraw",
                self.config.package_ids.liquidation_package_id
            ),
            vec![vault_obj, admin_cap_obj, amount_arg],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn liquidate_base(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        manager_address: &str,
        pool_key: &str,
        repay_amount: Option<f64>,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let vault_obj = tx.object(vault_id.to_string());
        let manager_obj = tx.object(manager_address.to_string());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_price_obj = tx.object(
            base.price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let quote_price_obj = tx.object(
            quote
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let repay_amount_arg = tx.pure_bytes(&encode_option_u64(
            repay_amount.map(|a| (a * base.scalar as f64).round() as u64),
        ));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::liquidate_base",
                self.config.package_ids.liquidation_package_id
            ),
            vec![
                vault_obj,
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                pool_obj,
                repay_amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn liquidate_quote(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        manager_address: &str,
        pool_key: &str,
        repay_amount: Option<f64>,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let vault_obj = tx.object(vault_id.to_string());
        let manager_obj = tx.object(manager_address.to_string());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_price_obj = tx.object(
            base.price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let quote_price_obj = tx.object(
            quote
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let repay_amount_arg = tx.pure_bytes(&encode_option_u64(
            repay_amount.map(|a| (a * quote.scalar as f64).round() as u64),
        ));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::liquidate_quote",
                self.config.package_ids.liquidation_package_id
            ),
            vec![
                vault_obj,
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                pool_obj,
                repay_amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn balance(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginLiquidationsError> {
        let coin = self.config.get_coin(coin_key)?;
        let vault_obj = tx.object(vault_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::liquidation_vault::balance",
                self.config.package_ids.liquidation_package_id
            ),
            vec![vault_obj],
            vec![coin.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_config() -> DeepBookConfig {
        let mut config = DeepBookConfig::default();
        let mut coins = config.coins.clone();
        for coin in coins.values_mut() {
            coin.price_info_object_id = Some("0x6".to_string());
        }
        config.coins = coins;
        config
    }

    #[test]
    fn test_create_liquidation_vault_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.create_liquidation_vault(&mut tx, "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_deposit_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deposit(&mut tx, "0x123", "0x456", "SUI", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw(&mut tx, "0x123", "0x456", "SUI", 50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidate_base_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.liquidate_base(&mut tx, "0x123", "0x789", "DEEP_SUI", Some(100.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidate_base_full_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.liquidate_base(&mut tx, "0x123", "0x789", "DEEP_SUI", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidate_quote_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.liquidate_quote(&mut tx, "0x123", "0x789", "DEEP_SUI", Some(100.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidate_quote_full_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.liquidate_quote(&mut tx, "0x123", "0x789", "DEEP_SUI", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_balance_build() {
        let config = test_config();
        let contract = MarginLiquidationsContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.balance(&mut tx, "0x123", "SUI");
        assert!(result.is_ok());
    }
}
