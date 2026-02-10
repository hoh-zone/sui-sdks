import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

void main() {
  test('uleb roundtrip', () {
    for (final value in <int>[0, 1, 127, 128, 16384, 0xFFFFFFFF]) {
      final encoded = encodeUleb128(value);
      final decoded = decodeUleb128(encoded);
      expect(decoded.value, value);
      expect(decoded.consumed, encoded.length);
    }
  });

  test('uleb non-canonical rejected', () {
    expect(() => decodeUleb128(const [0x80, 0x00]), throwsStateError);
  });

  test('uleb overflow rejected', () {
    expect(() => decodeUleb128(encodeUleb128(0x100000000)), throwsStateError);
  });
}
