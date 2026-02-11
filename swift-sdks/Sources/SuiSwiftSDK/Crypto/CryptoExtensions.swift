import Foundation

public struct Secp256k1ExtendedKeypair {
    private let keypair: Secp256k1Keypair
    private let chainCode: [UInt8]

    public init(keypair: Secp256k1Keypair, chainCode: [UInt8] = [UInt8](repeating: 0, count: 32)) {
        self.keypair = keypair
        self.chainCode = chainCode
    }

    public var publicKey: Secp256k1PublicKey {
        keypair.publicKey
    }

    public convenience init() throws {
        let keypair = try Secp256k1Keypair()
        try self.init(keypair: keypair)
    }

    public func deriveChild(index: UInt32) throws -> Secp256k1ExtendedKeypair {
        let indexBytes = withUnsafeBytes(of: index.littleEndian) { Array($0) }
        let message = keypair.privateKeyBytes + indexBytes

        var hmac = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        CCHmac(
            CCHmacAlgorithm(kCCHmacAlgSHA256),
            chainCode,
            chainCode.count,
            message,
            message.count,
            &hmac
        )

        let newChainCode = Array(hmac.suffix(32))
        let newPrivateKeyBytes = Array(hmac.prefix(32))

        let newKeypair = try Secp256k1Keypair(privateKeyBytes: newPrivateKeyBytes)

        return Secp256k1ExtendedKeypair(keypair: newKeypair, chainCode: newChainCode)
    }
}

public struct Ed25519ExtendedKeypair {
    private let keypair: Ed25519Keypair
    private let chainCode: [UInt8]

    public init(keypair: Ed25519Keypair, chainCode: [UInt8] = [UInt8](repeating: 0, count: 32)) {
        self.keypair = keypair
        self.chainCode = chainCode
    }

    public var publicKey: Ed25519PublicKey {
        keypair.publicKey
    }

    public convenience init() throws {
        let keypair = try Ed25519Keypair()
        try self.init(keypair: keypair)
    }

    public func deriveChild(index: UInt32) throws -> Ed25519ExtendedKeypair {
        let indexBytes = withUnsafeBytes(of: index.littleEndian) { Array($0) }
        let message = keypair.privateKeyBytes + indexBytes

        var hmac = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        CCHmac(
            CCHmacAlgorithm(kCCHmacAlgSHA256),
            chainCode,
            chainCode.count,
            message,
            message.count,
            &hmac
        )

        let newChainCode = Array(hmac.suffix(32))
        let newPrivateKeyBytes = Array(hmac.prefix(32))

        let newKeypair = try Ed25519Keypair(privateKeyBytes: newPrivateKeyBytes)

        return Ed25519ExtendedKeypair(keypair: newKeypair, chainCode: newChainCode)
    }
}

public struct SuiPrivateKeyWrapper {
    public let privateKeyBytes: [UInt8]
    public let scheme: SignatureScheme

    public init(privateKeyBytes: [UInt8], scheme: SignatureScheme) {
        self.privateKeyBytes = privateKeyBytes
        self.scheme = scheme
    }

    public func toBase64() -> String {
        let bytes = [UInt8(scheme.rawValue)] + privateKeyBytes.map { UInt8($0) }
        return Data(bytes).base64EncodedString()
    }

    public func encode() -> String {
        try! SuiPrivateKey.encode(secretKey: privateKeyBytes, scheme: scheme)
    }

    public static func fromBase64(_ encoded: String) throws -> SuiPrivateKeyWrapper {
        guard let data = Data(base64Encoded: encoded), data.count == 33 else {
            throw CryptoError.invalidKeyLength
        }

        let schemeValue = data[0]
        guard let scheme = SignatureScheme(rawValue: schemeValue) else {
            throw CryptoError.invalidScheme
        }

        let privateKeyBytes = Array(data.dropFirst())
        return SuiPrivateKeyWrapper(
            privateKeyBytes: privateKeyBytes,
            scheme: scheme
        )
    }
}

public struct SignatureWithBytes {
    public let bytes: String
    public let signature: String

    public init(bytes: String, signature: String) {
        self.bytes = bytes
        self.signature = signature
    }
}

public struct IntentMessage {
    public let intent: IntentScope
    public let data: [UInt8]

    public init(intent: IntentScope, data: [UInt8]) {
        self.intent = intent
        self.data = data
    }

    public func serialize() -> [UInt8] {
        var result = [UInt8]()
        result.append(UInt8(intent.rawValue))
        result.append(0)
        result.append(contentsOf: data)
        return result
    }

    public func hash() -> [UInt8] {
        let serialized = serialize()
        var digest = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        serialized.withUnsafeBytes {
            _ = CC_SHA256($0.baseAddress, CC_LONG(serialized.count), &digest)
        }
        return digest
    }
}

public enum CryptoError: Error, LocalizedError {
    case invalidKeyLength
    case invalidScheme
    case invalidSignature
    case invalidPublicKey
    case invalidPrivateKey
    case signingFailed

    public var errorDescription: String? {
        switch self {
        case .invalidKeyLength: return "Invalid key length"
        case .invalidScheme: return "Invalid signature scheme"
        case .invalidSignature: return "Invalid signature"
        case .invalidPublicKey: return "Invalid public key"
        case .invalidPrivateKey: return "Invalid private key"
        case .signingFailed: return "Signing failed"
        }
    }
}

public struct IntentScope {
    public let rawValue: Int

    public static let transactionData = IntentScope(rawValue: 0)
    public static let personalMessage = IntentScope(rawValue: 1)
    public static let transaction = IntentScope(rawValue: 3)
}

public class SignedTransaction {
    public let transaction: [UInt8]
    public let signature: String

    public init(transaction: [UInt8], signature: String) {
        self.transaction = transaction
        self.signature = signature
    }

    public func toBytes() -> [UInt8] {
        var result = [UInt8]()
        result.append(contentsOf: transaction)

        if let sigData = Data(base64Encoded: signature) {
            result.append(contentsOf: sigData)
        }

        return result
    }
}

public enum WalletScheme {
    case ed25519
    case secp256k1
    case secp256r1
    case multisig
    case zklogin

    public var identifier: String {
        switch self {
        case .ed25519: return "ED25519"
        case .secp256k1: return "SECP256K1"
        case .secp256r1: return "SECP256R1"
        case .multisig: return "MULTISIG"
        case .zklogin: return "ZKLOGIN"
        }
    }

    public static func fromIdentifier(_ identifier: String) -> WalletScheme? {
        switch identifier {
        case "ED25519": return .ed25519
        case "SECP256K1": return .secp256k1
        case "SECP256R1": return .secp256r1
        case "MULTISIG": return .multisig
        case "ZKLOGIN": return .zklogin
        default: return nil
        }
    }
}