use sui::sui::grpc::default_grpc_fullnode_url;

#[test]
fn default_grpc_urls() {
    assert_eq!(
        default_grpc_fullnode_url("mainnet"),
        "https://fullnode.mainnet.sui.io:443"
    );
    assert_eq!(
        default_grpc_fullnode_url("testnet"),
        "https://fullnode.testnet.sui.io:443"
    );
    assert_eq!(
        default_grpc_fullnode_url("devnet"),
        "https://fullnode.devnet.sui.io:443"
    );
    assert_eq!(
        default_grpc_fullnode_url("unknown"),
        "https://fullnode.testnet.sui.io:443"
    );
}
