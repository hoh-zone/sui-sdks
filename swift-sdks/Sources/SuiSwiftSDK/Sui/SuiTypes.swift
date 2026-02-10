import Foundation

public enum SuiValidationError: Error, Equatable {
    case invalidSuiAddress
    case invalidSuiObjectID
    case invalidTransactionDigest
    case duplicateObjectIDs
    case duplicateTransactionDigests
}

public enum SuiTypes {
    public static let suiAddressLength = 32

    public static func isValidTransactionDigest(_ value: String) -> Bool {
        guard let bytes = Base58.decode(value) else {
            return false
        }
        return bytes.count == 32
    }

    public static func isValidSuiAddress(_ value: String) -> Bool {
        isHex(value) && getHexByteLength(value) == suiAddressLength
    }

    public static func isValidSuiObjectID(_ value: String) -> Bool {
        isValidSuiAddress(value)
    }

    public static func normalizeSuiAddress(_ value: String, forceAdd0x: Bool = false) -> String {
        var address = value.lowercased()
        if !forceAdd0x, address.hasPrefix("0x") {
            address = String(address.dropFirst(2))
        }
        return "0x" + address.leftPadding(toLength: suiAddressLength * 2, withPad: "0")
    }

    public static func normalizeSuiObjectID(_ value: String, forceAdd0x: Bool = false) -> String {
        normalizeSuiAddress(value, forceAdd0x: forceAdd0x)
    }

    private static func isHex(_ value: String) -> Bool {
        if value.isEmpty || value.count % 2 != 0 {
            return false
        }

        let normalized: Substring
        if value.hasPrefix("0x") || value.hasPrefix("0X") {
            normalized = value.dropFirst(2)
        } else {
            normalized = Substring(value)
        }

        if normalized.isEmpty {
            return false
        }

        for char in normalized {
            guard char.isHexDigit else {
                return false
            }
        }
        return true
    }

    private static func getHexByteLength(_ value: String) -> Int {
        if value.hasPrefix("0x") || value.hasPrefix("0X") {
            return (value.count - 2) / 2
        }
        return value.count / 2
    }
}

private extension String {
    func leftPadding(toLength: Int, withPad pad: Character) -> String {
        if count >= toLength {
            return self
        }
        return String(repeating: String(pad), count: toLength - count) + self
    }
}
