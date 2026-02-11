pub const SUI_ADDRESS_LENGTH_BYTES: usize = 32;

pub fn normalize_sui_address(addr: &str) -> String {
    let mut raw = addr.trim().to_lowercase();
    if let Some(stripped) = raw.strip_prefix("0x") {
        raw = stripped.to_string();
    }

    let max_len = SUI_ADDRESS_LENGTH_BYTES * 2;
    if raw.len() > max_len {
        raw = raw[raw.len() - max_len..].to_string();
    }
    if raw.len() < max_len {
        raw = format!("{}{}", "0".repeat(max_len - raw.len()), raw);
    }
    format!("0x{raw}")
}

pub fn normalize_sui_object_id(id: &str) -> String {
    normalize_sui_address(id)
}

pub fn is_valid_sui_address(addr: &str) -> bool {
    let raw = addr.trim().strip_prefix("0x").unwrap_or(addr.trim());
    if raw.is_empty() || raw.len() > SUI_ADDRESS_LENGTH_BYTES * 2 {
        return false;
    }
    hex::decode(raw).is_ok()
}

pub fn is_valid_sui_object_id(id: &str) -> bool {
    is_valid_sui_address(id)
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
