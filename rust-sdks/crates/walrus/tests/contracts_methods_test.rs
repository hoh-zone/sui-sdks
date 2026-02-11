use sui::transactions::Transaction;

#[test]
fn walrus_generated_methods_are_callable() {
    let mut tx = Transaction::new();

    let blob = walrus::contracts::walrus::blob::Contract { package_id: "0x2" };
    let _ = blob.blobId(&mut tx, vec![], vec![]);
    let _ = blob.encodedSize(&mut tx, vec![], vec![]);
    let _ = blob.isDeletable(&mut tx, vec![], vec![]);

    let system = walrus::contracts::walrus::system::Contract { package_id: "0x2" };
    let _ = system.reserveSpace(&mut tx, vec![], vec![]);
    let _ = system.registerBlob(&mut tx, vec![], vec![]);
    let _ = system.certifyBlob(&mut tx, vec![], vec![]);

    let staking = walrus::contracts::walrus::staking::Contract { package_id: "0x2" };
    let _ = staking.stakeWithPool(&mut tx, vec![], vec![]);
    let _ = staking.withdrawStake(&mut tx, vec![], vec![]);
}

#[test]
fn wal_exchange_generated_methods_are_callable() {
    let mut tx = Transaction::new();

    let exchange = walrus::contracts::wal_exchange::wal_exchange::WalExchange { package_id: "0x3" };
    let _ = exchange.newExchangeRate(&mut tx, vec![], vec![]);
    let _ = exchange._new(&mut tx, vec![], vec![]);
    let _ = exchange.addWal(&mut tx, vec![], vec![]);
}
