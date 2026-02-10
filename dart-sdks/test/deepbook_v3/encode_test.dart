import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

void main() {
  test('encode bool', () {
    expect(encodeBool(true), [1]);
    expect(encodeBool(false), [0]);
  });

  test('encode u64', () {
    expect(encodeU64(123), [123, 0, 0, 0, 0, 0, 0, 0]);
  });

  test('encode u128', () {
    expect(encodeU128(1), [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    expect(() => encodeU128(-1), throwsRangeError);
  });

  test('encode vec u128', () {
    final out = encodeVecU128([1, 2]);
    expect(out.length, 33);
    expect(out.first, 2);
  });
}
