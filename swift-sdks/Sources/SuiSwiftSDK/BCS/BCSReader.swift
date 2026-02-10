import Foundation

public enum BCSError: Error, Equatable {
    case outOfRange
    case invalidBool(UInt8)
    case intOutOfRange
}

public struct BCSReader {
    private let data: [UInt8]
    private(set) var position: Int

    public init(data: [UInt8], position: Int = 0) {
        self.data = data
        self.position = position
    }

    public var remaining: Int {
        data.count - position
    }

    public mutating func readU8() throws -> UInt8 {
        guard remaining >= 1 else { throw BCSError.outOfRange }
        let value = data[position]
        position += 1
        return value
    }

    public mutating func readU16() throws -> UInt16 {
        let bytes = try readBytes(2)
        return UInt16(bytes[0]) | (UInt16(bytes[1]) << 8)
    }

    public mutating func readU32() throws -> UInt32 {
        let bytes = try readBytes(4)
        return UInt32(bytes[0])
            | (UInt32(bytes[1]) << 8)
            | (UInt32(bytes[2]) << 16)
            | (UInt32(bytes[3]) << 24)
    }

    public mutating func readU64() throws -> UInt64 {
        let bytes = try readBytes(8)
        var value: UInt64 = 0
        for i in 0..<8 {
            value |= UInt64(bytes[i]) << UInt64(i * 8)
        }
        return value
    }

    public mutating func readBytes(_ count: Int) throws -> [UInt8] {
        guard count >= 0, remaining >= count else { throw BCSError.outOfRange }
        let out = Array(data[position..<(position + count)])
        position += count
        return out
    }

    public mutating func readBool() throws -> Bool {
        let value = try readU8()
        switch value {
        case 0: return false
        case 1: return true
        default: throw BCSError.invalidBool(value)
        }
    }

    public mutating func readULEB128() throws -> UInt64 {
        let decoded = try ULEB128.decode(data, from: position)
        position += decoded.consumed
        return decoded.value
    }
}
