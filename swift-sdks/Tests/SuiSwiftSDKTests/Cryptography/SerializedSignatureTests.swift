import XCTest
@testable import SuiSwiftSDK

final class SerializedSignatureTests: XCTestCase {
    func testParseRejectsInvalidBase64() {
        XCTAssertThrowsError(try SerializedSignature.parse("not-base64")) { error in
            XCTAssertEqual(error as? SerializedSignatureError, .invalidBase64)
        }
    }

    func testToBase64RejectsWrongSizes() {
        XCTAssertThrowsError(
            try SerializedSignature.toBase64(
                scheme: .ed25519,
                signature: [UInt8](repeating: 1, count: 63),
                publicKey: [UInt8](repeating: 2, count: 32)
            )
        ) { error in
            XCTAssertEqual(error as? SerializedSignatureError, .invalidSignatureLength)
        }

        XCTAssertThrowsError(
            try SerializedSignature.toBase64(
                scheme: .ed25519,
                signature: [UInt8](repeating: 1, count: 64),
                publicKey: [UInt8](repeating: 2, count: 31)
            )
        ) { error in
            XCTAssertEqual(error as? SerializedSignatureError, .invalidPublicKeyLength)
        }
    }
}
