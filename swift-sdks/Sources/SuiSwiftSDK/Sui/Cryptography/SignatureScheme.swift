import Foundation

public enum SignatureScheme: UInt8, CaseIterable {
    case ed25519 = 0x00
    case secp256k1 = 0x01
    case secp256r1 = 0x02
    case multiSig = 0x03
    case zkLogin = 0x05
    case passkey = 0x06

    public var publicKeySize: Int? {
        switch self {
        case .ed25519:
            return 32
        case .secp256k1, .secp256r1, .passkey:
            return 33
        case .multiSig, .zkLogin:
            return nil
        }
    }

    public var supportsSuiPrivateKey: Bool {
        switch self {
        case .ed25519, .secp256k1, .secp256r1:
            return true
        case .multiSig, .zkLogin, .passkey:
            return false
        }
    }
}
