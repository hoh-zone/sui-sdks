# rust-sdks

Unofficial Rust SDK baseline for Sui.

## Implemented baseline modules

- `bcs`: serialization/deserialization and base64/hex helpers via crate `bcs`
- `crypto`: Sui intent/hash/public-key helper baseline, wired with `fastcrypto` dependency
- `sui::grpc`: gRPC channel bootstrap based on `tonic`
- `sui::graphql`: GraphQL client baseline with request/response models and `async-graphql` request conversion

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

## Test

```bash
cd /Users/mac/work/sui-sdks/rust-sdks
cargo test
```

Note: in restricted network environments, fetching crates/Git dependencies (especially `fastcrypto`) may fail.
