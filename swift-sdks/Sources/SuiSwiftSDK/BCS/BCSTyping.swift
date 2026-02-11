import Foundation

public enum BCSValue {
    case u8(UInt8)
    case u16(UInt16)
    case u32(UInt32)
    case u64(UInt64)
    case u128(UInt, UInt)
    case string(String)
    case bytes([UInt8])
    case bool(Bool)
    case option(BCSValue?)
    case array([BCSValue])
    case vector([BCSValue])
    case object([String: BCSValue])
    case address(String)

    public var isNumeric: Bool {
        switch self {
        case .u8, .u16, .u32, .u64, .u128:
            return true
        default:
            return false
        }
    }

    public func encodeULEB128() -> [UInt8] {
        switch self {
        case .u8(let value):
            return ULEB128.encode(value: UInt64(value))
        case .u16(let value):
            return ULEB128.encode(value: UInt64(value))
        case .u32(let value):
            return ULEB128.encode(value: UInt64(value))
        case .u64(let value):
            return ULEB128.encode(value: value)
        case .u128(let high, let low):
            // Encode as bytes for u128
            var result = [UInt8]()
            let highBytes = withUnsafeBytes(of: high.littleEndian) { Array($0) }
            let lowBytes = withUnsafeBytes(of: low.littleEndian) { Array($0) }
            result.append(contentsOf: ULEB128.encode(value: 16))
            result.append(contentsOf: lowBytes)
            result.append(contentsOf: highBytes)
            return result
        default:
            return []
        }
    }

    public func toSuiBCS() -> [UInt8] {
        switch self {
        case .u8(let value):
            return [value]
        case .u16(let value):
            return withUnsafeBytes(of: value.littleEndian) { Array($0) }
        case .u32(let value):
            return withUnsafeBytes(of: value.littleEndian) { Array($0) }
        case .u64(let value):
            return withUnsafeBytes(of: value.littleEndian) { Array($0) }
        case .string(let value):
            let bytes = value.data(using: .utf8) ?? Data()
            var result = ULEB128.encode(value: UInt64(bytes.count))
            result.append(contentsOf: bytes)
            return result
        case .bytes(let value):
            var result = ULEB128.encode(value: UInt64(value.count))
            result.append(contentsOf: value)
            return result
        case .bool(let value):
            return [value ? 1 : 0]
        case .option(let optional):
            guard let value = optional else {
                return [0]
            }
            return [1] + value.toSuiBCS()
        case .array(let elements):
            var result = ULEB128.encode(value: UInt64(elements.count))
            for element in elements {
                result.append(contentsOf: element.toSuiBCS())
            }
            return result
        case .vector(let elements):
            var result = ULEB128.encode(value: UInt64(elements.count))
            for element in elements {
                result.append(contentsOf: element.toSuiBCS())
            }
            return result
        case .object(let fields):
            var result = ULEB128.encode(value: UInt64(fields.count))
            for (key, value) in fields.sorted(by: { $0.key < $1.key }) {
                let keyBytes = key.data(using: .utf8) ?? Data()
                result.append(contentsOf: ULEB128.encode(value: UInt64(keyBytes.count)))
                result.append(contentsOf: keyBytes)
                result.append(contentsOf: value.toSuiBCS())
            }
            return result
        case .address(let value):
            guard value.hasPrefix("0x") else {
                return []
            }
            let hex = String(value.dropFirst(2))
            let bytes = hexToBytes(hex)
            return bytes.suffix(20)
        case .u128:
            return encodeULEB128()
        }
    }

    private func hexToBytes(_ hex: String) -> [UInt8] {
        var result = [UInt8]()
        var index = hex.startIndex

        while index < hex.endIndex {
            let char1 = hex[index]
            index = hex.index(after: index)

            let char2 = index < hex.endIndex ? hex[index] : "0"
            index = hex.index(after: index)

            if let value1 = hexDigitToValue(char1),
               let value2 = hexDigitToValue(char2) {
                result.append((value1 << 4) + value2)
            }
        }

        return result
    }

    private func hexDigitToValue(_ char: Character) -> UInt8? {
        switch char {
        case "0"..."9": return UInt8(char.asciiValue! - Character("0").asciiValue!)
        case "a"..."f": return UInt8(char.asciiValue! - Character("a").asciiValue! + 10)
        case "A"..."F": return UInt8(char.asciiValue! - Character("A").asciiValue! + 10)
        default: return nil
        }
    }
}

public class BCSTyping {
    public static func encode<T>(_ value: T) -> [UInt8] {
        switch value {
        case let v as UInt8:
            return [v]
        case let v as UInt16:
            return withUnsafeBytes(of: v.littleEndian) { Array($0) }
        case let v as UInt32:
            return withUnsafeBytes(of: v.littleEndian) { Array($0) }
        case let v as UInt64:
            return withUnsafeBytes(of: v.littleEndian) { Array($0) }
        case let v as String:
            return BCSValue.string(v).toSuiBCS()
        case let v as [UInt8]:
            return BCSValue.bytes(v).toSuiBCS()
        case let v as Bool:
            return [v ? 1 : 0]
        case let v as [UInt8]:
            return BCSValue.bytes(v).toSuiBCS()
        case let v as [Any]:
            var result = ULEB128.encode(value: UInt64(v.count))
            for item in v {
                result.append(contentsOf: encode(item))
            }
            return result
        case let v as [String: Any]:
            var result = ULEB128.encode(value: UInt64(v.count))
            for (key, value) in v.sorted(by: { $0.key < $1.key }) {
                let keyBytes = key.data(using: .utf8) ?? Data()
                result.append(contentsOf: ULEB128.encode(value: UInt64(keyBytes.count)))
                result.append(contentsOf: keyBytes)
                result.append(contentsOf: encode(value))
            }
            return result
        default:
            return []
        }
    }

    public static func decode<T>(_ type: T.Type, from bytes: [UInt8]) throws -> Any? {
        guard let first = bytes.first else {
            return nil
        }

        if type == UInt8.self {
            return first
        } else if type == UInt16.self {
            return bytes.withUnsafeBytes { $0.load(as: UInt16.self).littleEndian }
        } else if type == UInt32.self {
            return bytes.withUnsafeBytes { $0.load(as: UInt32.self).littleEndian }
        } else if type == UInt64.self {
            return bytes.withUnsafeBytes { $0.load(as: UInt64.self).littleEndian }
        } else if type == String.self {
            return String(bytes: Data(bytes))
        } else if type == Bool.self {
            return first != 0
        }

        return nil
    }
}

public struct BCSObject {
    public let bytes: [UInt8]

    public init(bytes: [UInt8]) {
        self.bytes = bytes
    }

    public init<T>(_ value: T) {
        switch value {
        case let v as BCSValue:
            self.bytes = v.toSuiBCS()
        case let v as String:
            self.bytes = BCSValue.string(v).toSuiBCS()
        case let v as [UInt8]:
            self.bytes = v
        case is UInt8:
            self.bytes = [v as! UInt8]
        case is UInt16:
            self.bytes = withUnsafeBytes(of: v as! UInt16) { Array($0) }
        case is UInt32:
            self.bytes = withUnsafeBytes(of: v as! UInt32) { Array($0) }
        case is UInt64:
            self.bytes = withUnsafeBytes(of: v as! UInt64) { Array($0) }
        case is Bool:
            self.bytes = [(v as! Bool) ? 1 : 0]
        default:
            self.bytes = []
        }
    }

    public func toHex() -> String {
        return "0x" + bytes.map { String(format: "%02x", $0) }.joined()
    }

    public func toBase64() -> String {
        return Data(bytes).base64EncodedString()
    }
}