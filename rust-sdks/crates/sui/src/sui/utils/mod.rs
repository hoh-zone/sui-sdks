pub mod address;
pub mod object;
pub mod resource;
pub mod validators;
pub mod wallet;

pub use address::normalize_sui_address;
pub use address::validate_sui_address;
pub use validators::validate_public_key;
pub use validators::validate_signature;
pub use validators::validate_transaction_digest;

pub const SUI_ADDRESS_LENGTH: usize = 32;

pub fn is_valid_sui_address(addr: &str) -> bool {
    validate_sui_address(addr)
}

pub fn is_valid_sui_object_id(id: &str) -> bool {
    validate_sui_address(id)
}

pub fn normalize_sui_object_id(id: &str) -> String {
    normalize_sui_address(id)
}

pub fn jsonrpc_fullnode_url(network: &str) -> Option<&'static str> {
    match network {
        "mainnet" => Some("https://fullnode.mainnet.sui.io:443"),
        "testnet" => Some("https://fullnode.testnet.sui.io:443"),
        "devnet" => Some("https://fullnode.devnet.sui.io:443"),
        _ => None,
    }
}

pub fn grpc_fullnode_url(network: &str) -> Option<&'static str> {
    jsonrpc_fullnode_url(network)
}

pub fn graphql_url(network: &str) -> Option<&'static str> {
    match network {
        "mainnet" => Some("https://sui-mainnet.mystenlabs.com/graphql"),
        "testnet" => Some("https://sui-testnet.mystenlabs.com/graphql"),
        "devnet" => Some("https://sui-devnet.mystenlabs.com/graphql"),
        _ => None,
    }
}

pub fn faucet_url(network: &str) -> Option<&'static str> {
    match network {
        "testnet" => Some("https://faucet.testnet.sui.io/v2/gas"),
        "devnet" => Some("https://faucet.devnet.sui.io/v2/gas"),
        "localnet" => Some("http://127.0.0.1:9123/v2/gas"),
        _ => None,
    }
}
