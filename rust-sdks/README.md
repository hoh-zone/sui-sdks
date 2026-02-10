# rust-sdks

Unofficial Rust SDK baseline for Sui.

## Implemented baseline modules

- `bcs`: serialization/deserialization and base64/hex helpers via crate `bcs`
- `crypto`: Sui intent/hash/public-key helper baseline, wired with `fastcrypto` dependency
- `sui::grpc`: gRPC channel bootstrap based on `tonic`
- `sui::graphql`: GraphQL client baseline with request/response models and `async-graphql` request conversion
- `sui::jsonrpc`: JSON-RPC client baseline with common methods (`rpc.discover`, `suix_getBalance`, `sui_getObject`, etc.)
- `sui::client`: unified client wiring `jsonrpc/graphql/grpc`
- `sui::faucet`: faucet v2 request helpers with network host mapping and rate-limit error mapping
- `sui::keypairs::ed25519`: keypair generate/import/sign/verify and Sui private-key/address helpers (backed by `fastcrypto`)
- `sui::keypairs::secp256k1`: keypair generate/import/sign/verify and Sui private-key/address helpers
- `sui::keypairs::secp256r1`: keypair generate/import/sign/verify and Sui private-key/address helpers
- `sui::transactions`: transaction baseline (inputs/commands/build/base64/sign/execute)
- `sui::verify`: signature verification helpers (raw/personal-message/transaction-intent)
- `sui::multisig`: weighted multisig baseline (serialize/parse/threshold verify)
- `sui::zklogin`: zkLogin baseline helpers (nonce/randomness/jwt/address/signature encoding)
- `sui::utils`: shared network/address helpers used across modules
- `deepbook_v3`: baseline config/types/encoding/contracts builders
  - includes `deepbook_v3::client` dry-run query parser baseline (`check_manager_balance`, `whitelisted`, `get_quote_quantity_out`, `get_base_quantity_out`, `get_quantity_out`, `mid_price`, `get_order`, `get_orders`, `account_open_orders`, `vault_balances`, `get_pool_id_by_assets`, `pool_trade_params`, `pool_book_params`, `account`, `locked_balance`, `get_pool_deep_price`, `balance_manager_referral_owner`, `balance_manager_referral_pool_id`, `get_balance_manager_referral_id`, `get_balance_manager_ids`, `get_pool_referral_balances`, `pool_referral_multiplier`, `stable_pool`, `registered_pool`, `can_place_limit_order`, `can_place_market_order`, `check_market_order_params`, `check_limit_order_params`, `decode_order_id`, `get_margin_pool_id`, `is_deepbook_pool_allowed`, `get_margin_pool_total_supply`, `get_margin_pool_supply_shares`, `get_margin_pool_total_borrow`, `get_margin_pool_borrow_shares`, `get_margin_pool_last_update_timestamp`, `get_margin_pool_supply_cap`, `get_margin_pool_max_utilization_rate`, `get_margin_pool_protocol_spread`, `get_margin_pool_min_borrow`, `get_margin_pool_interest_rate`, `get_user_supply_shares`, `get_user_supply_amount`, `get_margin_manager_owner`, `get_margin_manager_deepbook_pool`, `get_margin_manager_margin_pool_id`, `get_margin_manager_borrowed_shares`, `get_margin_manager_borrowed_base_shares`, `get_margin_manager_borrowed_quote_shares`, `get_margin_manager_has_base_debt`, `get_margin_manager_balance_manager_id`, `get_margin_manager_assets`, `get_margin_manager_debts`, `get_margin_manager_base_balance`, `get_margin_manager_quote_balance`, `get_margin_manager_deep_balance`, `get_margin_manager_state`, `get_margin_manager_states`, `get_price_info_object_age`, `get_quote_quantity_out_input_fee`, `get_base_quantity_out_input_fee`, `get_quantity_out_input_fee`, `get_base_quantity_in`, `get_quote_quantity_in`, `get_account_order_details`, `get_order_deep_required`, `pool_trade_params_next`, `get_level2_range`, `get_level2_ticks_from_mid`, `account_exists`, `quorum`, `pool_id`, `get_margin_account_order_details`)

## Dependencies (as requested)

- BCS: [`bcs`](https://crates.io/crates/bcs)
- Crypto: [`fastcrypto`](https://github.com/MystenLabs/fastcrypto)
- gRPC: [`tonic`](https://crates.io/crates/tonic)
- GraphQL: [`async-graphql`](https://crates.io/crates/async-graphql)

## Structure

- `/Users/mac/work/sui-sdks/rust-sdks/src/bcs.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/crypto.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/grpc.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/graphql.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/jsonrpc.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/client.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/faucet.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/keypairs/ed25519.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/keypairs/secp256k1.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/keypairs/secp256r1.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/transactions.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/verify.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/multisig.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/zklogin.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/sui/utils.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/deepbook_v3/config.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/deepbook_v3/types.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/deepbook_v3/encode.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/deepbook_v3/contracts.rs`
- `/Users/mac/work/sui-sdks/rust-sdks/src/deepbook_v3/client.rs`

## Test

```bash
cd /Users/mac/work/sui-sdks/rust-sdks
cargo test
```

Note: in restricted network environments, fetching crates/Git dependencies (especially `fastcrypto`) may fail.
