import Foundation

public enum VerifyError: Error, Equatable {
    case unsupportedScheme
}

public enum SuiVerify {
    public static func verifyRawSignature(
        message: [UInt8],
        signature: [UInt8],
        publicKey: [UInt8],
        scheme: SignatureScheme
    ) throws -> Bool {
        switch scheme {
        case .ed25519:
            return Ed25519Keypair.verifyWithPublicKey(publicKey: publicKey, message: message, signature: signature)
        case .secp256k1:
            return Secp256k1Keypair.verifyWithPublicKey(publicKey: publicKey, message: message, signature: signature)
        case .secp256r1:
            return Secp256r1Keypair.verifyWithPublicKey(publicKey: publicKey, message: message, signature: signature)
        default:
            throw VerifyError.unsupportedScheme
        }
    }

    public static func verifyPersonalMessage(
        message: [UInt8],
        signature: [UInt8],
        publicKey: [UInt8],
        scheme: SignatureScheme
    ) throws -> Bool {
        let serializedMessage = try SuiIntent.serializePersonalMessage(message)
        return try verifyWithIntent(
            message: serializedMessage,
            signature: signature,
            publicKey: publicKey,
            scheme: scheme,
            intent: .personalMessage
        )
    }

    public static func verifyTransaction(
        transaction: [UInt8],
        signature: [UInt8],
        publicKey: [UInt8],
        scheme: SignatureScheme
    ) throws -> Bool {
        return try verifyWithIntent(
            message: transaction,
            signature: signature,
            publicKey: publicKey,
            scheme: scheme,
            intent: .transactionData
        )
    }

    public static func verifySerializedSignature(message: [UInt8], serializedSignature: String) throws -> Bool {
        let parsed = try SerializedSignature.parse(serializedSignature)
        return try verifyRawSignature(
            message: message,
            signature: parsed.signature,
            publicKey: parsed.publicKey,
            scheme: parsed.scheme
        )
    }

    public static func verifySerializedTransactionSignature(
        transaction: [UInt8],
        serializedSignature: String
    ) throws -> Bool {
        let parsed = try SerializedSignature.parse(serializedSignature)
        return try verifyTransaction(
            transaction: transaction,
            signature: parsed.signature,
            publicKey: parsed.publicKey,
            scheme: parsed.scheme
        )
    }

    public static func verifySerializedPersonalMessageSignature(
        message: [UInt8],
        serializedSignature: String
    ) throws -> Bool {
        let parsed = try SerializedSignature.parse(serializedSignature)
        return try verifyPersonalMessage(
            message: message,
            signature: parsed.signature,
            publicKey: parsed.publicKey,
            scheme: parsed.scheme
        )
    }

    public static func verifyWithIntent(
        message: [UInt8],
        signature: [UInt8],
        publicKey: [UInt8],
        scheme: SignatureScheme,
        intent: IntentScope
    ) throws -> Bool {
        let digest = SuiIntent.hashIntentMessage(scope: intent, message: message)
        return try verifyRawSignature(message: digest, signature: signature, publicKey: publicKey, scheme: scheme)
    }
}
