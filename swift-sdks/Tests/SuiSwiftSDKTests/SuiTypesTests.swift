import XCTest
@testable import SuiSwiftSDK

final class SuiTypesTests: XCTestCase {
    func testNormalizeSuiAddress() {
        XCTAssertEqual(
            SuiTypes.normalizeSuiAddress("0x2"),
            "0x0000000000000000000000000000000000000000000000000000000000000002"
        )
    }

    func testIsValidSuiAddress() {
        XCTAssertTrue(SuiTypes.isValidSuiAddress("0x" + String(repeating: "a1", count: 32)))
        XCTAssertFalse(SuiTypes.isValidSuiAddress("0x1"))
        XCTAssertFalse(SuiTypes.isValidSuiAddress("xyz"))
    }

    func testIsValidTransactionDigest() {
        XCTAssertTrue(SuiTypes.isValidTransactionDigest(String(repeating: "1", count: 32)))
        XCTAssertFalse(SuiTypes.isValidTransactionDigest(""))
        XCTAssertFalse(SuiTypes.isValidTransactionDigest("0OIl"))
    }
}
