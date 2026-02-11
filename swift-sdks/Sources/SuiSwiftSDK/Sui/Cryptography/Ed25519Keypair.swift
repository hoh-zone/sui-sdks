import Foundation
import Crypto

public enum Ed25519Error: Error, Equatable {
    case invalidPrivateKeyLength
    case invalidPublicKeyLength
    case invalidSignatureLength
    case invalidSerializedSignature
    case invalidSecretKeyScheme
}

public struct Ed25519PublicKey: SuiPublicKeyProtocol {
    private let key: Curve25519.Signing.PublicKey

    public init(rawBytes: [UInt8]) throws {
        guard rawBytes.count == 32 else {
            throw Ed25519Error.invalidPublicKeyLength
        }
        self.key = try Curve25519.Signing.PublicKey(rawRepresentation: Data(rawBytes))
    }

    init(_ key: Curve25519.Signing.PublicKey) {
        self.key = key
    }

    public var rawBytes: [UInt8] {
        [UInt8](key.rawRepresentation)
    }

    public func toSuiBytes() -> [UInt8] {
        SuiPublicKeyEncoding.toSuiBytes(rawPublicKey: rawBytes, scheme: .ed25519)
    }

    public func toSuiPublicKey() -> String {
        SuiPublicKeyEncoding.toSuiPublicKeyBase64(rawPublicKey: rawBytes, scheme: .ed25519)
    }

    public func toSuiAddress() -> String {
        SuiPublicKeyEncoding.toSuiAddress(rawPublicKey: rawBytes, scheme: .ed25519)
    }

    public func verify(message: [UInt8], signature: [UInt8]) -> Bool {
        guard signature.count == 64 else {
            return false
        }
        return key.isValidSignature(Data(signature), for: Data(message))
    }

    public func verifySerialized(message: [UInt8], serializedSignature: String) throws -> Bool {
        let parsed = try SerializedSignature.parse(serializedSignature)
        guard parsed.scheme == .ed25519 else {
            throw Ed25519Error.invalidSerializedSignature
        }
        guard parsed.publicKey == rawBytes else {
            return false
        }
        return verify(message: message, signature: parsed.signature)
    }
}

public struct Ed25519Keypair {
    private let privateKey: Curve25519.Signing.PrivateKey

    public init() {
        self.privateKey = Curve25519.Signing.PrivateKey()
    }

    public init(privateKeyBytes: [UInt8]) throws {
        guard privateKeyBytes.count == 32 else {
            throw Ed25519Error.invalidPrivateKeyLength
        }
        self.privateKey = try Curve25519.Signing.PrivateKey(rawRepresentation: Data(privateKeyBytes))
    }

    public init(secretKey: String) throws {
        let parsed = try SuiPrivateKey.decode(secretKey)
        guard parsed.scheme == .ed25519 else {
            throw Ed25519Error.invalidSecretKeyScheme
        }
        try self.init(privateKeyBytes: parsed.secretKey)
    }

    public var publicKey: Ed25519PublicKey {
        Ed25519PublicKey(privateKey.publicKey)
    }

    public var privateKeyBytes: [UInt8] {
        [UInt8](privateKey.rawRepresentation)
    }

    public func toSuiAddress() -> String {
        publicKey.toSuiAddress()
    }

    public func getSecretKey() throws -> String {
        try SuiPrivateKey.encode(secretKey: privateKeyBytes, scheme: .ed25519)
    }

    public func sign(message: [UInt8]) throws -> [UInt8] {
        let signature = try privateKey.signature(for: Data(message))
        return [UInt8](signature)
    }

    public func verify(message: [UInt8], signature: [UInt8]) -> Bool {
        publicKey.verify(message: message, signature: signature)
    }

    public func toSerializedSignature(message: [UInt8]) throws -> String {
        let signature = try sign(message: message)
        return try SerializedSignature.toBase64(scheme: .ed25519, signature: signature, publicKey: publicKey.rawBytes)
    }

    public func signWithIntent(bytes: [UInt8], intent: IntentScope) throws -> SignatureWithBytes {
        let digest = SuiIntent.hashIntentMessage(scope: intent, message: bytes)
        let serialized = try toSerializedSignature(message: digest)
        return SignatureWithBytes(bytes: Data(bytes).base64EncodedString(), signature: serialized)
    }

    public func signTransaction(bytes: [UInt8]) throws -> SignatureWithBytes {
        try signWithIntent(bytes: bytes, intent: .transactionData)
    }

    public func signPersonalMessage(message: [UInt8]) throws -> SignatureWithBytes {
        let serializedMessage = try SuiIntent.serializePersonalMessage(message)
        let signed = try signWithIntent(bytes: serializedMessage, intent: .personalMessage)
        return SignatureWithBytes(bytes: Data(message).base64EncodedString(), signature: signed.signature)
    }

    public static func verifyWithPublicKey(publicKey: [UInt8], message: [UInt8], signature: [UInt8]) -> Bool {
        guard let key = try? Ed25519PublicKey(rawBytes: publicKey) else {
            return false
        }
        return key.verify(message: message, signature: signature)
    }
}

extension Ed25519Keypair: SuiSigner {}
