# dart-sdks

Dart implementation workspace aligned with `ts-sdks` / `go-sdks` / `py-sdks` efforts.

## Implemented baseline

- `dart_sdks.bcs`
  - ULEB128 with canonical checks and u32 upper bound
  - BCS reader/writer for primitive values (`u8/u16/u32/u64/bool/bytes`)
- `dart_sdks.sui`
  - `JsonRpcClient` with network defaults and pluggable transport
  - `GraphQlClient` powered by official Dart `graphql` package
    - includes helper methods: `fromNetwork`, `discoverRpcApi`, `query`, `mutation`, `getBalance`, `getAllBalances`, `listBalances`, `iterAllBalances`, `getCoins`, `listCoins`, `getAllCoins`, `getGas`, `iterCoins`, `iterAllCoins`, `getOwnedObjects`, `listOwnedObjects`, `iterOwnedObjects`, `getDynamicFields`, `listDynamicFields`, `iterDynamicFields`, `getCoinMetadata`, `getTotalSupply`, `getDefaultSuinsName`, `defaultNameServiceName`, `resolveNameServiceAddress`, `resolveNameServiceNames`, `getObject`, `getObjects`, `multiGetObjects`, `getEventsByTransaction`, `queryEvents`, `iterEvents`, `queryTransactionBlocks`, `iterTransactionBlocks`, `getMoveFunction`, `simulateTransaction`, `simulateTransactionBlock`, `dryRun`, `dryRunTransactionBlock`, `resolveTransaction`, `getTransaction`, `getTransactionBlock`, `getTransactions`, `multiGetTransactionBlocks`, `executeTransaction`, `executeTransactionBlock`, `getCheckpoint`, `getCheckpoints`, `iterCheckpoints`, `getLatestCheckpointSequenceNumber`, `getCommitteeInfo`, `getStakes`, `getStakesByIds`, `getChainIdentifier`, `getProtocolConfig`, `getReferenceGasPrice`, `getCurrentSystemState`, `getLatestSuiSystemState`
  - `SuiGrpcClient` powered by official Dart `grpc` + `protobuf` packages
    - includes `fromStubInvoker(...)` adapter for integrating generated official protobuf stubs
    - includes `fromNetwork(...)` + default network endpoint map
    - includes helper methods: `discoverRpcApi`, `getRpcApiVersion`, `dryRun`, `dryRunTransactionBlock`, `simulateTransaction`, `devInspectTransactionBlock`, `waitForTransaction`, `getCoins`, `listCoins`, `getGas`, `getAllCoins`, `iterAllCoins`, `getBalance`, `getAllBalances`, `listBalances`, `getCoinMetadata`, `getTotalSupply`, `getObject`, `getObjects`, `getPackage`, `getOwnedObjects`, `listOwnedObjects`, `iterOwnedObjects`, `getDynamicFields`, `listDynamicFields`, `iterDynamicFields`, `getDynamicFieldObject`, `getDynamicField`, `multiGetObjects`, `getTransaction`, `getTransactionBlock`, `getTransactions`, `multiGetTransactionBlocks`, `getEventsByTransaction`, `queryTransactionBlocks`, `iterTransactionBlocks`, `queryEvents`, `getEvents`, `iterEvents`, `getCheckpoint`, `getCheckpoints`, `iterCheckpoints`, `getReferenceGasPrice`, `getLatestSuiSystemState`, `getCurrentSystemState`, `getValidatorsApy`, `getNetworkMetrics`, `getAddressMetrics`, `getEpochMetrics`, `getAllEpochAddressMetrics`, `getMoveCallMetrics`, `getCurrentEpoch`, `getEpochs`, `getCommitteeInfo`, `getProtocolConfig`, `getChainIdentifier`, `getTotalTransactionBlocks`, `getStakes`, `getStakesByIds`, `tryGetPastObject`, `tryMultiGetPastObjects`, `getNormalizedMoveModulesByPackage`, `getNormalizedMoveModule`, `getNormalizedMoveFunction`, `getMoveFunctionArgTypes`, `getNormalizedMoveStruct`, `resolveNameServiceAddress`, `resolveNameServiceNames`, `defaultNameServiceName`, `executeTransaction`, `executeTransactionBlock`, `signAndExecuteTransaction`, `verifyZkLoginSignature`
    - supports configurable method mapping (`identityGrpcMethodMapper` / `defaultJsonRpcToGrpcMethodMapper`) for JSON-RPC style and gRPC-style method names
  - `Ed25519Keypair` + verify helpers powered by official Dart `cryptography` package
  - `multisig` baseline
  - `FaucetClient` + `FaucetRateLimitError` + top-level `requestSuiFromFaucetV2(...)` + `getFaucetHost(testnet/devnet/localnet)`
  - `SuiClient` JSON-RPC facade (coin/object/events/tx/checkpoint/name-service helpers + list aliases + `defaultNameServiceName` + metrics/epoch/zkLogin helpers + `signAndExecuteTransaction` + paginated stream helpers)
  - `Transaction` baseline:
    - command/input builders
    - build/serialize/restore
    - execute/inspect helpers
    - resolver plugin pipeline
    - caching/serial/parallel executors
- `dart_sdks.deepbook_v3`
  - types/config defaults
  - transaction encode helpers (`encode_bool`/`u64`/`u128`/`vec<u128>`)
  - contract builders (core subset)
  - dry-run style `DeepBookClient` baseline

## Run tests

```bash
dart pub get
dart test
```

## Quick example

```dart
import 'package:dart_sdks/dart_sdks.dart';

Future<void> main() async {
  final client = SuiClient.fromNetwork(network: 'testnet');

  final tx = Transaction(client: client);
  tx.setSender('0x1');
  tx.moveCall('0x2::foo::bar', [tx.object('0xabc')], ['0x2::sui::SUI']);
  print(await tx.inspectAll());

  final cfg = DeepBookConfig(
    address: '0x1',
    balanceManagers: {'m1': const BalanceManager(address: '0x2')},
  );
  final deepbook = DeepBookClient(client: client, config: cfg);
  print(await deepbook.checkManagerBalance('m1', 'SUI'));
}
```

## Notes

- This is a baseline implementation for practical parity expansion, not full parity with `ts-sdks` yet.
- `secp256k1` now uses a pluggable provider interface (`Secp256k1Provider`); without an injected provider it remains unsupported because official `dart:cryptography` has no secp256k1 primitives.
- `secp256r1` keypair generation/signing is currently unavailable on pure Dart runtime in `cryptography`; raw verification helpers remain available for compatible public key formats.
