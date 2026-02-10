import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

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
}
