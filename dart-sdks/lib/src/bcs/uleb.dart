import 'dart:typed_data';

const int maxUleb128Value = 0xFFFFFFFF;

class DecodedUleb128 {
  const DecodedUleb128(this.value, this.consumed);

  final int value;
  final int consumed;
}

Uint8List encodeUleb128(int value) {
  if (value < 0) {
    throw ArgumentError.value(value, 'value', 'uleb128 cannot encode negative value');
  }
  if (value == 0) {
    return Uint8List.fromList(const [0]);
  }

  final out = <int>[];
  var v = value;
  while (v > 0) {
    var b = v & 0x7F;
    v >>= 7;
    if (v > 0) {
      b |= 0x80;
    }
    out.add(b);
  }
  return Uint8List.fromList(out);
}

DecodedUleb128 decodeUleb128(List<int> data) {
  var total = 0;
  var shift = 0;

  for (var i = 0; i < data.length; i++) {
    final byte = data[i];
    total |= (byte & 0x7F) << shift;

    if ((byte & 0x80) == 0) {
      final consumed = i + 1;
      if (total > maxUleb128Value) {
        throw StateError('uleb128 exceeds u32 range');
      }
      final canonical = encodeUleb128(total);
      if (canonical.length != consumed) {
        throw StateError('non-canonical uleb128 encoding');
      }
      return DecodedUleb128(total, consumed);
    }

    shift += 7;
    if (shift >= 64) {
      throw StateError('uleb128 overflow');
    }
  }

  throw StateError('uleb128 buffer overflow');
}
