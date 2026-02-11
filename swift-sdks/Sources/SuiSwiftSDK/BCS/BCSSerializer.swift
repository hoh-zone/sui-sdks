import Foundation

// ULEB128 encoding/decoding for Swift SDK
public class ULEB128 {
    
    public static func encode(value: UInt64) -> [UInt8] {
        var output = [UInt8]()
        var v = value
        
        repeat {
            var byte: UInt8 = UInt8(v & 0x7f)
            v >>= 7
            
            if v != 0 {
                byte |= 0x80
            }
            
            output.append(byte)
        } while v != 0
        
        return output
    }
    
    public static func decode(data: [UInt8], offset: Int = 0) -> (value: UInt64, length: Int) {
        var result: UInt64 = 0
        var shift = 0
        var i = offset
        
        while true {
            if i >= data.count {
                fatalError("ULEB128 decode error: unexpected end of data")
            }
            
            let byte = data[i]
            i += 1
            
            result |= (UInt64(byte & 0x7f) << shift)
            shift += 7
            
            if (byte & 0x80) == 0 {
                break
            }
        }
        
        return (result, i - offset)
    }
}

// BCS Writer for Swift SDK
public class BCSWriter {
    private var output: [UInt8] = []
    
    public init() {}
    
    public func toByteArray() -> [UInt8] {
        output
    }
    
    public func writeUInt8(_ value: UInt8) {
        output.append(value)
    }
    
    public func writeUInt16(_ value: UInt16) {
        writeBytes(withUnsafeBytes(of: value.littleEndian) { Array($0) })
    }
    
    public func writeUInt32(_ value: UInt32) {
        writeBytes(withUnsafeBytes(of: value.littleEndian) { Array($0) })
    }
    
    public func writeUInt64(_ value: UInt64) {
        writeBytes(withUnsafeBytes(of: value.littleEndian) { Array($0) })
    }
    
    public func writeInt8(_ value: Int8) {
        output.append(UInt8(bitPattern: value))
    }
    
    public func writeInt16(_ value: Int16) {
        writeUInt16(UInt16(bitPattern: value))
    }
    
    public func writeInt32(_ value: Int32) {
        writeUInt32(UInt32(bitPattern: value))
    }
    
    public func writeInt64(_ value: Int64) {
        writeUInt64(UInt64(bitPattern: value))
    }
    
    public func writeBool(_ value: Bool) {
        writeUInt8(value ? 1 : 0)
    }
    
    public func writeString(_ value: String) {
        let bytes = value.data(using: .utf8) ?? Data()
        writeULEB128(UInt64(bytes.count))
        output.append(contentsOf: bytes)
    }
    
    public func writeBytes(_ bytes: [UInt8]) {
        output.append(contentsOf: bytes)
    }
    
    public func writeULEB128(_ value: UInt64) {
        output.append(contentsOf: ULEB128.encode(value: value))
    }
    
    public func writeAddress(_ address: String) {
        guard address.hasPrefix("0x") else {
            fatalError("Invalid address format")
        }
        
        let hex = String(address.dropFirst(2))
        let bytes = hexToBytes(hex)
        output.append(contentsOf: bytes.suffix(20))
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

// BCS Reader for Swift SDK
public class BCSReader {
    private let data: [UInt8]
    private var offset: Int = 0
    
    public init(data: [UInt8]) {
        self.data = data
    }
    
    public var remainingBytes: Int {
        data.count - offset
    }
    
    public func readUInt8() -> UInt8 {
       guard offset < data.count else {
            fatalError("Unexpected end of input")
        }
        defer { offset += 1 }
        return data[offset]
    }
    
    public func readUInt16() -> UInt16 {
        let bytes = readBytes(2)
        return bytes.withUnsafeBytes {
            $0.load(as: UInt16.self).littleEndian
        }
    }
    
    public func readUInt32() -> UInt32 {
        let bytes = readBytes(4)
        return bytes.withUnsafeBytes {
            $0.load(as: UInt32.self).littleEndian
        }
    }
    
    public func readUInt64() -> UInt64 {
        let bytes = readBytes(8)
        return bytes.withUnsafeBytes {
            $0.load(as: UInt64.self).littleEndian
        }
    }
    
    public func readInt8() -> Int8 {
        Int8(bitPattern: readUInt8())
    }
    
    public func readInt16() -> Int16 {
        Int16(bitPattern: readUInt16())
    }
    
    public func readInt32() -> Int32 {
        Int32(bitPattern: readUInt32())
    }
    
    public func readInt64() -> Int64 {
        Int64(bitPattern: readUInt64())
    }
    
    public func readBool() -> Bool {
        readUInt8() != 0
    }
    
    public func readString() -> String {
        let length = readULEB128()
        let bytes = readBytes(Int(length))
        return String(bytes: Data(bytes), encoding: .utf8) ?? ""
    }
    
    public func readULEB128() -> UInt64 {
        let (value, length) = ULEB128.decode(data: data, offset: offset)
        offset += length
        return value
    }
    
    public func readAddress() -> String {
        let bytes = readBytes(20)
        return "0x" + bytes.map { String(format: "%02x", $0) }.joined()
    }
    
    private func readBytes(_ count: Int) -> [UInt8] {
        guard offset + count <= data.count else {
            fatalError("Unexpected end of input")
        }
        
        let start = offset
        offset += count
        return Array(data[start..<start + count])
    }
}

// SuiBCS for Swift SDK
public class SuiBCS {
    private let writer: BCSWriter
    private var reader: BCSReader?
    
    public init() {
        self.writer = BCSWriter()
    }
    
    public init(data: [UInt8]) {
        self.writer = BCSWriter()
        self.reader = BCSReader(data: data)
    }
    
    public func serialize(_ value: Any) -> [UInt8] {
        switch value {
        case let str as String:
            writer.writeString(str)
        case let num as Int:
            writer.writeUInt32(UInt32(num))
        case let num as UInt32:
            writer.writeUInt32(num)
        case let num as UInt64:
            writer.writeUInt64(num)
        case let bool as Bool:
            writer.writeBool(bool)
        case let byte as UInt8:
            writer.writeUInt8(byte)
        case let bytes as [UInt8]:
            writer.writeBytes(bytes)
        case let list as [Any]:
            serializeList(list)
        case let dict as [String: Any]:
            serializeMap(dict)
        default:
            fatalError("Unsupported type: \(type(of: value))")
        }
        
        return writer.toByteArray()
    }
    
    private func serializeList(_ list: [Any]) {
        writer.writeULEB128(UInt64(list.count))
        for item in list {
            _ = serialize(item)
        }
    }
    
    private func serializeMap(_ map: [String: Any]) {
        writer.writeULEB128(UInt64(map.count))
        for (key, value) in map {
            writer.writeString(key)
            _ = serialize(value)
        }
    }
    
    public func deserialize() -> Any? {
        guard let reader = reader else {
            fatalError("Reader not initialized")
        }
        
        let firstByte = reader.offset < reader.remainingBytes ? reader.data[reader.offset] : 0
        
        if firstByte >= 0x80 {
            return reader.readString()
        } else {
            return UInt64(reader.readUInt8())
        }
    }
}