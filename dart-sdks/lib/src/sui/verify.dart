import 'dart:convert';
import 'dart:typed_data';

import 'crypto.dart';

class ParsedSerializedSignature {
  const ParsedSerializedSignature({
    required this.scheme,
    required this.signature,
    required this.publicKey,
  });

  final SignatureScheme scheme;
  final List<int> signature;
  final List<int> publicKey;
}

const int _signatureSize = 64;
const Map<SignatureScheme, int> _schemePubkeySize = {
  SignatureScheme.ed25519: 32,
  SignatureScheme.secp256k1: 33,
  SignatureScheme.secp256r1: 33,
};

Future<bool> verifyRawSignature({
  required List<int> message,
  required List<int> signature,
  required List<int> publicKey,
  required SignatureScheme scheme,
}) {
  switch (scheme) {
    case SignatureScheme.ed25519:
      return verifyRawSignatureEd25519(
          message: message, signature: signature, publicKey: publicKey);
    case SignatureScheme.secp256r1:
      return verifyRawSignatureSecp256r1(
          message: message, signature: signature, publicKey: publicKey);
    case SignatureScheme.secp256k1:
      return verifyRawSignatureSecp256k1(
          message: message, signature: signature, publicKey: publicKey);
  }
}

Future<bool> verifyPersonalMessage({
  required List<int> message,
  required List<int> signature,
  required List<int> publicKey,
  required SignatureScheme scheme,
}) {
  final prefix = utf8.encode('\x19Sui Signed Message:\n${message.length}\n');
  return verifyRawSignature(
    message: <int>[...prefix, ...message],
    signature: signature,
    publicKey: publicKey,
    scheme: scheme,
  );
}

String toSerializedSignature({
  required SignatureScheme scheme,
  required List<int> signature,
  required List<int> publicKey,
}) {
  final expectedPkLen = _schemePubkeySize[scheme];
  if (expectedPkLen == null) {
    throw StateError('unsupported scheme: $scheme');
  }
  if (signature.length != _signatureSize) {
    throw ArgumentError.value(
        signature.length, 'signature.length', 'invalid signature length');
  }
  if (publicKey.length != expectedPkLen) {
    throw ArgumentError.value(publicKey.length, 'publicKey.length',
        'invalid public key length for $scheme');
  }

  final data =
      Uint8List.fromList(<int>[scheme.index, ...signature, ...publicKey]);
  return base64Encode(data);
}

ParsedSerializedSignature parseSerializedSignature(String serialized) {
  final raw = base64Decode(serialized);
  if (raw.length < 1 + _signatureSize) {
    throw StateError('serialized signature too short');
  }

  final schemeIndex = raw[0];
  if (schemeIndex < 0 || schemeIndex >= SignatureScheme.values.length) {
    throw StateError('unsupported scheme flag: $schemeIndex');
  }
  final scheme = SignatureScheme.values[schemeIndex];
  final pkSize = _schemePubkeySize[scheme];
  if (pkSize == null) {
    throw StateError('unsupported scheme: $scheme');
  }

  final expectedLen = 1 + _signatureSize + pkSize;
  if (raw.length != expectedLen) {
    throw StateError(
        'invalid serialized signature length for $scheme: ${raw.length}');
  }

  final signature = raw.sublist(1, 1 + _signatureSize);
  final publicKey = raw.sublist(1 + _signatureSize);
  return ParsedSerializedSignature(
      scheme: scheme, signature: signature, publicKey: publicKey);
}
