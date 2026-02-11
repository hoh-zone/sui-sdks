use crate::config::{ConfigError, DeepBookConfig};
use crate::encode::{encode_bool, encode_u64};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum MarginMaintainerError {
    #[error("config error: {0}")]
    Config(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("margin maintainer cap not set")]
    MaintainerCapNotSet,
}

impl From<ConfigError> for MarginMaintainerError {
    fn from(e: ConfigError) -> Self {
        MarginMaintainerError::Config(e.to_string())
    }
}

pub struct MarginMaintainerContract<'a> {
    pub config: &'a DeepBookConfig,
    pub margin_maintainer_cap: Option<String>,
}

impl<'a> MarginMaintainerContract<'a> {
    pub fn new(config: &'a DeepBookConfig, margin_maintainer_cap: Option<String>) -> Self {
        Self {
            config,
            margin_maintainer_cap,
        }
    }

    fn get_maintainer_cap(&self) -> Result<String, MarginMaintainerError> {
        self.margin_maintainer_cap
            .clone()
            .ok_or(MarginMaintainerError::MaintainerCapNotSet)
    }

    pub fn new_protocol_config(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_config: &MarginPoolConfigParams,
        interest_config: &InterestConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let margin_pool_config_obj = if margin_pool_config.rate_limit_capacity.is_some()
            && margin_pool_config.rate_limit_refill_rate_per_ms.is_some()
            && margin_pool_config.rate_limit_enabled.is_some()
        {
            self.new_margin_pool_config_with_rate_limit(tx, coin_key, margin_pool_config)?
        } else {
            self.new_margin_pool_config(tx, coin_key, margin_pool_config)?
        };
        let interest_config_obj = self.new_interest_config(tx, interest_config)?;

        Ok(tx.move_call(
            &format!(
                "{}::protocol_config::new_protocol_config",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_config_obj, interest_config_obj],
            vec![],
        ))
    }

    pub fn new_margin_pool_config(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_config: &MarginPoolConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let coin = self.config.get_coin(coin_key)?;
        let supply_cap_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.supply_cap * coin.scalar as f64).round() as u64,
        ));
        let max_utilization_rate_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.max_utilization_rate * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let referral_spread_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.referral_spread * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let min_borrow_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.min_borrow * coin.scalar as f64).round() as u64,
        ));

        Ok(tx.move_call(
            &format!(
                "{}::protocol_config::new_margin_pool_config",
                self.config.package_ids.margin_package_id
            ),
            vec![
                supply_cap_arg,
                max_utilization_rate_arg,
                referral_spread_arg,
                min_borrow_arg,
            ],
            vec![],
        ))
    }

    pub fn new_margin_pool_config_with_rate_limit(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_config: &MarginPoolConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let coin = self.config.get_coin(coin_key)?;
        let supply_cap_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.supply_cap * coin.scalar as f64).round() as u64,
        ));
        let max_utilization_rate_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.max_utilization_rate * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let referral_spread_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.referral_spread * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let min_borrow_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config.min_borrow * coin.scalar as f64).round() as u64,
        ));
        let rate_limit_capacity_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config
                .rate_limit_capacity
                .expect("rate_limit_capacity required")
                * coin.scalar as f64)
                .round() as u64,
        ));
        let rate_limit_refill_rate_per_ms_arg = tx.pure_bytes(&encode_u64(
            (margin_pool_config
                .rate_limit_refill_rate_per_ms
                .expect("rate_limit_refill_rate_per_ms required")
                * coin.scalar as f64)
                .round() as u64,
        ));
        let rate_limit_enabled_arg = tx.pure_bytes(&encode_bool(
            margin_pool_config
                .rate_limit_enabled
                .expect("rate_limit_enabled required"),
        ));

        Ok(tx.move_call(
            &format!(
                "{}::protocol_config::new_margin_pool_config_with_rate_limit",
                self.config.package_ids.margin_package_id
            ),
            vec![
                supply_cap_arg,
                max_utilization_rate_arg,
                referral_spread_arg,
                min_borrow_arg,
                rate_limit_capacity_arg,
                rate_limit_refill_rate_per_ms_arg,
                rate_limit_enabled_arg,
            ],
            vec![],
        ))
    }

    pub fn new_interest_config(
        &self,
        tx: &mut Transaction,
        interest_config: &InterestConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let base_rate_arg = tx.pure_bytes(&encode_u64(
            (interest_config.base_rate * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let base_slope_arg = tx.pure_bytes(&encode_u64(
            (interest_config.base_slope * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let optimal_utilization_arg = tx.pure_bytes(&encode_u64(
            (interest_config.optimal_utilization * crate::config::FLOAT_SCALAR).round() as u64,
        ));
        let excess_slope_arg = tx.pure_bytes(&encode_u64(
            (interest_config.excess_slope * crate::config::FLOAT_SCALAR).round() as u64,
        ));

        Ok(tx.move_call(
            &format!(
                "{}::protocol_config::new_interest_config",
                self.config.package_ids.margin_package_id
            ),
            vec![
                base_rate_arg,
                base_slope_arg,
                optimal_utilization_arg,
                excess_slope_arg,
            ],
            vec![],
        ))
    }

    pub fn enable_deepbook_pool_for_loan(
        &self,
        tx: &mut Transaction,
        deepbook_pool_key: &str,
        coin_key: &str,
        margin_pool_cap: &str,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let deepbook_pool = self.config.get_pool(deepbook_pool_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let deepbook_pool_obj = tx.object(deepbook_pool.address.clone());
        let margin_pool_cap_obj = tx.object(margin_pool_cap.to_string());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::enable_deepbook_pool_for_loan",
                self.config.package_ids.margin_package_id
            ),
            vec![
                margin_pool_obj,
                registry_obj,
                deepbook_pool_obj,
                margin_pool_cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn disable_deepbook_pool_for_loan(
        &self,
        tx: &mut Transaction,
        deepbook_pool_key: &str,
        coin_key: &str,
        margin_pool_cap: &str,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let deepbook_pool = self.config.get_pool(deepbook_pool_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let deepbook_pool_obj = tx.object(deepbook_pool.address.clone());
        let margin_pool_cap_obj = tx.object(margin_pool_cap.to_string());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::disable_deepbook_pool_for_loan",
                self.config.package_ids.margin_package_id
            ),
            vec![
                margin_pool_obj,
                registry_obj,
                deepbook_pool_obj,
                margin_pool_cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn update_interest_params(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_cap: &str,
        interest_config: &InterestConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let interest_config_obj = self.new_interest_config(tx, interest_config)?;
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let margin_pool_cap_obj = tx.object(margin_pool_cap.to_string());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::update_interest_params",
                self.config.package_ids.margin_package_id
            ),
            vec![
                margin_pool_obj,
                registry_obj,
                interest_config_obj,
                margin_pool_cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn update_margin_pool_config(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_cap: &str,
        margin_pool_config: &MarginPoolConfigParams,
    ) -> Result<serde_json::Value, MarginMaintainerError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        if margin_pool_config.rate_limit_capacity.is_some()
            && margin_pool_config.rate_limit_refill_rate_per_ms.is_some()
            && margin_pool_config.rate_limit_enabled.is_some()
        {
            let margin_pool_config_obj =
                self.new_margin_pool_config_with_rate_limit(tx, coin_key, margin_pool_config)?;
            let margin_pool_obj = tx.object(margin_pool.address.clone());
            let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
            let margin_pool_cap_obj = tx.object(margin_pool_cap.to_string());
            let clock_obj = tx.object("0x6");

            Ok(tx.move_call(
                &format!(
                    "{}::margin_pool::update_margin_pool_config",
                    self.config.package_ids.margin_package_id
                ),
                vec![
                    margin_pool_obj,
                    registry_obj,
                    margin_pool_config_obj,
                    margin_pool_cap_obj,
                    clock_obj,
                ],
                vec![margin_pool.type_tag.clone()],
            ))
        } else {
            let margin_pool_config_obj =
                self.new_margin_pool_config(tx, coin_key, margin_pool_config)?;
            let margin_pool_obj = tx.object(margin_pool.address.clone());
            let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
            let margin_pool_cap_obj = tx.object(margin_pool_cap.to_string());
            let clock_obj = tx.object("0x6");

            Ok(tx.move_call(
                &format!(
                    "{}::margin_pool::update_margin_pool_config",
                    self.config.package_ids.margin_package_id
                ),
                vec![
                    margin_pool_obj,
                    registry_obj,
                    margin_pool_config_obj,
                    margin_pool_cap_obj,
                    clock_obj,
                ],
                vec![margin_pool.type_tag.clone()],
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarginPoolConfigParams {
    pub supply_cap: f64,
    pub max_utilization_rate: f64,
    pub referral_spread: f64,
    pub min_borrow: f64,
    pub rate_limit_capacity: Option<f64>,
    pub rate_limit_refill_rate_per_ms: Option<f64>,
    pub rate_limit_enabled: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InterestConfigParams {
    pub base_rate: f64,
    pub base_slope: f64,
    pub optimal_utilization: f64,
    pub excess_slope: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> DeepBookConfig {
        DeepBookConfig::default()
    }

    #[test]
    fn test_maintainer_constructor() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        assert_eq!(contract.margin_maintainer_cap, Some("cap_id".to_string()));
    }

    #[test]
    fn test_get_maintainer_cap_success() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        assert_eq!(contract.get_maintainer_cap().unwrap(), "cap_id");
    }

    #[test]
    fn test_get_maintainer_cap_not_set() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, None);
        assert!(contract.get_maintainer_cap().is_err());
    }

    #[test]
    fn test_new_margin_pool_config_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let margin_config = MarginPoolConfigParams {
            supply_cap: 10000.0,
            max_utilization_rate: 0.8,
            referral_spread: 0.05,
            min_borrow: 1.0,
            rate_limit_capacity: None,
            rate_limit_refill_rate_per_ms: None,
            rate_limit_enabled: None,
        };
        let result = contract.new_margin_pool_config(&mut tx, "SUI", &margin_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_margin_pool_config_with_rate_limit_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let margin_config = MarginPoolConfigParams {
            supply_cap: 10000.0,
            max_utilization_rate: 0.8,
            referral_spread: 0.05,
            min_borrow: 1.0,
            rate_limit_capacity: Some(1000.0),
            rate_limit_refill_rate_per_ms: Some(10.0),
            rate_limit_enabled: Some(true),
        };
        let result =
            contract.new_margin_pool_config_with_rate_limit(&mut tx, "SUI", &margin_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_interest_config_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let interest_config = InterestConfigParams {
            base_rate: 0.02,
            base_slope: 0.15,
            optimal_utilization: 0.8,
            excess_slope: 0.5,
        };
        let result = contract.new_interest_config(&mut tx, &interest_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_enable_deepbook_pool_for_loan_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let result =
            contract.enable_deepbook_pool_for_loan(&mut tx, "DEEP_SUI", "SUI", "pool_cap_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_disable_deepbook_pool_for_loan_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let result =
            contract.disable_deepbook_pool_for_loan(&mut tx, "DEEP_SUI", "SUI", "pool_cap_id");
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_protocol_config_without_rate_limit_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let margin_config = MarginPoolConfigParams {
            supply_cap: 10000.0,
            max_utilization_rate: 0.8,
            referral_spread: 0.05,
            min_borrow: 1.0,
            rate_limit_capacity: None,
            rate_limit_refill_rate_per_ms: None,
            rate_limit_enabled: None,
        };
        let interest_config = InterestConfigParams {
            base_rate: 0.02,
            base_slope: 0.15,
            optimal_utilization: 0.8,
            excess_slope: 0.5,
        };
        let result = contract.new_protocol_config(&mut tx, "SUI", &margin_config, &interest_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_protocol_config_with_rate_limit_build() {
        let config = test_config();
        let contract = MarginMaintainerContract::new(&config, Some("cap_id".to_string()));
        let mut tx = Transaction::new();
        let margin_config = MarginPoolConfigParams {
            supply_cap: 10000.0,
            max_utilization_rate: 0.8,
            referral_spread: 0.05,
            min_borrow: 1.0,
            rate_limit_capacity: Some(1000.0),
            rate_limit_refill_rate_per_ms: Some(10.0),
            rate_limit_enabled: Some(true),
        };
        let interest_config = InterestConfigParams {
            base_rate: 0.02,
            base_slope: 0.15,
            optimal_utilization: 0.8,
            excess_slope: 0.5,
        };
        let result = contract.new_protocol_config(&mut tx, "SUI", &margin_config, &interest_config);
        assert!(result.is_ok());
    }
}
