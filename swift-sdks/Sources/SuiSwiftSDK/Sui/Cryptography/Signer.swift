import Foundation

public protocol SuiSigner {
    func signTransaction(bytes: [UInt8]) throws -> SignatureWithBytes
    func toSuiAddress() -> String
}
