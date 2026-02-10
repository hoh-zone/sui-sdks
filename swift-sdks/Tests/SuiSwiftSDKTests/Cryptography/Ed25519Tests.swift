import XCTest
@testable import SuiSwiftSDK

final class Ed25519Tests: XCTestCase {
    func testSignAndVerify() throws {
        let keypair = Ed25519Keypair()
        let message = Array("hello sui".utf8)

        let signature = try keypair.sign(message: message)
        XCTAssertEqual(signature.count, 64)
        XCTAssertTrue(keypair.verify(message: message, signature: signature))
        XCTAssertFalse(keypair.verify(message: Array("bad".utf8), signature: signature))
    }

    func testSerializedSignatureRoundTrip() throws {
        let keypair = Ed25519Keypair()
        let message = Array("serialized".utf8)

        let serialized = try keypair.toSerializedSignature(message: message)
        let parsed = try SerializedSignature.parse(serialized)

        XCTAssertEqual(parsed.scheme, .ed25519)
        XCTAssertEqual(parsed.publicKey, keypair.publicKey.rawBytes)
        XCTAssertTrue(try SuiVerify.verifySerializedSignature(message: message, serializedSignature: serialized))
    }

    func testVerifyPersonalMessage() throws {
        let keypair = Ed25519Keypair()
        let message = Array("wallet msg".utf8)
        let signed = try keypair.signPersonalMessage(message: message)
        let parsed = try SerializedSignature.parse(signed.signature)

        XCTAssertTrue(
            try SuiVerify.verifyPersonalMessage(
                message: message,
                signature: parsed.signature,
                publicKey: parsed.publicKey,
                scheme: parsed.scheme
            )
        )
    }

    func testSignTransactionAndVerify() throws {
        let keypair = Ed25519Keypair()
        let txBytes = Array("tx-bytes".utf8)
        let signed = try keypair.signTransaction(bytes: txBytes)
        XCTAssertEqual(signed.bytes, Data(txBytes).base64EncodedString())
        XCTAssertTrue(try SuiVerify.verifySerializedTransactionSignature(transaction: txBytes, serializedSignature: signed.signature))
    }
}
