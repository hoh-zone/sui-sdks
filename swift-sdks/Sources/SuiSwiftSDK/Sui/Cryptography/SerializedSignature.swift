import Foundation

public struct ParsedSerializedSignature {
    public let scheme: SignatureScheme
    public let signature: [UInt8]
    public let publicKey: [UInt8]

    public init(scheme: SignatureScheme, signature: [UInt8], publicKey: [UInt8]) {
        self.scheme = scheme
        self.signature = signature
        self.publicKey = publicKey
    }
}

public enum SerializedSignatureError: Error, Equatable {
    case unsupportedScheme
    case invalidBase64
    case invalidSignatureLength
    case invalidPublicKeyLength
    case serializedTooShort
    case serializedLengthMismatch
}

public enum SerializedSignature {
    public static let rawSignatureSize = 64

    public static func toBase64(
        scheme: SignatureScheme,
        signature: [UInt8],
        publicKey: [UInt8]
    ) throws -> String {
        guard let expectedPublicKeySize = scheme.publicKeySize else {
            throw SerializedSignatureError.unsupportedScheme
        }
        guard signature.count == rawSignatureSize else {
            throw SerializedSignatureError.invalidSignatureLength
        }
        guard publicKey.count == expectedPublicKeySize else {
            throw SerializedSignatureError.invalidPublicKeyLength
        }

        var bytes: [UInt8] = [scheme.rawValue]
        bytes.append(contentsOf: signature)
        bytes.append(contentsOf: publicKey)
        return Data(bytes).base64EncodedString()
    }

    public static func parse(_ serialized: String) throws -> ParsedSerializedSignature {
        guard let raw = Data(base64Encoded: serialized) else {
            throw SerializedSignatureError.invalidBase64
        }
        let bytes = [UInt8](raw)
        guard bytes.count >= 1 + rawSignatureSize else {
            throw SerializedSignatureError.serializedTooShort
        }
        guard let scheme = SignatureScheme(rawValue: bytes[0]) else {
            throw SerializedSignatureError.unsupportedScheme
        }
        guard let expectedPublicKeySize = scheme.publicKeySize else {
            throw SerializedSignatureError.unsupportedScheme
        }

        let expectedLength = 1 + rawSignatureSize + expectedPublicKeySize
        guard bytes.count == expectedLength else {
            throw SerializedSignatureError.serializedLengthMismatch
        }

        let signature = Array(bytes[1..<(1 + rawSignatureSize)])
        let publicKey = Array(bytes[(1 + rawSignatureSize)...])
        return ParsedSerializedSignature(scheme: scheme, signature: signature, publicKey: publicKey)
    }
}
