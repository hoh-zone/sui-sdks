use crate::config::{ConfigError, DeepBookConfig, FLOAT_SCALAR, MAX_TIMESTAMP};
use crate::encode::{encode_bool, encode_u64, encode_u8};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum PoolProxyError {
    #[error("config error: {0}")]
    Config(String),
    #[error("margin manager not found: {0}")]
    MarginManagerNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
}

impl From<ConfigError> for PoolProxyError {
    fn from(e: ConfigError) -> Self {
        PoolProxyError::Config(e.to_string())
    }
}

pub struct PoolProxyContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> PoolProxyContract<'a> {
    pub fn place_limit_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        client_order_id: u64,
        order_type: u8,
        self_matching_option: u8,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expiration: u64,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let order_type_arg = tx.pure_bytes(&encode_u8(order_type));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let price_arg = tx.pure_bytes(&encode_u64(input_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expiration_arg = tx.pure_bytes(&encode_u64(expiration));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::place_limit_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                client_order_id_arg,
                order_type_arg,
                self_matching_option_arg,
                price_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                expiration_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn place_market_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        client_order_id: u64,
        self_matching_option: u8,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::place_market_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                client_order_id_arg,
                self_matching_option_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn place_reduce_only_limit_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        client_order_id: u64,
        order_type: u8,
        self_matching_option: u8,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expiration: u64,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let margin_pool_key = if is_bid {
            &pool.base_coin
        } else {
            &pool.quote_coin
        };
        let margin_pool = self.config.get_margin_pool(margin_pool_key)?;
        let debt_type = if is_bid {
            &base.type_tag
        } else {
            &quote.type_tag
        };
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let order_type_arg = tx.pure_bytes(&encode_u8(order_type));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let price_arg = tx.pure_bytes(&encode_u64(input_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expiration_arg = tx.pure_bytes(&encode_u64(expiration));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::place_reduce_only_limit_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                margin_pool_obj,
                client_order_id_arg,
                order_type_arg,
                self_matching_option_arg,
                price_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                expiration_arg,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                debt_type.clone(),
            ],
        ))
    }

    pub fn place_reduce_only_market_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        client_order_id: u64,
        self_matching_option: u8,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let margin_pool_key = if is_bid {
            &pool.base_coin
        } else {
            &pool.quote_coin
        };
        let margin_pool = self.config.get_margin_pool(margin_pool_key)?;
        let debt_type = if is_bid {
            &base.type_tag
        } else {
            &quote.type_tag
        };
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::place_reduce_only_market_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                margin_pool_obj,
                client_order_id_arg,
                self_matching_option_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                debt_type.clone(),
            ],
        ))
    }

    pub fn modify_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        order_id: u128,
        new_quantity: f64,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let input_quantity = (new_quantity * base.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_id_arg = tx.pure_bytes(&crate::encode::encode_u128(order_id));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::modify_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                order_id_arg,
                quantity_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        order_id: u128,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_id_arg = tx.pure_bytes(&crate::encode::encode_u128(order_id));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::cancel_order",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj, order_id_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_orders(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        order_ids: &[u128],
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_ids_arg = tx.pure_bytes(&crate::encode::encode_vec_u128(order_ids));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::cancel_orders",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                order_ids_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_all_orders(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::cancel_all_orders",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_settled_amounts(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::withdraw_settled_amounts",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn stake(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        stake_amount: f64,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let deep = self.config.get_coin("DEEP")?;
        let stake_input = (stake_amount * deep.scalar as f64).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let stake_arg = tx.pure_bytes(&encode_u64(stake_input));

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::stake",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj, stake_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unstake(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::unstake",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn submit_proposal(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        taker_fee: f64,
        maker_fee: f64,
        stake_required: f64,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let stake_input = (stake_required * FLOAT_SCALAR).round() as u64;
        let taker_fee_input = (taker_fee * FLOAT_SCALAR).round() as u64;
        let maker_fee_input = (maker_fee * FLOAT_SCALAR).round() as u64;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let taker_fee_arg = tx.pure_bytes(&encode_u64(taker_fee_input));
        let maker_fee_arg = tx.pure_bytes(&encode_u64(maker_fee_input));
        let stake_arg = tx.pure_bytes(&encode_u64(stake_input));

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::submit_proposal",
                self.config.package_ids.margin_package_id
            ),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
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
        margin_manager_key: &str,
        proposal_id: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let proposal_id_arg = tx.pure_bytes(&encode_u64(0u64));

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::vote",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj, proposal_id_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn claim_rebate(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::claim_rebate",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_margin_settled_amounts(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, PoolProxyError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(margin_manager_id.to_string());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool_proxy::withdraw_settled_amounts_permissionless",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> DeepBookConfig {
        let mut config = DeepBookConfig::default();
        use crate::types::MarginManager;
        let mut mgrs = std::collections::HashMap::new();
        mgrs.insert(
            "mgr1".to_string(),
            MarginManager {
                address: "0x123".to_string(),
                pool_key: "DEEP_SUI".to_string(),
            },
        );
        config.margin_managers = mgrs;
        config
    }

    #[test]
    fn test_place_limit_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.place_limit_order(
            &mut tx,
            "mgr1",
            1,
            0,
            0,
            1.0,
            10.0,
            true,
            true,
            MAX_TIMESTAMP,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_place_market_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.place_market_order(&mut tx, "mgr1", 1, 0, 10.0, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_place_reduce_only_limit_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.place_reduce_only_limit_order(
            &mut tx,
            "mgr1",
            1,
            0,
            0,
            1.0,
            10.0,
            true,
            true,
            MAX_TIMESTAMP,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_place_reduce_only_market_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result =
            contract.place_reduce_only_market_order(&mut tx, "mgr1", 1, 0, 10.0, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_modify_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.modify_order(&mut tx, "mgr1", 12345, 5.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancel_order_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.cancel_order(&mut tx, "mgr1", 12345);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancel_orders_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.cancel_orders(&mut tx, "mgr1", &[1, 2, 3]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancel_all_orders_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.cancel_all_orders(&mut tx, "mgr1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_settled_amounts_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_settled_amounts(&mut tx, "mgr1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_stake_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.stake(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unstake_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.unstake(&mut tx, "mgr1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_proposal_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.submit_proposal(&mut tx, "mgr1", 0.001, 0.001, 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vote_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.vote(&mut tx, "mgr1", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_claim_rebate_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.claim_rebate(&mut tx, "mgr1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_margin_settled_amounts_build() {
        let config = test_config();
        let contract = PoolProxyContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_margin_settled_amounts(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }
}
