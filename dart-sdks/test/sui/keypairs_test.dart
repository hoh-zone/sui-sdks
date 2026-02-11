import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockSecp256k1Provider implements Secp256k1Provider {
  @override
  Future<Object> generate() async => 'k-gen';

  @override
  Future<Object> fromPrivateKeyBytes(List<int> privateKey) async =>
      'k-${privateKey.length}';

  @override
  Future<List<int>> sign(Object key, List<int> message) async =>
      <int>[message.length, 1, 2];

  @override
  Future<bool> verify(
          Object key, List<int> message, List<int> signature) async =>
      signature.isNotEmpty && signature.first == message.length;

  @override
  Future<List<int>> publicKeyBytes(Object key) async => <int>[2, 3, 4];

  @override
  Future<bool> verifyRawSignature({
    required List<int> message,
    required List<int> signature,
    required List<int> publicKey,
  }) async =>
      signature.isNotEmpty && publicKey.isNotEmpty;
}

void main() {
  test('ed25519 sign verify', () async {
    final kp = await Ed25519Keypair.generate();
    final pub = await kp.publicKeyBytes();
    expect(pub.length, 32);

    final msg = utf8.encode('hello');
    final sig = await kp.sign(msg);
    expect(sig.length, 64);

    expect(await kp.verify(msg, sig), isTrue);
    expect(await kp.verify(utf8.encode('other'), sig), isFalse);
  });

  test('secp256r1 currently unsupported on pure Dart runtime', () async {
    await expectLater(Secp256r1Keypair.generate(), throwsUnsupportedError);
  });

  test('secp256r1 fromPrivateKeyBytes validates length', () async {
    await expectLater(Secp256r1Keypair.fromPrivateKeyBytes(List.filled(31, 1)),
        throwsArgumentError);
  });

  test('secp256k1 currently unsupported', () async {
    Secp256k1Keypair.registerProvider(null);
    await expectLater(Secp256k1Keypair.generate(), throwsUnsupportedError);
  });

  test('secp256k1 works with injected provider', () async {
    Secp256k1Keypair.registerProvider(_MockSecp256k1Provider());
    addTearDown(() => Secp256k1Keypair.registerProvider(null));

    final kp = await Secp256k1Keypair.generate();
    final sig = await kp.sign(const [7, 8, 9]);
    expect(sig, [3, 1, 2]);
    expect(await kp.verify(const [7, 8, 9], sig), isTrue);
    expect(await kp.publicKeyBytes(), [2, 3, 4]);
  });
}
