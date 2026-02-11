use sui::sui::utils;

#[test]
fn normalize_and_validate_address() {
    let normalized = utils::normalize_sui_address("0x1");
    assert_eq!(normalized.len(), 66);
    assert!(normalized.starts_with("0x"));

    assert!(utils::is_valid_sui_address("0x01"));
    assert!(utils::is_valid_sui_object_id("0x0abc"));
    assert!(!utils::is_valid_sui_address("xyz"));
}

#[test]
fn network_urls_lookup() {
    assert_eq!(
        utils::jsonrpc_fullnode_url("mainnet"),
        Some("https://fullnode.mainnet.sui.io:443")
    );
    assert_eq!(
        utils::graphql_url("testnet"),
        Some("https://sui-testnet.mystenlabs.com/graphql")
    );
    assert_eq!(
        utils::faucet_url("devnet"),
        Some("https://faucet.devnet.sui.io/v2/gas")
    );
    assert_eq!(utils::jsonrpc_fullnode_url("unknown"), None);
}
