use base64::Engine as _;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{ConfigError, DEEP_SCALAR, FLOAT_SCALAR, DeepBookConfig};
use crate::contracts::{
    BalanceManagerContract, ContractError, DeepBookContract, MarginManagerContract,
    MarginPoolContract, MarginRegistryContract, MarginTPSLContract,
};
use crate::encode::encode_vec_u128;
use sui::jsonrpc;
use sui::utils::normalize_sui_address;
use sui::transactions::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Contract(#[from] ContractError),
    #[error(transparent)]
    JsonRpc(#[from] jsonrpc::JsonRpcError),
    #[error("missing commandResults[{0}]")]
    MissingCommandResult(usize),
    #[error("missing returnValues[{return_index}] in commandResults[{command_index}]")]
    MissingReturnValue { command_index: usize, return_index: usize },
    #[error("invalid bcs return value")]
    InvalidBcsValue,
    #[error("missing price_info_object_id for coin `{0}`")]
    MissingPriceInfoObject(String),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
}

pub struct DeepBookClient {
    pub client: jsonrpc::Client,
    pub config: DeepBookConfig,
}

impl DeepBookClient {
    fn format_token_amount(raw_amount: u64, scalar: u64, decimals: u32) -> String {
        let scalar_u128 = scalar as u128;
        let raw = raw_amount as u128;
        let integer = raw / scalar_u128;
        let fractional = raw % scalar_u128;
        if decimals == 0 {
            return integer.to_string();
        }
        let scale_digits = scalar.to_string().len().saturating_sub(1) as u32;
        let use_decimals = decimals.min(scale_digits);
        let divisor = 10u128.pow(scale_digits.saturating_sub(use_decimals));
        let frac = fractional / divisor;
        format!("{integer}.{frac:0width$}", width = use_decimals as usize)
    }

    fn read_bool(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<bool, ClientError> {
        let b = self.return_bcs(sim, command_index, return_index)?;
        Ok(!b.is_empty() && b[0] == 1)
    }

    fn read_address(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<String, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        Ok(normalize_sui_address(&format!("0x{}", hex::encode(raw))))
    }

    fn read_u8(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<u8, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        raw.first().copied().ok_or(ClientError::InvalidBcsValue)
    }

    fn read_uleb128(raw: &[u8], start: usize) -> Result<(usize, usize), ClientError> {
        let mut result: usize = 0;
        let mut shift = 0usize;
        let mut idx = start;
        while idx < raw.len() {
            let byte = raw[idx];
            result |= ((byte & 0x7F) as usize) << shift;
            idx += 1;
            if byte & 0x80 == 0 {
                return Ok((result, idx - start));
            }
            shift += 7;
            if shift > (std::mem::size_of::<usize>() * 8) {
                return Err(ClientError::InvalidBcsValue);
            }
        }
        Err(ClientError::InvalidBcsValue)
    }

    fn read_vec_u64(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<Vec<u64>, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        let (len, used) = Self::read_uleb128(&raw, 0)?;
        let mut idx = used;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            if raw.len() < idx + 8 {
                return Err(ClientError::InvalidBcsValue);
            }
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&raw[idx..idx + 8]);
            out.push(u64::from_le_bytes(arr));
            idx += 8;
        }
        Ok(out)
    }

    fn read_vec_set_addresses(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<Vec<String>, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        let (len, used) = Self::read_uleb128(&raw, 0)?;
        let mut idx = used;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            if raw.len() < idx + 32 {
                return Err(ClientError::InvalidBcsValue);
            }
            let addr = normalize_sui_address(&format!("0x{}", hex::encode(&raw[idx..idx + 32])));
            out.push(addr);
            idx += 32;
        }
        Ok(out)
    }

    pub fn new(client: jsonrpc::Client, config: DeepBookConfig) -> Self {
        Self { client, config }
    }

    pub async fn check_manager_balance(
        &self,
        manager_key: &str,
        coin_key: &str,
    ) -> Result<Value, ClientError> {
        let manager = self.config.get_balance_manager(manager_key)?;
        let coin = self.config.get_coin(coin_key)?;

        let mut tx = Transaction::new();
        let manager_obj = tx.object(manager.address.clone());
        tx.move_call(
            &format!(
                "{}::balance_manager::balance",
                self.config.package_ids.deepbook_package_id
            ),
            vec![manager_obj],
            vec![coin.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        let balance = self.read_u64(&sim, 0, 0)?;

        Ok(json!({
            "coinType": coin.type_tag,
            "balance": balance as f64 / coin.scalar as f64,
        }))
    }

    pub async fn whitelisted(&self, pool_key: &str) -> Result<bool, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!("{}::pool::whitelisted", self.config.package_ids.deepbook_package_id),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        let b = self.return_bcs(&sim, 0, 0)?;
        Ok(!b.is_empty() && b[0] == 1)
    }

    pub async fn get_quote_quantity_out(
        &self,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<Value, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };

        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        deepbook.get_quote_quantity_out(&mut tx, pool_key, base_quantity)?;
        let sim = self.simulate(&tx).await?;

        let base_out = self.read_u64(&sim, 0, 0)?;
        let quote_out = self.read_u64(&sim, 0, 1)?;
        let deep_required = self.read_u64(&sim, 0, 2)?;

        Ok(json!({
            "baseQuantity": base_quantity,
            "baseOut": base_out as f64 / base.scalar as f64,
            "quoteOut": quote_out as f64 / quote.scalar as f64,
            "deepRequired": deep_required as f64 / DEEP_SCALAR,
        }))
    }

    pub async fn get_base_quantity_out(
        &self,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<Value, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let quantity = (quote_quantity * quote.scalar as f64).round() as u64;
        let qty_arg = tx.pure_bytes(&quantity.to_le_bytes());
        let clock_obj = tx.object("0x6");
        tx.move_call(
            &format!(
                "{}::pool::get_base_quantity_out",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, qty_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        let base_out = self.read_u64(&sim, 0, 0)?;
        let quote_out = self.read_u64(&sim, 0, 1)?;
        let deep_required = self.read_u64(&sim, 0, 2)?;

        Ok(json!({
            "quoteQuantity": quote_quantity,
            "baseOut": base_out as f64 / base.scalar as f64,
            "quoteOut": quote_out as f64 / quote.scalar as f64,
            "deepRequired": deep_required as f64 / DEEP_SCALAR,
        }))
    }

    pub async fn get_quantity_out(
        &self,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<Value, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let base_qty = (base_quantity * base.scalar as f64).round() as u64;
        let quote_qty = (quote_quantity * quote.scalar as f64).round() as u64;
        let base_arg = tx.pure_bytes(&base_qty.to_le_bytes());
        let quote_arg = tx.pure_bytes(&quote_qty.to_le_bytes());
        let clock_obj = tx.object("0x6");
        tx.move_call(
            &format!(
                "{}::pool::get_quantity_out",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, base_arg, quote_arg, clock_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        let base_out = self.read_u64(&sim, 0, 0)?;
        let quote_out = self.read_u64(&sim, 0, 1)?;
        let deep_required = self.read_u64(&sim, 0, 2)?;

        Ok(json!({
            "baseQuantity": base_quantity,
            "quoteQuantity": quote_quantity,
            "baseOut": base_out as f64 / base.scalar as f64,
            "quoteOut": quote_out as f64 / quote.scalar as f64,
            "deepRequired": deep_required as f64 / DEEP_SCALAR,
        }))
    }

    pub async fn mid_price(&self, pool_key: &str) -> Result<f64, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!("{}::pool::mid_price", self.config.package_ids.deepbook_package_id),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok((value as f64 * base.scalar as f64) / (FLOAT_SCALAR * quote.scalar as f64))
    }

    pub async fn get_order(&self, pool_key: &str, order_id: u128) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let order_arg = tx.pure_bytes(&order_id.to_le_bytes());
        tx.move_call(
            &format!("{}::pool::get_order", self.config.package_ids.deepbook_package_id),
            vec![pool_obj, order_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_orders(&self, pool_key: &str, order_ids: &[u128]) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let order_ids_arg = tx.pure_bytes(&encode_vec_u128(order_ids));
        tx.move_call(
            &format!("{}::pool::get_orders", self.config.package_ids.deepbook_package_id),
            vec![pool_obj, order_ids_arg],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_order_normalized(
        &self,
        pool_key: &str,
        order_id: u128,
    ) -> Result<Value, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let raw_order_bcs = self.get_order(pool_key, order_id).await?;
        let (is_bid, raw_price, decoded_order_id) = self.decode_order_id(order_id);
        let normalized_price = (raw_price as f64 * base.scalar as f64) / (FLOAT_SCALAR * quote.scalar as f64);
        Ok(json!({
            "orderId": order_id.to_string(),
            "decodedOrderId": decoded_order_id.to_string(),
            "isBid": is_bid,
            "rawPrice": raw_price.to_string(),
            "normalizedPrice": format!("{normalized_price:.9}"),
            "rawOrderBcs": raw_order_bcs,
        }))
    }

    pub async fn account_open_orders(
        &self,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::account_open_orders",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn vault_balances(&self, pool_key: &str) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!("{}::pool::vault_balances", self.config.package_ids.deepbook_package_id),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_pool_id_by_assets(
        &self,
        base_type: &str,
        quote_type: &str,
    ) -> Result<String, ClientError> {
        let mut tx = Transaction::new();
        tx.move_call(
            &format!(
                "{}::pool::get_pool_id_by_asset",
                self.config.package_ids.deepbook_package_id
            ),
            vec![],
            vec![base_type.to_string(), quote_type.to_string()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn pool_trade_params(&self, pool_key: &str) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::pool_trade_params",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn pool_book_params(&self, pool_key: &str) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::pool_book_params",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn account(&self, pool_key: &str, manager_key: &str) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        tx.move_call(
            &format!("{}::pool::account", self.config.package_ids.deepbook_package_id),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn locked_balance(
        &self,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let manager = self.config.get_balance_manager(manager_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        let manager_obj = tx.object(manager.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::locked_balance",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, manager_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_pool_deep_price(&self, pool_key: &str) -> Result<String, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::get_order_deep_price",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub fn decode_order_id(&self, encoded_order_id: u128) -> (bool, u64, u64) {
        let is_bid = (encoded_order_id >> 127) == 0;
        let price = ((encoded_order_id >> 64) & ((1u128 << 63) - 1)) as u64;
        let order_id = (encoded_order_id & ((1u128 << 64) - 1)) as u64;
        (is_bid, price, order_id)
    }

    pub async fn balance_manager_referral_owner(
        &self,
        referral_id: &str,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let mut tx = Transaction::new();
        bm.balance_manager_referral_owner(&mut tx, referral_id)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    fn price_info_object_age_internal(&self, coin_key: &str) -> Result<i64, ClientError> {
        let coin = self.config.get_coin(coin_key)?;
        match coin.price_info_object_id.as_deref() {
            Some(id) if !id.is_empty() => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0);
                Ok(now)
            }
            _ => Ok(-1),
        }
    }

    pub async fn get_price_info_object_age(&self, coin_key: &str) -> Result<i64, ClientError> {
        self.price_info_object_age_internal(coin_key)
    }

    pub fn get_price_info_object_age_sync(&self, coin_key: &str) -> Result<i64, ClientError> {
        self.price_info_object_age_internal(coin_key)
    }

    pub async fn get_price_info_object(&self, coin_key: &str) -> Result<String, ClientError> {
        let coin = self.config.get_coin(coin_key)?;
        let price_info = coin
            .price_info_object_id
            .clone()
            .ok_or_else(|| ClientError::MissingPriceInfoObject(coin_key.to_string()))?;
        Ok(normalize_sui_address(&price_info))
    }

    pub async fn get_price_info_objects(
        &self,
        coin_keys: &[&str],
    ) -> Result<Value, ClientError> {
        let mut out = serde_json::Map::new();
        for key in coin_keys {
            let id = self.get_price_info_object(key).await?;
            out.insert((*key).to_string(), Value::String(id));
        }
        Ok(Value::Object(out))
    }

    pub async fn get_quote_quantity_out_input_fee(
        &self,
        pool_key: &str,
        base_quantity: f64,
    ) -> Result<Value, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_quote_quantity_out_input_fee(&mut tx, pool_key, base_quantity)?;
        let sim = self.simulate(&tx).await?;
        let result = self.read_u64(&sim, 0, 0)?;
        Ok(json!({
            "baseQuantity": base_quantity,
            "result": result,
        }))
    }

    pub async fn get_base_quantity_out_input_fee(
        &self,
        pool_key: &str,
        quote_quantity: f64,
    ) -> Result<Value, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_base_quantity_out_input_fee(&mut tx, pool_key, quote_quantity)?;
        let sim = self.simulate(&tx).await?;
        let result = self.read_u64(&sim, 0, 0)?;
        Ok(json!({
            "quoteQuantity": quote_quantity,
            "result": result,
        }))
    }

    pub async fn get_quantity_out_input_fee(
        &self,
        pool_key: &str,
        base_quantity: f64,
        quote_quantity: f64,
    ) -> Result<Value, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_quantity_out_input_fee(&mut tx, pool_key, base_quantity, quote_quantity)?;
        let sim = self.simulate(&tx).await?;
        let result = self.read_u64(&sim, 0, 0)?;
        Ok(json!({
            "baseQuantity": base_quantity,
            "quoteQuantity": quote_quantity,
            "result": result,
        }))
    }

    pub async fn get_base_quantity_in(
        &self,
        pool_key: &str,
        target_quote_quantity: f64,
        pay_with_deep: bool,
    ) -> Result<u64, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_base_quantity_in(&mut tx, pool_key, target_quote_quantity, pay_with_deep)?;
        let sim = self.simulate(&tx).await?;
        self.read_u64(&sim, 0, 0)
    }

    pub async fn get_quote_quantity_in(
        &self,
        pool_key: &str,
        target_base_quantity: f64,
        pay_with_deep: bool,
    ) -> Result<u64, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_quote_quantity_in(&mut tx, pool_key, target_base_quantity, pay_with_deep)?;
        let sim = self.simulate(&tx).await?;
        self.read_u64(&sim, 0, 0)
    }

    pub async fn get_account_order_details(
        &self,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_account_order_details(&mut tx, pool_key, manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_order_deep_required(
        &self,
        pool_key: &str,
        base_quantity: f64,
        price: f64,
    ) -> Result<u64, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_order_deep_required(&mut tx, pool_key, base_quantity, price)?;
        let sim = self.simulate(&tx).await?;
        self.read_u64(&sim, 0, 0)
    }

    pub async fn pool_trade_params_next(&self, pool_key: &str) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.pool_trade_params_next(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_level2_range(
        &self,
        pool_key: &str,
        price_low: f64,
        price_high: f64,
        is_bid: bool,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_level2_range(&mut tx, pool_key, price_low, price_high, is_bid)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_level2_ticks_from_mid(
        &self,
        pool_key: &str,
        tick_from_mid: u64,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_level2_ticks_from_mid(&mut tx, pool_key, tick_from_mid)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn account_exists(
        &self,
        pool_key: &str,
        manager_key: &str,
    ) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.account_exists(&mut tx, pool_key, manager_key)?;
        let sim = self.simulate(&tx).await?;
        let b = self.return_bcs(&sim, 0, 0)?;
        Ok(!b.is_empty() && b[0] == 1)
    }

    pub async fn quorum(&self, pool_key: &str) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.quorum(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn pool_id(&self, pool_key: &str) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.pool_id(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_margin_account_order_details(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;

        let mut tx = Transaction::new();
        let manager_obj = tx.object(manager.address.clone());
        let registry_obj = tx.object(self.config.package_ids.margin_registry_id.clone());
        let bm = tx.move_call(
            &format!(
                "{}::margin_manager::balance_manager",
                self.config.package_ids.margin_package_id
            ),
            vec![manager_obj, registry_obj],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );
        let pool_obj = tx.object(pool.address.clone());
        tx.move_call(
            &format!(
                "{}::pool::get_account_order_details",
                self.config.package_ids.deepbook_package_id
            ),
            vec![pool_obj, bm],
            vec![base.type_tag.clone(), quote.type_tag.clone()],
        );

        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 1, 0)
    }

    pub async fn get_balance_manager_ids(&self, owner: &str) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_balance_manager_ids(&mut tx, owner)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_pool_referral_balances(
        &self,
        pool_key: &str,
        referral_id: &str,
    ) -> Result<Value, ClientError> {
        let pool = self.config.get_pool(pool_key)?;
        let base = self.config.get_coin(&pool.base_coin)?;
        let quote = self.config.get_coin(&pool.quote_coin)?;
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.get_pool_referral_balances(&mut tx, pool_key, referral_id)?;
        let sim = self.simulate(&tx).await?;
        let base_bal = self.read_u64(&sim, 0, 0)?;
        let quote_bal = self.read_u64(&sim, 0, 1)?;
        let deep_bal = self.read_u64(&sim, 0, 2)?;
        Ok(json!({
            "base": base_bal as f64 / base.scalar as f64,
            "quote": quote_bal as f64 / quote.scalar as f64,
            "deep": deep_bal as f64 / DEEP_SCALAR,
        }))
    }

    pub async fn balance_manager_referral_pool_id(
        &self,
        referral_id: &str,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let mut tx = Transaction::new();
        bm.balance_manager_referral_pool_id(&mut tx, referral_id)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn pool_referral_multiplier(
        &self,
        pool_key: &str,
        referral_id: &str,
    ) -> Result<f64, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.pool_referral_multiplier(&mut tx, pool_key, referral_id)?;
        let sim = self.simulate(&tx).await?;
        let multiplier = self.read_u64(&sim, 0, 0)?;
        Ok(multiplier as f64 / FLOAT_SCALAR)
    }

    pub async fn get_balance_manager_referral_id(
        &self,
        manager_key: &str,
        pool_key: &str,
    ) -> Result<String, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let mut tx = Transaction::new();
        bm.get_balance_manager_referral_id(&mut tx, manager_key, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn stable_pool(&self, pool_key: &str) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.stable_pool(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn registered_pool(&self, pool_key: &str) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.registered_pool(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn can_place_limit_order(
        &self,
        pool_key: &str,
        balance_manager_key: &str,
        price: f64,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
        expire_timestamp: u64,
    ) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.can_place_limit_order(
            &mut tx,
            pool_key,
            balance_manager_key,
            price,
            quantity,
            is_bid,
            pay_with_deep,
            Some(expire_timestamp),
        )?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn can_place_market_order(
        &self,
        pool_key: &str,
        balance_manager_key: &str,
        quantity: f64,
        is_bid: bool,
        pay_with_deep: bool,
    ) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.can_place_market_order(
            &mut tx,
            pool_key,
            balance_manager_key,
            quantity,
            is_bid,
            pay_with_deep,
        )?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn check_market_order_params(
        &self,
        pool_key: &str,
        quantity: f64,
    ) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.check_market_order_params(&mut tx, pool_key, quantity)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn check_limit_order_params(
        &self,
        pool_key: &str,
        price: f64,
        quantity: f64,
        expire_timestamp: u64,
    ) -> Result<bool, ClientError> {
        let bm = BalanceManagerContract { config: &self.config };
        let deepbook = DeepBookContract {
            config: &self.config,
            balance_manager: bm,
        };
        let mut tx = Transaction::new();
        deepbook.check_limit_order_params(&mut tx, pool_key, price, quantity, expire_timestamp)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn get_margin_pool_id(&self, coin_key: &str) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.get_id(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_deepbook_pool_margin_pool_ids(
        &self,
        pool_key: &str,
    ) -> Result<String, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.get_deepbook_pool_margin_pool_ids(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn is_deepbook_pool_allowed(
        &self,
        coin_key: &str,
        deepbook_pool_id: &str,
    ) -> Result<bool, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.deepbook_pool_allowed(&mut tx, coin_key, deepbook_pool_id)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn get_margin_pool_total_supply(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.total_supply(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_supply_shares(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.supply_shares(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_total_borrow(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.total_borrow(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_borrow_shares(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.borrow_shares(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_last_update_timestamp(
        &self,
        coin_key: &str,
    ) -> Result<u64, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.last_update_timestamp(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_u64(&sim, 0, 0)
    }

    pub async fn get_margin_pool_supply_cap(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.supply_cap(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_max_utilization_rate(
        &self,
        coin_key: &str,
    ) -> Result<f64, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.max_utilization_rate(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(value as f64 / FLOAT_SCALAR)
    }

    pub async fn get_margin_pool_protocol_spread(
        &self,
        coin_key: &str,
    ) -> Result<f64, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.protocol_spread(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(value as f64 / FLOAT_SCALAR)
    }

    pub async fn get_margin_pool_min_borrow(
        &self,
        coin_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.min_borrow(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_pool_interest_rate(&self, coin_key: &str) -> Result<f64, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.interest_rate(&mut tx, coin_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(value as f64 / FLOAT_SCALAR)
    }

    pub async fn get_user_supply_shares(
        &self,
        coin_key: &str,
        supplier_cap_id: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.user_supply_shares(&mut tx, coin_key, supplier_cap_id)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_user_supply_amount(
        &self,
        coin_key: &str,
        supplier_cap_id: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_pool = MarginPoolContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_pool.user_supply_amount(&mut tx, coin_key, supplier_cap_id)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        let coin = self.config.get_coin(coin_key)?;
        Ok(Self::format_token_amount(value, coin.scalar, decimals))
    }

    pub async fn get_margin_manager_owner(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.owner(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_margin_manager_deepbook_pool(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.deepbook_pool(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_margin_manager_margin_pool_id(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.margin_pool_id(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_margin_manager_borrowed_shares(
        &self,
        margin_manager_key: &str,
    ) -> Result<Value, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.borrowed_shares(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let base_shares = self.read_u64(&sim, 0, 0)?;
        let quote_shares = self.read_u64(&sim, 0, 1)?;
        Ok(json!({
            "baseShares": base_shares.to_string(),
            "quoteShares": quote_shares.to_string(),
        }))
    }

    pub async fn get_margin_manager_borrowed_base_shares(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.borrowed_base_shares(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)?.to_string())
    }

    pub async fn get_margin_manager_borrowed_quote_shares(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.borrowed_quote_shares(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)?.to_string())
    }

    pub async fn get_margin_manager_has_base_debt(
        &self,
        margin_manager_key: &str,
    ) -> Result<bool, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.has_base_debt(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn get_margin_manager_balance_manager_id(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_manager.balance_manager(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_margin_manager_assets(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<Value, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;
        let mut tx = Transaction::new();
        margin_manager.calculate_assets(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let base_asset = self.read_u64(&sim, 0, 0)?;
        let quote_asset = self.read_u64(&sim, 0, 1)?;
        Ok(json!({
            "baseAsset": Self::format_token_amount(base_asset, base_coin.scalar, decimals),
            "quoteAsset": Self::format_token_amount(quote_asset, quote_coin.scalar, decimals),
        }))
    }

    pub async fn get_margin_manager_debts(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<Value, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let has_base_debt = self.get_margin_manager_has_base_debt(margin_manager_key).await?;
        let debt_coin_key = if has_base_debt {
            pool.base_coin.as_str()
        } else {
            pool.quote_coin.as_str()
        };
        let debt_coin = self.config.get_coin(debt_coin_key)?;

        let mut tx = Transaction::new();
        margin_manager.calculate_debts(&mut tx, margin_manager_key, debt_coin_key)?;
        let sim = self.simulate(&tx).await?;
        let base_debt = self.read_u64(&sim, 0, 0)?;
        let quote_debt = self.read_u64(&sim, 0, 1)?;
        Ok(json!({
            "baseDebt": Self::format_token_amount(base_debt, debt_coin.scalar, decimals),
            "quoteDebt": Self::format_token_amount(quote_debt, debt_coin.scalar, decimals),
        }))
    }

    pub async fn get_margin_manager_base_balance(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let mut tx = Transaction::new();
        margin_manager.base_balance(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(Self::format_token_amount(value, base_coin.scalar, decimals))
    }

    pub async fn get_margin_manager_quote_balance(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;
        let mut tx = Transaction::new();
        margin_manager.quote_balance(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(Self::format_token_amount(value, quote_coin.scalar, decimals))
    }

    pub async fn get_margin_manager_deep_balance(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<String, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let deep_coin = self.config.get_coin("DEEP")?;
        let mut tx = Transaction::new();
        margin_manager.deep_balance(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let value = self.read_u64(&sim, 0, 0)?;
        Ok(Self::format_token_amount(value, deep_coin.scalar, decimals))
    }

    pub async fn get_margin_manager_state(
        &self,
        margin_manager_key: &str,
        decimals: u32,
    ) -> Result<Value, ClientError> {
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let manager = self.config.get_margin_manager(margin_manager_key)?;
        let pool = self.config.get_pool(&manager.pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;
        let mut tx = Transaction::new();
        margin_manager.manager_state(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;

        let manager_id = self.read_address(&sim, 0, 0)?;
        let deepbook_pool_id = self.read_address(&sim, 0, 1)?;
        let risk_ratio = self.read_u64(&sim, 0, 2)? as f64 / FLOAT_SCALAR;
        let base_asset = Self::format_token_amount(self.read_u64(&sim, 0, 3)?, base_coin.scalar, decimals);
        let quote_asset =
            Self::format_token_amount(self.read_u64(&sim, 0, 4)?, quote_coin.scalar, decimals);
        let base_debt = Self::format_token_amount(self.read_u64(&sim, 0, 5)?, base_coin.scalar, decimals);
        let quote_debt =
            Self::format_token_amount(self.read_u64(&sim, 0, 6)?, quote_coin.scalar, decimals);
        let base_pyth_price = self.read_u64(&sim, 0, 7)?.to_string();
        let base_pyth_decimals = self.read_u8(&sim, 0, 8)? as u64;
        let quote_pyth_price = self.read_u64(&sim, 0, 9)?.to_string();
        let quote_pyth_decimals = self.read_u8(&sim, 0, 10)? as u64;
        let current_price = self.read_u64(&sim, 0, 11)?.to_string();
        let lowest_trigger_above_price = self.read_u64(&sim, 0, 12)?.to_string();
        let highest_trigger_below_price = self.read_u64(&sim, 0, 13)?.to_string();

        Ok(json!({
            "managerId": manager_id,
            "deepbookPoolId": deepbook_pool_id,
            "riskRatio": risk_ratio,
            "baseAsset": base_asset,
            "quoteAsset": quote_asset,
            "baseDebt": base_debt,
            "quoteDebt": quote_debt,
            "basePythPrice": base_pyth_price,
            "basePythDecimals": base_pyth_decimals,
            "quotePythPrice": quote_pyth_price,
            "quotePythDecimals": quote_pyth_decimals,
            "currentPrice": current_price,
            "lowestTriggerAbovePrice": lowest_trigger_above_price,
            "highestTriggerBelowPrice": highest_trigger_below_price,
        }))
    }

    pub async fn get_margin_manager_states(
        &self,
        margin_manager_keys: &[&str],
        decimals: u32,
    ) -> Result<Value, ClientError> {
        if margin_manager_keys.is_empty() {
            return Ok(json!({}));
        }
        let margin_manager = MarginManagerContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        for key in margin_manager_keys {
            margin_manager.manager_state(&mut tx, key)?;
        }
        let sim = self.simulate(&tx).await?;
        let mut out = serde_json::Map::new();

        for (idx, key) in margin_manager_keys.iter().enumerate() {
            let manager = self.config.get_margin_manager(key)?;
            let pool = self.config.get_pool(&manager.pool_key)?;
            let base_coin = self.config.get_coin(&pool.base_coin)?;
            let quote_coin = self.config.get_coin(&pool.quote_coin)?;

            let manager_id = self.read_address(&sim, idx, 0)?;
            let deepbook_pool_id = self.read_address(&sim, idx, 1)?;
            let risk_ratio = self.read_u64(&sim, idx, 2)? as f64 / FLOAT_SCALAR;
            let base_asset =
                Self::format_token_amount(self.read_u64(&sim, idx, 3)?, base_coin.scalar, decimals);
            let quote_asset =
                Self::format_token_amount(self.read_u64(&sim, idx, 4)?, quote_coin.scalar, decimals);
            let base_debt =
                Self::format_token_amount(self.read_u64(&sim, idx, 5)?, base_coin.scalar, decimals);
            let quote_debt =
                Self::format_token_amount(self.read_u64(&sim, idx, 6)?, quote_coin.scalar, decimals);
            let base_pyth_price = self.read_u64(&sim, idx, 7)?.to_string();
            let base_pyth_decimals = self.read_u8(&sim, idx, 8)? as u64;
            let quote_pyth_price = self.read_u64(&sim, idx, 9)?.to_string();
            let quote_pyth_decimals = self.read_u8(&sim, idx, 10)? as u64;
            let current_price = self.read_u64(&sim, idx, 11)?.to_string();
            let lowest_trigger_above_price = self.read_u64(&sim, idx, 12)?.to_string();
            let highest_trigger_below_price = self.read_u64(&sim, idx, 13)?.to_string();

            out.insert(
                manager_id.clone(),
                json!({
                    "managerId": manager_id,
                    "deepbookPoolId": deepbook_pool_id,
                    "riskRatio": risk_ratio,
                    "baseAsset": base_asset,
                    "quoteAsset": quote_asset,
                    "baseDebt": base_debt,
                    "quoteDebt": quote_debt,
                    "basePythPrice": base_pyth_price,
                    "basePythDecimals": base_pyth_decimals,
                    "quotePythPrice": quote_pyth_price,
                    "quotePythDecimals": quote_pyth_decimals,
                    "currentPrice": current_price,
                    "lowestTriggerAbovePrice": lowest_trigger_above_price,
                    "highestTriggerBelowPrice": highest_trigger_below_price,
                    "marginManagerKey": key,
                }),
            );
        }

        Ok(Value::Object(out))
    }

    pub async fn get_conditional_order_ids(
        &self,
        margin_manager_key: &str,
    ) -> Result<Vec<String>, ClientError> {
        let margin_tpsl = MarginTPSLContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_tpsl.conditional_order_ids(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        let ids = self.read_vec_u64(&sim, 0, 0)?;
        Ok(ids.into_iter().map(|id| id.to_string()).collect())
    }

    pub async fn get_conditional_order(
        &self,
        margin_manager_key: &str,
        conditional_order_id: u64,
    ) -> Result<String, ClientError> {
        let margin_tpsl = MarginTPSLContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_tpsl.conditional_order(&mut tx, margin_manager_key, conditional_order_id)?;
        let sim = self.simulate(&tx).await?;
        self.read_return_bcs_base64(&sim, 0, 0)
    }

    pub async fn get_lowest_trigger_above_price(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_tpsl = MarginTPSLContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_tpsl.lowest_trigger_above_price(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)?.to_string())
    }

    pub async fn get_highest_trigger_below_price(
        &self,
        margin_manager_key: &str,
    ) -> Result<String, ClientError> {
        let margin_tpsl = MarginTPSLContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_tpsl.highest_trigger_below_price(&mut tx, margin_manager_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)?.to_string())
    }

    pub async fn is_pool_enabled_for_margin(&self, pool_key: &str) -> Result<bool, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.pool_enabled(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_bool(&sim, 0, 0)
    }

    pub async fn get_margin_manager_ids_for_owner(
        &self,
        owner: &str,
    ) -> Result<Vec<String>, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.get_margin_manager_ids(&mut tx, owner)?;
        let sim = self.simulate(&tx).await?;
        self.read_vec_set_addresses(&sim, 0, 0)
    }

    pub async fn get_base_margin_pool_id(&self, pool_key: &str) -> Result<String, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.base_margin_pool_id(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_quote_margin_pool_id(&self, pool_key: &str) -> Result<String, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.quote_margin_pool_id(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        self.read_address(&sim, 0, 0)
    }

    pub async fn get_min_withdraw_risk_ratio(&self, pool_key: &str) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.min_withdraw_risk_ratio(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_min_borrow_risk_ratio(&self, pool_key: &str) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.min_borrow_risk_ratio(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_liquidation_risk_ratio(&self, pool_key: &str) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.liquidation_risk_ratio(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_target_liquidation_risk_ratio(
        &self,
        pool_key: &str,
    ) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.target_liquidation_risk_ratio(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_user_liquidation_reward(&self, pool_key: &str) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.user_liquidation_reward(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_pool_liquidation_reward(&self, pool_key: &str) -> Result<f64, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.pool_liquidation_reward(&mut tx, pool_key)?;
        let sim = self.simulate(&tx).await?;
        Ok(self.read_u64(&sim, 0, 0)? as f64 / FLOAT_SCALAR)
    }

    pub async fn get_allowed_maintainers(&self) -> Result<Vec<String>, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.allowed_maintainers(&mut tx)?;
        let sim = self.simulate(&tx).await?;
        self.read_vec_set_addresses(&sim, 0, 0)
    }

    pub async fn get_allowed_pause_caps(&self) -> Result<Vec<String>, ClientError> {
        let margin_registry = MarginRegistryContract {
            config: &self.config,
        };
        let mut tx = Transaction::new();
        margin_registry.allowed_pause_caps(&mut tx)?;
        let sim = self.simulate(&tx).await?;
        self.read_vec_set_addresses(&sim, 0, 0)
    }

    pub async fn simulate(&self, tx: &Transaction) -> Result<Value, ClientError> {
        let tx_bytes_base64 = tx.build_base64().map_err(|_| ClientError::InvalidBcsValue)?;
        let out: Value = self
            .client
            .call(
                "sui_dryRunTransactionBlock",
                vec![Value::String(tx_bytes_base64)],
            )
            .await?;

        if let Some(result) = out.get("result") {
            Ok(result.clone())
        } else {
            Ok(out)
        }
    }

    pub fn return_bcs(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<Vec<u8>, ClientError> {
        let command_results = sim
            .get("commandResults")
            .and_then(Value::as_array)
            .ok_or(ClientError::MissingCommandResult(command_index))?;

        let command = command_results
            .get(command_index)
            .ok_or(ClientError::MissingCommandResult(command_index))?;

        let return_values = command
            .get("returnValues")
            .and_then(Value::as_array)
            .ok_or(ClientError::MissingReturnValue {
                command_index,
                return_index,
            })?;

        let ret = return_values
            .get(return_index)
            .ok_or(ClientError::MissingReturnValue {
                command_index,
                return_index,
            })?;

        let b64 = ret
            .get("bcs")
            .and_then(Value::as_str)
            .ok_or(ClientError::InvalidBcsValue)?;

        Ok(base64::engine::general_purpose::STANDARD.decode(b64)?)
    }

    pub fn read_u64(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<u64, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        if raw.len() < 8 {
            return Err(ClientError::InvalidBcsValue);
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&raw[..8]);
        Ok(u64::from_le_bytes(arr))
    }

    pub fn read_return_bcs_base64(
        &self,
        sim: &Value,
        command_index: usize,
        return_index: usize,
    ) -> Result<String, ClientError> {
        let raw = self.return_bcs(sim, command_index, return_index)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(raw))
    }
}
