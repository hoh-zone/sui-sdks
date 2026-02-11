import 'dart:typed_data';

import 'package:cryptography/cryptography.dart';

enum SignatureScheme {
  ed25519,
  secp256k1,
  secp256r1,
}

const int intentScopePersonalMessage = 3;

Uint8List messageWithIntent(List<int> message, {int scope = intentScopePersonalMessage}) {
  return Uint8List.fromList(<int>[scope, 0, 0, ...message]);
}

class Ed25519Keypair {
  Ed25519Keypair._(this._keyPair);

  final SimpleKeyPair _keyPair;
  static final Ed25519 _algorithm = Ed25519();

  static Future<Ed25519Keypair> generate() async {
    final keyPair = await _algorithm.newKeyPair();
    final extracted = await keyPair.extract();
    return Ed25519Keypair._(extracted);
  }

  static Future<Ed25519Keypair> fromSeed(List<int> seed32) async {
    if (seed32.length != 32) {
      throw ArgumentError.value(seed32.length, 'seed32.length', 'ed25519 seed must be 32 bytes');
    }
    final keyPair = await _algorithm.newKeyPairFromSeed(seed32);
    final extracted = await keyPair.extract();
    return Ed25519Keypair._(extracted);
  }

  Future<Uint8List> sign(List<int> message) async {
    final sig = await _algorithm.sign(message, keyPair: _keyPair);
    return Uint8List.fromList(sig.bytes);
  }

  Future<Uint8List> signWithIntent(List<int> message, {int scope = intentScopePersonalMessage}) {
    return sign(messageWithIntent(message, scope: scope));
  }

  Future<bool> verify(List<int> message, List<int> signature) async {
    final publicKey = await _keyPair.extractPublicKey();
    return _algorithm.verify(
      message,
      signature: Signature(signature, publicKey: publicKey),
    );
  }

  Future<bool> verifyWithIntent(List<int> message, List<int> signature, {int scope = intentScopePersonalMessage}) {
    return verify(messageWithIntent(message, scope: scope), signature);
  }

  Future<Uint8List> publicKeyBytes() async {
    final pk = await _keyPair.extractPublicKey();
    return Uint8List.fromList(pk.bytes);
  }

  Future<Uint8List> privateKeySeed() async {
    final data = await _keyPair.extract();
    return Uint8List.fromList(data.bytes);
  }
}

class Secp256r1Keypair {
  Secp256r1Keypair._(this._keyPair);

  final KeyPair _keyPair;
  static final Ecdsa _algorithm = Ecdsa.p256(Sha256());

  static Future<Secp256r1Keypair> generate() async {
    try {
      return Secp256r1Keypair._(await _algorithm.newKeyPair());
    } on UnimplementedError {
      throw UnsupportedError('Secp256r1 is not available in pure dart:cryptography runtime yet');
    }
  }

  static Future<Secp256r1Keypair> fromPrivateKeyBytes(List<int> privateKey) async {
    throw UnsupportedError('Secp256r1Keypair.fromPrivateKeyBytes is not implemented yet');
  }

  Future<Uint8List> sign(List<int> message) async {
    final sig = await _algorithm.sign(message, keyPair: _keyPair);
    return Uint8List.fromList(sig.bytes);
  }

  Future<bool> verify(List<int> message, List<int> signature) async {
    final publicKey = await _keyPair.extractPublicKey();
    return _algorithm.verify(
      message,
      signature: Signature(signature, publicKey: publicKey),
    );
  }

  Future<Uint8List> publicKeyBytes() async {
    final pk = await _keyPair.extractPublicKey();
    if (pk is EcPublicKey) {
      // Return SEC1 uncompressed form: 0x04 || X || Y
      return Uint8List.fromList(<int>[4, ...pk.x, ...pk.y]);
    }
    if (pk is SimplePublicKey) {
      return Uint8List.fromList(pk.bytes);
    }
    throw StateError('unsupported p256 public key type: ${pk.runtimeType}');
  }
}

class Secp256k1Keypair {
  Secp256k1Keypair._();

  static Future<Secp256k1Keypair> generate() async {
    throw UnsupportedError('Secp256k1 is not available in dart:cryptography yet');
  }

  static Future<Secp256k1Keypair> fromPrivateKeyBytes(List<int> privateKey) async {
    throw UnsupportedError('Secp256k1 is not available in dart:cryptography yet');
  }

  Future<Uint8List> sign(List<int> message) async {
    throw UnsupportedError('Secp256k1 is not available in dart:cryptography yet');
  }

  Future<bool> verify(List<int> message, List<int> signature) async {
    throw UnsupportedError('Secp256k1 is not available in dart:cryptography yet');
  }

  Future<Uint8List> publicKeyBytes() async {
    throw UnsupportedError('Secp256k1 is not available in dart:cryptography yet');
  }
}

Future<bool> verifyRawSignatureEd25519({
  required List<int> message,
  required List<int> signature,
  required List<int> publicKey,
}) {
  final algo = Ed25519();
  return algo.verify(
    message,
    signature: Signature(signature, publicKey: SimplePublicKey(publicKey, type: KeyPairType.ed25519)),
  );
}

Future<bool> verifyRawSignatureSecp256r1({
  required List<int> message,
  required List<int> signature,
  required List<int> publicKey,
}) {
  final algo = Ecdsa.p256(Sha256());
  final PublicKey key;
  if (publicKey.length == 65 && publicKey.first == 4) {
    key = EcPublicKey(
      x: publicKey.sublist(1, 33),
      y: publicKey.sublist(33, 65),
      type: KeyPairType.p256,
    );
  } else if (publicKey.length == 64) {
    key = EcPublicKey(
      x: publicKey.sublist(0, 32),
      y: publicKey.sublist(32, 64),
      type: KeyPairType.p256,
    );
  } else {
    key = SimplePublicKey(publicKey, type: KeyPairType.p256);
  }
  return algo.verify(
    message,
    signature: Signature(signature, publicKey: key),
  );
}

Future<bool> verifyPersonalMessageEd25519({
  required List<int> message,
  required List<int> signature,
  required List<int> publicKey,
}) {
  return verifyRawSignatureEd25519(
    message: messageWithIntent(message),
    signature: signature,
    publicKey: publicKey,
  );
}
