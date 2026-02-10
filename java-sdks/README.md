# java-sdks

Java baseline SDK implementation aligned with the go/python tracks for:

- `sui/grpc`
- `sui/cryptography`
- `sui/keypairs`
- `sui/multisig`

## Dependencies

This implementation directly uses official libraries:

- gRPC: `io.grpc:grpc-netty-shaded`, `io.grpc:grpc-protobuf`, `io.grpc:grpc-stub`
- Protobuf: `com.google.protobuf:protobuf-java-util`
- Crypto: `org.bouncycastle:bcprov-jdk18on`

## Modules

- `com.suisdks.sui.grpc`
  - `SuiGrpcClient`
  - `OfficialGrpcTransport` (official gRPC path)
  - request/response/transport abstractions
- `com.suisdks.sui.cryptography`
  - `SignatureScheme`
  - `Verify` helpers (raw/personal message/serialized signature)
- `com.suisdks.sui.keypairs`
  - `Ed25519Keypair`
  - `Secp256k1Keypair`
  - `Secp256r1Keypair`
- `com.suisdks.sui.multisig`
  - threshold multisig model and verifier

## Build / Test

Use Gradle:

```bash
gradle test
```

If `gradle` is not installed in your environment, install Gradle or add Gradle Wrapper (`./gradlew`) and run:

```bash
./gradlew test
```
