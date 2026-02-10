import Foundation
import Crypto

public enum IntentScope: UInt8 {
    case transactionData = 0
    case personalMessage = 3
}

public struct SignatureWithBytes {
    public let bytes: String
    public let signature: String

    public init(bytes: String, signature: String) {
        self.bytes = bytes
        self.signature = signature
    }
}

public enum SuiIntent {
    public static let intentVersionV0: UInt8 = 0
    public static let intentAppSui: UInt8 = 0

    public static func messageWithIntent(scope: IntentScope, message: [UInt8]) -> [UInt8] {
        [scope.rawValue, intentVersionV0, intentAppSui] + message
    }

    public static func hashIntentMessage(scope: IntentScope, message: [UInt8]) -> [UInt8] {
        let intentMessage = messageWithIntent(scope: scope, message: message)
        let digest = SHA256.hash(data: Data(intentMessage))
        return [UInt8](digest)
    }

    public static func serializePersonalMessage(_ message: [UInt8]) throws -> [UInt8] {
        var writer = BCSWriter()
        try writer.writeULEB128(message.count)
        writer.writeBytes(message)
        return writer.toBytes()
    }
}
