use crate::config::{ConfigError, DeepBookConfig};
use crate::encode::{encode_option_u64, encode_u64};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum MarginPoolError {
    #[error("config error: {0}")]
    Config(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
}

impl From<ConfigError> for MarginPoolError {
    fn from(e: ConfigError) -> Self {
        MarginPoolError::Config(e.to_string())
    }
}

pub struct MarginPoolContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginPoolContract<'a> {
    pub fn mint_supplier_cap(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::mint_supplier_cap",
                self.config.package_ids.margin_package_id
            ),
            vec![registry_obj, clock_obj],
            vec![],
        ))
    }

    pub fn supply(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let deposit_input = (amount * coin.scalar as f64).round() as u64;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let amount_arg = tx.pure_bytes(&encode_u64(deposit_input));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::supply",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, registry_obj, amount_arg, clock_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn withdraw(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        amount: Option<f64>,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let coin = self.config.get_coin(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let withdraw_input = amount.map(|a| (a * coin.scalar as f64).round() as u64);
        let amount_arg = tx.pure_bytes(&encode_option_u64(withdraw_input));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::withdraw",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, registry_obj, amount_arg, clock_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn mint_supply_referral(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::mint_supply_referral",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, registry_obj, clock_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn withdraw_referral_fees(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let referral_obj = tx.object(referral_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::withdraw_referral_fees",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, registry_obj, referral_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn id(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::id",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn deepbook_pool_allowed(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        deepbook_pool_id: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let pool_obj = tx.object(deepbook_pool_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::deepbook_pool_allowed",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn total_supply(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::total_supply",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn supply_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::supply_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn total_borrow(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::total_borrow",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn borrow_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::borrow_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn last_update_timestamp(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::last_update_timestamp",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn supply_cap(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::supply_cap",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn max_utilization_rate(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::max_utilization_rate",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn protocol_spread(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::protocol_spread",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn min_borrow(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::min_borrow",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn interest_rate(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::interest_rate",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn user_supply_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supplier_cap_id: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let cap_obj = tx.object(supplier_cap_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::user_supply_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, cap_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn user_supply_amount(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supplier_cap_id: &str,
    ) -> Result<serde_json::Value, MarginPoolError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;

        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let cap_obj = tx.object(supplier_cap_id.to_string());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::user_supply_amount",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, cap_obj, clock_obj],
            vec![margin_pool.type_tag.clone()],
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
    fn test_mint_supplier_cap_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.mint_supplier_cap(&mut tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_supply_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.supply(&mut tx, "SUI", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw(&mut tx, "SUI", Some(50.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_all_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw(&mut tx, "SUI", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mint_supply_referral_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.mint_supply_referral(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_referral_fees_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_referral_fees(&mut tx, "SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_id_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.id(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_deepbook_pool_allowed_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deepbook_pool_allowed(&mut tx, "SUI", "0x456");
        assert!(result.is_ok());
    }

    #[test]
    fn test_total_supply_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.total_supply(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_supply_shares_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.supply_shares(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_total_borrow_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.total_borrow(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow_shares_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.borrow_shares(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_last_update_timestamp_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.last_update_timestamp(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_supply_cap_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.supply_cap(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_utilization_rate_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.max_utilization_rate(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_protocol_spread_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.protocol_spread(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_borrow_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.min_borrow(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_interest_rate_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.interest_rate(&mut tx, "SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_supply_shares_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.user_supply_shares(&mut tx, "SUI", "0x789");
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_supply_amount_build() {
        let config = test_config();
        let contract = MarginPoolContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.user_supply_amount(&mut tx, "SUI", "0x789");
        assert!(result.is_ok());
    }
}
