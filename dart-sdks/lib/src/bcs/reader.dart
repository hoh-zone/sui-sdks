import 'dart:typed_data';

import 'uleb.dart';

class BcsReader {
  BcsReader(List<int> bytes) : _bytes = Uint8List.fromList(bytes);

  final Uint8List _bytes;
  int _pos = 0;

  int get position => _pos;

  int remaining() => _bytes.length - _pos;

  int readU8() {
    _ensureInRange(1);
    return _bytes[_pos++];
  }

  int readU16() {
    final bytes = readBytes(2);
    return bytes.buffer.asByteData(bytes.offsetInBytes, 2).getUint16(0, Endian.little);
  }

  int readU32() {
    final bytes = readBytes(4);
    return bytes.buffer.asByteData(bytes.offsetInBytes, 4).getUint32(0, Endian.little);
  }

  int readU64() {
    final bytes = readBytes(8);
    return bytes.buffer.asByteData(bytes.offsetInBytes, 8).getUint64(0, Endian.little);
  }

  Uint8List readBytes(int n) {
    if (n < 0) {
      throw RangeError.value(n, 'n', 'must be non-negative');
    }
    _ensureInRange(n);
    final out = _bytes.sublist(_pos, _pos + n);
    _pos += n;
    return Uint8List.fromList(out);
  }

  bool readBool() {
    final v = readU8();
    if (v == 0) {
      return false;
    }
    if (v == 1) {
      return true;
    }
    throw StateError('invalid bool byte: $v');
  }

  int readUleb128() {
    final decoded = decodeUleb128(_bytes.sublist(_pos));
    _pos += decoded.consumed;
    return decoded.value;
  }

  void _ensureInRange(int len) {
    if (remaining() < len) {
      throw StateError('bcs: out of range');
    }
  }
}
