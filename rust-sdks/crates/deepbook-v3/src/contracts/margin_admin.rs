use crate::config::DeepBookConfig;
use crate::config::FLOAT_SCALAR;

#[derive(Debug, thiserror::Error)]
pub enum MarginAdminError {
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("config error: {0}")]
    Config(String),
}

pub struct MarginAdminContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginAdminContract<'a> {
    pub fn mint_maintainer_cap(
        &self,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), MarginAdminError> {
        let target = format!(
            "{}::margin_registry::mint_maintainer_cap",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.margin_registry_id),
                serde_json::json!("0x6"), // clock
            ],
            vec![],
        ))
    }

    pub fn register_deepbook_pool(
        &self,
        pool_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), MarginAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| MarginAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_registry::register_deepbook_pool",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.margin_registry_id),
                serde_json::json!(pool.address),
                serde_json::json!("0x6"), // config placeholder
                serde_json::json!("0x6"), // clock placeholder
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn enable_deepbook_pool(
        &self,
        pool_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), MarginAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| MarginAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_registry::enable_deepbook_pool",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.margin_registry_id),
                serde_json::json!(pool.address),
                serde_json::json!("0x6"), // clock
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn disable_deepbook_pool(
        &self,
        pool_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), MarginAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| MarginAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_registry::disable_deepbook_pool",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.margin_registry_id),
                serde_json::json!(pool.address),
                serde_json::json!("0x6"), // clock
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_risk_params(
        &self,
        pool_key: &str,
        min_withdraw_risk_ratio: f64,
        min_borrow_risk_ratio: f64,
        liquidation_risk_ratio: f64,
        target_liquidation_risk_ratio: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), MarginAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| MarginAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| MarginAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::margin_registry::update_risk_params",
            self.config.package_ids.margin_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.margin_registry_id),
                serde_json::json!(pool.address),
                serde_json::json!("0x6"), // config placeholder
                serde_json::json!("0x6"), // clock placeholder
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_admin_contract() {
        let config = DeepBookConfig::default();
        let admin = MarginAdminContract { config: &config };
        assert_eq!(admin.config.network, "testnet");
    }

    #[test]
    fn test_register_deepbook_pool() {
        let config = DeepBookConfig::default();
        let admin = MarginAdminContract { config: &config };
        let result = admin.register_deepbook_pool("DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_enable_deepbook_pool() {
        let config = DeepBookConfig::default();
        let admin = MarginAdminContract { config: &config };
        let result = admin.enable_deepbook_pool("DEEP_SUI");
        assert!(result.is_ok());
    }
}
