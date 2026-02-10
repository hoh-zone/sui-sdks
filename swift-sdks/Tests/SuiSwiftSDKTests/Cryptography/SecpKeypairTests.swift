import XCTest
@testable import SuiSwiftSDK

final class SecpKeypairTests: XCTestCase {
    func testSecp256r1SignAndVerify() throws {
        let keypair = Secp256r1Keypair()
        let message = Array("secp256r1".utf8)

        let signature = try keypair.sign(message: message)
        XCTAssertEqual(signature.count, 64)
        XCTAssertTrue(keypair.verify(message: message, signature: signature))
    }

    func testSecp256k1SignAndVerify() throws {
        let keypair = try Secp256k1Keypair()
        let message = Array("secp256k1".utf8)

        let signature = try keypair.sign(message: message)
        XCTAssertEqual(signature.count, 64)
        XCTAssertTrue(keypair.verify(message: message, signature: signature))
    }

    func testSerializedSignatureVerifyForSecpSchemes() throws {
        let msg = Array("serialized-secp".utf8)

        let k1 = try Secp256k1Keypair()
        let sigK1 = try k1.toSerializedSignature(message: msg)
        XCTAssertTrue(try SuiVerify.verifySerializedSignature(message: msg, serializedSignature: sigK1))

        let r1 = Secp256r1Keypair()
        let sigR1 = try r1.toSerializedSignature(message: msg)
        XCTAssertTrue(try SuiVerify.verifySerializedSignature(message: msg, serializedSignature: sigR1))
    }

    func testIntentSigningForSecpSchemes() throws {
        let tx = Array("tx-secp".utf8)
        let personal = Array("personal-secp".utf8)

        let k1 = try Secp256k1Keypair()
        let k1TxSig = try k1.signTransaction(bytes: tx)
        XCTAssertTrue(try SuiVerify.verifySerializedTransactionSignature(transaction: tx, serializedSignature: k1TxSig.signature))
        let k1MsgSig = try k1.signPersonalMessage(message: personal)
        XCTAssertTrue(try SuiVerify.verifySerializedPersonalMessageSignature(message: personal, serializedSignature: k1MsgSig.signature))

        let r1 = Secp256r1Keypair()
        let r1TxSig = try r1.signTransaction(bytes: tx)
        XCTAssertTrue(try SuiVerify.verifySerializedTransactionSignature(transaction: tx, serializedSignature: r1TxSig.signature))
        let r1MsgSig = try r1.signPersonalMessage(message: personal)
        XCTAssertTrue(try SuiVerify.verifySerializedPersonalMessageSignature(message: personal, serializedSignature: r1MsgSig.signature))
    }
}
