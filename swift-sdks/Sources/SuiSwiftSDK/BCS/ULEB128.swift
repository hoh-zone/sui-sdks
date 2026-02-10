import Foundation

public enum ULEB128Error: Error, Equatable {
    case negativeValue
    case overflow
    case exceedsU32
    case nonCanonical
    case bufferOverflow
}

public enum ULEB128 {
    public static let maxValue: UInt64 = 0xFFFF_FFFF

    public static func encode(_ value: Int) throws -> [UInt8] {
        if value < 0 {
            throw ULEB128Error.negativeValue
        }
        return encode(UInt64(value))
    }

    public static func encode(_ value: UInt64) -> [UInt8] {
        if value == 0 {
            return [0]
        }

        var out: [UInt8] = []
        var v = value
        while v > 0 {
            var byte = UInt8(v & 0x7f)
            v >>= 7
            if v > 0 {
                byte |= 0x80
            }
            out.append(byte)
        }
        return out
    }

    public static func decode(_ bytes: [UInt8], from start: Int = 0) throws -> (value: UInt64, consumed: Int) {
        var total: UInt64 = 0
        var shift: UInt64 = 0

        for index in start..<bytes.count {
            let byte = bytes[index]
            total |= UInt64(byte & 0x7f) << shift

            if (byte & 0x80) == 0 {
                let consumed = index - start + 1
                if total > maxValue {
                    throw ULEB128Error.exceedsU32
                }
                if encode(total).count != consumed {
                    throw ULEB128Error.nonCanonical
                }
                return (total, consumed)
            }

            shift += 7
            if shift >= 64 {
                throw ULEB128Error.overflow
            }
        }

        throw ULEB128Error.bufferOverflow
    }
}
