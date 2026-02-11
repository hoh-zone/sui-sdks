import Foundation

public enum SuiPublicKeyError: Error, Equatable {
    case invalidSuiPublicKeyBase64
    case invalidSuiPublicKeyBytes
    case unsupportedScheme
}

public enum SuiPublicKeyFactory {
    public static func fromRawBytes(scheme: SignatureScheme, bytes: [UInt8]) throws -> any SuiPublicKeyProtocol {
        switch scheme {
        case .ed25519:
            return try Ed25519PublicKey(rawBytes: bytes)
        case .secp256k1:
            return try Secp256k1PublicKey(rawBytes: bytes)
        case .secp256r1:
            return try Secp256r1PublicKey(rawBytes: bytes)
        default:
            throw SuiPublicKeyError.unsupportedScheme
        }
    }

    public static func fromSuiBytes(_ bytes: [UInt8]) throws -> (scheme: SignatureScheme, publicKey: any SuiPublicKeyProtocol) {
        guard bytes.count >= 2 else {
            throw SuiPublicKeyError.invalidSuiPublicKeyBytes
        }
        guard let scheme = SignatureScheme(rawValue: bytes[0]) else {
            throw SuiPublicKeyError.unsupportedScheme
        }
        let key = try fromRawBytes(scheme: scheme, bytes: Array(bytes.dropFirst()))
        return (scheme, key)
    }

    public static func fromSuiPublicKeyBase64(_ value: String) throws -> (scheme: SignatureScheme, publicKey: any SuiPublicKeyProtocol) {
        guard let data = Data(base64Encoded: value) else {
            throw SuiPublicKeyError.invalidSuiPublicKeyBase64
        }
        return try fromSuiBytes([UInt8](data))
    }
}
