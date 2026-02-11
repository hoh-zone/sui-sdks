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
    - `getRPCAPIVersion`
    - `getObject`
    - `getObjects`
    - `multiGetObjects`
    - `dryRunTransactionBlock`
    - `devInspectTransactionBlock`
    - `getCoins`
    - `getAllCoins`
    - `getBalance`
    - `getAllBalances`
    - `getCoinMetadata`
    - `getTotalSupply`
    - `getMoveFunctionArgTypes`
    - `getNormalizedMoveModulesByPackage`
    - `getNormalizedMoveModule`
    - `getNormalizedMoveFunction`
    - `getNormalizedMoveStruct`
    - `getOwnedObjects`
    - `getDynamicFields`
    - `getDynamicFieldObject`
    - `queryEvents`
    - `getEventsByTransaction`
    - `queryTransactionBlocks`
    - `getCheckpoints`
    - `getCheckpoint`
    - `getLatestCheckpointSequenceNumber`
    - `getTransactionBlock`
    - `multiGetTransactionBlocks`
    - `tryGetPastObject`
    - `executeTransactionBlock`
    - `executeTransactionBlock` (bytes overload)
    - `signAndExecuteTransaction` (supports optional confirmation wait)
    - `signAndExecuteTransaction` (base64 overload)
    - `getLatestSuiSystemState`
    - `getTotalTransactionBlocks`
    - `getCommitteeInfo`
    - `getNetworkMetrics`
    - `getAddressMetrics`
    - `getEpochMetrics`
    - `getAllEpochAddressMetrics`
    - `getEpochs`
    - `getMoveCallMetrics`
    - `getCurrentEpoch`
    - `getStakes`
    - `getStakesByIDs`
    - `getValidatorsApy`
    - `getChainIdentifier`
    - `resolveNameServiceAddress`
    - `resolveNameServiceNames`
    - `resolveNameServiceNames` (supports `at` / `dot` formatting)
    - `getProtocolConfig`
    - `verifyZkLoginSignature`
    - `waitForTransaction`
    - `getPackage`
    - `getReferenceGasPrice`
  - Pagination helper + aggregated fetch:
    - `allCoins`
    - `allOwnedObjects`
    - `allDynamicFields`
    - `allEvents`
    - `allTransactionBlocks`
    - `allCheckpoints`
    - `allEpochMetrics`
    - `allEpochs`
  - Typed API (beta, additive):
    - object/object-page methods:
      - `getObjectTyped` / `getObjectsTyped` / `multiGetObjectsTyped`
      - `getOwnedObjectsTyped` / `allOwnedObjectsTyped`
      - `getDynamicFieldObjectTyped`
    - `getAllCoinsTyped` / `getCoinsTyped` / `allCoinsTyped`
    - `getBalanceTyped` / `getAllBalancesTyped`
    - `getCoinMetadataTyped` / `getTotalSupplyTyped`
    - dynamic/event/tx methods:
      - `getDynamicFieldsTyped` / `allDynamicFieldsTyped`
      - `getEventsByTransactionTyped`
      - `queryEventsTyped` / `allEventsTyped`
      - `queryTransactionBlocksTyped` / `allTransactionBlocksTyped`
      - `getTransactionBlockTyped` / `multiGetTransactionBlocksTyped`
      - `tryGetPastObjectTyped`
    - `getCheckpointsTyped` / `allCheckpointsTyped` / `getCheckpointTyped`
    - `resolveNameServiceNamesTyped`
    - epoch/system methods:
      - `getEpochMetricsTyped` / `allEpochMetricsTyped`
      - `getEpochsTyped` / `allEpochsTyped`
      - `getLatestSuiSystemStateTyped`
      - `getCommitteeInfoTyped`
      - `getNetworkMetricsTyped` / `getAddressMetricsTyped`
      - `getAllEpochAddressMetricsTyped`
      - `getMoveCallMetricsTyped` / `getCurrentEpochTyped`
      - `getValidatorsApyTyped`
      - `getStakesTyped` / `getStakesByIDsTyped`
      - `getProtocolConfigTyped`
      - `verifyZkLoginSignatureTyped`
    - tx simulation methods:
      - `dryRunTransactionBlockTyped`
      - `devInspectTransactionBlockTyped`
- `Cryptography (initial)`
  - `SignatureScheme` flags aligned with Sui convention
  - `ed25519` / `secp256k1` / `secp256r1` keypair/public key:
    - generate keypair
    - `suiprivkey` bech32 encode/decode (`getSecretKey` / `init(secretKey:)`)
    - sign / verify raw bytes
    - serialized signature (`flag || signature || pubkey`, base64)
  - verify helpers:
    - raw signature verify
    - intent-based verify (`TransactionData` / `PersonalMessage`)
    - serialized signature verify
    - serialized signature verify with expected address check
    - extract `(scheme, publicKey)` from serialized signature
  - intent signing:
    - `signWithIntent`
    - `signTransaction`
    - `signPersonalMessage`
  - intent digest:
    - currently `SHA-256` (aligned with this repository's Go implementation)
  - public key factory:
    - from raw bytes by signature scheme
    - from `flag || pubkey` bytes (or base64 Sui public key)

## Run tests

```bash
cd swift-sdks
swift test
```

## Next steps

- Add typed response models for key JSON-RPC methods
- Add address/object/digest validation parity with Go/TypeScript SDKs
- Add keypairs, signatures, verify, and transaction builders
