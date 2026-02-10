import Foundation
import Crypto

public enum SuiPublicKeyEncoding {
    public static func toSuiBytes(rawPublicKey: [UInt8], scheme: SignatureScheme) -> [UInt8] {
        [scheme.rawValue] + rawPublicKey
    }

    public static func toSuiPublicKeyBase64(rawPublicKey: [UInt8], scheme: SignatureScheme) -> String {
        Data(toSuiBytes(rawPublicKey: rawPublicKey, scheme: scheme)).base64EncodedString()
    }

    public static func toSuiAddress(rawPublicKey: [UInt8], scheme: SignatureScheme) -> String {
        let suiBytes = toSuiBytes(rawPublicKey: rawPublicKey, scheme: scheme)
        let digest = SHA256.hash(data: Data(suiBytes))
        let hex = digest.map { String(format: "%02x", $0) }.joined()
        return SuiTypes.normalizeSuiAddress(hex)
    }
}
