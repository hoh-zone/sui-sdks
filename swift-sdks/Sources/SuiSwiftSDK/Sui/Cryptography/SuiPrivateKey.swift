import Foundation

public struct ParsedSuiPrivateKey {
    public let scheme: SignatureScheme
    public let secretKey: [UInt8]

    public init(scheme: SignatureScheme, secretKey: [UInt8]) {
        self.scheme = scheme
        self.secretKey = secretKey
    }
}

public enum SuiPrivateKeyError: Error, Equatable {
    case invalidPrefix
    case invalidLength
    case invalidScheme
    case unsupportedScheme
    case invalidBech32
}

public enum SuiPrivateKey {
    public static let prefix = "suiprivkey"
    public static let privateKeySize = 32

    public static func encode(secretKey: [UInt8], scheme: SignatureScheme) throws -> String {
        guard secretKey.count == privateKeySize else {
            throw SuiPrivateKeyError.invalidLength
        }
        guard scheme.supportsSuiPrivateKey else {
            throw SuiPrivateKeyError.unsupportedScheme
        }

        let extended = [scheme.rawValue] + secretKey
        return try Bech32.encode(hrp: prefix, data: extended)
    }

    public static func decode(_ value: String) throws -> ParsedSuiPrivateKey {
        let decoded = try Bech32.decode(value)
        guard decoded.hrp == prefix else {
            throw SuiPrivateKeyError.invalidPrefix
        }
        guard decoded.data.count == privateKeySize + 1 else {
            throw SuiPrivateKeyError.invalidLength
        }

        guard let scheme = SignatureScheme(rawValue: decoded.data[0]) else {
            throw SuiPrivateKeyError.invalidScheme
        }
        guard scheme.supportsSuiPrivateKey else {
            throw SuiPrivateKeyError.unsupportedScheme
        }

        return ParsedSuiPrivateKey(scheme: scheme, secretKey: Array(decoded.data.dropFirst()))
    }
}

private enum Bech32 {
    private static let charset = Array("qpzry9x8gf2tvdw0s3jn54khce6mua7l")
    private static let generator: [UInt32] = [
        0x3b6a57b2,
        0x26508e6d,
        0x1ea119fa,
        0x3d4233dd,
        0x2a1462b3,
    ]

    private static let revMap: [Character: UInt8] = {
        var m: [Character: UInt8] = [:]
        for (i, c) in charset.enumerated() {
            m[c] = UInt8(i)
        }
        return m
    }()

    struct Decoded {
        let hrp: String
        let data: [UInt8]
    }

    static func encode(hrp: String, data: [UInt8]) throws -> String {
        let words = try convertBits(data, from: 8, to: 5, pad: true)
        let checksum = createChecksum(hrp: hrp, data: words)
        let combined = words + checksum
        let payload = combined.map { String(charset[Int($0)]) }.joined()
        return hrp + "1" + payload
    }

    static func decode(_ value: String) throws -> Decoded {
        guard !value.isEmpty else {
            throw SuiPrivateKeyError.invalidBech32
        }

        let lower = value.lowercased()
        guard value == lower || value == value.uppercased() else {
            throw SuiPrivateKeyError.invalidBech32
        }

        guard let sepIndex = lower.lastIndex(of: "1") else {
            throw SuiPrivateKeyError.invalidBech32
        }

        let hrp = String(lower[..<sepIndex])
        let dataPart = String(lower[lower.index(after: sepIndex)...])
        guard !hrp.isEmpty, dataPart.count >= 6 else {
            throw SuiPrivateKeyError.invalidBech32
        }

        var values: [UInt8] = []
        values.reserveCapacity(dataPart.count)
        for c in dataPart {
            guard let v = revMap[c] else {
                throw SuiPrivateKeyError.invalidBech32
            }
            values.append(v)
        }

        guard verifyChecksum(hrp: hrp, data: values) else {
            throw SuiPrivateKeyError.invalidBech32
        }

        let payload = Array(values.dropLast(6))
        let bytes = try convertBits(payload, from: 5, to: 8, pad: false)
        return Decoded(hrp: hrp, data: bytes)
    }

    private static func hrpExpand(_ hrp: String) -> [UInt8] {
        let chars = Array(hrp.utf8)
        let high = chars.map { $0 >> 5 }
        let low = chars.map { $0 & 0x1f }
        return high + [0] + low
    }

    private static func polymod(_ values: [UInt8]) -> UInt32 {
        var chk: UInt32 = 1
        for v in values {
            let top = chk >> 25
            chk = ((chk & 0x1ffffff) << 5) ^ UInt32(v)
            for (i, g) in generator.enumerated() {
                if ((top >> i) & 1) != 0 {
                    chk ^= g
                }
            }
        }
        return chk
    }

    private static func createChecksum(hrp: String, data: [UInt8]) -> [UInt8] {
        let values = hrpExpand(hrp) + data + Array(repeating: UInt8(0), count: 6)
        let mod = polymod(values) ^ 1
        return (0..<6).map { i in
            UInt8((mod >> UInt32(5 * (5 - i))) & 0x1f)
        }
    }

    private static func verifyChecksum(hrp: String, data: [UInt8]) -> Bool {
        polymod(hrpExpand(hrp) + data) == 1
    }

    private static func convertBits(_ data: [UInt8], from: Int, to: Int, pad: Bool) throws -> [UInt8] {
        var acc = 0
        var bits = 0
        var ret: [UInt8] = []
        let maxv = (1 << to) - 1
        let maxAcc = (1 << (from + to - 1)) - 1

        for value in data {
            let v = Int(value)
            if (v >> from) != 0 {
                throw SuiPrivateKeyError.invalidBech32
            }
            acc = ((acc << from) | v) & maxAcc
            bits += from
            while bits >= to {
                bits -= to
                ret.append(UInt8((acc >> bits) & maxv))
            }
        }

        if pad {
            if bits > 0 {
                ret.append(UInt8((acc << (to - bits)) & maxv))
            }
        } else if bits >= from || ((acc << (to - bits)) & maxv) != 0 {
            throw SuiPrivateKeyError.invalidBech32
        }

        return ret
    }
}
