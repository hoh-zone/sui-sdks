import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

void main() {
  test('ed25519 sign and verify', () async {
    final keypair = await Ed25519Keypair.generate();
    final msg = utf8.encode('hello');

    final sig = await keypair.sign(msg);
    final ok = await keypair.verify(msg, sig);
    expect(ok, isTrue);
  });

  test('personal message intent verify', () async {
    final keypair = await Ed25519Keypair.generate();
    final msg = utf8.encode('sui');
    final sig = await keypair.signWithIntent(msg);
    final pub = await keypair.publicKeyBytes();

    final ok = await verifyPersonalMessageEd25519(
      message: msg,
      signature: sig,
      publicKey: pub,
    );
    expect(ok, isTrue);
  });

  test('from seed validates length', () async {
    await expectLater(Ed25519Keypair.fromSeed(List.filled(31, 0)), throwsArgumentError);
  });
}
