import 'dart:convert';

import 'verify.dart';
import 'crypto.dart';

const int maxSignerInMultisig = 10;
const int minSignerInMultisig = 1;

class MultisigPublicKey {
  MultisigPublicKey({
    required this.publicKeys,
    required this.weights,
    required this.threshold,
  }) {
    if (publicKeys.length != weights.length) {
      throw ArgumentError('publicKeys and weights length mismatch');
    }
    if (publicKeys.length < minSignerInMultisig) {
      throw ArgumentError('min number of signers in a multisig is $minSignerInMultisig');
    }
    if (publicKeys.length > maxSignerInMultisig) {
      throw ArgumentError('max number of signers in a multisig is $maxSignerInMultisig');
    }
    if (threshold <= 0) {
      throw ArgumentError('invalid threshold');
    }
    if (weights.any((w) => w <= 0)) {
      throw ArgumentError('weights must be positive');
    }
    if (threshold > totalWeight()) {
      throw ArgumentError('unreachable threshold');
    }
  }

  final List<List<int>> publicKeys;
  final List<int> weights;
  final int threshold;

  int totalWeight() => weights.fold(0, (a, b) => a + b);
}

class MultisigSignature {
  const MultisigSignature({required this.signatures, required this.bitmap});

  final List<List<int>> signatures;
  final List<int> bitmap;

  void validate() {
    if (signatures.length != bitmap.length) {
      throw ArgumentError('signatures and bitmap length mismatch');
    }
    if (signatures.length > maxSignerInMultisig) {
      throw ArgumentError('too many signatures');
    }

    final seen = <int>{};
    for (final idx in bitmap) {
      if (idx < 0) {
        throw ArgumentError('bitmap index must be non-negative');
      }
      if (seen.contains(idx)) {
        throw ArgumentError('duplicate bitmap index');
      }
      seen.add(idx);
    }
  }

  String toBase64() {
    validate();
    final payload = <String, dynamic>{
      'bitmap': bitmap,
      'signatures': signatures.map(base64Encode).toList(),
    };
    return base64Encode(utf8.encode(jsonEncode(payload)));
  }

  static MultisigSignature fromBase64(String serialized) {
    final payload = jsonDecode(utf8.decode(base64Decode(serialized))) as Map<String, dynamic>;
    return MultisigSignature(
      signatures: ((payload['signatures'] as List?) ?? const <dynamic>[])
          .map((x) => base64Decode('$x'))
          .toList(),
      bitmap: ((payload['bitmap'] as List?) ?? const <dynamic>[])
          .map((x) => int.parse('$x'))
          .toList(),
    );
  }
}

typedef RawSigner = Future<List<int>> Function(List<int> message);

class MultisigSigner {
  const MultisigSigner(this.pubkey, {this.scheme = SignatureScheme.ed25519});

  final MultisigPublicKey pubkey;
  final SignatureScheme scheme;

  Future<bool> verify(List<int> message, MultisigSignature multisig) async {
    try {
      multisig.validate();
    } catch (_) {
      return false;
    }

    final seen = <int>{};
    var totalWeight = 0;

    for (var i = 0; i < multisig.bitmap.length; i++) {
      final idx = multisig.bitmap[i];
      if (seen.contains(idx)) {
        return false;
      }
      seen.add(idx);
      if (idx < 0 || idx >= pubkey.publicKeys.length) {
        return false;
      }

      final ok = await verifyRawSignature(
        message: message,
        signature: multisig.signatures[i],
        publicKey: pubkey.publicKeys[idx],
        scheme: scheme,
      );
      if (!ok) {
        return false;
      }

      totalWeight += pubkey.weights[idx];
    }

    return totalWeight >= pubkey.threshold;
  }

  bool isThresholdMet(MultisigSignature multisig) {
    try {
      multisig.validate();
    } catch (_) {
      return false;
    }

    var totalWeight = 0;
    for (final idx in multisig.bitmap) {
      if (idx < 0 || idx >= pubkey.weights.length) {
        return false;
      }
      totalWeight += pubkey.weights[idx];
    }
    return totalWeight >= pubkey.threshold;
  }

  Future<MultisigSignature> build(
    List<int> message,
    List<(int, List<int>)> indexedSignatures, {
    bool requireThreshold = true,
  }) async {
    if (indexedSignatures.isEmpty) {
      throw ArgumentError('no signatures provided');
    }

    final multisig = MultisigSignature(
      signatures: indexedSignatures.map((x) => x.$2).toList(),
      bitmap: indexedSignatures.map((x) => x.$1).toList(),
    );
    multisig.validate();

    if (!await verify(message, multisig)) {
      throw ArgumentError('invalid multisig signatures');
    }
    if (requireThreshold && !isThresholdMet(multisig)) {
      throw ArgumentError('threshold not met');
    }
    return multisig;
  }

  Future<MultisigSignature> sign(
    List<int> message,
    List<(int, RawSigner)> indexedSigners, {
    bool requireThreshold = true,
  }) async {
    if (indexedSigners.isEmpty) {
      throw ArgumentError('no signers provided');
    }

    final indexedSignatures = <(int, List<int>)>[];
    for (final (idx, signer) in indexedSigners) {
      indexedSignatures.add((idx, await signer(message)));
    }
    return build(message, indexedSignatures, requireThreshold: requireThreshold);
  }
}
