import 'dart:typed_data';

import '../../bcs/writer.dart';

Uint8List encodeBool(bool value) => Uint8List.fromList([value ? 1 : 0]);

Uint8List encodeU64(int value) {
  final w = BcsWriter();
  w.writeU64(value);
  return w.toBytes();
}

Uint8List encodeU128(dynamic value) {
  final n = value is BigInt ? value : BigInt.parse('$value');
  if (n < BigInt.zero || n >= (BigInt.one << 128)) {
    throw RangeError.value(value, 'value', 'u128 out of range');
  }

  final out = Uint8List(16);
  var tmp = n;
  for (var i = 0; i < 16; i++) {
    out[i] = (tmp & BigInt.from(0xFF)).toInt();
    tmp = tmp >> 8;
  }
  return out;
}

Uint8List encodeVecU128(Iterable<dynamic> values) {
  final list = values.toList();
  final w = BcsWriter();
  w.writeUleb128(list.length);
  for (final value in list) {
    w.writeBytes(encodeU128(value));
  }
  return w.toBytes();
}
