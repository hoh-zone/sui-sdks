import Foundation

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

public struct ZkLoginIntentMessage {
    public let scheme: String
    public let message: Data
    public let intent: IntentScope

    public init(scheme: String, message: Data, intent: IntentScope) {
        self.scheme = scheme
        self.message = message
        self.intent = intent
    }

    public func serialize() -> Data {
        var result = message
        var intentData = Data([UInt8(intent.rawValue)])
        intentData.append(0)
        intentData.append(contentsOf: result)
        return intentData
    }
}

public struct ZkLoginAccount {
    public let address: String
    public let ephemeralPublicKey: String
    public let userIdentifier: String
    public let issuer: String

    public init(address: String, ephemeralPublicKey: String, userIdentifier: String, issuer: String) {
        self.address = address
        self.ephemeralPublicKey = ephemeralPublicKey
        self.userIdentifier = userIdentifier
        self.issuer = issuer
    }
}

public enum ZkLoginProvider {
    case google
    case facebook
    case apple
    case kakao
    case twitch

    public var issuer: String {
        switch self {
        case .google: return "https://accounts.google.com"
        case .facebook: return "https://www.facebook.com"
        case .apple: return "https://appleid.apple.com"
        case .kakao: return "https://kauth.kakao.com"
        case .twitch: return "https://id.twitch.tv"
        }
    }
}

public class ZkLoginJWTParser {
    public static func parseJWT(_ jwt: String) throws -> (header: [String: Any], payload: [String: Any]) {
        let parts = jwt.split(separator: ".", omittingEmptySubsequences: false)
        guard parts.count == 3 else {
            throw ZkLoginError.invalidFormat
        }

        let headerData = try base64UrlDecode(String(parts[0]))
        let payloadData = try base64UrlDecode(String(parts[1]))

        guard let header = try JSONSerialization.jsonObject(with: Data(headerData)) as? [String: Any],
              let payload = try JSONSerialization.jsonObject(with: Data(payloadData)) as? [String: Any] else {
            throw ZkLoginError.invalidFormat
        }

        return (header, payload)
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
}

public struct ZkLoginSalt {
    public let value: String

    public init(value: String) {
        self.value = value
    }

    public static func generate() -> ZkLoginSalt {
        var salt = [UInt8](repeating: 0, count: 32)
        let _ = SecRandomCopyBytes(kSecRandomDefault, 32, &salt)
        return ZkLoginSalt(value: salt.map { String(format: "%02x", $0) }.joined())
    }
}