import Foundation

public protocol SuiPublicKeyProtocol {
    var rawBytes: [UInt8] { get }
    func verify(message: [UInt8], signature: [UInt8]) -> Bool
    func toSuiAddress() -> String
    func toSuiBytes() -> [UInt8]
    func toSuiPublicKey() -> String
}
