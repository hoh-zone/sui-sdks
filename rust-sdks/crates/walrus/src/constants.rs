use crate::types::WalrusPackageConfig;
use once_cell::sync::Lazy;

pub static MAINNET_WALRUS_PACKAGE_CONFIG: Lazy<WalrusPackageConfig> = Lazy::new(|| WalrusPackageConfig {
    system_object_id: "0x2133f8f90b17dce7f64c86c0125d8d2f8f0f6f9bcbec3abf7f8f2f6a6672a1bd".to_string(),
    staking_pool_id: "0x869f0f4c3a9f65f0b5b5e0a02f2de8f0a361ee5f77d7278d9f4dc3f5ec95b7d5".to_string(),
});

pub static TESTNET_WALRUS_PACKAGE_CONFIG: Lazy<WalrusPackageConfig> = Lazy::new(|| WalrusPackageConfig {
    system_object_id: "0x6f8c8d4e35b1a7eb6e3a884ecf9ee4f8c8ce94e74f3ed8478bb6e4f3a9ef56ff".to_string(),
    staking_pool_id: "0x71f4ea7ea502f2b0804cb45af8f3cf59f236f6b2320c2d73ecb8eecf483eedf0".to_string(),
});

pub fn status_lifecycle_rank(status: &str) -> u8 {
    match status {
        "nonexistent" => 0,
        "invalid" => 1,
        "deletable" => 2,
        "permanent" => 3,
        _ => 0,
    }
}
