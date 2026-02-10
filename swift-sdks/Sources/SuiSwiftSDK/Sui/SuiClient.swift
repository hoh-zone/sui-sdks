import Foundation

public final class SuiClient {
    public let network: SuiNetwork?
    public let endpoint: URL
    private let transport: JSONRPCTransport

    public init(endpoint: URL, transport: JSONRPCTransport? = nil) {
        self.network = nil
        self.endpoint = endpoint
        self.transport = transport ?? HTTPJSONRPCTransport(url: endpoint)
    }

    public init(network: SuiNetwork, transport: JSONRPCTransport? = nil) {
        self.network = network
        self.endpoint = network.fullnodeURL
        self.transport = transport ?? HTTPJSONRPCTransport(url: network.fullnodeURL)
    }

    @discardableResult
    public func call(method: String, params: [Any?] = []) async throws -> Any {
        try await transport.request(method: method, params: params)
    }

    public func discoverRPCAPI() async throws -> [String: Any] {
        let result = try await call(method: "rpc.discover")
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "rpc.discover result type mismatch")
        }
        return value
    }

    public func getObject(objectID: String, options: [String: Any] = [:]) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(objectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "sui_getObject", params: [objectID, options])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getObject result type mismatch")
        }
        return value
    }

    public func multiGetObjects(objectIDs: [String], options: [String: Any] = [:]) async throws -> [[String: Any]] {
        for objectID in objectIDs {
            if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(objectID)) {
                throw SuiValidationError.invalidSuiObjectID
            }
        }
        if Set(objectIDs).count != objectIDs.count {
            throw SuiValidationError.duplicateObjectIDs
        }
        let result = try await call(method: "sui_multiGetObjects", params: [objectIDs, options])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "sui_multiGetObjects result type mismatch")
        }
        return value
    }

    public func dryRunTransactionBlock(txBytesBase64: String) async throws -> [String: Any] {
        let result = try await call(method: "sui_dryRunTransactionBlock", params: [txBytesBase64])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_dryRunTransactionBlock result type mismatch")
        }
        return value
    }

    public func getCoins(
        owner: String,
        coinType: String? = nil,
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getCoins", params: [owner, coinType, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getCoins result type mismatch")
        }
        return value
    }

    public func getAllCoins(owner: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getAllCoins", params: [owner, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getAllCoins result type mismatch")
        }
        return value
    }

    public func getBalance(owner: String, coinType: String? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getBalance", params: [owner, coinType])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getBalance result type mismatch")
        }
        return value
    }

    public func getAllBalances(owner: String) async throws -> [[String: Any]] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getAllBalances", params: [owner])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getAllBalances result type mismatch")
        }
        return value
    }

    public func getOwnedObjects(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getOwnedObjects", params: [owner, query, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getOwnedObjects result type mismatch")
        }
        return value
    }

    public func getDynamicFields(parentObjectID: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(parentObjectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "suix_getDynamicFields", params: [parentObjectID, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getDynamicFields result type mismatch")
        }
        return value
    }

    public func queryEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_queryEvents", params: [query, cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_queryEvents result type mismatch")
        }
        return value
    }

    public func queryTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_queryTransactionBlocks", params: [query, cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_queryTransactionBlocks result type mismatch")
        }
        return value
    }

    public func getCheckpoints(cursor: String? = nil, limit: Int? = nil, descendingOrder: Bool = false) async throws -> [String: Any] {
        let result = try await call(method: "sui_getCheckpoints", params: [cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getCheckpoints result type mismatch")
        }
        return value
    }

    public func getTransactionBlock(digest: String, options: [String: Any] = [:]) async throws -> [String: Any] {
        if !SuiTypes.isValidTransactionDigest(digest) {
            throw SuiValidationError.invalidTransactionDigest
        }
        let result = try await call(method: "sui_getTransactionBlock", params: [digest, options])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getTransactionBlock result type mismatch")
        }
        return value
    }

    public func allCoins(
        owner: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getAllCoins(owner: owner, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allOwnedObjects(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getOwnedObjects(owner: owner, query: query, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allDynamicFields(
        parentObjectID: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getDynamicFields(parentObjectID: parentObjectID, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.queryEvents(query: query, cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.queryTransactionBlocks(
                    query: query,
                    cursor: pageCursor,
                    limit: limit,
                    descendingOrder: descendingOrder
                )
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allCheckpoints(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getCheckpoints(cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func getReferenceGasPrice() async throws -> String {
        let result = try await call(method: "suix_getReferenceGasPrice")
        guard let value = result as? String else {
            throw JSONRPCMalformedResponseError(reason: "suix_getReferenceGasPrice result type mismatch")
        }
        return value
    }
}
