use crate::config::{ConfigError, DeepBookConfig, FLOAT_SCALAR, MAX_TIMESTAMP};
use crate::encode::{
    encode_bool, encode_option_u64, encode_u128, encode_u64, encode_u8, encode_vec_u128,
};

pub mod deepbook_admin;
pub mod flash_loans;
pub mod governance;
pub mod margin_admin;
pub mod margin_liquidations;
pub mod margin_maintainer;
pub mod margin_manager;
pub mod margin_pool;
pub mod margin_registry;
pub mod pool_proxy;
pub mod pyth_oracle;
pub mod tpsl;
pub mod wormhole;
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error(transparent)]
    Config(#[from] ConfigError),
}

pub struct BalanceManagerContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> BalanceManagerContract<'a> {
    pub fn create_balance_manager(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, ContractError> {
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::new",
                self.config.package_ids.deepbook_package_id
            ),
            vec![],
            vec![],
        ))
    }

    pub fn new_with_custom_owner(
        &self,
        tx: &mut Transaction,
        owner_address: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let owner_obj = tx.object(owner_address.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::new_with_custom_owner",
                self.config.package_ids.deepbook_package_id
            ),
            vec![owner_obj],
            vec![],
        ))
    }

    pub fn deposit(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        coin_key: &str,
        coin_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let coin_obj = tx.object(coin_object_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::deposit",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, coin_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn withdraw(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        coin_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let input_amount = (amount * coin.scalar as f64).round() as u64;
        let manager_obj = tx.object(manager.address.clone());
        let amount_arg = tx.pure_bytes(&encode_u64(input_amount));
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::withdraw",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, amount_arg],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn deposit_with_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        coin_key: &str,
        deposit_cap_id: &str,
        coin_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let deposit_cap_obj = tx.object(deposit_cap_id.to_string());
        let coin_obj = tx.object(coin_object_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::deposit_with_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, deposit_cap_obj, coin_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn withdraw_with_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        coin_key: &str,
        withdraw_cap_id: &str,
        amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let input_amount = (amount * coin.scalar as f64).round() as u64;
        let manager_obj = tx.object(manager.address.clone());
        let withdraw_cap_obj = tx.object(withdraw_cap_id.to_string());
        let amount_arg = tx.pure_bytes(&encode_u64(input_amount));
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::withdraw_with_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, withdraw_cap_obj, amount_arg],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn withdraw_all(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::withdraw_all",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn mint_trade_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::mint_trade_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![],
        ))
    }

    pub fn mint_deposit_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::mint_deposit_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![],
        ))
    }

    pub fn mint_withdraw_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::mint_withdraw_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![],
        ))
    }

    pub fn register_balance_manager(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::register_balance_manager",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, registry_obj],
            vec![],
        ))
    }

    pub fn owner(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::owner",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![],
        ))
    }

    pub fn id(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::id",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![],
        ))
    }

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

    pub fn generate_proof_as_owner(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
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

    pub fn generate_proof_as_trader(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        trade_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let trade_cap_obj = tx.object(trade_cap_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::generate_proof_as_trader",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, trade_cap_obj],
            vec![],
        ))
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

    pub fn set_balance_manager_referral(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        let proof = self.generate_proof(tx, manager_key)?;
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::set_balance_manager_referral",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, referral_obj, proof],
            vec![],
        ))
    }

    pub fn unset_balance_manager_referral(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let pool = self.config.get_pool(pool_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let proof = self.generate_proof(tx, manager_key)?;
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::unset_balance_manager_referral",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, pool_obj, proof],
            vec![],
        ))
    }

    pub fn revoke_trade_cap(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        trade_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let trade_cap_obj = tx.object(trade_cap_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::balance_manager::revoke_trade_cap",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj, trade_cap_obj],
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
            &crate::types::Pool,
            &crate::types::Coin,
            &crate::types::Coin,
        ),
        ContractError,
    > {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((pool, base, quote))
    }

    fn pool_target(&self, function: &str) -> String {
        format!(
            "{}::pool::{function}",
            self.config.package_ids.deepbook_package_id
        )
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
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;
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

    pub fn place_limit_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        client_order_id: u64,
        order_type: u8,
        self_matching_option: u8,
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
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let expiration = expire_timestamp.unwrap_or(MAX_TIMESTAMP);

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
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
            &self.pool_target("place_limit_order"),
            vec![
                pool_obj,
                manager_obj,
                proof,
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
        pool_key: &str,
        balance_manager_key: &str,
        client_order_id: u64,
        self_matching_option: u8,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("place_market_order"),
            vec![
                pool_obj,
                manager_obj,
                proof,
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

    pub fn modify_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        order_id: u128,
        new_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;
        let input_quantity = (new_quantity * base.scalar as f64).round() as u64;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let order_id_arg = tx.pure_bytes(&encode_u128(order_id));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("modify_order"),
            vec![
                pool_obj,
                manager_obj,
                proof,
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
        pool_key: &str,
        balance_manager_key: &str,
        order_id: u128,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let order_id_arg = tx.pure_bytes(&encode_u128(order_id));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("cancel_order"),
            vec![pool_obj, manager_obj, proof, order_id_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_all_orders(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.pool_target("cancel_all_orders"),
            vec![pool_obj, manager_obj, proof, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_settled_amounts(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());

        Ok(tx.move_call(
            &self.pool_target("withdraw_settled_amounts"),
            vec![pool_obj, manager_obj, proof],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_settled_amounts_permissionless(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());

        Ok(tx.move_call(
            &self.pool_target("withdraw_settled_amounts_permissionless"),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn claim_rebates(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;

        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());

        Ok(tx.move_call(
            &self.pool_target("claim_rebates"),
            vec![pool_obj, manager_obj, proof],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_pool_allowed_versions(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        Ok(tx.move_call(
            &self.pool_target("update_pool_allowed_versions"),
            vec![pool_obj, registry_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn add_deep_price_point(
        &self,
        tx: &mut Transaction,
        target_pool_key: &str,
        reference_pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let target_pool = self.config.get_pool(target_pool_key)?;
        let reference_pool = self.config.get_pool(reference_pool_key)?;
        let target_base = self.config.get_coin(&target_pool.base_coin)?;
        let target_quote = self.config.get_coin(&target_pool.quote_coin)?;
        let reference_base = self.config.get_coin(&reference_pool.base_coin)?;
        let reference_quote = self.config.get_coin(&reference_pool.quote_coin)?;
        let target_pool_obj = tx.object(target_pool.address.clone());
        let reference_pool_obj = tx.object(reference_pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("add_deep_price_point"),
            vec![target_pool_obj, reference_pool_obj, clock_obj],
            vec![
                target_base.type_tag.clone(),
                target_quote.type_tag.clone(),
                reference_base.type_tag.clone(),
                reference_quote.type_tag.clone(),
            ],
        ))
    }

    pub fn mint_referral(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        multiplier: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let adjusted = (multiplier * FLOAT_SCALAR).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let multiplier_arg = tx.pure_bytes(&encode_u64(adjusted));
        Ok(tx.move_call(
            &self.pool_target("mint_referral"),
            vec![pool_obj, multiplier_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_pool_referral_multiplier(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        referral_id: &str,
        multiplier: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let adjusted = (multiplier * FLOAT_SCALAR).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        let multiplier_arg = tx.pure_bytes(&encode_u64(adjusted));
        Ok(tx.move_call(
            &self.pool_target("update_pool_referral_multiplier"),
            vec![pool_obj, referral_obj, multiplier_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn claim_pool_referral_rewards(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("claim_pool_referral_rewards"),
            vec![pool_obj, referral_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn burn_deep(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let deep_treasury_obj = tx.object(self.config.package_ids.deep_treasury_id.clone());
        Ok(tx.move_call(
            &self.pool_target("burn_deep"),
            vec![pool_obj, deep_treasury_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn mid_price(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("mid_price"),
            vec![pool_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn whitelisted(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_target("whitelisted"),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn create_permissionless_pool(
        &self,
        tx: &mut Transaction,
        base_coin_key: &str,
        quote_coin_key: &str,
        tick_size: f64,
        lot_size: f64,
        min_size: f64,
        deep_coin_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let base_coin = self.config.get_coin(base_coin_key)?;
        let quote_coin = self.config.get_coin(quote_coin_key)?;
        let adjusted_tick_size = ((tick_size * FLOAT_SCALAR * quote_coin.scalar as f64)
            / base_coin.scalar as f64)
            .round() as u64;
        let adjusted_lot_size = (lot_size * base_coin.scalar as f64).round() as u64;
        let adjusted_min_size = (min_size * base_coin.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let tick_arg = tx.pure_bytes(&encode_u64(adjusted_tick_size));
        let lot_arg = tx.pure_bytes(&encode_u64(adjusted_lot_size));
        let min_arg = tx.pure_bytes(&encode_u64(adjusted_min_size));
        let deep_coin_obj = tx.object(deep_coin_object_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("create_permissionless_pool"),
            vec![registry_obj, tick_arg, lot_arg, min_arg, deep_coin_obj],
            vec![base_coin.type_tag.clone(), quote_coin.type_tag.clone()],
        ))
    }

    pub fn swap_exact_base_for_quote(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_coin_object_id: &str,
        deep_coin_object_id: &str,
        min_quote_out: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let min_quote_input = (min_quote_out * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let base_coin_obj = tx.object(base_coin_object_id.to_string());
        let deep_coin_obj = tx.object(deep_coin_object_id.to_string());
        let min_quote_arg = tx.pure_bytes(&encode_u64(min_quote_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_base_for_quote"),
            vec![
                pool_obj,
                base_coin_obj,
                deep_coin_obj,
                min_quote_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn swap_exact_quote_for_base(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quote_coin_object_id: &str,
        deep_coin_object_id: &str,
        min_base_out: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let min_base_input = (min_base_out * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let quote_coin_obj = tx.object(quote_coin_object_id.to_string());
        let deep_coin_obj = tx.object(deep_coin_object_id.to_string());
        let min_base_arg = tx.pure_bytes(&encode_u64(min_base_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_quote_for_base"),
            vec![
                pool_obj,
                quote_coin_obj,
                deep_coin_obj,
                min_base_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn swap_exact_quantity(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_coin_object_id: &str,
        quote_coin_object_id: &str,
        deep_coin_object_id: &str,
        min_out: f64,
        is_base_to_quote: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let out_scalar = if is_base_to_quote {
            quote.scalar as f64
        } else {
            base.scalar as f64
        };
        let min_out_input = (min_out * out_scalar).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let base_coin_obj = tx.object(base_coin_object_id.to_string());
        let quote_coin_obj = tx.object(quote_coin_object_id.to_string());
        let deep_coin_obj = tx.object(deep_coin_object_id.to_string());
        let min_out_arg = tx.pure_bytes(&encode_u64(min_out_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_quantity"),
            vec![
                pool_obj,
                base_coin_obj,
                quote_coin_obj,
                deep_coin_obj,
                min_out_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn swap_exact_base_for_quote_with_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        trade_cap_id: &str,
        deposit_cap_id: &str,
        withdraw_cap_id: &str,
        base_coin_object_id: &str,
        min_quote_out: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let min_quote_input = (min_quote_out * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let trade_cap_obj = tx.object(trade_cap_id.to_string());
        let deposit_cap_obj = tx.object(deposit_cap_id.to_string());
        let withdraw_cap_obj = tx.object(withdraw_cap_id.to_string());
        let base_coin_obj = tx.object(base_coin_object_id.to_string());
        let min_quote_arg = tx.pure_bytes(&encode_u64(min_quote_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_base_for_quote_with_manager"),
            vec![
                pool_obj,
                manager_obj,
                trade_cap_obj,
                deposit_cap_obj,
                withdraw_cap_obj,
                base_coin_obj,
                min_quote_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn swap_exact_quote_for_base_with_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        trade_cap_id: &str,
        deposit_cap_id: &str,
        withdraw_cap_id: &str,
        quote_coin_object_id: &str,
        min_base_out: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let min_base_input = (min_base_out * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let trade_cap_obj = tx.object(trade_cap_id.to_string());
        let deposit_cap_obj = tx.object(deposit_cap_id.to_string());
        let withdraw_cap_obj = tx.object(withdraw_cap_id.to_string());
        let quote_coin_obj = tx.object(quote_coin_object_id.to_string());
        let min_base_arg = tx.pure_bytes(&encode_u64(min_base_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_quote_for_base_with_manager"),
            vec![
                pool_obj,
                manager_obj,
                trade_cap_obj,
                deposit_cap_obj,
                withdraw_cap_obj,
                quote_coin_obj,
                min_base_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn swap_exact_quantity_with_manager(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        balance_manager_key: &str,
        trade_cap_id: &str,
        deposit_cap_id: &str,
        withdraw_cap_id: &str,
        base_coin_object_id: &str,
        quote_coin_object_id: &str,
        min_out: f64,
        is_base_to_quote: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(balance_manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let out_scalar = if is_base_to_quote {
            quote.scalar as f64
        } else {
            base.scalar as f64
        };
        let min_out_input = (min_out * out_scalar).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let trade_cap_obj = tx.object(trade_cap_id.to_string());
        let deposit_cap_obj = tx.object(deposit_cap_id.to_string());
        let withdraw_cap_obj = tx.object(withdraw_cap_id.to_string());
        let base_coin_obj = tx.object(base_coin_object_id.to_string());
        let quote_coin_obj = tx.object(quote_coin_object_id.to_string());
        let min_out_arg = tx.pure_bytes(&encode_u64(min_out_input));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("swap_exact_quantity_with_manager"),
            vec![
                pool_obj,
                manager_obj,
                trade_cap_obj,
                deposit_cap_obj,
                withdraw_cap_obj,
                base_coin_obj,
                quote_coin_obj,
                min_out_arg,
                clock_obj,
            ],
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

    pub fn get_pool_id_by_asset(
        &self,
        tx: &mut Transaction,
        base_type: &str,
        quote_type: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.get_pool_id_by_assets(tx, base_type, quote_type)
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

    pub fn get_order_deep_price(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.get_pool_deep_price(tx, pool_key)
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
            &crate::types::MarginManager,
            &crate::types::Coin,
            &crate::types::Coin,
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

    pub fn conditional_order(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        conditional_order_id: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let id_arg = tx.pure_bytes(&encode_u64(conditional_order_id));
        Ok(tx.move_call(
            &self.margin_manager_target("conditional_order"),
            vec![manager_obj, id_arg],
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

    pub fn new_condition(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        trigger_below_price: bool,
        trigger_price: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let input_price = ((trigger_price * FLOAT_SCALAR * quote.scalar as f64)
            / base.scalar as f64)
            .round() as u64;
        let trigger_below_arg = tx.pure_bytes(&encode_bool(trigger_below_price));
        let trigger_price_arg = tx.pure_bytes(&encode_u64(input_price));
        Ok(tx.move_call(
            &format!(
                "{}::tpsl::new_condition",
                self.config.package_ids.margin_package_id
            ),
            vec![trigger_below_arg, trigger_price_arg],
            vec![],
        ))
    }

    pub fn new_pending_limit_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        client_order_id: u64,
        order_type: u8,
        self_matching_option: u8,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: Option<u64>,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let expiry = expire_timestamp.unwrap_or(MAX_TIMESTAMP);

        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let order_type_arg = tx.pure_bytes(&encode_u8(order_type));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let price_arg = tx.pure_bytes(&encode_u64(input_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expire_arg = tx.pure_bytes(&encode_u64(expiry));

        Ok(tx.move_call(
            &format!(
                "{}::tpsl::new_pending_limit_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                client_order_id_arg,
                order_type_arg,
                self_matching_option_arg,
                price_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
                expire_arg,
            ],
            vec![],
        ))
    }

    pub fn new_pending_market_order(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        client_order_id: u64,
        self_matching_option: u8,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;

        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&encode_u8(self_matching_option));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));

        Ok(tx.move_call(
            &format!(
                "{}::tpsl::new_pending_market_order",
                self.config.package_ids.margin_package_id
            ),
            vec![
                client_order_id_arg,
                self_matching_option_arg,
                quantity_arg,
                is_bid_arg,
                pay_with_deep_arg,
            ],
            vec![],
        ))
    }

    pub fn add_conditional_limit_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        conditional_order_id: u64,
        trigger_below_price: bool,
        trigger_price: f64,
        client_order_id: u64,
        order_type: u8,
        self_matching_option: u8,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: Option<u64>,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let condition =
            self.new_condition(tx, &manager.pool_key, trigger_below_price, trigger_price)?;
        let pending_order = self.new_pending_limit_order(
            tx,
            &manager.pool_key,
            client_order_id,
            order_type,
            self_matching_option,
            price,
            quantity,
            is_bid,
            pay_with_deep,
            expire_timestamp,
        )?;

        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
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
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let conditional_id_arg = tx.pure_bytes(&encode_u64(conditional_order_id));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.margin_manager_target("add_conditional_order"),
            vec![
                manager_obj,
                pool_obj,
                base_price_obj,
                quote_price_obj,
                registry_obj,
                conditional_id_arg,
                condition,
                pending_order,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn add_conditional_market_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        conditional_order_id: u64,
        trigger_below_price: bool,
        trigger_price: f64,
        client_order_id: u64,
        self_matching_option: u8,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let condition =
            self.new_condition(tx, &manager.pool_key, trigger_below_price, trigger_price)?;
        let pending_order = self.new_pending_market_order(
            tx,
            &manager.pool_key,
            client_order_id,
            self_matching_option,
            quantity,
            is_bid,
            pay_with_deep,
        )?;

        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
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
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let conditional_id_arg = tx.pure_bytes(&encode_u64(conditional_order_id));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.margin_manager_target("add_conditional_order"),
            vec![
                manager_obj,
                pool_obj,
                base_price_obj,
                quote_price_obj,
                registry_obj,
                conditional_id_arg,
                condition,
                pending_order,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn add_conditional_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        conditional_order_id: u64,
        condition: serde_json::Value,
        pending_order: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
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
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let conditional_id_arg = tx.pure_bytes(&encode_u64(conditional_order_id));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("add_conditional_order"),
            vec![
                manager_obj,
                pool_obj,
                base_price_obj,
                quote_price_obj,
                registry_obj,
                conditional_id_arg,
                condition,
                pending_order,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_all_conditional_orders(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("cancel_all_conditional_orders"),
            vec![manager_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_conditional_order(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        conditional_order_id: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let id_arg = tx.pure_bytes(&encode_u64(conditional_order_id));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("cancel_conditional_order"),
            vec![manager_obj, id_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn execute_conditional_orders(
        &self,
        tx: &mut Transaction,
        manager_address: &str,
        pool_key: &str,
        max_orders_to_execute: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let manager_obj = tx.object(manager_address.to_string());
        let pool_obj = tx.object(pool.address.clone());
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
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let max_orders_arg = tx.pure_bytes(&encode_u64(max_orders_to_execute));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("execute_conditional_orders"),
            vec![
                manager_obj,
                pool_obj,
                base_price_obj,
                quote_price_obj,
                registry_obj,
                max_orders_arg,
                clock_obj,
            ],
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

    pub fn get_margin_pool_id(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        Ok(tx.move_call(
            &self.margin_registry_target("get_margin_pool_id"),
            vec![registry_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn get_deepbook_pool_margin_pool_ids(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.margin_registry_target("get_deepbook_pool_margin_pool_ids"),
            vec![registry_obj, pool_obj],
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
        format!(
            "{}::margin_pool::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    pub fn mint_supplier_cap(
        &self,
        tx: &mut Transaction,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("mint_supplier_cap"),
            vec![registry_obj, clock_obj],
            vec![],
        ))
    }

    pub fn mint_supply_referral(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("mint_supply_referral"),
            vec![pool_obj, registry_obj, clock_obj],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn supply(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supplier_cap_id: &str,
        supply_coin_object_id: &str,
        referral_id: Option<&str>,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let supplier_cap_obj = tx.object(supplier_cap_id.to_string());
        let supply_coin_obj = tx.object(supply_coin_object_id.to_string());
        let referral_obj = tx.object(referral_id.unwrap_or("0x0").to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("supply"),
            vec![
                pool_obj,
                registry_obj,
                supplier_cap_obj,
                supply_coin_obj,
                referral_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn withdraw_referral_fees(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &self.margin_pool_target("withdraw_referral_fees"),
            vec![pool_obj, registry_obj, referral_obj],
            vec![margin_pool.type_tag.clone()],
        ))
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

pub struct PoolProxyContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> PoolProxyContract<'a> {
    fn pool_proxy_target(&self, function: &str) -> String {
        format!(
            "{}::pool_proxy::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    fn manager_pool_types(
        &self,
        margin_manager_key: &str,
    ) -> Result<
        (
            &crate::types::MarginManager,
            &crate::types::Pool,
            &crate::types::Coin,
            &crate::types::Coin,
        ),
        ContractError,
    > {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((manager, pool, base, quote))
    }

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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let order_type_arg = tx.pure_bytes(&[order_type]);
        let self_matching_option_arg = tx.pure_bytes(&[self_matching_option]);
        let price_arg = tx.pure_bytes(&encode_u64(input_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expiration_arg = tx.pure_bytes(&encode_u64(expiration));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("place_limit_order"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&[self_matching_option]);
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("place_market_order"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let input_price =
            ((price * FLOAT_SCALAR * quote.scalar as f64) / base.scalar as f64).round() as u64;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let debt_margin_pool = if is_bid {
            self.config.get_margin_pool(&pool.base_coin)?
        } else {
            self.config.get_margin_pool(&pool.quote_coin)?
        };
        let debt_coin = if is_bid { base } else { quote };
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let debt_margin_pool_obj = tx.object(debt_margin_pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let order_type_arg = tx.pure_bytes(&[order_type]);
        let self_matching_option_arg = tx.pure_bytes(&[self_matching_option]);
        let price_arg = tx.pure_bytes(&encode_u64(input_price));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let expiration_arg = tx.pure_bytes(&encode_u64(expiration));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("place_reduce_only_limit_order"),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                debt_margin_pool_obj,
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
                debt_coin.type_tag.clone(),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let input_quantity = (quantity * base.scalar as f64).round() as u64;
        let debt_margin_pool = if is_bid {
            self.config.get_margin_pool(&pool.base_coin)?
        } else {
            self.config.get_margin_pool(&pool.quote_coin)?
        };
        let debt_coin = if is_bid { base } else { quote };
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let debt_margin_pool_obj = tx.object(debt_margin_pool.address.clone());
        let client_order_id_arg = tx.pure_bytes(&encode_u64(client_order_id));
        let self_matching_option_arg = tx.pure_bytes(&[self_matching_option]);
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let is_bid_arg = tx.pure_bytes(&encode_bool(is_bid));
        let pay_with_deep_arg = tx.pure_bytes(&encode_bool(pay_with_deep));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("place_reduce_only_market_order"),
            vec![
                registry_obj,
                manager_obj,
                pool_obj,
                debt_margin_pool_obj,
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
                debt_coin.type_tag.clone(),
            ],
        ))
    }

    pub fn modify_order(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        order_id: u128,
        new_quantity: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let input_quantity = (new_quantity * base.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_id_arg = tx.pure_bytes(&encode_u128(order_id));
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("modify_order"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_id_arg = tx.pure_bytes(&encode_u128(order_id));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("cancel_order"),
            vec![registry_obj, manager_obj, pool_obj, order_id_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn cancel_orders(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        order_ids: &[u128],
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let order_ids_arg = tx.pure_bytes(&encode_vec_u128(order_ids));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("cancel_orders"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_proxy_target("cancel_all_orders"),
            vec![registry_obj, manager_obj, pool_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_settled_amounts(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_proxy_target("withdraw_settled_amounts"),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn stake(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
        stake_amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let deep = self.config.get_coin("DEEP")?;
        let stake_input = (stake_amount * deep.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let stake_arg = tx.pure_bytes(&encode_u64(stake_input));
        Ok(tx.move_call(
            &self.pool_proxy_target("stake"),
            vec![registry_obj, manager_obj, pool_obj, stake_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unstake(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_proxy_target("unstake"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
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
            &self.pool_proxy_target("submit_proposal"),
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
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let proposal_obj = tx.object(proposal_id.to_string());
        Ok(tx.move_call(
            &self.pool_proxy_target("vote"),
            vec![registry_obj, manager_obj, pool_obj, proposal_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn claim_rebate(
        &self,
        tx: &mut Transaction,
        margin_manager_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_pool_types(margin_manager_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_proxy_target("claim_rebate"),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn withdraw_margin_settled_amounts(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_manager_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let manager_obj = tx.object(margin_manager_id.to_string());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.pool_proxy_target("withdraw_settled_amounts_permissionless"),
            vec![registry_obj, manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

pub struct MarginLiquidationsContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginLiquidationsContract<'a> {
    fn liquidation_target(&self, function: &str) -> String {
        format!(
            "{}::liquidation_vault::{function}",
            self.config.package_ids.liquidation_package_id
        )
    }

    pub fn create_liquidation_vault(
        &self,
        tx: &mut Transaction,
        liquidation_admin_cap: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let cap_obj = tx.object(liquidation_admin_cap.to_string());
        Ok(tx.move_call(
            &self.liquidation_target("create_liquidation_vault"),
            vec![cap_obj],
            vec![],
        ))
    }

    pub fn deposit(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        liquidation_admin_cap: &str,
        coin_key: &str,
        coin_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let vault_obj = tx.object(vault_id.to_string());
        let cap_obj = tx.object(liquidation_admin_cap.to_string());
        let coin_obj = tx.object(coin_object_id.to_string());
        Ok(tx.move_call(
            &self.liquidation_target("deposit"),
            vec![vault_obj, cap_obj, coin_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn withdraw(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        liquidation_admin_cap: &str,
        coin_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let input_amount = (amount * coin.scalar as f64).round() as u64;
        let vault_obj = tx.object(vault_id.to_string());
        let cap_obj = tx.object(liquidation_admin_cap.to_string());
        let amount_arg = tx.pure_bytes(&encode_u64(input_amount));
        Ok(tx.move_call(
            &self.liquidation_target("withdraw"),
            vec![vault_obj, cap_obj, amount_arg],
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
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let repay_amount_scaled =
            repay_amount.map(|v| (v * base_coin.scalar as f64).round() as u64);

        let vault_obj = tx.object(vault_id.to_string());
        let manager_obj = tx.object(manager_address.to_string());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_price_obj = tx.object(
            base_coin
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let quote_price_obj = tx.object(
            quote_coin
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let repay_arg = tx.pure_bytes(&encode_option_u64(repay_amount_scaled));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.liquidation_target("liquidate_base"),
            vec![
                vault_obj,
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                pool_obj,
                repay_arg,
                clock_obj,
            ],
            vec![base_coin.type_tag.clone(), quote_coin.type_tag.clone()],
        ))
    }

    pub fn liquidate_quote(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        manager_address: &str,
        pool_key: &str,
        repay_amount: Option<f64>,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let repay_amount_scaled =
            repay_amount.map(|v| (v * quote_coin.scalar as f64).round() as u64);

        let vault_obj = tx.object(vault_id.to_string());
        let manager_obj = tx.object(manager_address.to_string());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_price_obj = tx.object(
            base_coin
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let quote_price_obj = tx.object(
            quote_coin
                .price_info_object_id
                .clone()
                .unwrap_or_else(|| "0x0".to_string()),
        );
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let repay_arg = tx.pure_bytes(&encode_option_u64(repay_amount_scaled));
        let clock_obj = tx.object("0x6");

        Ok(tx.move_call(
            &self.liquidation_target("liquidate_quote"),
            vec![
                vault_obj,
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                base_margin_pool_obj,
                quote_margin_pool_obj,
                pool_obj,
                repay_arg,
                clock_obj,
            ],
            vec![base_coin.type_tag.clone(), quote_coin.type_tag.clone()],
        ))
    }

    pub fn balance(
        &self,
        tx: &mut Transaction,
        vault_id: &str,
        coin_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let vault_obj = tx.object(vault_id.to_string());
        Ok(tx.move_call(
            &self.liquidation_target("balance"),
            vec![vault_obj],
            vec![coin.type_tag.clone()],
        ))
    }
}

pub struct FlashLoanContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> FlashLoanContract<'a> {
    fn flash_target(&self, function: &str) -> String {
        format!(
            "{}::pool::{function}",
            self.config.package_ids.deepbook_package_id
        )
    }

    fn pool_types(
        &self,
        pool_key: &str,
    ) -> Result<
        (
            &crate::types::Pool,
            &crate::types::Coin,
            &crate::types::Coin,
        ),
        ContractError,
    > {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((pool, base, quote))
    }

    pub fn borrow_base_asset(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let input_quantity = (borrow_amount * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        Ok(tx.move_call(
            &self.flash_target("borrow_flashloan_base"),
            vec![pool_obj, quantity_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrow_flashloan_base(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        self.borrow_base_asset(tx, pool_key, borrow_amount)
    }

    pub fn return_base_asset(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_coin_object_id: &str,
        flashloan_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let base_coin_obj = tx.object(base_coin_object_id.to_string());
        let flashloan_obj = tx.object(flashloan_object_id.to_string());
        Ok(tx.move_call(
            &self.flash_target("return_flashloan_base"),
            vec![pool_obj, base_coin_obj, flashloan_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn return_flashloan_base(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        base_coin_object_id: &str,
        flashloan_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.return_base_asset(tx, pool_key, base_coin_object_id, flashloan_object_id)
    }

    pub fn borrow_quote_asset(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let input_quantity = (borrow_amount * quote.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let quantity_arg = tx.pure_bytes(&encode_u64(input_quantity));
        Ok(tx.move_call(
            &self.flash_target("borrow_flashloan_quote"),
            vec![pool_obj, quantity_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn borrow_flashloan_quote(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        borrow_amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        self.borrow_quote_asset(tx, pool_key, borrow_amount)
    }

    pub fn return_quote_asset(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quote_coin_object_id: &str,
        flashloan_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (pool, base, quote) = self.pool_types(pool_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let quote_coin_obj = tx.object(quote_coin_object_id.to_string());
        let flashloan_obj = tx.object(flashloan_object_id.to_string());
        Ok(tx.move_call(
            &self.flash_target("return_flashloan_quote"),
            vec![pool_obj, quote_coin_obj, flashloan_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn return_flashloan_quote(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        quote_coin_object_id: &str,
        flashloan_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.return_quote_asset(tx, pool_key, quote_coin_object_id, flashloan_object_id)
    }
}

pub struct MarginMaintainerContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginMaintainerContract<'a> {
    fn margin_pool_target(&self, function: &str) -> String {
        format!(
            "{}::margin_pool::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    fn protocol_config_target(&self, function: &str) -> String {
        format!(
            "{}::protocol_config::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    pub fn new_interest_config(
        &self,
        tx: &mut Transaction,
        base_rate: f64,
        base_slope: f64,
        optimal_utilization: f64,
        excess_slope: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let base_rate_arg = tx.pure_bytes(&encode_u64((base_rate * FLOAT_SCALAR).round() as u64));
        let base_slope_arg = tx.pure_bytes(&encode_u64((base_slope * FLOAT_SCALAR).round() as u64));
        let optimal_utilization_arg = tx.pure_bytes(&encode_u64(
            (optimal_utilization * FLOAT_SCALAR).round() as u64,
        ));
        let excess_slope_arg =
            tx.pure_bytes(&encode_u64((excess_slope * FLOAT_SCALAR).round() as u64));
        Ok(tx.move_call(
            &self.protocol_config_target("new_interest_config"),
            vec![
                base_rate_arg,
                base_slope_arg,
                optimal_utilization_arg,
                excess_slope_arg,
            ],
            vec![],
        ))
    }

    pub fn new_margin_pool_config(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        supply_cap: f64,
        max_utilization_rate: f64,
        referral_spread: f64,
        min_borrow: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let supply_cap_arg =
            tx.pure_bytes(&encode_u64((supply_cap * coin.scalar as f64).round() as u64));
        let max_utilization_rate_arg = tx.pure_bytes(&encode_u64(
            (max_utilization_rate * FLOAT_SCALAR).round() as u64,
        ));
        let referral_spread_arg =
            tx.pure_bytes(&encode_u64((referral_spread * FLOAT_SCALAR).round() as u64));
        let min_borrow_arg =
            tx.pure_bytes(&encode_u64((min_borrow * coin.scalar as f64).round() as u64));
        Ok(tx.move_call(
            &self.protocol_config_target("new_margin_pool_config"),
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
        supply_cap: f64,
        max_utilization_rate: f64,
        referral_spread: f64,
        min_borrow: f64,
        rate_limit_capacity: f64,
        rate_limit_refill_rate_per_ms: f64,
        rate_limit_enabled: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let supply_cap_arg =
            tx.pure_bytes(&encode_u64((supply_cap * coin.scalar as f64).round() as u64));
        let max_utilization_rate_arg = tx.pure_bytes(&encode_u64(
            (max_utilization_rate * FLOAT_SCALAR).round() as u64,
        ));
        let referral_spread_arg =
            tx.pure_bytes(&encode_u64((referral_spread * FLOAT_SCALAR).round() as u64));
        let min_borrow_arg =
            tx.pure_bytes(&encode_u64((min_borrow * coin.scalar as f64).round() as u64));
        let rate_limit_capacity_arg = tx.pure_bytes(&encode_u64(
            (rate_limit_capacity * coin.scalar as f64).round() as u64,
        ));
        let rate_limit_refill_rate_per_ms_arg = tx.pure_bytes(&encode_u64(
            (rate_limit_refill_rate_per_ms * coin.scalar as f64).round() as u64,
        ));
        let rate_limit_enabled_arg = tx.pure_bytes(&encode_bool(rate_limit_enabled));
        Ok(tx.move_call(
            &self.protocol_config_target("new_margin_pool_config_with_rate_limit"),
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

    pub fn new_protocol_config(
        &self,
        tx: &mut Transaction,
        margin_pool_config_object: serde_json::Value,
        interest_config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        Ok(tx.move_call(
            &self.protocol_config_target("new_protocol_config"),
            vec![margin_pool_config_object, interest_config_object],
            vec![],
        ))
    }

    pub fn create_margin_pool(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        pool_config_object: serde_json::Value,
        margin_maintainer_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let cap_obj = tx.object(margin_maintainer_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("create_margin_pool"),
            vec![registry_obj, pool_config_object, cap_obj, clock_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn enable_deepbook_pool_for_loan(
        &self,
        tx: &mut Transaction,
        deepbook_pool_key: &str,
        coin_key: &str,
        margin_pool_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let deepbook_pool = self.config.get_pool(deepbook_pool_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let deepbook_pool_obj = tx.object(deepbook_pool.address.clone());
        let cap_obj = tx.object(margin_pool_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("enable_deepbook_pool_for_loan"),
            vec![
                pool_obj,
                registry_obj,
                deepbook_pool_obj,
                cap_obj,
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
        margin_pool_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let deepbook_pool = self.config.get_pool(deepbook_pool_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let deepbook_pool_obj = tx.object(deepbook_pool.address.clone());
        let cap_obj = tx.object(margin_pool_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("disable_deepbook_pool_for_loan"),
            vec![
                pool_obj,
                registry_obj,
                deepbook_pool_obj,
                cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn update_interest_params(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_cap_id: &str,
        interest_config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let cap_obj = tx.object(margin_pool_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("update_interest_params"),
            vec![
                pool_obj,
                registry_obj,
                interest_config_object,
                cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }

    pub fn update_margin_pool_config(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_pool_cap_id: &str,
        margin_pool_config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let cap_obj = tx.object(margin_pool_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_pool_target("update_margin_pool_config"),
            vec![
                pool_obj,
                registry_obj,
                margin_pool_config_object,
                cap_obj,
                clock_obj,
            ],
            vec![margin_pool.type_tag.clone()],
        ))
    }
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
            &crate::types::MarginManager,
            &crate::types::Pool,
            &crate::types::Coin,
            &crate::types::Coin,
        ),
        ContractError,
    > {
        let manager = self.config.get_margin_manager(manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        Ok((manager, pool, base, quote))
    }

    pub fn new(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let margin_registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("new"),
            vec![pool_obj, registry_obj, margin_registry_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn new_with_initializer(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let margin_registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("new_with_initializer"),
            vec![pool_obj, registry_obj, margin_registry_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn share(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        manager_object_id: &str,
        initializer_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (_manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager_object_id.to_string());
        let initializer_obj = tx.object(initializer_object_id.to_string());
        Ok(tx.move_call(
            &self.margin_manager_target("share"),
            vec![manager_obj, initializer_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
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

    pub fn borrow_base(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let input_amount = (amount * base.scalar as f64).round() as u64;
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
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
        let amount_arg = tx.pure_bytes(&encode_u64(input_amount));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("borrow_base"),
            vec![
                manager_obj,
                registry_obj,
                base_margin_pool_obj,
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
        manager_key: &str,
        amount: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let input_amount = (amount * quote.scalar as f64).round() as u64;
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
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
        let amount_arg = tx.pure_bytes(&encode_u64(input_amount));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("borrow_quote"),
            vec![
                manager_obj,
                registry_obj,
                quote_margin_pool_obj,
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
        manager_key: &str,
        amount: Option<f64>,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let repay_amount_scaled = amount.map(|v| (v * base.scalar as f64).round() as u64);
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let base_margin_pool_obj = tx.object(base_margin_pool.address.clone());
        let repay_arg = tx.pure_bytes(&encode_option_u64(repay_amount_scaled));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("repay_base"),
            vec![
                manager_obj,
                registry_obj,
                base_margin_pool_obj,
                repay_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn repay_quote(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        amount: Option<f64>,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, pool, base, quote) = self.manager_types(manager_key)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let repay_amount_scaled = amount.map(|v| (v * quote.scalar as f64).round() as u64);
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let quote_margin_pool_obj = tx.object(quote_margin_pool.address.clone());
        let repay_arg = tx.pure_bytes(&encode_option_u64(repay_amount_scaled));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("repay_quote"),
            vec![
                manager_obj,
                registry_obj,
                quote_margin_pool_obj,
                repay_arg,
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
        repay_coin_object_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let base_margin_pool = self.config.get_margin_pool(&pool.base_coin)?;
        let quote_margin_pool = self.config.get_margin_pool(&pool.quote_coin)?;
        let margin_pool = if debt_is_base {
            base_margin_pool
        } else {
            quote_margin_pool
        };
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
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        let repay_coin_obj = tx.object(repay_coin_object_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_manager_target("liquidate"),
            vec![
                manager_obj,
                registry_obj,
                base_price_obj,
                quote_price_obj,
                margin_pool_obj,
                pool_obj,
                repay_coin_obj,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn set_margin_manager_referral(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        referral_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _pool, base, quote) = self.manager_types(manager_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let referral_obj = tx.object(referral_id.to_string());
        Ok(tx.move_call(
            &self.margin_manager_target("set_margin_manager_referral"),
            vec![manager_obj, referral_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn unset_margin_manager_referral(
        &self,
        tx: &mut Transaction,
        manager_key: &str,
        pool_key: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let (manager, _manager_pool, base, quote) = self.manager_types(manager_key)?;
        let pool = self.config.get_pool(pool_key)?;
        let manager_obj = tx.object(manager.address.clone());
        let pool_obj = tx.object(pool.address.clone());
        Ok(tx.move_call(
            &self.margin_manager_target("unset_margin_manager_referral"),
            vec![manager_obj, pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}

pub struct DeepBookAdminContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> DeepBookAdminContract<'a> {
    fn pool_target(&self, function: &str) -> String {
        format!(
            "{}::pool::{function}",
            self.config.package_ids.deepbook_package_id
        )
    }

    fn registry_target(&self, function: &str) -> String {
        format!(
            "{}::registry::{function}",
            self.config.package_ids.deepbook_package_id
        )
    }

    pub fn create_pool_admin(
        &self,
        tx: &mut Transaction,
        base_coin_key: &str,
        quote_coin_key: &str,
        tick_size: f64,
        lot_size: f64,
        min_size: f64,
        whitelisted: bool,
        stable_pool: bool,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let base_coin = self.config.get_coin(base_coin_key)?;
        let quote_coin = self.config.get_coin(quote_coin_key)?;
        let adjusted_tick_size = ((tick_size * FLOAT_SCALAR * quote_coin.scalar as f64)
            / base_coin.scalar as f64)
            .round() as u64;
        let adjusted_lot_size = (lot_size * base_coin.scalar as f64).round() as u64;
        let adjusted_min_size = (min_size * base_coin.scalar as f64).round() as u64;
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let tick_arg = tx.pure_bytes(&encode_u64(adjusted_tick_size));
        let lot_arg = tx.pure_bytes(&encode_u64(adjusted_lot_size));
        let min_arg = tx.pure_bytes(&encode_u64(adjusted_min_size));
        let whitelist_arg = tx.pure_bytes(&encode_bool(whitelisted));
        let stable_arg = tx.pure_bytes(&encode_bool(stable_pool));
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("create_pool_admin"),
            vec![
                registry_obj,
                tick_arg,
                lot_arg,
                min_arg,
                whitelist_arg,
                stable_arg,
                cap_obj,
            ],
            vec![base_coin.type_tag.clone(), quote_coin.type_tag.clone()],
        ))
    }

    pub fn unregister_pool_admin(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("unregister_pool_admin"),
            vec![pool_obj, registry_obj, cap_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_allowed_versions(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.pool_target("update_allowed_versions"),
            vec![pool_obj, registry_obj, cap_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn enable_version(
        &self,
        tx: &mut Transaction,
        version: u64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let version_arg = tx.pure_bytes(&encode_u64(version));
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("enable_version"),
            vec![registry_obj, version_arg, cap_obj],
            vec![],
        ))
    }

    pub fn disable_version(
        &self,
        tx: &mut Transaction,
        version: u64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let version_arg = tx.pure_bytes(&encode_u64(version));
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("disable_version"),
            vec![registry_obj, version_arg, cap_obj],
            vec![],
        ))
    }

    pub fn set_treasury_address(
        &self,
        tx: &mut Transaction,
        treasury_address: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let treasury_obj = tx.object(treasury_address.to_string());
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("set_treasury_address"),
            vec![registry_obj, treasury_obj, cap_obj],
            vec![],
        ))
    }

    pub fn add_stable_coin(
        &self,
        tx: &mut Transaction,
        stable_coin_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(stable_coin_key)?;
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("add_stablecoin"),
            vec![registry_obj, cap_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn add_stablecoin(
        &self,
        tx: &mut Transaction,
        stable_coin_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.add_stable_coin(tx, stable_coin_key, admin_cap_id)
    }

    pub fn remove_stable_coin(
        &self,
        tx: &mut Transaction,
        stable_coin_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(stable_coin_key)?;
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("remove_stablecoin"),
            vec![registry_obj, cap_obj],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn remove_stablecoin(
        &self,
        tx: &mut Transaction,
        stable_coin_key: &str,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.remove_stable_coin(tx, stable_coin_key, admin_cap_id)
    }

    pub fn adjust_tick_size(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        new_tick_size: f64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let adjusted_tick_size = ((new_tick_size * FLOAT_SCALAR * quote.scalar as f64)
            / base.scalar as f64)
            .round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let tick_arg = tx.pure_bytes(&encode_u64(adjusted_tick_size));
        let cap_obj = tx.object(admin_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("adjust_tick_size_admin"),
            vec![pool_obj, tick_arg, cap_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn adjust_tick_size_admin(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        new_tick_size: f64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.adjust_tick_size(tx, pool_key, new_tick_size, admin_cap_id)
    }

    pub fn adjust_min_lot_size(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        new_lot_size: f64,
        new_min_size: f64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let adjusted_lot_size = (new_lot_size * base.scalar as f64).round() as u64;
        let adjusted_min_size = (new_min_size * base.scalar as f64).round() as u64;
        let pool_obj = tx.object(pool.address.clone());
        let lot_arg = tx.pure_bytes(&encode_u64(adjusted_lot_size));
        let min_arg = tx.pure_bytes(&encode_u64(adjusted_min_size));
        let cap_obj = tx.object(admin_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("adjust_min_lot_size_admin"),
            vec![pool_obj, lot_arg, min_arg, cap_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn adjust_min_lot_size_admin(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        new_lot_size: f64,
        new_min_size: f64,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        self.adjust_min_lot_size(tx, pool_key, new_lot_size, new_min_size, admin_cap_id)
    }

    pub fn init_balance_manager_map(
        &self,
        tx: &mut Transaction,
        admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let admin_cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("init_balance_manager_map"),
            vec![registry_obj, admin_cap_obj],
            vec![],
        ))
    }

    pub fn set_ewma_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        admin_cap_id: &str,
        alpha: f64,
        z_score_threshold: f64,
        additional_taker_fee: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let admin_cap_obj = tx.object(admin_cap_id.to_string());
        let alpha_arg = tx.pure_bytes(&encode_u64((alpha * FLOAT_SCALAR).round() as u64));
        let z_score_arg = tx.pure_bytes(&encode_u64(
            (z_score_threshold * FLOAT_SCALAR).round() as u64
        ));
        let fee_arg = tx.pure_bytes(&encode_u64(
            (additional_taker_fee * FLOAT_SCALAR).round() as u64
        ));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("set_ewma_params"),
            vec![
                pool_obj,
                admin_cap_obj,
                alpha_arg,
                z_score_arg,
                fee_arg,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn enable_ewma_state(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        admin_cap_id: &str,
        enabled: bool,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let pool_obj = tx.object(pool.address.clone());
        let admin_cap_obj = tx.object(admin_cap_id.to_string());
        let enabled_arg = tx.pure_bytes(&encode_bool(enabled));
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.pool_target("enable_ewma_state"),
            vec![pool_obj, admin_cap_obj, enabled_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn authorize_app(
        &self,
        tx: &mut Transaction,
        admin_cap_id: &str,
        app_type_tag: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let admin_cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("authorize_app"),
            vec![registry_obj, admin_cap_obj],
            vec![app_type_tag.to_string()],
        ))
    }

    pub fn deauthorize_app(
        &self,
        tx: &mut Transaction,
        admin_cap_id: &str,
        app_type_tag: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.registry_id.clone());
        let admin_cap_obj = tx.object(admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.registry_target("deauthorize_app"),
            vec![registry_obj, admin_cap_obj],
            vec![app_type_tag.to_string()],
        ))
    }
}

pub struct MarginAdminContract<'a> {
    pub config: &'a DeepBookConfig,
}

impl<'a> MarginAdminContract<'a> {
    fn margin_registry_target(&self, function: &str) -> String {
        format!(
            "{}::margin_registry::{function}",
            self.config.package_ids.margin_package_id
        )
    }

    pub fn mint_maintainer_cap(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let cap_obj = tx.object(margin_admin_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("mint_maintainer_cap"),
            vec![registry_obj, cap_obj, clock_obj],
            vec![],
        ))
    }

    pub fn revoke_maintainer_cap(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
        maintainer_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let maintainer_cap_obj = tx.object(maintainer_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("revoke_maintainer_cap"),
            vec![registry_obj, admin_cap_obj, maintainer_cap_obj, clock_obj],
            vec![],
        ))
    }

    pub fn register_deepbook_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_admin_cap_id: &str,
        pool_config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("register_deepbook_pool"),
            vec![
                registry_obj,
                admin_cap_obj,
                pool_obj,
                pool_config_object,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn enable_deepbook_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("enable_deepbook_pool"),
            vec![registry_obj, admin_cap_obj, pool_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn disable_deepbook_pool(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("disable_deepbook_pool"),
            vec![registry_obj, admin_cap_obj, pool_obj, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn update_risk_params(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        margin_admin_cap_id: &str,
        pool_config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let pool_obj = tx.object(pool.address.clone());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("update_risk_params"),
            vec![
                registry_obj,
                admin_cap_obj,
                pool_obj,
                pool_config_object,
                clock_obj,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn add_config(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
        config_object: serde_json::Value,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("add_config"),
            vec![registry_obj, admin_cap_obj, config_object],
            vec![format!(
                "{}::oracle::PythConfig",
                self.config.package_ids.margin_package_id
            )],
        ))
    }

    pub fn remove_config(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("remove_config"),
            vec![registry_obj, admin_cap_obj],
            vec![format!(
                "{}::oracle::PythConfig",
                self.config.package_ids.margin_package_id
            )],
        ))
    }

    pub fn new_pool_config(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        min_withdraw_risk_ratio: f64,
        min_borrow_risk_ratio: f64,
        liquidation_risk_ratio: f64,
        target_liquidation_risk_ratio: f64,
        user_liquidation_reward: f64,
        pool_liquidation_reward: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let min_withdraw_arg = tx.pure_bytes(&encode_u64(
            (min_withdraw_risk_ratio * FLOAT_SCALAR).round() as u64,
        ));
        let min_borrow_arg = tx.pure_bytes(&encode_u64(
            (min_borrow_risk_ratio * FLOAT_SCALAR).round() as u64,
        ));
        let liquidation_arg = tx.pure_bytes(&encode_u64(
            (liquidation_risk_ratio * FLOAT_SCALAR).round() as u64,
        ));
        let target_liquidation_arg = tx.pure_bytes(&encode_u64(
            (target_liquidation_risk_ratio * FLOAT_SCALAR).round() as u64,
        ));
        let user_reward_arg = tx.pure_bytes(&encode_u64(
            (user_liquidation_reward * FLOAT_SCALAR).round() as u64,
        ));
        let pool_reward_arg = tx.pure_bytes(&encode_u64(
            (pool_liquidation_reward * FLOAT_SCALAR).round() as u64,
        ));
        Ok(tx.move_call(
            &self.margin_registry_target("new_pool_config"),
            vec![
                registry_obj,
                min_withdraw_arg,
                min_borrow_arg,
                liquidation_arg,
                target_liquidation_arg,
                user_reward_arg,
                pool_reward_arg,
            ],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn new_pool_config_with_leverage(
        &self,
        tx: &mut Transaction,
        pool_key: &str,
        leverage: f64,
    ) -> Result<serde_json::Value, ContractError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let leverage_arg = tx.pure_bytes(&encode_u64((leverage * FLOAT_SCALAR).round() as u64));
        Ok(tx.move_call(
            &self.margin_registry_target("new_pool_config_with_leverage"),
            vec![registry_obj, leverage_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }

    pub fn new_coin_type_data_from_currency(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        currency_id: &str,
        price_feed_id_bytes: &[u8],
        max_conf_bps: u64,
        max_ewma_difference_bps: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let currency_obj = tx.object(currency_id.to_string());
        let feed_id_arg = tx.pure_bytes(price_feed_id_bytes);
        let max_conf_arg = tx.pure_bytes(&encode_u64(max_conf_bps));
        let max_ewma_arg = tx.pure_bytes(&encode_u64(max_ewma_difference_bps));
        Ok(tx.move_call(
            &format!(
                "{}::oracle::new_coin_type_data_from_currency",
                self.config.package_ids.margin_package_id
            ),
            vec![currency_obj, feed_id_arg, max_conf_arg, max_ewma_arg],
            vec![coin.type_tag.clone()],
        ))
    }

    pub fn new_pyth_config(
        &self,
        tx: &mut Transaction,
        coin_type_data_vector: serde_json::Value,
        max_age_seconds: u64,
    ) -> Result<serde_json::Value, ContractError> {
        let max_age_arg = tx.pure_bytes(&encode_u64(max_age_seconds));
        Ok(tx.move_call(
            &format!(
                "{}::oracle::new_pyth_config",
                self.config.package_ids.margin_package_id
            ),
            vec![coin_type_data_vector, max_age_arg],
            vec![],
        ))
    }

    pub fn enable_version(
        &self,
        tx: &mut Transaction,
        version: u64,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let version_arg = tx.pure_bytes(&encode_u64(version));
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("enable_version"),
            vec![registry_obj, version_arg, admin_cap_obj],
            vec![],
        ))
    }

    pub fn disable_version(
        &self,
        tx: &mut Transaction,
        version: u64,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let version_arg = tx.pure_bytes(&encode_u64(version));
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("disable_version"),
            vec![registry_obj, version_arg, admin_cap_obj],
            vec![],
        ))
    }

    pub fn mint_pause_cap(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        Ok(tx.move_call(
            &self.margin_registry_target("mint_pause_cap"),
            vec![registry_obj, admin_cap_obj, clock_obj],
            vec![],
        ))
    }

    pub fn revoke_pause_cap(
        &self,
        tx: &mut Transaction,
        margin_admin_cap_id: &str,
        pause_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        let clock_obj = tx.object("0x6");
        let pause_cap_obj = tx.object(pause_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("revoke_pause_cap"),
            vec![registry_obj, admin_cap_obj, clock_obj, pause_cap_obj],
            vec![],
        ))
    }

    pub fn disable_version_pause_cap(
        &self,
        tx: &mut Transaction,
        version: u64,
        pause_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let version_arg = tx.pure_bytes(&encode_u64(version));
        let pause_cap_obj = tx.object(pause_cap_id.to_string());
        Ok(tx.move_call(
            &self.margin_registry_target("disable_version_pause_cap"),
            vec![registry_obj, version_arg, pause_cap_obj],
            vec![],
        ))
    }

    pub fn admin_withdraw_default_referral_fees(
        &self,
        tx: &mut Transaction,
        coin_key: &str,
        margin_admin_cap_id: &str,
    ) -> Result<serde_json::Value, ContractError> {
        let coin = self.config.get_coin(coin_key)?;
        let margin_pool = self.config.get_margin_pool(coin_key)?;
        let margin_pool_obj = tx.object(margin_pool.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let admin_cap_obj = tx.object(margin_admin_cap_id.to_string());
        Ok(tx.move_call(
            &format!(
                "{}::margin_pool::admin_withdraw_default_referral_fees",
                self.config.package_ids.margin_package_id
            ),
            vec![margin_pool_obj, registry_obj, admin_cap_obj],
            vec![coin.type_tag.clone()],
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
        let proof = self
            .balance_manager
            .generate_proof(tx, balance_manager_key)?;
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        let proposal_arg = tx.pure_bytes(&encode_u128(proposal_id));

        Ok(tx.move_call(
            &format!(
                "{}::pool::vote",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj, proof, proposal_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        ))
    }
}
