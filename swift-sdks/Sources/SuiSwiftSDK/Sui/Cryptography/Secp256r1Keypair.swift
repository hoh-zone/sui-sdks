import Foundation
import Crypto

public enum Secp256r1Error: Error, Equatable {
    case invalidPrivateKeyLength
    case invalidPublicKeyLength
    case invalidPublicKeyEncoding
    case invalidSignatureLength
    case invalidSignatureEncoding
    case invalidSerializedSignature
    case invalidSecretKeyScheme
}

public struct Secp256r1PublicKey: SuiPublicKeyProtocol {
    private let key: P256.Signing.PublicKey

    public init(rawBytes: [UInt8]) throws {
        guard rawBytes.count == 33 else {
            throw Secp256r1Error.invalidPublicKeyLength
        }
        guard let key = try? P256.Signing.PublicKey(compactRepresentation: Data(rawBytes)) else {
            throw Secp256r1Error.invalidPublicKeyEncoding
        }
        self.key = key
    }

    init(_ key: P256.Signing.PublicKey) {
        self.key = key
    }

    public var rawBytes: [UInt8] {
        if let compact = key.compactRepresentation {
            return [UInt8](compact)
        }
        return [UInt8](key.x963Representation)
    }

    public func toSuiBytes() -> [UInt8] {
        SuiPublicKeyEncoding.toSuiBytes(rawPublicKey: rawBytes, scheme: .secp256r1)
    }

    public func toSuiPublicKey() -> String {
        SuiPublicKeyEncoding.toSuiPublicKeyBase64(rawPublicKey: rawBytes, scheme: .secp256r1)
    }

    public func toSuiAddress() -> String {
        SuiPublicKeyEncoding.toSuiAddress(rawPublicKey: rawBytes, scheme: .secp256r1)
    }

    public func verify(message: [UInt8], signature: [UInt8]) -> Bool {
        guard signature.count == 64 else {
            return false
        }
        guard let sig = try? P256.Signing.ECDSASignature(rawRepresentation: Data(signature)) else {
            return false
        }
        return key.isValidSignature(sig, for: Data(message))
    }

    public func verifySerialized(message: [UInt8], serializedSignature: String) throws -> Bool {
        let parsed = try SerializedSignature.parse(serializedSignature)
        guard parsed.scheme == .secp256r1 else {
            throw Secp256r1Error.invalidSerializedSignature
        }
        guard parsed.publicKey == rawBytes else {
            return false
        }
        return verify(message: message, signature: parsed.signature)
    }
}

public struct Secp256r1Keypair {
    private let privateKey: P256.Signing.PrivateKey

    public init() {
        self.privateKey = P256.Signing.PrivateKey()
    }

    public init(privateKeyBytes: [UInt8]) throws {
        guard privateKeyBytes.count == 32 else {
            throw Secp256r1Error.invalidPrivateKeyLength
        }
        self.privateKey = try P256.Signing.PrivateKey(rawRepresentation: Data(privateKeyBytes))
    }

    public init(secretKey: String) throws {
        let parsed = try SuiPrivateKey.decode(secretKey)
        guard parsed.scheme == .secp256r1 else {
            throw Secp256r1Error.invalidSecretKeyScheme
        }
        try self.init(privateKeyBytes: parsed.secretKey)
    }

    public var publicKey: Secp256r1PublicKey {
        Secp256r1PublicKey(privateKey.publicKey)
    }

    public var privateKeyBytes: [UInt8] {
        [UInt8](privateKey.rawRepresentation)
    }

    public func toSuiAddress() -> String {
        publicKey.toSuiAddress()
    }

    public func getSecretKey() throws -> String {
        try SuiPrivateKey.encode(secretKey: privateKeyBytes, scheme: .secp256r1)
    }

    public func sign(message: [UInt8]) throws -> [UInt8] {
        let signature = try privateKey.signature(for: Data(message))
        return [UInt8](signature.rawRepresentation)
    }

    public func verify(message: [UInt8], signature: [UInt8]) -> Bool {
        publicKey.verify(message: message, signature: signature)
    }

    public func toSerializedSignature(message: [UInt8]) throws -> String {
        let signature = try sign(message: message)
        return try SerializedSignature.toBase64(scheme: .secp256r1, signature: signature, publicKey: publicKey.rawBytes)
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
        guard let key = try? Secp256r1PublicKey(rawBytes: publicKey) else {
            return false
        }
        return key.verify(message: message, signature: signature)
    }
}

extension Secp256r1Keypair: SuiSigner {}
