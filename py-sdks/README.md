# py-sdks

Python implementation workspace aligned with ts-sdks/go-sdks efforts.

## Implemented baseline

- `pysdks.bcs`
  - ULEB128 with canonical checks and u32 upper bound
  - BCS reader/writer for primitive values
- `pysdks.sui`
  - `JsonRpcClient`
  - `GraphQLClient`
  - `FaucetClient` + rate-limit error mapping
  - `transactions` baseline:
    - `Transaction` builder
    - `Resolver` plugin pipeline
    - `CachingExecutor` / `SerialExecutor` / `ParallelExecutor`
  - keypairs baseline:
    - `Ed25519Keypair`
    - `Secp256k1Keypair`
    - `Secp256r1Keypair`
  - `verify` helpers
  - `multisig` baseline
  - `grpc` placeholder (`SuiGrpcClient`, `GrpcCoreClient`)
- `pysdks.deepbook_v3`
  - shared types and config defaults
  - contract builders (core subset)
  - dry-run style `DeepBookClient` baseline
  - transaction compatibility now reuses `pysdks.sui.transactions.Transaction`
  - encoding helpers (`u64/u128/bool/vector<u128>`)

## Run tests

```bash
PYTHONPATH=src python3 -m unittest discover -s tests -p 'test_*.py' -v
```

## Security note

Current keypair implementations use the Python `cryptography` dependency for real signature operations.
