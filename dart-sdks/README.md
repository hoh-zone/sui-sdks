# dart-sdks

Dart implementation workspace aligned with `ts-sdks` / `go-sdks` / `py-sdks` efforts.

## Implemented baseline

- `dart_sdks.bcs`
  - ULEB128 with canonical checks and u32 upper bound
  - BCS reader/writer for primitive values (`u8/u16/u32/u64/bool/bytes`)
- `dart_sdks.sui`
  - `JsonRpcClient` with network defaults and pluggable transport
  - `GraphQlClient` powered by official Dart `graphql` package
  - `SuiGrpcClient` powered by official Dart `grpc` + `protobuf` packages
    - includes `fromStubInvoker(...)` adapter for integrating generated official protobuf stubs
    - includes helper methods: `discoverRpcApi`, `getCoins`, `getAllCoins`, `getBalance`, `getAllBalances`, `getObject`, `getOwnedObjects`, `getDynamicFields`, `multiGetObjects`, `getTransactionBlock`, `queryTransactionBlocks`, `queryEvents`, `getCheckpoints`, `getReferenceGasPrice`, `getLatestSuiSystemState`, `resolveNameServiceAddress`, `resolveNameServiceNames`, `executeTransactionBlock`
    - supports configurable method mapping (`identityGrpcMethodMapper` / `defaultJsonRpcToGrpcMethodMapper`) for JSON-RPC style and gRPC-style method names
  - `Ed25519Keypair` + `Secp256r1Keypair` + verify helpers powered by official Dart `cryptography` package
  - `multisig` baseline
  - `FaucetClient` + `FaucetRateLimitError`
  - `SuiClient` JSON-RPC facade (coin/object/events/tx/checkpoint/name-service helpers + paginated stream helpers)
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
- `secp256k1` keypair/signature support is not included yet (official `dart:cryptography` does not provide secp256k1 primitives).
