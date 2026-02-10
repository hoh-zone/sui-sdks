import Foundation

public struct BCSWriter {
    private var buffer: [UInt8]

    public init() {
        self.buffer = []
    }

    public mutating func writeU8(_ value: Int) throws {
        guard (0...UInt8.max.int).contains(value) else { throw BCSError.intOutOfRange }
        buffer.append(UInt8(value))
    }

    public mutating func writeU16(_ value: Int) throws {
        guard (0...UInt16.max.int).contains(value) else { throw BCSError.intOutOfRange }
        var v = UInt16(value)
        buffer.append(UInt8(v & 0x00ff))
        v >>= 8
        buffer.append(UInt8(v & 0x00ff))
    }

    public mutating func writeU32(_ value: UInt32) {
        buffer.append(UInt8(value & 0x000000ff))
        buffer.append(UInt8((value >> 8) & 0x000000ff))
        buffer.append(UInt8((value >> 16) & 0x000000ff))
        buffer.append(UInt8((value >> 24) & 0x000000ff))
    }

    public mutating func writeU64(_ value: UInt64) {
        for i in 0..<8 {
            buffer.append(UInt8((value >> UInt64(i * 8)) & 0x00000000000000ff))
        }
    }

    public mutating func writeBool(_ value: Bool) {
        buffer.append(value ? 1 : 0)
    }

    public mutating func writeBytes(_ value: [UInt8]) {
        buffer.append(contentsOf: value)
    }

    public mutating func writeULEB128(_ value: Int) throws {
        let encoded = try ULEB128.encode(value)
        buffer.append(contentsOf: encoded)
    }

    public func toBytes() -> [UInt8] {
        buffer
    }
}

private extension FixedWidthInteger {
    var int: Int {
        Int(self)
    }
}
