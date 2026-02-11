import Foundation
import P256K

public struct Secp256k1SignatureChecker {
    public static func verifyPublicKey(
        publicKeyBytes: [UInt8],
        message: Data,
        signature: [UInt8],
        compressed: Bool = true
    ) -> Bool {
        guard let publicKey = try? P256K.Signing.PublicKey(
            dataRepresentation: Data(publicKeyBytes),
            format: compressed ? .compressed : .raw
        ) else {
            return false
        }

        guard signature.count == 64 else {
            return false
        }

        guard let signature = try? P256K.Signing.ECDSASignature(
            dataRepresentation: Data(signature)
        ) else {
            return false
        }

        return publicKey.isValidSignature(signature, for: message)
    }

    public static func verifyRecoverableSignature(
        publicKeyBytes: [UInt8],
        message: Data,
        signatureBytes: [UInt8],
        recoveryId: UInt8
    ) -> Bool {
        guard publicKeyBytes.count == 33 || publicKeyBytes.count == 65 else {
            return false
        }

        return verifyPublicKey(
            publicKeyBytes: publicKeyBytes,
            message: message,
            signature: signatureBytes,
            compressed: publicKeyBytes.count == 33
        )
    }

    public static func recoverPublicKey(
        message: Data,
        signatureBytes: [UInt8],
        recoveryId: UInt8
    ) throws -> P256K.Signing.PublicKey {
        guard signatureBytes.count == 64 else {
            throw Secp256k1Error.invalidSignatureLength
        }

        let r = Array(signatureBytes[0..<32])
        let v = Array(signatureBytes[32..<64])

        let signature = try P256K.Signing.ECDSASignature(
            rawRepresentation: Data(r + v)
        )

        return try P256K.Signing.PublicKey(
            rawRepresentation: signature.rawRepresentation
        )
    }

    public static func compressPublicKey(_ publicKey: P256K.Signing.PublicKey) -> Data {
        publicKey.dataRepresentation
    }

    public static func decompressPublicKey(_ compressed: Data) throws -> P256K.Signing.PublicKey {
        return try P256K.Signing.PublicKey(
            dataRepresentation: compressed,
            format: .compressed
        )
    }

    public static func tweakPublicKey(
        _ publicKey: P256K.Signing.PublicKey,
        tweak: Data
    ) throws -> Data {
        guard tweak.count == 32 else {
            throw Secp256k1Error.invalidTweak
        }

        let bytes = publicKey.dataRepresentation

        var result = Data(bytes)

        for i in 0..<tweak.count {
            let index = i % result.count
            result[index] = result[index] ^ tweak[i]
        }

        return result
    }

    public static func aggregatePublicKeys(_ publicKeys: [P256K.Signing.PublicKey]) throws -> Data {
        guard !publicKeys.isEmpty else {
            throw Secp256k1Error.invalidPublicKeyEncoding
        }

        var result = Data()

        for pubKey in publicKeys {
            result.append(pubKey.dataRepresentation)
        }

        return result
    }

    public static func schnorrSign(
        privateKey: P256K.Signing.PrivateKey,
        message: Data
    ) throws -> [UInt8] {
        let signature = try privateKey.signature(for: message)
        return [UInt8](signature.dataRepresentation)
    }

    public static func schnorrVerify(
        publicKey: P256K.Signing.PublicKey,
        message: Data,
        signature: [UInt8]
    ) -> Bool {
        guard let signature = try? P256K.Signing.ECDSASignature(
            dataRepresentation: Data(signature)
        ) else {
            return false
        }

        return publicKey.isValidSignature(signature, for: message)
    }
}

public struct Secp256k1KeyAgreement {
    public static func ecdh(
        privateKey: P256K.Signing.PrivateKey,
        publicKey: P256K.Signing.PublicKey
    ) throws -> Data {
        let sharedSecret = privateKey.sharedSecretFromKeyAgreement(with: publicKey)
        return sharedSecret.withUnsafeBytes { Data($0) }
    }

    public static func deriveKey(
        baseKey: P256K.Signing.PrivateKey,
        info: Data,
        salt: Data
    ) throws -> Data {
        let sharedSecret = try ecdh(
            privateKey: baseKey,
            publicKey: baseKey.publicKey
        )

        var result = sharedSecret
        result.append(info)
        result.append(salt)

        return Data(sha256(result))
    }

    private static func sha256(_ data: Data) -> [UInt8] {
        var digest = [UInt8](repeating: 0, count: Int(CC_SHA256_DIGEST_LENGTH))
        data.withUnsafeBytes {
            _ = CC_SHA256($0.baseAddress, CC_LONG(data.count), &digest)
        }
        return digest
    }
}