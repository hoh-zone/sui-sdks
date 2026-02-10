# Python SDK Implementation Status

## Current baseline

Implemented:

- BCS core helpers (`uleb`, `reader`, `writer`) with Rust-compatible canonical checks.
- Sui network clients:
  - `jsonrpc`
  - `graphql`
  - `faucet`
  - `grpc` transport baseline (pluggable transport with JSON-RPC adapter)
- Sui transactions baseline:
  - transaction builder and serialization
  - resolver plugin pipeline
  - caching / serial / parallel executors
- Sui crypto baseline:
  - keypairs (`ed25519`, `secp256k1`, `secp256r1`) via `cryptography` backend
  - verify helpers
  - multisig baseline
- Deepbook v3 baseline:
  - shared types
  - config defaults (testnet package IDs / coins / pools)
  - contract builders:
    - `BalanceManagerContract`
    - `DeepBookContract`
    - `GovernanceContract`
    - `FlashLoanContract`
    - `MarginManagerContract`
    - `PoolProxyContract`
    - `MarginTPSLContract`
  - baseline `DeepBookClient` dry-run parser
  - encoding helpers (`u64/u128/bool/vector<u128>`)

## Tests

- `tests/bcs/*`
- `tests/sui/test_jsonrpc.py`
- `tests/sui/test_graphql.py`
- `tests/sui/test_faucet.py`
- `tests/sui/test_transactions.py`
- `tests/sui/test_grpc.py`
- `tests/sui/test_verify.py`
- `tests/sui/test_multisig.py`
- `tests/sui/keypairs/test_keypairs.py`
- `tests/deepbook_v3/test_encode.py`
- `tests/deepbook_v3/test_contracts.py`
- `tests/deepbook_v3/test_client.py`

Run:

```bash
PYTHONPATH=src python3 -m unittest discover -s tests -p 'test_*.py' -v
```

## Pending for full parity

- Real cryptographic backends for keypairs/signature verification.
- grpc-native protobuf stubs and typed request/response parity (native transport scaffold exists).
- Walrus package parity.
- Seal package parity.
- Deepbook v3 full contract method coverage and complete result structure parsers.

## Rust vector format compatibility

Added a Rust-style fixture-driven compatibility suite for ULEB128:

- Fixture: `tests/vectors/rust_bcs_uleb_vectors.json`
- Runner: `tests/bcs/test_rust_vectors.py`

Fixture schema supports:

- success vectors: `name`, `value`, `encoded_hex`
- failure vectors: `name`, `encoded_hex`, `error_contains`

This keeps BCS compatibility checks extensible and easy to sync with upstream Rust/TS vector updates.
