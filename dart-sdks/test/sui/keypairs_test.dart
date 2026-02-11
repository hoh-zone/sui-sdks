import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

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

  test('secp256k1 currently unsupported', () async {
    await expectLater(Secp256k1Keypair.generate(), throwsUnsupportedError);
  });
}
