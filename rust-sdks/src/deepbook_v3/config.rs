use std::collections::HashMap;

use crate::deepbook_v3::types::{BalanceManager, Coin, MarginManager, MarginPool, Pool};

pub const FLOAT_SCALAR: f64 = 1_000_000_000.0;
pub const DEEP_SCALAR: f64 = 1_000_000.0;
pub const MAX_TIMESTAMP: u64 = 1_844_674_407_370_955_161;

#[derive(Debug, Clone)]
pub struct PackageIds {
    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
    pub margin_package_id: String,
    pub margin_registry_id: String,
}

#[derive(Debug, Clone)]
pub struct DeepBookConfig {
    pub address: String,
    pub network: String,
    pub balance_managers: HashMap<String, BalanceManager>,
    pub margin_managers: HashMap<String, MarginManager>,
    pub margin_pools: HashMap<String, MarginPool>,
    pub coins: HashMap<String, Coin>,
    pub pools: HashMap<String, Pool>,
    pub package_ids: PackageIds,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("coin not found: {0}")]
    CoinNotFound(String),
    #[error("pool not found: {0}")]
    PoolNotFound(String),
    #[error("balance manager not found: {0}")]
    BalanceManagerNotFound(String),
    #[error("margin manager not found: {0}")]
    MarginManagerNotFound(String),
    #[error("margin pool not found: {0}")]
    MarginPoolNotFound(String),
}

impl Default for DeepBookConfig {
    fn default() -> Self {
        Self {
            address: "0x0".to_string(),
            network: "testnet".to_string(),
            balance_managers: HashMap::new(),
            margin_managers: HashMap::new(),
            margin_pools: testnet_margin_pools(),
            coins: testnet_coins(),
            pools: testnet_pools(),
            package_ids: testnet_package_ids(),
        }
    }
}

impl DeepBookConfig {
    pub fn get_coin(&self, key: &str) -> Result<&Coin, ConfigError> {
        self.coins
            .get(key)
            .ok_or_else(|| ConfigError::CoinNotFound(key.to_string()))
    }

    pub fn get_pool(&self, key: &str) -> Result<&Pool, ConfigError> {
        self.pools
            .get(key)
            .ok_or_else(|| ConfigError::PoolNotFound(key.to_string()))
    }

    pub fn get_balance_manager(&self, key: &str) -> Result<&BalanceManager, ConfigError> {
        self.balance_managers
            .get(key)
            .ok_or_else(|| ConfigError::BalanceManagerNotFound(key.to_string()))
    }

    pub fn get_margin_manager(&self, key: &str) -> Result<&MarginManager, ConfigError> {
        self.margin_managers
            .get(key)
            .ok_or_else(|| ConfigError::MarginManagerNotFound(key.to_string()))
    }

    pub fn get_margin_pool(&self, key: &str) -> Result<&MarginPool, ConfigError> {
        self.margin_pools
            .get(key)
            .ok_or_else(|| ConfigError::MarginPoolNotFound(key.to_string()))
    }
}

pub fn testnet_package_ids() -> PackageIds {
    PackageIds {
        deepbook_package_id:
            "0x22be4cade64bf2d02412c7e8d0e8beea2f78828b948118d46735315409371a3c".to_string(),
        registry_id: "0x7c256edbda983a2cd6f946655f4bf3f00a41043993781f8674a7046e8c0e11d1".to_string(),
        deep_treasury_id:
            "0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb".to_string(),
        margin_package_id:
            "0xd6a42f4df4db73d68cbeb52be66698d2fe6a9464f45ad113ca52b0c6ebd918b6".to_string(),
        margin_registry_id:
            "0x48d7640dfae2c6e9ceeada197a7a1643984b5a24c55a0c6c023dac77e0339f75".to_string(),
    }
}

pub fn testnet_coins() -> HashMap<String, Coin> {
    let mut coins = HashMap::new();
    coins.insert(
        "DEEP".to_string(),
        Coin {
            address: "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8"
                .to_string(),
            type_tag:
                "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP"
                    .to_string(),
            scalar: 1_000_000,
            price_info_object_id: None,
        },
    );
    coins.insert(
        "SUI".to_string(),
        Coin {
            address: "0x2".to_string(),
            type_tag: "0x2::sui::SUI".to_string(),
            scalar: 1_000_000_000,
            price_info_object_id: None,
        },
    );
    coins
}

pub fn testnet_pools() -> HashMap<String, Pool> {
    let mut pools = HashMap::new();
    pools.insert(
        "DEEP_SUI".to_string(),
        Pool {
            address: "0x48c95963e9eac37a316b7ae04a0deb761bcdcc2b67912374d6036e7f0e9bae9f"
                .to_string(),
            base_coin: "DEEP".to_string(),
            quote_coin: "SUI".to_string(),
        },
    );
    pools
}

pub fn testnet_margin_pools() -> HashMap<String, MarginPool> {
    let mut pools = HashMap::new();
    pools.insert(
        "DEEP".to_string(),
        MarginPool {
            address: "0x7".to_string(),
            type_tag:
                "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP"
                    .to_string(),
        },
    );
    pools.insert(
        "SUI".to_string(),
        MarginPool {
            address: "0x4".to_string(),
            type_tag: "0x2::sui::SUI".to_string(),
        },
    );
    pools
}
