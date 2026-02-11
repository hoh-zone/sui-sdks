import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

class _MockSecp256k1Provider implements Secp256k1Provider {
  @override
  Future<Object> generate() async => 'k';

  @override
  Future<Object> fromPrivateKeyBytes(List<int> privateKey) async => 'k';

  @override
  Future<List<int>> sign(Object key, List<int> message) async => <int>[1];

  @override
  Future<bool> verify(
          Object key, List<int> message, List<int> signature) async =>
      signature.isNotEmpty;

  @override
  Future<List<int>> publicKeyBytes(Object key) async => <int>[3];

  @override
  Future<bool> verifyRawSignature({
    required List<int> message,
    required List<int> signature,
    required List<int> publicKey,
  }) async =>
      message.isNotEmpty && signature.isNotEmpty && publicKey.isNotEmpty;
}

void main() {
  test('verify raw and personal message ed25519', () async {
    final kp = await Ed25519Keypair.generate();
    final msg = utf8.encode('abc');

    final sigRaw = await kp.sign(msg);
    final pub = await kp.publicKeyBytes();
    expect(
      await verifyRawSignature(
        message: msg,
        signature: sigRaw,
        publicKey: pub,
        scheme: SignatureScheme.ed25519,
      ),
      isTrue,
    );

    final personalPayload = utf8.encode('\x19Sui Signed Message:\n3\nabc');
    final sigPersonal = await kp.sign(personalPayload);
    expect(
      await verifyPersonalMessage(
        message: msg,
        signature: sigPersonal,
        publicKey: pub,
        scheme: SignatureScheme.ed25519,
      ),
      isTrue,
    );
  });

  test('serialized signature roundtrip ed25519', () async {
    final kp = await Ed25519Keypair.generate();
    final sig = await kp.sign(utf8.encode('hello'));
    final pub = await kp.publicKeyBytes();

    final ser = toSerializedSignature(
      scheme: SignatureScheme.ed25519,
      signature: sig,
      publicKey: pub,
    );
    final parsed = parseSerializedSignature(ser);
    expect(parsed.scheme, SignatureScheme.ed25519);
    expect(parsed.signature, sig);
    expect(parsed.publicKey, pub);
  });

  test('serialized signature length validation', () async {
    final kp = await Ed25519Keypair.generate();
    final sig = await kp.sign(utf8.encode('m'));
    final pub = await kp.publicKeyBytes();

    expect(
      () => toSerializedSignature(
        scheme: SignatureScheme.secp256k1,
        signature: sig,
        publicKey: pub,
      ),
      throwsArgumentError,
    );
  });

  test('parse serialized signature rejects unknown scheme', () {
    final raw = <int>[9, ...List.filled(64, 0), ...List.filled(32, 0)];
    final bad = base64Encode(raw);
    expect(() => parseSerializedSignature(bad), throwsStateError);
  });

  test('verify raw secp256k1 works with injected provider', () async {
    Secp256k1Keypair.registerProvider(_MockSecp256k1Provider());
    addTearDown(() => Secp256k1Keypair.registerProvider(null));

    final ok = await verifyRawSignature(
      message: const [1],
      signature: const [2],
      publicKey: const [3],
      scheme: SignatureScheme.secp256k1,
    );
    expect(ok, isTrue);
  });
}
