use crate::config::DeepBookConfig;
use crate::encode::encode_u64;

#[derive(Debug, thiserror::Error)]
pub enum FlashLoansError {
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
}

pub struct FlashLoansContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> FlashLoansContract<'a> {
    pub fn borrow_base_asset(
        &self,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), FlashLoansError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| FlashLoansError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;

        let input_quantity = (borrow_amount * base.scalar as f64).round() as u64;

        let target = format!(
            "{}::pool::borrow_flashloan_base",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(input_quantity),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn return_base_asset(
        &self,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), FlashLoansError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| FlashLoansError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;

        let input_quantity = (borrow_amount * base.scalar as f64).round() as u64;

        let target = format!(
            "{}::pool::return_flashloan_base",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(input_quantity),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrow_quote_asset(
        &self,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), FlashLoansError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| FlashLoansError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;

        let input_quantity = (borrow_amount * quote.scalar as f64).round() as u64;

        let target = format!(
            "{}::pool::borrow_flashloan_quote",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(input_quantity),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn return_quote_asset(
        &self,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<(String, Vec<serde_json::Value>, Vec<String>), FlashLoansError> {
        let pool = self
            .config
            .get_pool(pool_key)
            .map_err(|e| FlashLoansError::PoolNotFound(e.to_string()))?;
        let base = self
            .config
            .get_coin(&pool.base_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;
        let quote = self
            .config
            .get_coin(&pool.quote_coin)
            .map_err(|e| FlashLoansError::CoinNotFound(e.to_string()))?;

        let input_quantity = (borrow_amount * quote.scalar as f64).round() as u64;

        let target = format!(
            "{}::pool::return_flashloan_quote",
            self.config.package_ids.deepbook_package_id
        );
        Ok((
            target,
            vec![
                serde_json::json!(pool.address),
                serde_json::json!(input_quantity),
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_loans_contract() {
        let config = DeepBookConfig::default();
        let flash_loans = FlashLoansContract { config: &config };
        assert_eq!(flash_loans.config.network, "testnet");
    }

    #[test]
    fn test_borrow_base_asset() {
        let config = DeepBookConfig::default();
        let flash_loans = FlashLoansContract { config: &config };
        let result = flash_loans.borrow_base_asset("DEEP_SUI", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow_quote_asset() {
        let config = DeepBookConfig::default();
        let flash_loans = FlashLoansContract { config: &config };
        let result = flash_loans.borrow_quote_asset("DEEP_SUI", 100.0);
        assert!(result.is_ok());
    }
}
