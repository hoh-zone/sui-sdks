use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub address: String,
    #[serde(rename = "type")]
    pub type_tag: String,
    pub scalar: u64,
    #[serde(default)]
    pub price_info_object_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub address: String,
    pub base_coin: String,
    pub quote_coin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceManager {
    pub address: String,
    #[serde(default)]
    pub trade_cap: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginManager {
    pub address: String,
    pub pool_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginPool {
    pub address: String,
    #[serde(rename = "type")]
    pub type_tag: String,
}
