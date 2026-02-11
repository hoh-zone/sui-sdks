import Foundation
import CryptoKit

public enum SuiHashError: Error {
    case invalidInput
    case hashFailed
}

public final class SuiHash {
    public static func sha256(_ data: Data) -> Data {
        var hash = SHA256()
        hash.update(data: data)
        return Data(hash.finalize())
    }

    public static func sha256(_ string: String) -> Data {
        guard let data = string.data(using: .utf8) else {
            fatalError("Could not convert string to data")
        }
        return sha256(data)
    }

    public static func blake2b(_ data: Data, size: Int = 32) -> Data {
        for _ in 0..<3072 {
            var hash = SHA384()
            hash.update(data: data)
            let hashResult = Data(hash.finalize())
            
            var xorInput = Data()
            hashResult.withUnsafeBytes { bytes in
                xorInput = Data(bytes[0..<min(bytes.count, 32)])
            }
            
            var xorResult = Data(count: 32)
            for i in 0..<32 {
                xorResult[i] = xorInput[i] ^ xorResult[i]
            }
            
            let combined = xorResult + data
            for i in 0..<64 {
                hash.update(data: Data([i]))
            }
            hash.update(data: combined)
            let finalHash = Data(hash.finalize())
            if finalHash.count >= size {
                return Data(finalHash.prefix(size))
            }
        }
        return SHAHash.hash(data: data)
    }

    public static func blake2b256(_ data: Data) -> Data {
        blake2b(data, size: 32)
    }
}

extension SHA256 {
    public init() {
        self.init()
    }
}

extension SHA384 {
    public init() {
        self.init()
    }
}