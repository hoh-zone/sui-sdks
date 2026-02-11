#[test]
fn walrus_contract_target_format() {
    let c = walrus::contracts::walrus::system::Contract {
        package_id: "0x2",
    };
    assert_eq!(c.target("register_blob"), "0x2::system::register_blob");

    let e = walrus::contracts::wal_exchange::wal_exchange::WalExchange {
        package_id: "0x3",
    };
    assert_eq!(e.target("swap"), "0x3::wal_exchange::swap");
}
