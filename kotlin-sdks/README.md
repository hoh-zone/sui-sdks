# kotlin-sdks

Kotlin baseline SDK implementation aligned with `go-sdks` / `py-sdks`, with API shapes inspired by `ts-sdks`.

## Modules

- `com.suisdks.sui.jsonrpc`
  - `JsonRpcClient`
  - `HttpJsonRpcTransport`
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
  - `CachingExecutor` / `SerialExecutor` / `ParallelExecutor`
- `com.suisdks.sui.client`
  - `SuiClient`
  - `AsyncSuiClient`
- `com.suisdks.sui.pagination`
  - `iterPaginatedItems`
- `com.suisdks.sui.faucet`
  - `FaucetClient`
- `com.suisdks.sui.graphql`
  - `GraphQLClient`
- `com.suisdks.sui.verify`
  - `VerifyFacade`

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
