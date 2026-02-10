import XCTest
@testable import SuiSwiftSDK

final class BCSReaderWriterTests: XCTestCase {
    func testWriterReader() throws {
        var writer = BCSWriter()
        try writer.writeU8(1)
        try writer.writeU16(0x0203)
        writer.writeU32(0x04050607)
        writer.writeU64(0x08090A0B0C0D0E0F)
        writer.writeBool(true)
        try writer.writeULEB128(128)

        var reader = BCSReader(data: writer.toBytes())
        XCTAssertEqual(try reader.readU8(), 1)
        XCTAssertEqual(try reader.readU16(), 0x0203)
        XCTAssertEqual(try reader.readU32(), 0x04050607)
        XCTAssertEqual(try reader.readU64(), 0x08090A0B0C0D0E0F)
        XCTAssertEqual(try reader.readBool(), true)
        XCTAssertEqual(try reader.readULEB128(), 128)
        XCTAssertEqual(reader.remaining, 0)
    }

    func testInvalidBool() throws {
        var reader = BCSReader(data: [2])
        XCTAssertThrowsError(try reader.readBool()) { error in
            guard case let BCSError.invalidBool(value) = error else {
                return XCTFail("expected invalidBool")
            }
            XCTAssertEqual(value, 2)
        }
    }
}
