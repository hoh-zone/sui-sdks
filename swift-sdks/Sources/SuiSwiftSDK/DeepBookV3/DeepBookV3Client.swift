import Foundation

// DeepBook v3 Client for Swift SDK
public class DeepBookV3Client {
    private let transport: HTTPJSONRPCTransport
    private let config: DeepBookConfig

    public init(endpoint: URL, config: DeepBookConfig) {
        self.transport = HTTPJSONRPCTransport(url: endpoint)
        self.config = config
    }

    public static func forMainnet() -> DeepBookV3Client {
        return DeepBookV3Client(
            endpoint: URL(string: "https://fullnode.mainnet.sui.io")!,
            config: .mainnet
        )
    }

    public static func forTestnet() -> DeepBookV3Client {
        return DeepBookV3Client(
            endpoint: URL(string: "https://fullnode.testnet.sui.io")!,
            config: .testnet
        )
    }

    // MARK: - Trading Operations

    public func placeLimitOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        isBid: Bool,
        price: UInt64,
        quantity: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::place_limit_order",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey, "isBid": isBid, "price": price, "quantity": quantity]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func placeMarketOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        isBid: Bool,
        quantity: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::place_market_order",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey, "isBid": isBid, "quantity": quantity]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        orderId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::cancel_order",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey, "orderId": orderId]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelAllOrders(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::cancel_all_orders",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func modifyOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        orderId: String,
        newQuantity: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::modify_order",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey, "orderId": orderId, "quantity": newQuantity]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Swap Operations

    public func swapExactBaseForQuote(
        sender: String,
        poolKey: String,
        baseAsset: String,
        quoteAsset: String,
        baseAmount: UInt64,
        minQuoteAmount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::swap_exact_base_for_quote",
            arguments: [
                ["poolKey": poolKey, "baseAsset": baseAsset, "quoteAsset": quoteAsset,
                 "baseAmount": baseAmount, "minQuoteAmount": minQuoteAmount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func swapExactQuoteForBase(
        sender: String,
        poolKey: String,
        quoteAsset: String,
        baseAsset: String,
        quoteAmount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\(config.deepbookPackage)::deepbook::swap_exact_quote_for_base",
            arguments: [
                ["poolKey": poolKey, "quoteAsset": quoteAsset, "baseAsset": baseAsset, "quoteAmount": quoteAmount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Pool Queries

    public func getPoolObject(poolKey: String) async throws -> [String: Any] {
        let options: [String: Any] = [
            "showContent": true,
            "showOwner": true
        ]
        
        return try await call("sui_getObject", params: [
            "objectId": poolKey,
            "options": options
        ])
    }

    public func midPrice(poolKey: String, deepbookPackage: String) async throws -> [String: Any] {
        let pool = try await getPoolObject(poolKey: poolKey)
        
        if let data = pool["data"] as? [String: Any],
           let fields = data["fields"] as? [String: Any] {
            return ["midPrice": fields["mid_price"] as Any]
        }
        
        return ["midPrice": 0]
    }

    public func poolTradeParams(poolKey: String) async throws -> [String: Any] {
        return try await getPoolObject(poolKey: poolKey)
    }

    public func vaultBalances(poolKey: String) async throws -> [String: Any] {
        return try await getPoolObject(poolKey: poolKey)
    }

    // MARK: - Order Queries

    public func getOrder(orderId: String) async throws -> [String: Any] {
        let options: [String: Any] = ["showContent": true]
        
        return try await call("sui_getObject", params: [
            "objectId": orderId,
            "options": options
        ])
    }

    func call(_ method: String, params params: [Any?] = []) async throws -> [String: Any] {
        let result = try await transport.request(method: method, params: params)
        guard let value = result as? [String: Any] else {
            throw DeepBookError.invalidResponse
        }
        return value
    }

    private func buildTransaction(target: String, arguments: [[String: Any]]) -> [String: Any] {
        var transactions: [[String: Any]] = []
        
        for args in arguments {
            var moveCall: [String: Any] = [:]
            moveCall["kind"] = "moveCall"
            moveCall["target"] = target
            moveCall["arguments"] = args
            transactions.append(moveCall)
        }

        return [
            "kind": "programmableTransaction",
            "inputs": [],
            "transactions": transactions
        ]
    }
}

public enum DeepBookError: Error, LocalizedError {
    case invalidResponse
    case networkError(String)
    
    public var errorDescription: String? {
        switch self {
        case .invalidResponse: return "Invalid response from RPC"
        case .networkError(let message): return "Network error: \(message)"
        }
    }
}

public struct DeepBookConfig {
    public let network: String
    public let deepbookPackage: String
    public let address: String

    public init(network: String, deepbookPackage: String, address: String) {
        self.network = network
        self.deepbookPackage = deepbookPackage
        self.address = address
    }

    public static let mainnet = DeepBookConfig(
        network: "mainnet",
        deepbookPackage: "0x1bf2db9e6c4f647011f6091efb275e28efc0426c6c8e54908bb2dd743d4e2ec",
        address: "0x0"
    )

    public static let testnet = DeepBookConfig(
        network: "testnet",
        deepbookPackage: "0x1bf2db9e6c4f647011f6091efb275e28efc0426c6c8e54908bb2dd743d4e2ec",
        address: "0x0"
    )
}