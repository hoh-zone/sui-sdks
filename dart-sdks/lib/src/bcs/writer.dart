import 'dart:typed_data';

import 'uleb.dart';

class BcsWriter {
  final BytesBuilder _builder = BytesBuilder(copy: false);

  void writeU8(int value) {
    _assertInRange(value, 0, 0xFF, 'u8');
    _builder.add([value]);
  }

  void writeU16(int value) => _writeInt(value, 2);

  void writeU32(int value) => _writeInt(value, 4);

  void writeU64(int value) => _writeInt(value, 8);

  void writeBool(bool value) {
    writeU8(value ? 1 : 0);
  }

  void writeBytes(List<int> value) {
    _builder.add(value);
  }

  void writeUleb128(int value) {
    _builder.add(encodeUleb128(value));
  }

  Uint8List toBytes() => _builder.toBytes();

  void _writeInt(int value, int len) {
    if (value < 0) {
      throw ArgumentError.value(value, 'value', 'negative integer not allowed');
    }
    final max = (BigInt.one << (8 * len)) - BigInt.one;
    if (BigInt.from(value) > max) {
      throw RangeError.value(value, 'value', 'u${8 * len} out of range');
    }

    final data = ByteData(len);
    switch (len) {
      case 2:
        data.setUint16(0, value, Endian.little);
        break;
      case 4:
        data.setUint32(0, value, Endian.little);
        break;
      case 8:
        data.setUint64(0, value, Endian.little);
        break;
      default:
        throw UnsupportedError('unsupported int width: $len');
    }
    _builder.add(data.buffer.asUint8List());
  }

  void _assertInRange(int value, int min, int max, String type) {
    if (value < min || value > max) {
      throw RangeError.value(value, 'value', '$type out of range');
    }
  }
}
