# kotlin-sdks

Kotlin baseline SDK implementation aligned with `go-sdks` / `py-sdks`, with API shapes inspired by `ts-sdks`.

## Modules

- `com.suisdks.sui.jsonrpc`
  - `JsonRpcClient`
  - `HttpJsonRpcTransport`
  - Networks: `mainnet` / `testnet` / `devnet` / `localnet`
  - `fromEndpoint` / `callRaw` / `batchCall`
- `com.suisdks.sui.grpc`
  - `SuiGrpcClient`
  - `OfficialGrpcTransport`
  - `GrpcSecurityOptions` / `GrpcTlsOptions`
- `com.suisdks.sui.cryptography`
  - `SignatureScheme`
  - `Verify`
  - `Encryption` (Tink AEAD)
- `com.suisdks.sui.keypairs`
  - `Ed25519Keypair`
  - `Secp256k1Keypair`
  - `Secp256r1Keypair`
- `com.suisdks.sui.multisig`
  - `MultiSigPublicKey`
  - `MultiSigSignature`
  - `MultiSigVerifier`
- `com.suisdks.sui.transactions`
  - `Transaction`
  - `TransactionCommands`
  - `Inputs`
  - `Resolver`
  - `AsyncResolver`
  - `CachingExecutor` / `SerialExecutor` / `ParallelExecutor`
- `com.suisdks.sui.client`
  - `SuiClient`
  - `AsyncSuiClient`
  - Includes sync/async wrappers for JSON-RPC coverage (objects/coins/transactions/checkpoints/metrics/epochs)
  - `fromNetwork` / `fromEndpoint` support timeout and headers
- `com.suisdks.sui.pagination`
  - `iterPaginatedItems`
- `com.suisdks.sui.batch`
  - `mapSync`
  - `mapAsync`
- `com.suisdks.sui.bcs`
  - `encodeUleb128` / `decodeUleb128`
  - `BcsReader` / `BcsWriter`
- `com.suisdks.sui.faucet`
  - `FaucetClient`
  - Timeout/headers configurable
- `com.suisdks.sui.graphql`
  - `GraphQLClient`
  - `executeNamed`
  - `executePersistedQuery` (Apollo persisted query extension payload)
  - Timeout/headers configurable
- `com.suisdks.sui.verify`
  - `VerifyFacade`
- `com.suisdks.sui.utils`
  - `normalizeSuiAddress` / `normalizeSuiObjectId`
  - `isValidSuiAddress` / `isValidSuiObjectId` / `isValidTransactionDigest`
  - `formatAddress` / `formatDigest`
  - `isValidSuiNSName` / `normalizeSuiNSName`
  - `parseStructTag` / `normalizeStructTag`

## Build / Test

```bash
gradle test
```

If Gradle is not installed, add wrapper and run `./gradlew test`.

## Official Dependencies

- gRPC: `io.grpc` official Java stack (`grpc-netty-shaded`, `grpc-protobuf`, `grpc-stub`, `grpc-auth`)
- Encryption: `com.google.crypto.tink:tink`
- GraphQL: `com.graphql-java:graphql-java`

`OfficialGrpcTransport` supports:
- TLS by default (`plaintext = false`)
- authority override via `GrpcTlsOptions`
- static and per-request metadata headers via `GrpcSecurityOptions` and `GrpcRequest.metadata`
- channel creation via official gRPC credentials API (`Grpc.newChannelBuilder`, `TlsChannelCredentials`, `InsecureChannelCredentials`)
