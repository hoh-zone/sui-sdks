use crate::config::DeepBookConfig;
use crate::config::{FLOAT_SCALAR, MAX_TIMESTAMP};
use crate::encode::{encode_bool, encode_u64, encode_u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    NoRestriction = 0,
    ImmediateOrCancel = 1,
    FillOrKill = 2,
    PostOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfMatchingOptions {
    SelfMatchingAllowed = 0,
    CancelTaker = 1,
    CancelMaker = 2,
}

#[derive(Debug, thiserror::Error)]
pub enum TpslError {
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("margin manager not found: {0}")]
    MarginManagerNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
    #[error("config error: {0}")]
    Config(String),
}

pub struct TpslContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> TpslContract<'a> {
    pub fn new_condition(
        &self,
        pool_key: &str,
        trigger_below_price: bool,
        trigger_price: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let input_price = ((trigger_price * FLOAT_SCALAR * quote.scalar as f64)
            / base.scalar as f64)
            .round() as u64;

        let target = format!(
            "{}::tpsl::new_condition",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(trigger_below_price),
                serde_json::json!(input_price),
            ],
            vec![],
        ))
    }

    pub fn new_pending_limit_order(
        &self,
        pool_key: &str,
        client_order_id: u64,
        order_type: OrderType,
        self_matching_option: SelfMatchingOptions,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: Option<u64>,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let expiration = expire_timestamp.unwrap_or(MAX_TIMESTAMP);

        let target = format!(
            "{}::tpsl::new_pending_limit_order",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(client_order_id),
                serde_json::json!(order_type as u8),
                serde_json::json!(self_matching_option as u8),
                serde_json::json!(input_price),
                serde_json::json!(input_quantity),
                serde_json::json!(is_bid),
                serde_json::json!(pay_with_deep),
                serde_json::json!(expiration),
            ],
            vec![],
        ))
    }

    pub fn new_pending_market_order(
        &self,
        pool_key: &str,
        client_order_id: u64,
        self_matching_option: SelfMatchingOptions,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let target = format!(
            "{}::tpsl::new_pending_market_order",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(client_order_id),
                serde_json::json!(self_matching_option as u8),
                serde_json::json!(input_quantity),
                serde_json::json!(is_bid),
                serde_json::json!(pay_with_deep),
            ],
            vec![],
        ))
    }

    pub fn add_conditional_order(
        &self,
        margin_manager_key: &str,
        pool_key: &str,
        conditional_order_id: u64,
        trigger_below_price: bool,
        trigger_price: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::add_conditional_order",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(margin_manager_key),
                serde_json::json!(conditional_order_id),
                serde_json::json!(trigger_below_price),
                serde_json::json!(trigger_price),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_conditional_order(
        &self,
        margin_manager_key: &str,
        conditional_order_id: u64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let manager = self
            .config
            .get_margin_manager(margin_manager_key)
            .map_err(|e| TpslError::MarginManagerNotFound(e.to_string()))?;
        let pool = self
            .config
            .get_pool(&manager.pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::cancel_conditional_order",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(manager.address),
                serde_json::json!(conditional_order_id),
                serde_json::json!("0x6"), // clock
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_all_conditional_orders(
        &self,
        margin_manager_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let manager = self
            .config
            .get_margin_manager(margin_manager_key)
            .map_err(|e| TpslError::MarginManagerNotFound(e.to_string()))?;
        let pool = self
            .config
            .get_pool(&manager.pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::cancel_all_conditional_orders",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(manager.address),
                serde_json::json!("0x6"), // clock
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn execute_conditional_orders(
        &self,
        manager_address: &str,
        pool_key: &str,
        max_orders_to_execute: u64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::execute_conditional_orders",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(manager_address),
                serde_json::json!(pool.address),
                serde_json::json!(max_orders_to_execute),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn conditional_order_ids(
        &self,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::conditional_order_ids",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![serde_json::json!(margin_manager_id)],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn conditional_order(
        &self,
        pool_key: &str,
        margin_manager_id: &str,
        conditional_order_id: u64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::conditional_order",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(margin_manager_id),
                serde_json::json!(conditional_order_id),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn lowest_trigger_above_price(
        &self,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::lowest_trigger_above_price",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![serde_json::json!(margin_manager_id)],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn highest_trigger_below_price(
        &self,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), TpslError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| TpslError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| TpslError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_manager::highest_trigger_below_price",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![serde_json::json!(margin_manager_id)],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> DeepBookConfig {
        let mut config = DeepBookConfig::default();
        let margin_managers = std::collections::HashMap::new();
        config.margin_managers = margin_managers;
        config
    }

    #[test]
    fn test_tpsl_contract_create() {
        let config = create_test_config();
        let tpsl = TpslContract { config: &config };
        assert_eq!(tpsl.config.network, "testnet");
    }

    #[test]
    fn test_new_condition() {
        let config = create_test_config();
        let tpsl = TpslContract { config: &config };
        let result = tpsl.new_condition("DEEP_SUI", true, 1.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_pending_limit_order() {
        let config = create_test_config();
        let tpsl = TpslContract { config: &config };
        let result = tpsl.new_pending_limit_order(
            "DEEP_SUI",
            1,
            OrderType::NoRestriction,
            SelfMatchingOptions::SelfMatchingAllowed,
            1.0,
            10.0,
            true,
            true,
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_pending_market_order() {
        let config = create_test_config();
        let tpsl = TpslContract { config: &config };
        let result = tpsl.new_pending_market_order(
            "DEEP_SUI",
            1,
            SelfMatchingOptions::SelfMatchingAllowed,
            10.0,
            true,
            true,
        );
        assert!(result.is_ok());
    }
}
