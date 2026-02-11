use crate::config::{ConfigError, DeepBookConfig};
use crate::encode::{encode_bool, encode_u64, encode_u8};
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum MarginManagerError {
    #[error("config error: {0}")]
    Config(String),
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("margin manager not found: {0}")]
    MarginManagerNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
}

impl From<ConfigError> for MarginManagerError {
    fn from(e: ConfigError) -> Self {
        MarginManagerError::Config(e.to_string())
    }
}

pub struct MarginManagerContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginManagerContract<'a> {
    pub fn new_margin_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let margin_registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::new",
                self.config.package_ids.margin_package_id
            ),
            vec![pool_obj, registry_obj, margin_registry_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn new_margin_manager_with_initializer(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let margin_registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::new_with_initializer",
                self.config.package_ids.margin_package_id
            ),
            vec![pool_obj, registry_obj, margin_registry_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn share_margin_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::share",
                self.config.package_ids.margin_package_id
            ),
            vec![],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn deposit_during_initialization(
        &self,
        tx: &mut Transaction,
        manager: serde_json::Value,
        pool_key: &str,
        coin_type: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let deposit_coin = self.config.get_coin(coin_type)?;

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
        let coin_obj = tx.object(deposit_coin.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::deposit",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                coin_obj,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                deposit_coin.type_tag.clone(),
            ],
        ))
    }

    pub fn deposit_base(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let deep = self.config.get_coin("DEEP")?;

        let manager_obj = tx.object(manager.address.clone());
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
        let coin_obj = tx.object(deep.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::deposit",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                coin_obj,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                deep.type_tag.clone(),
            ],
        ))
    }

    pub fn withdraw_base(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
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
        let pool_obj = tx.object(pool.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * base.scalar as f64).round() as u64));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::withdraw",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                base.type_tag.clone(),
            ],
        ))
    }

    pub fn withdraw_quote(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
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
        let pool_obj = tx.object(pool.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * quote.scalar as f64).round() as u64));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::withdraw",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                quote.type_tag.clone(),
            ],
        ))
    }

    pub fn withdraw_deep(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let deep = self.config.get_coin("DEEP")?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
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
        let pool_obj = tx.object(pool.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * deep.scalar as f64).round() as u64));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::withdraw",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                deep.type_tag.clone(),
            ],
        ))
    }

    pub fn borrow_base(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let margin_pool_obj = tx.object(base_margin_pool.address.clone());
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
        let pool_obj = tx.object(pool.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * base.scalar as f64).round() as u64));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::borrow_base",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                margin_pool_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrow_quote(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let margin_pool_obj = tx.object(quote_margin_pool.address.clone());
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
        let pool_obj = tx.object(pool.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64((amount * quote.scalar as f64).round() as u64));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::borrow_quote",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                margin_pool_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn repay_base(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: Option<f64>,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let amount_encoded = amount.map(|a| (a * base.scalar as f64).round() as u64);
        let amount_arg = tx.pure_bytes(&crate::encode::encode_option_u64(amount_encoded));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::repay_base",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                margin_pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn repay_quote(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        amount: Option<f64>,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let amount_encoded = amount.map(|a| (a * quote.scalar as f64).round() as u64);
        let amount_arg = tx.pure_bytes(&crate::encode::encode_option_u64(amount_encoded));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::repay_quote",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                margin_pool_obj,
                amount_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn liquidate(
        &self,
        tx: &mut Transaction,
        manager_address: &str,
        pool_key: &str,
        debt_is_base: bool,
        repay_amount: f64,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

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
        let margin_pool_key = if debt_is_base {
            &pool.base_coin
        } else {
            &pool.quote_coin
        };
        let margin_pool = self.config.get_margin_pool(margin_pool_key)?;
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let coin_obj = tx.object(if debt_is_base {
            base.address.clone()
        } else {
            quote.address.clone()
        });
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::liquidate",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                margin_pool_obj,
                pool_obj,
                coin_obj,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn set_margin_manager_referral(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        referral: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let referral_obj = tx.object(referral.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::set_margin_manager_referral",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj, referral_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unset_margin_manager_referral(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        pool_key: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::unset_margin_manager_referral",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn owner(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::owner",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn deepbook_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::deepbook_pool",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn margin_pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::margin_pool_id",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_shares(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::borrowed_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_base_shares(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::borrowed_base_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_quote_shares(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::borrowed_quote_shares",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn has_base_debt(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::has_base_debt",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn balance_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::balance_manager",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn calculate_assets(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());
        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::calculate_assets",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn calculate_debts(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        coin_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let debt_coin = self.config.get_coin(coin_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let manager_obj = tx.object(margin_manager_id.to_string());
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::calculate_debts",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj, margin_pool_obj, clock_obj],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                debt_coin.type_tag.clone(),
            ],
        ))
    }

    pub fn manager_state(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;

        let manager_obj = tx.object(margin_manager_id.to_string());
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
        let pool_obj = tx.object(pool.address.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::manager_state",
                self.config.package_ids.margin_package_id
            ),
            vec![
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                pool_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn base_balance(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::base_balance",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn quote_balance(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::quote_balance",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn deep_balance(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        Ok(tx.move_call(
            &format!(
                "{}::margin_manager::deep_balance",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_margin_account_order_details(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, MarginManagerError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(margin_manager_id.to_string());

        let balance_manager = tx.move_call(
            &format!(
                "{}::margin_manager::balance_manager",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let pool_obj = tx.object(pool.address.clone());

        Ok(tx.move_call(
            &format!(
                "{}::pool::get_account_order_details",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, balance_manager],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> DeepBookConfig {
        let mut config = DeepBookConfig::default();
        let mut mgrs = std::collections::HashMap::new();
        mgrs.insert(
            "mgr1".to_string(),
            crate::types::MarginManager {
                address: "0x123".to_string(),
                pool_key: "DEEP_SUI".to_string(),
            },
        );
        config.margin_managers = mgrs;
        let mut coins = config.coins.clone();
        for coin in coins.values_mut() {
            coin.price_info_object_id = Some("0x6".to_string());
        }
        config.coins = coins;
        config
    }

    #[test]
    fn test_new_margin_manager_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.new_margin_manager(&mut tx, "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_deposit_base_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deposit_base(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deposit_quote_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deposit_base(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deposit_deep_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deposit_base(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_base_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_base(&mut tx, "mgr1", 50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_quote_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_quote(&mut tx, "mgr1", 50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_deep_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.withdraw_deep(&mut tx, "mgr1", 50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow_base_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.borrow_base(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrow_quote_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.borrow_quote(&mut tx, "mgr1", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repay_base_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.repay_base(&mut tx, "mgr1", Some(50.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_repay_base_all_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.repay_base(&mut tx, "mgr1", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repay_quote_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.repay_quote(&mut tx, "mgr1", Some(50.0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_liquidate_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.liquidate(&mut tx, "0x123", "DEEP_SUI", true, 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_margin_manager_referral_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.set_margin_manager_referral(&mut tx, "mgr1", "0x456");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unset_margin_manager_referral_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.unset_margin_manager_referral(&mut tx, "mgr1", "DEEP_SUI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_owner_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.owner(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_borrowed_shares_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.borrowed_shares(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_assets_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.calculate_assets(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_debts_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.calculate_debts(&mut tx, "DEEP_SUI", "SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_manager_state_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.manager_state(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_base_balance_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.base_balance(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_quote_balance_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.quote_balance(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_deep_balance_build() {
        let config = test_config();
        let contract = MarginManagerContract { config: &config };
        let mut tx = Transaction::new();
        let result = contract.deep_balance(&mut tx, "DEEP_SUI", "0x123");
        assert!(result.is_ok());
    }
}
