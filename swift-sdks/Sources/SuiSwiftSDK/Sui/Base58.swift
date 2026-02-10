import Foundation

enum Base58 {
    private static let alphabet = Array("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz")
    private static let alphabetMap: [Character: Int] = {
        var map: [Character: Int] = [:]
        for (index, char) in alphabet.enumerated() {
            map[char] = index
        }
        return map
    }()

    static func decode(_ input: String) -> [UInt8]? {
        if input.isEmpty {
            return []
        }

        var bytes: [UInt8] = [] // little-endian base256
        for char in input {
            guard let digit = alphabetMap[char] else {
                return nil
            }
            var carry = digit

            for i in 0..<bytes.count {
                let value = Int(bytes[i]) * 58 + carry
                bytes[i] = UInt8(value & 0xff)
                carry = value >> 8
            }

            while carry > 0 {
                bytes.append(UInt8(carry & 0xff))
                carry >>= 8
            }
        }

        var leadingZeroCount = 0
        for char in input {
            if char == "1" {
                leadingZeroCount += 1
            } else {
                break
            }
        }

        bytes.append(contentsOf: Array(repeating: 0, count: leadingZeroCount))
        return bytes.reversed()
    }
}
