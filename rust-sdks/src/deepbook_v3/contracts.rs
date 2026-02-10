use crate::deepbook_v3::config::{ConfigError, DeepBookConfig, FLOAT_SCALAR, MAX_TIMESTAMP};
use crate::deepbook_v3::encode::{encode_bool, encode_u64, encode_u128, encode_vec_u128};
use crate::sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error(transparent)]
    Config(#[from] ConfigError),
}

pub struct BalanceManagerContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> BalanceManagerContract<'a> {
    pub fn generate_proof(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        if let Some(trade_cap) = &manager.trade_cap {
            let manager_obj = tx.object(manager.address.clone());
            let trade_cap_obj = tx.object(trade_cap.clone());
            Ok(tx.move_call(
                &format!(
                    "{}::balance_manager::generate_proof_as_trader",
                    self.config.package_ids.deepbook_package_id
                ),
                vec![manager_obj, trade_cap_obj],
                vec![],
            ))
        } else {
            let manager_obj = tx.object(manager.address.clone());
            Ok(tx.move_call(
                &format!(
                    "{}::balance_manager::generate_proof_as_owner",
                    self.config.package_ids.deepbook_package_id
                ),
                vec![manager_obj],
                vec![],
            ))
        }
    }

    pub fn balance_manager_referral_owner(
        &self,
        tx: &mut Transaction,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::balance_manager_referral_owner",
                self.config.package_ids.deepbook_package_id
            ),
            vec![referral_obj],
            vec![],
        ))
    }

    pub fn balance_manager_referral_pool_id(
        &self,
        tx: &mut Transaction,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::balance_manager_referral_pool_id",
                self.config.package_ids.deepbook_package_id
            ),
            vec![referral_obj],
            vec![],
        ))
    }

    pub fn get_balance_manager_referral_id(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool = self.config.get_pool(pool_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::get_balance_manager_referral_id",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, pool_obj],
            vec![],
        ))
    }
}

pub struct DeepBookContract<'a> {
    pub config: &'a DeepBookConfig,
    pub balance_manager: BalanceManagerContract<'a>,
}

impl<'a> DeepBookContract<'a> {
    fn pool_types(
        &self,
        pool_key: &str,
    ) -> Result<
        (
            &crate::deepbook_v3::types::Pool,
            &crate::deepbook_v3::types::Coin,
            &crate::deepbook_v3::types::Coin,
        ),
        ContractError,
    > {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((pool, base, quote))
    }

    fn pool_target(&self, function: &str) -> String {
        format!("{}::pool::{function}", self.config.package_ids.deepbook_package_id)
    }

    pub fn get_quote_quantity_out(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let quantity = (base_quantity * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_quote_quantity_out"),
            vec![pool_obj, qty_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_base_quantity_out(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (quote_quantity * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_base_quantity_out"),
            vec![pool_obj, qty_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_quantity_out(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let base_qty = (base_quantity * base.scalar as f64).round() as u64;
        let quote_qty = (quote_quantity * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let base_arg = tx.pure_bytes(&encode_u64(base_qty));
        let quote_arg = tx.pure_bytes(&encode_u64(quote_qty));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_quantity_out"),
            vec![pool_obj, base_arg, quote_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_quote_quantity_out_input_fee(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (base_quantity * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_quote_quantity_out_input_fee"),
            vec![pool_obj, qty_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_base_quantity_out_input_fee(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (quote_quantity * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_base_quantity_out_input_fee"),
            vec![pool_obj, qty_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_quantity_out_input_fee(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let base_qty = (base_quantity * base.scalar as f64).round() as u64;
        let quote_qty = (quote_quantity * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let base_arg = tx.pure_bytes(&encode_u64(base_qty));
        let quote_arg = tx.pure_bytes(&encode_u64(quote_qty));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_quantity_out_input_fee"),
            vec![pool_obj, base_arg, quote_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_base_quantity_in(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        target_quote_quantity: f64,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (target_quote_quantity * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_base_quantity_in"),
            vec![pool_obj, qty_arg, pay_with_deep_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_quote_quantity_in(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        target_base_quantity: f64,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (target_base_quantity * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let qty_arg = tx.pure_bytes(&encode_u64(quantity));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("get_quote_quantity_in"),
            vec![pool_obj, qty_arg, pay_with_deep_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_level2_range(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        price_low: f64,
        price_high: f64,
        is_bid: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let price_low_encoded =
            ((price_low * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let price_high_encoded =
            ((price_high * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let price_low_arg = tx.pure_bytes(&encode_u64(price_low_encoded));
        let price_high_arg = tx.pure_bytes(&encode_u64(price_high_encoded));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));

        Ok(tx.move_call(
            &self.pool_target("get_level2_range"),
            vec![pool_obj, price_low_arg, price_high_arg, is_bid_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_level2_ticks_from_mid(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        tick_from_mid: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let tick_arg = tx.pure_bytes(&encode_u64(tick_from_mid));
        Ok(tx.move_call(
            &self.pool_target("get_level2_ticks_from_mid"),
            vec![pool_obj, tick_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_orders(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        order_ids: &[u128],
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self.balance_manager.generate_proof(tx, balance_manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let order_ids_arg = tx.pure_bytes(&encode_vec_u128(order_ids));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("cancel_orders"),
            vec![pool_obj, manager_obj, proof, order_ids_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn can_place_limit_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: Option<u64>,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let encoded_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let encoded_quantity = (quantity * base.scalar as f64).round() as u64;
        let expiration = expire_timestamp.unwrap_or(MAX_TIMESTAMP);
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let price_arg = tx.pure_bytes(&encode_u64(encoded_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(encoded_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expiration_arg = tx.pure_bytes(&encode_u64(expiration));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("can_place_limit_order"),
            vec![
                pool_obj,
                manager_obj,
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

    pub fn can_place_market_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let encoded_quantity = (quantity * base.scalar as f64).round() as u64;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let quantity_arg = tx.pure_bytes(&encode_u64(encoded_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("can_place_market_order"),
            vec![
                pool_obj,
                manager_obj,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn check_market_order_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let encoded_quantity = (quantity * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let quantity_arg = tx.pure_bytes(&encode_u64(encoded_quantity));
        Ok(tx.move_call(
            &self.pool_target("check_market_order_params"),
            vec![pool_obj, quantity_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn check_limit_order_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        price: f64,
        quantity: f64,
        expire_timestamp: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let encoded_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let encoded_quantity = (quantity * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let price_arg = tx.pure_bytes(&encode_u64(encoded_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(encoded_quantity));
        let expire_arg = tx.pure_bytes(&encode_u64(expire_timestamp));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("check_limit_order_params"),
            vec![pool_obj, price_arg, quantity_arg, expire_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        order_id: u128,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let order_id_arg = tx.pure_bytes(&encode_u128(order_id));
        Ok(tx.move_call(
            &self.pool_target("get_order"),
            vec![pool_obj, order_id_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_orders(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        order_ids: &[u128],
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let order_ids_arg = tx.pure_bytes(&encode_vec_u128(order_ids));
        Ok(tx.move_call(
            &self.pool_target("get_orders"),
            vec![pool_obj, order_ids_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn account_open_orders(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.pool_target("account_open_orders"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn vault_balances(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("vault_balances"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_pool_id_by_assets(
        &self,
        tx: &mut Transaction,
        base_type: &str,
        quote_type: &str,
    ) -> Result<serde_json::Value, ContractError> {
        Ok(tx.move_call(
            &self.pool_target("get_pool_id_by_asset"),
            vec![],
            vec![base_type.to_string(), quote_type.to_string()],
        ))
    }

    pub fn pool_trade_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("pool_trade_params"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn pool_book_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("pool_book_params"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn account(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.pool_target("account"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn locked_balance(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.pool_target("locked_balance"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_pool_deep_price(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("get_order_deep_price"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_account_order_details(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.pool_target("get_account_order_details"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_order_deep_required(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_quantity: f64,
        price: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let quantity = (base_quantity * base.scalar as f64).round() as u64;
        let encoded_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let quantity_arg = tx.pure_bytes(&encode_u64(quantity));
        let price_arg = tx.pure_bytes(&encode_u64(encoded_price));
        Ok(tx.move_call(
            &self.pool_target("get_order_deep_required"),
            vec![pool_obj, quantity_arg, price_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn pool_trade_params_next(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("pool_trade_params_next"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn account_exists(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.pool_target("account_exists"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn quorum(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("quorum"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("id"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn stable_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("stable_pool"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn registered_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("registered_pool"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_pool_referral_balances(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("get_pool_referral_balances"),
            vec![pool_obj, referral_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_balance_manager_ids(
        &self,
        tx: &mut Transaction,
        owner: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let owner_obj = tx.object(owner.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::registry::get_balance_manager_ids",
                self.config.package_ids.deepbook_package_id
            ),
            vec![registry_obj, owner_obj],
            vec![],
        ))
    }

    pub fn pool_referral_multiplier(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("pool_referral_multiplier"),
            vec![pool_obj, referral_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

pub struct MarginPoolContract<'a> {
    pub config: &'a DeepBookConfig,
}

pub struct MarginTPSLContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginTPSLContract<'a> {
    fn margin_manager_target(&self, function: &str) -> String {
        format!(
            "{}::margin_manager::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    fn manager_types(
        &self,
        manager_key: &str,
    ) -> Result<
        (
            &crate::deepbook_v3::types::MarginManager,
            &crate::deepbook_v3::types::Coin,
            &crate::deepbook_v3::types::Coin,
        ),
        ContractError,
    > {
        let manager = self.config.get_margin_manager(manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((manager, base, quote))
    }

    pub fn conditional_order_ids(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("conditional_order_ids"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn lowest_trigger_above_price(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("lowest_trigger_above_price"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn highest_trigger_below_price(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("highest_trigger_below_price"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

pub struct MarginRegistryContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginRegistryContract<'a> {
    fn margin_registry_target(&self, function: &str) -> String {
        format!(
            "{}::margin_registry::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    pub fn pool_enabled(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.margin_registry_target("pool_enabled"),
            vec![registry_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn get_margin_manager_ids(
        &self,
        tx: &mut Transaction,
        owner: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let owner_arg = tx.object(owner.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("get_margin_manager_ids"),
            vec![registry_obj, owner_arg],
            vec![],
        ))
    }

    fn pool_id_arg(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<(serde_json::Value, serde_json::Value), ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok((registry_obj, pool_obj))
    }

    pub fn base_margin_pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("base_margin_pool_id"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn quote_margin_pool_id(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("quote_margin_pool_id"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn min_withdraw_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("min_withdraw_risk_ratio"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn min_borrow_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("min_borrow_risk_ratio"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn liquidation_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("liquidation_risk_ratio"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn target_liquidation_risk_ratio(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("target_liquidation_risk_ratio"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn user_liquidation_reward(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("user_liquidation_reward"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn pool_liquidation_reward(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (registry_obj, pool_obj) = self.pool_id_arg(tx, pool_key)?;
        Ok(tx.move_call(
            &self.margin_registry_target("pool_liquidation_reward"),
            vec![registry_obj, pool_obj],
            vec![],
        ))
    }

    pub fn allowed_maintainers(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        Ok(tx.move_call(
            &self.margin_registry_target("allowed_maintainers"),
            vec![registry_obj],
            vec![],
        ))
    }

    pub fn allowed_pause_caps(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        Ok(tx.move_call(
            &self.margin_registry_target("allowed_pause_caps"),
            vec![registry_obj],
            vec![],
        ))
    }
}

impl<'a> MarginPoolContract<'a> {
    fn margin_pool_target(&self, function: &str) -> String {
        format!("{}::margin_pool::{function}", self.config.package_ids.margin_package_id)
    }

    pub fn get_id(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("id"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn deepbook_pool_allowed(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        deepbook_pool_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let deepbook_pool_obj = tx.object(deepbook_pool_id.to_string());
        Ok(tx.move_call(
            &self.margin_pool_target("deepbook_pool_allowed"),
            vec![pool_obj, deepbook_pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn total_supply(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("total_supply"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn supply_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("supply_shares"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn total_borrow(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("total_borrow"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn borrow_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("borrow_shares"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn last_update_timestamp(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("last_update_timestamp"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn supply_cap(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("supply_cap"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn max_utilization_rate(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("max_utilization_rate"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn protocol_spread(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("protocol_spread"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn min_borrow(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("min_borrow"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn interest_rate(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        Ok(tx.move_call(
            &self.margin_pool_target("interest_rate"),
            vec![pool_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn user_supply_shares(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supplier_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let supplier_cap_obj = tx.object(supplier_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_pool_target("user_supply_shares"),
            vec![pool_obj, supplier_cap_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn user_supply_amount(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supplier_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let supplier_cap_obj = tx.object(supplier_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("user_supply_amount"),
            vec![pool_obj, supplier_cap_obj, clock_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }
}

pub struct MarginManagerContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginManagerContract<'a> {
    fn margin_manager_target(&self, function: &str) -> String {
        format!(
            "{}::margin_manager::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    fn manager_types(
        &self,
        manager_key: &str,
    ) -> Result<
        (
            &crate::deepbook_v3::types::MarginManager,
            &crate::deepbook_v3::types::Pool,
            &crate::deepbook_v3::types::Coin,
            &crate::deepbook_v3::types::Coin,
        ),
        ContractError,
    > {
        let manager = self.config.get_margin_manager(manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((manager, pool, base, quote))
    }

    pub fn owner(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("owner"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn deepbook_pool(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("deepbook_pool"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn margin_pool_id(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("margin_pool_id"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_shares(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("borrowed_shares"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_base_shares(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("borrowed_base_shares"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrowed_quote_shares(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("borrowed_quote_shares"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn has_base_debt(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("has_base_debt"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn balance_manager(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("balance_manager"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn calculate_assets(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("calculate_assets"),
            vec![manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn calculate_debts(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        debt_coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let debt_coin = self.config.get_coin(debt_coin_key)?;
        let margin_pool = self.config.get_margin_pool(debt_coin_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("calculate_debts"),
            vec![manager_obj, margin_pool_obj, clock_obj],
            vec![
                base.type_tag.clone(),
                quote.type_tag.clone(),
                debt_coin.type_tag.clone(),
            ],
        ))
    }

    pub fn base_balance(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("base_balance"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn quote_balance(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("quote_balance"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn deep_balance(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("deep_balance"),
            vec![manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn manager_state(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let manager_obj = tx.object(manager.address.clone());
        let margin_registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
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
            &self.margin_manager_target("manager_state"),
            vec![
                manager_obj,
                margin_registry_obj,
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
}

pub struct GovernanceContract<'a> {
    pub config: &'a DeepBookConfig,
    pub balance_manager: BalanceManagerContract<'a>,
}

impl<'a> GovernanceContract<'a> {
    pub fn vote(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        proposal_id: u128,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self.balance_manager.generate_proof(tx, balance_manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let proposal_arg = tx.pure_bytes(&encode_u128(proposal_id));

        Ok(tx.move_call(
            &format!("{}::pool::vote", self.config.package_ids.deepbook_package_id),
            vec![pool_obj, manager_obj, proof, proposal_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}
