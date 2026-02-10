# swift-sdks

Swift SDK implementation for Sui, bootstrapped from `go-sdks` and `py-sdks`, with API conventions aligned to `ts-sdks` JSON-RPC network defaults.

## Current status

Implemented foundations:

- Dependency-first crypto stack:
  - [`apple/swift-crypto`](https://github.com/apple/swift-crypto)
  - [`21-DOT-DEV/swift-secp256k1`](https://github.com/21-DOT-DEV/swift-secp256k1)

- `BCS`
  - ULEB128 encode/decode with canonical checks and u32 bound
  - `BCSReader` (`u8/u16/u32/u64/bool/bytes/uleb128`)
  - `BCSWriter` (`u8/u16/u32/u64/bool/bytes/uleb128`)
- `Sui JSON-RPC`
  - Network mapping: `mainnet/testnet/devnet/localnet`
  - Sui type helpers: address/object id normalization + validation, transaction digest validation
  - HTTP JSON-RPC transport based on `URLSession`
  - Error modeling for HTTP status, JSON-RPC server error, malformed response
  - `SuiClient` base methods:
    - `call`
    - `discoverRPCAPI`
    - `getObject`
    - `multiGetObjects`
    - `dryRunTransactionBlock`
    - `getCoins`
    - `getAllCoins`
    - `getBalance`
    - `getAllBalances`
    - `getOwnedObjects`
    - `getDynamicFields`
    - `queryEvents`
    - `queryTransactionBlocks`
    - `getCheckpoints`
    - `getTransactionBlock`
    - `getReferenceGasPrice`
  - Pagination helper + aggregated fetch:
    - `allCoins`
    - `allOwnedObjects`
    - `allDynamicFields`
    - `allEvents`
    - `allTransactionBlocks`
    - `allCheckpoints`
- `Cryptography (initial)`
  - `SignatureScheme` flags aligned with Sui convention
  - `ed25519` / `secp256k1` / `secp256r1` keypair/public key:
    - generate keypair
    - sign / verify raw bytes
    - serialized signature (`flag || signature || pubkey`, base64)
  - verify helpers:
    - raw signature verify
    - intent-based verify (`TransactionData` / `PersonalMessage`)
    - serialized signature verify
  - intent signing:
    - `signWithIntent`
    - `signTransaction`
    - `signPersonalMessage`
  - intent digest:
    - currently `SHA-256` (aligned with this repository's Go implementation)

## Run tests

```bash
cd swift-sdks
swift test
```

## Next steps

- Add typed response models for key JSON-RPC methods
- Add address/object/digest validation parity with Go/TypeScript SDKs
- Add keypairs, signatures, verify, and transaction builders
