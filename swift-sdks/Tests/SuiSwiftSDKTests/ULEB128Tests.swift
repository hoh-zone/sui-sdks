import XCTest
@testable import SuiSwiftSDK

final class ULEB128Tests: XCTestCase {
    func testEncodeDecodeRoundTrip() throws {
        let encoded = ULEB128.encode(624485)
        XCTAssertEqual(encoded, [0xE5, 0x8E, 0x26])

        let decoded = try ULEB128.decode(encoded)
        XCTAssertEqual(decoded.value, 624485)
        XCTAssertEqual(decoded.consumed, 3)
    }

    func testDecodeNonCanonical() throws {
        XCTAssertThrowsError(try ULEB128.decode([0x80, 0x00])) { error in
            XCTAssertEqual(error as? ULEB128Error, .nonCanonical)
        }
    }

    func testDecodeExceedsU32() throws {
        XCTAssertThrowsError(try ULEB128.decode([0xff, 0xff, 0xff, 0xff, 0x10])) { error in
            XCTAssertEqual(error as? ULEB128Error, .exceedsU32)
        }
    }
}
