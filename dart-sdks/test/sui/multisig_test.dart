import 'dart:convert';

import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

void main() {
  test('threshold verification', () async {
    final k1 = await Ed25519Keypair.generate();
    final k2 = await Ed25519Keypair.generate();
    final k3 = await Ed25519Keypair.generate();

    final message = utf8.encode('m');
    final pub = MultisigPublicKey(
      publicKeys: [await k1.publicKeyBytes(), await k2.publicKeyBytes(), await k3.publicKeyBytes()],
      weights: [1, 1, 1],
      threshold: 2,
    );

    final signer = MultisigSigner(pub, scheme: SignatureScheme.ed25519);

    final ms = MultisigSignature(
      signatures: [await k1.sign(message), await k2.sign(message)],
      bitmap: [0, 1],
    );
    expect(await signer.verify(message, ms), isTrue);

    final low = MultisigSignature(signatures: [await k1.sign(message)], bitmap: [0]);
    expect(await signer.verify(message, low), isFalse);
  });

  test('duplicate index rejected', () async {
    final k1 = await Ed25519Keypair.generate();
    final message = utf8.encode('m');
    final pub = MultisigPublicKey(
      publicKeys: [await k1.publicKeyBytes()],
      weights: [2],
      threshold: 2,
    );
    final signer = MultisigSigner(pub, scheme: SignatureScheme.ed25519);
    final ms = MultisigSignature(
      signatures: [await k1.sign(message), await k1.sign(message)],
      bitmap: [0, 0],
    );
    expect(await signer.verify(message, ms), isFalse);
  });

  test('signature roundtrip serialization', () {
    final ms = MultisigSignature(signatures: [utf8.encode('sig1'), utf8.encode('sig2')], bitmap: [0, 2]);
    final encoded = ms.toBase64();
    final decoded = MultisigSignature.fromBase64(encoded);
    expect(decoded.bitmap, [0, 2]);
    expect(decoded.signatures.length, 2);
  });

  test('build and sign helpers', () async {
    final k1 = await Ed25519Keypair.generate();
    final k2 = await Ed25519Keypair.generate();
    final k3 = await Ed25519Keypair.generate();
    final message = utf8.encode('m');

    final pub = MultisigPublicKey(
      publicKeys: [await k1.publicKeyBytes(), await k2.publicKeyBytes(), await k3.publicKeyBytes()],
      weights: [1, 1, 1],
      threshold: 2,
    );
    final signer = MultisigSigner(pub, scheme: SignatureScheme.ed25519);

    final built = await signer.build(
      message,
      [
        (0, await k1.sign(message)),
        (1, await k2.sign(message)),
      ],
    );
    expect(built.bitmap, [0, 1]);
    expect(signer.isThresholdMet(built), isTrue);

    final signed = await signer.sign(
      message,
      [
        (1, (msg) => k2.sign(msg)),
        (2, (msg) => k3.sign(msg)),
      ],
    );
    expect(signed.bitmap, [1, 2]);
  });

  test('build validation errors', () async {
    final k1 = await Ed25519Keypair.generate();
    final k2 = await Ed25519Keypair.generate();
    final message = utf8.encode('m');

    final pub = MultisigPublicKey(
      publicKeys: [await k1.publicKeyBytes(), await k2.publicKeyBytes()],
      weights: [1, 1],
      threshold: 2,
    );
    final signer = MultisigSigner(pub, scheme: SignatureScheme.ed25519);

    await expectLater(signer.build(message, []), throwsArgumentError);

    await expectLater(
      signer.build(
        message,
        [
          (0, await k1.sign(message)),
          (0, await k2.sign(message)),
        ],
      ),
      throwsArgumentError,
    );
  });
}
