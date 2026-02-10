import 'package:dart_sdks/dart_sdks.dart';
import 'package:test/test.dart';

void main() {
  test('read and write primitives', () {
    final writer = BcsWriter();
    writer.writeU8(1);
    writer.writeU16(0x2233);
    writer.writeU32(0x44556677);
    writer.writeU64(0x1122334455667788);
    writer.writeBool(true);
    writer.writeUleb128(300);

    final reader = BcsReader(writer.toBytes());
    expect(reader.readU8(), 1);
    expect(reader.readU16(), 0x2233);
    expect(reader.readU32(), 0x44556677);
    expect(reader.readU64(), 0x1122334455667788);
    expect(reader.readBool(), isTrue);
    expect(reader.readUleb128(), 300);
    expect(reader.remaining(), 0);
  });

  test('bool non canonical rejected', () {
    final reader = BcsReader(const [2]);
    expect(reader.readBool, throwsStateError);
  });
}
