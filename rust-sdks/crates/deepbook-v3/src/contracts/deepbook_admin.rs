use crate::config::DeepBookConfig;
use crate::config::FLOAT_SCALAR;

#[derive(Debug, thiserror::Error)]
pub enum DeepBookAdminError {
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("config error: {0}")]
    Config(String),
}

pub struct DeepBookAdminContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> DeepBookAdminContract<'a> {
    pub fn create_pool_admin(
        &self,
        base_coin_key: &str,
        quote_coin_key: &str,
        tick_size: f64,
        lot_size: f64,
        min_size: f64,
        whitelisted: bool,
        stable_pool: bool,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), DeepBookAdminError> {
        let base = self
            .config
            .get_coin(base_coin_key)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(quote_coin_key)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;

        let adjusted_tick_size =
            ((tick_size * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let adjusted_lot_size = (lot_size * base.scalar as f64).round() as u64;
        let adjusted_min_size = (min_size * base.scalar as f64).round() as u64;

        let target = format!(
            "{}::pool::create_pool_admin",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.registry_id),
                serde_json::json!(adjusted_tick_size),
                serde_json::json!(adjusted_lot_size),
                serde_json::json!(adjusted_min_size),
                serde_json::json!(whitelisted),
                serde_json::json!(stable_pool),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unregister_pool_admin(
        &self,
        pool_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), DeepBookAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| DeepBookAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::pool::unregister_pool_admin",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(self.config.package_ids.registry_id),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_allowed_versions(
        &self,
        pool_key: &str,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), DeepBookAdminError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| DeepBookAdminError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| DeepBookAdminError::CoinNotFound(e.to_string()))?;

        let target = format!(
            "{}::pool::update_allowed_versions",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(self.config.package_ids.registry_id),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn enable_version(
        &self,
        version: u64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), DeepBookAdminError> {
        let target = format!(
            "{}::registry::enable_version",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.registry_id),
                serde_json::json!(version),
            ],
            vec![],
        ))
    }

    pub fn disable_version(
        &self,
        version: u64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), DeepBookAdminError> {
        let target = format!(
            "{}::registry::disable_version",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(self.config.package_ids.registry_id),
                serde_json::json!(version),
            ],
            vec![],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepbook_admin_contract() {
        let config = DeepBookConfig::default();
        let admin = DeepBookAdminContract { config: &config };
        assert_eq!(admin.config.network, "testnet");
    }

    #[test]
    fn test_create_pool_admin() {
        let config = DeepBookConfig::default();
        let admin = DeepBookAdminContract { config: &config };
        let result = admin.create_pool_admin("DEEP", "SUI", 0.0001, 0.01, 0.1, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unregister_pool_admin() {
        let config = DeepBookConfig::default();
        let admin = DeepBookAdminContract { config: &config };
        let result = admin.unregister_pool_admin("DEEP_SUI");
        assert!(result.is_ok());
    }
}
