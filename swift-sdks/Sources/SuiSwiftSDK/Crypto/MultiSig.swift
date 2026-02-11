import Foundation

public struct MultiSigPublicKey {
    public let scheme: SignatureScheme
    public let threshold: UInt8
    public let publicKeys: [SuiPublicKeyBytes]

    public init(scheme: SignatureScheme, threshold: UInt8, publicKeys: [SuiPublicKeyBytes]) {
        self.scheme = scheme
        self.threshold = threshold
        self.publicKeys = publicKeys
    }

    public func toBytes() -> [UInt8] {
        var result = [UInt]()
        result.append(UInt(scheme.rawValue))
        result.append(UInt(publicKeys.count))
        result.append(UInt(threshold))

        for publicKey in publicKeys {
            result.append(UInt(publicKey.bytes.count))
            result.append(contentsOf: publicKey.bytes.map { UInt($0) })
        }

        return result.map { UInt8($0) }
    }

    public static func fromBytes(_ bytes: [UInt8]) throws -> MultiSigPublicKey {
        guard bytes.count >= 3 else {
            throw MultiSigError.invalidLength
        }

        let schemeRaw = bytes[0]
        guard let scheme = SignatureScheme(rawValue: schemeRaw) else {
            throw MultiSigError.invalidScheme
        }

        let numKeys = Int(bytes[1])
        let threshold = bytes[2]

        guard numKeys > 0 else {
            throw MultiSigError.invalidThreshold
        }

        guard threshold > 0 && threshold <= numKeys else {
            throw MultiSigError.invalidThreshold
        }

        var publicKeys: [SuiPublicKeyBytes] = []
        var offset = 3

        for _ in 0..<numKeys {
            guard offset + 1 <= bytes.count else {
                throw MultiSigError.invalidLength
            }

            let keyLength = Int(bytes[offset])
            offset += 1

            guard offset + keyLength <= bytes.count else {
                throw MultiSigError.invalidLength
            }

            let keyBytes = Array(bytes[offset..<(offset + keyLength)])
            publicKeys.append(SuiPublicKeyBytes(bytes: keyBytes))
            offset += keyLength
        }

        return MultiSigPublicKey(scheme: scheme, threshold: threshold, publicKeys: publicKeys)
    }

    public func toSuiAddress() throws -> String {
        let bytes = toBytes()
        return SuiPublicKeyEncoding.toSuiAddress(rawPublicKey: bytes, scheme: .multisig)
    }
}

public struct MultiSigSignature {
    public let scheme: SignatureScheme
    public let signatures: [[UInt8]]
    public let bitmap: ByteArray

    public init(scheme: SignatureScheme, signatures: [[UInt8]], bitmap: ByteArray) {
        self.scheme = scheme
        self.signatures = signatures
        self.bitmap = bitmap
    }

    public func toBytes() -> [UInt8] {
        var result = [UInt]()
        result.append(UInt(scheme.rawValue))
        result.append(contentsOf: bitmap.data.map { UInt($0) })

        for signature in signatures {
            result.append(UInt(signature.count))
            result.append(contentsOf: signature.map { UInt($0) })
        }

        return result.map { UInt8($0) }
    }

    public static func fromBytes(_ bytes: [UInt8]) throws -> MultiSigSignature {
        guard bytes.count >= 2 else {
            throw MultiSigError.invalidLength
        }

        let schemeRaw = bytes[0]
        guard let scheme = SignatureScheme(rawValue: schemeRaw) else {
            throw MultiSigError.invalidScheme
        }

        let bitmapLength = Int(bytes[1])
        guard bitmapLength <= bytes.count - 2 else {
            throw MultiSigError.invalidLength
        }

        let bitmapData = Array(bytes[2..<(2 + bitmapLength)])
        let bitmap = ByteArray(data: bitmapData)

        var offset = 2 + bitmapLength
        var signatures: [[UInt8]] = []

        while offset < bytes.count {
            guard offset + 1 <= bytes.count else {
                throw MultiSigError.invalidLength
            }

            let sigLength = Int(bytes[offset])
            offset += 1

            guard offset + sigLength <= bytes.count else {
                throw MultiSigError.invalidLength
            }

            let signatureData = Array(bytes[offset..<(offset + sigLength)])
            signatures.append(signatureData)
            offset += sigLength
        }

        return MultiSigSignature(scheme: scheme, signatures: signatures, bitmap: bitmap)
    }
}

public enum MultiSigError: Error, LocalizedError {
    case invalidLength
    case invalidScheme
    case invalidThreshold
    case invalidSignature
    case invalidBitmap
    case verificationFailed

    public var errorDescription: String? {
        switch self {
        case .invalidLength: return "Invalid length"
        case .invalidScheme: return "Invalid signature scheme"
        case .invalidThreshold: return "Invalid threshold"
        case .invalidSignature: return "Invalid signature"
        case .invalidBitmap: return "Invalid bitmap"
        case .verificationFailed: return "Verification failed"
        }
    }
}

public class MultiSigVerifier {
    public static func verifyMultiSig(
        message: Data,
        publicKey: MultiSigPublicKey,
        signature: MultiSigSignature
    ) -> Bool {
        guard publicKey.toBytes().count > 0 else {
            return false
        }

        let bitmapBits = bitmapToBits(signature.bitmap.data)
        var validSignatures = 0
        var sigIndex = 0

        for (i, bit) in bitmapBits.enumerated() {
            guard i < publicKey.publicKeys.count else { break }

            if bit {
                guard sigIndex < signature.signatures.count else { return false }
                let sig = signature.signatures[sigIndex]
                let pubKey = publicKey.publicKeys[i]

                if verifySingleSignature(message: message, publicKey: pubKey, signature: sig) {
                    validSignatures += 1
                }

                sigIndex += 1
            }
        }

        return validSignatures >= Int(publicKey.threshold)
    }

    private static func verifySingleSignature(
        message: Data,
        publicKey: SuiPublicKeyBytes,
        signature: [UInt8]
    ) -> Bool {
        switch publicKey.scheme {
        case .ed25519:
            return try? Ed25519Keypair(publicKey: publicKey).verify(message: Array(message), signature: signature) ?? false
        case .secp256k1:
            return try? Secp256k1Keypair(publicKey: publicKey).verify(message: Array(message), signature: signature) ?? false
        case .secp256r1:
            return try? Secp256r1Keypair(publicKey: publicKey).verify(message: Array(message), signature: signature) ?? false
        case .multisig:
            return false
        default:
            return false
        }
    }

    private static func bitmapToBits(_ bitmap: [UInt8]) -> [Bool] {
        var bits: [Bool] = []
        for byte in bitmap {
            for i in 0..<8 {
                bits.append((byte & (1 << i)) != 0)
            }
        }
        return bits
    }
}

public struct Bitmap {
    public static func encode(_ bits: [Bool]) -> [UInt8] {
        var bitmap = [UInt8]()
        var currentByte: UInt8 = 0
        var bitIndex = 0

        for bit in bits {
            if bit {
                currentByte |= (1 << bitIndex)
            }
            bitIndex += 1

            if bitIndex == 8 {
                bitmap.append(currentByte)
                currentByte = 0
                bitIndex = 0
            }
        }

        if bitIndex > 0 {
            bitmap.append(currentByte)
        }

        return bitmap
    }

    public static func decode(_ bitmap: [UInt8], count: Int) -> [Bool] {
        var bits: [Bool] = []
        for byte in bitmap.prefix((count + 7) / 8) {
            for i in 0..<8 {
                bits.append((byte & (1 << i)) != 0)
            }
        }
        return Array(bits.prefix(count))
    }
}

public class MultiSigBuilder {
    private var signatures: [[UInt8]] = []
    private var bits: [Bool] = []

    public init() {}

    public func addSignature(_ index: Int, signature: [UInt8]) {
        while bits.count <= index {
            bits.append(false)
        }
        bits[index] = true
        signatures.append(signature)
    }

    public func build() throws -> MultiSigSignature {
        let bitmap = Bitmap.encode(bits)
        return MultiSigSignature(
            scheme: .multisig,
            signatures: signatures,
            bitmap: ByteArray(data: bitmap)
        )
    }
}