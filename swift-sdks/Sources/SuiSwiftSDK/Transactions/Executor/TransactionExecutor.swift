import Foundation

public enum TransactionExecutionError: Error, LocalizedError {
    case queueFull
    case queueClosed
    case transactionFailed(String)
    case executorBusy

    public var errorDescription: String? {
        switch self {
        case .queueFull: return "Transaction queue is full"
        case .queueClosed: return "Transaction queue is closed"
        case .transactionFailed(let msg): return "Transaction failed: \(msg)"
        case .executorBusy: return "Transaction executor is busy"
        }
    }
}

public struct TransactionResult {
    public let success: Bool
    public let error: String?

    public init(success: Bool, error: String? = nil) {
        self.success = success
        self.error = error
    }

    public static func success() -> TransactionResult {
        return TransactionResult(success: true)
    }

    public static func failure(_ error: String) -> TransactionResult {
        return TransactionResult(success: false, error: error)
    }
}

public class ObjectCache {
    private var cache: [String: [String: Any]] = [:]
    private var ownedObjects: [String: [String: Any]] = [:]
    private var customCache: [String: [String: Any]] = [:]

    public init() {}

    public func set(_ id: String, object: [String: Any]) {
        cache[id] = object
    }

    public func get(_ id: String) -> [String: Any]? {
        return cache[id]
    }

    public func delete(_ id: String) {
        cache.removeValue(forKey: id)
        ownedObjects.removeValue(forKey: id)
        customCache.removeValue(forKey: id)
    }

    public func clearOwnedObjects() {
        ownedObjects.removeAll()
    }

    public func clearCustom() {
        customCache.removeAll()
    }

    public func getOwnedObject(_ id: String) -> [String: Any]? {
        return ownedObjects[id]
    }

    public func setOwnedObject(_ id: String, object: [String: Any]) {
        ownedObjects[id] = object
    }

    public func getCustom(_ key: String) -> [String: Any]? {
        return customCache[key]
    }

    public func setCustom(_ key: String, value: [String: Any]) {
        customCache[key] = value
    }

    public func deleteCustom(_ key: String) {
        customCache.removeValue(forKey: key)
    }
}

public class CachingTransactionExecutor {
    private let cache: ObjectCache

    public init(cache: ObjectCache = ObjectCache()) {
        self.cache = cache
    }

    public func execute(_ transaction: SuiTransaction) async throws -> TransactionResult {
        return TransactionResult.success()
    }

    public func applyEffects(_ effects: [String: Any]) {
        if let changedObjects = effects["changedObjects"] as? [[String: Any]] {
            for changedObj in changedObjects {
                if let objectId = changedObj["objectId"] as? String,
                   let outputState = changedObj["outputState"] as? [String: Any],
                   outputState["ObjectWrite"] != nil {
                    cache.setOwnedObject(objectId, object: changedObj)
                }
            }
        }
    }

    public func reset() {
        cache.clearOwnedObjects()
        cache.clearCustom()
    }

    public func getCache() -> ObjectCache {
        return cache
    }
}

public class SerialQueue {
    private var taskQueue: [() async throws -> Void] = []
    private var isProcessing = false

    public init() {}

    public func runTask<T>(_ task: @escaping () async throws -> T) async throws -> T {
        return try await task()
    }
}

public class ParallelQueue {
    private let maxConcurrency: Int
    private var activeTasks = 0
    private var taskQueue: [() async throws -> Void] = []
    private var continuations: [CheckedContinuation<Void, Never>] = []

    public init(maxConcurrency: Int = 4) {
        self.maxConcurrency = maxConcurrency
    }

    public func runTask<T>(_ task: @escaping () async throws -> T) async throws -> T {
        if activeTasks >= maxConcurrency {
            await withCheckedContinuation { continuation in
                continuations.append(continuation)
            }
        }

        activeTasks += 1
        defer { activeTasks -= 1 }

        let result = try await task()

        if let continuation = continuations.first {
            continuations.removeFirst()
            continuation.resume()
        }

        return result
    }
}

public class SerialTransactionExecutor {
    private let cacher: CachingTransactionExecutor
    private let queue = SerialQueue()
    private var lastDigest: String?

    public init(cache: ObjectCache = ObjectCache()) {
        self.cacher = CachingTransactionExecutor(cache: cache)
    }

    public func execute(_ transaction: SuiTransaction) async throws -> TransactionResult {
        return try await queue.runTask {
            return try await self.cacher.execute(transaction)
        }
    }

    public func executeTransaction(_ transaction: SuiTransaction, signatures: [String]) async throws -> TransactionResult {
        return try await execute(transaction)
    }

    public func resetCache() {
        cacher.reset()
    }

    public func getCache() -> ObjectCache {
        return cacher.getCache()
    }
}

public struct CoinInfo {
    public let id: String
    public let version: String
    public let digest: String
    public let balance: UInt

    public init(id: String, version: String, digest: String, balance: UInt) {
        self.id = id
        self.version = version
        self.digest = digest
        self.balance = balance
    }
}

public class ParallelTransactionExecutor {
    private let maxPoolSize: Int
    private let executeQueue = ParallelQueue()
    private let cacher: CachingTransactionExecutor
    private var objectQueues: [String: [() -> Void]] = [:]
    private var coinPool: [CoinInfo] = []
    private var pendingTransactions = 0

    public init(maxPoolSize: Int = 50, cache: ObjectCache = ObjectCache()) {
        self.maxPoolSize = maxPoolSize
        self.cacher = CachingTransactionExecutor(cache: cache)
    }

    public func executeAll(_ transactions: [SuiTransaction]) async throws -> [TransactionResult] {
        return try await withThrowingTaskGroup(of: TransactionResult.self) { group in
            for transaction in transactions {
                group.addTask {
                    try await self.execute(transaction)
                }
            }

            var results: [TransactionResult] = []
            for try await result in group {
                results.append(result)
            }
            return results
        }
    }

    public func execute(_ transaction: SuiTransaction) async throws -> TransactionResult {
        let usedObjects = try await getUsedObjects(transaction)
        return try await executeQueue.runTask {
            let result = try await self.cacher.execute(transaction)
            self.pendingTransactions -= 1
            return result
        }
    }

    private func getUsedObjects(_ transaction: SuiTransaction) async throws -> Set<String> {
        return Set()
    }

    public func resetCache() {
        cacher.reset()
    }

    public func getCache() -> ObjectCache {
        return cacher.getCache()
    }
}

public struct TransactionExecutor {
    private var transactions: [SuiTransaction] = []

    public init() {}

    public mutating func execute(_ transaction: SuiTransaction) -> TransactionResult {
        transactions.append(transaction)
        return TransactionResult.success()
    }

    public func reset() {
        transactions.removeAll()
    }
}