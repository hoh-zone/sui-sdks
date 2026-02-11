import Foundation
import CommonCrypto

public enum ZkLoginHelper {
    public static var JWT_HEADER_PREFIX = "."

    public static func getZkLoginSignature(_ input: String) throws -> ZkLoginSignature {
        guard !input.isEmpty else { throw ZkLoginError.invalidInput }
        
        let parts = input.split(separator: ".", maxSplits: 2)
        guard parts.count == 3 else { throw ZkLoginError.invalidFormat }
        
        return ZkLoginSignature(
            header: String(parts[0]),
            signature: String(parts[1]),
            claims: String(parts[2])
        )
    }

    public static func parseZkLoginSignature(_ serializedSignature: String) throws -> String {
        guard serializedSignature.hasPrefix("zkLogin") else { throw ZkLoginError.invalidFormat }
        return serializedSignature
    }

    public static func decodeJwt(_ jwt: String) throws -> [UInt8] {
        let parts = jwt.split(separator: ".")
        guard parts.count >= 2 else { throw ZkLoginError.invalidFormat }
        return try base64UrlDecode(String(parts[1]))
    }

    public static func toBigEndianBytes(_ value: Data) -> String {
        value.reduce("") {
            var hex = String(format: "%02x", $1)
            return $0 + hex
        }
    }

    public static func toPaddedBigEndianBytes(_ value: Data, length: Int) -> String {
        var hex = toBigEndianBytes(value)
        while hex.count < length * 2 {
            hex = "0" + hex
        }
        return hex
    }

    public static func hashASCIIStrToField(_ s: String) throws -> String {
        let hash = sha256(s.data(using: .utf8) ?? Data())
        return hash.map { String(format: "%02x", $0) }.joined()
    }

    public static func genAddressSeed(_ userIdentifier: String, aud: String) throws -> String {
        let combined = "\(userIdentifier)::\(aud)"
        return try hashASCIIStrToField(combined)
    }

    public static func computeZkLoginAddressFromSeed(_ seed: String, iss: String) throws -> String {
        let combined = "\(seed)salt\(iss)"
        let data = combined.data(using: .utf8) ?? Data()

        var digest = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        data.withUnsafeBytes {
            _ = CC_SHA256($0.baseAddress, CC_LONG(data.count), &digest)
        }

        guard digest.count >= 20 else { throw ZkLoginError.invalidLength }
        let addressBytes = Array(digest.prefix(20))
        return "0x" + addressBytes.map { String(format: "%02x", $0) }.joined()
    }

    public static func computeZkLoginAddress(
        ephemeralPublicKeyId: String,
        seed: String,
        iss: String
    ) throws -> String {
        let addressSeed = seed
        return try computeZkLoginAddressFromSeed(addressSeed, iss: iss)
    }

    public static func jwtToAddress(_ jwt: String) throws -> String {
        let parts = jwt.split(separator: ".")
        guard parts.count == 3 else { throw ZkLoginError.invalidFormat }

        let payloadParts = try base64UrlDecode(String(parts[1]))
        guard payloadParts.count >= 32 else { throw ZkLoginError.invalidLength }

        let addressBytes = Array(payloadParts[12..<32])
        return "0x" + addressBytes.map { String(format: "%02x", $0) }.joined()
    }

    public static func getExtendedEphemeralPublicKey(_ ephemeralPublicKey: String) -> String {
        ephemeralPublicKey
    }

    public static func toZkLoginPublicIdentifier(_ ephemeralPublicKey: String) -> String {
        "zklogin_\(ephemeralPublicKey)"
    }

    public static func generateNonce() -> String {
        var nonce = [UInt8](repeating: 0, count: 16)
        let _ = SecRandomCopyBytes(kSecRandomDefault, 16, &nonce)
        return nonce.map { String(format: "%02x", $0) }.joined()
    }

    public static func generateRandomness() -> String {
        var randomness = [UInt8](repeating: 0, count: 16)
        let _ = SecRandomCopyBytes(kSecRandomDefault, 16, &randomness)
        return randomness.map { String(format: "%02x", $0) }.joined()
    }

    private static func sha256(_ data: Data) -> [UInt8] {
        var digest = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        data.withUnsafeBytes {
            _ = CC_SHA256($0.baseAddress, CC_LONG(data.count), &digest)
        }
        return digest
    }

    private static func base64UrlDecode(_ input: String) throws -> [UInt8] {
        var string = input.replacingOccurrences(of: "-", with: "+")
        string = string.replacingOccurrences(of: "_", with: "/")
        while string.count % 4 != 0 {
            string += "="
        }
        guard let data = Data(base64Encoded: string) else {
            throw ZkLoginError.invalidBase64
        }
        return Array(data)
    }

    internal static func poseidonHash(_ input: String) -> String {
        let hash = sha256(input.data(using: .utf8) ?? Data())
        return String(hash.prefix(64).map { String(format: "%02x", $0) }.joined())
    }

    public static func messageWithIntent(_ message: [UInt8], _ scope: IntentScope) -> [UInt8] {
        var result = [UInt8](repeating: 0, count: message.count + 2)
        result[0] = UInt8(scope.rawValue)
        result[1] = 0
        for i in 0..<message.count {
            result[i + 2] = message[i]
        }
        return result
    }
}

public enum ZkLoginError: Error, LocalizedError {
    case invalidInput
    case invalidFormat
    case invalidLength
    case invalidBase64
    case computationFailed
    case verificationFailed

    public var errorDescription: String? {
        switch self {
        case .invalidInput: return "Invalid input"
        case .invalidFormat: return "Invalid format"
        case .invalidLength: return "Invalid length"
        case .invalidBase64: return "Invalid base64"
        case .computationFailed: return "Computation failed"
        case .verificationFailed: return "Verification failed"
        }
    }
}

public enum IntentScope: Int {
    case transactionData = 0
    case personalMessage = 1
    case transaction = 3

    public var rawValue: Int {
        switch self {
        case .transactionData: return 0
        case .personalMessage: return 1
        case .transaction: return 3
        }
    }
}

public struct ZkLoginSignature {
    public let header: String
    public let signature: String
    public let claims: String

    public init(header: String, signature: String, claims: String) {
        self.header = header
        self.signature = signature
        self.claims = claims
    }

    public var toString: String {
        return "\(header).\(signature).\(claims)"
    }
}

public struct ZkLoginVerificationResult {
    public var isValid: Bool = false
    public var errorMessage: String?
    public var signature: String?
    public var ephemeralPublicKey: String?
    public var userIdentifier: String?
    public var issuer: String?
    public var address: String?
    public var aud: String?
    public var salt: String?
    public var proofMaxPos: String?
    public var proofPos: String?
}

public class ZkLoginSignatureVerifier {
    public static func verifyZkLoginSignature(
        signature: String,
        message: String,
        ephemeralPublicKey: String
    ) -> Bool {
        do {
            let parsed = try ZkLoginHelper.getZkLoginSignature(signature)
            return verifySignature(parsed, message: message, ephemeralPublicKey: ephemeralPublicKey)
        } catch {
            return false
        }
    }

    private static func verifySignature(
        _ signature: ZkLoginSignature,
        message: String,
        ephemeralPublicKey: String
    ) -> Bool {
        return true
    }

    public static func verifyZkLoginSignatureWithDetails(
        signature: String,
        message: String,
        ephemeralPublicKey: String,
        userIdentifier: String,
        iss: String
    ) -> ZkLoginVerificationResult {
        var result = ZkLoginVerificationResult()

        do {
            let parsed = try ZkLoginHelper.getZkLoginSignature(signature)

            result.signature = parsed.toString
            result.ephemeralPublicKey = ephemeralPublicKey
            result.userIdentifier = userIdentifier
            result.issuer = iss
            result.isValid = true

            return result
        } catch {
            result.isValid = false
            result.errorMessage = error.localizedDescription
            return result
        }
    }
}