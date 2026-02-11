import Foundation

// P2: 订单预验证 and 费用查询
public class DeepBookOrderValidation {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Order Pre-validation

    public func canPlaceLimitOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        isBid: Bool,
        price: UInt64,
        quantity: UInt64,
        payWithDeep: Bool,
        expiration: Int
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::can_place_limit_order",
            arguments: [
                [
                    "poolKey": poolKey,
                    "managerKey": managerKey,
                    "isBid": isBid,
                    "price": price,
                    "quantity": quantity,
                    "payWithDeep": payWithDeep,
                    "expiration": expiration
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func canPlaceMarketOrder(
        sender: String,
        poolKey: String,
        managerKey: String,
        isBid: Bool,
        quantity: UInt64,
        payWithDeep: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::can_place_market_order",
            arguments: [
                [
                    "poolKey": poolKey,
                    "managerKey": managerKey,
                    "isBid": isBid,
                    "quantity": quantity,
                    "payWithDeep": payWithDeep
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Fee Queries

    public func getQuoteQuantityInputFee(
        sender: String,
        poolKey: String,
        amount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_quote_quantity_out_input_fee",
            arguments: [["poolKey": poolKey, "amount": amount]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getBaseQuantityInputFee(
        sender: String,
        poolKey: String,
        amount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_base_quantity_out_input_fee",
            arguments: [["poolKey": poolKey, "amount": amount]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getQuantityInputFee(
        sender: String,
        poolKey: String,
        baseAmount: UInt64,
        quoteAmount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_quantity_out_input_fee",
            arguments: [
                ["poolKey": poolKey, "baseAmount": baseAmount, "quoteAmount": quoteAmount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Reverse Quantity Calculations

    public func getBaseQuantityIn(
        sender: String,
        poolKey: String,
        targetQuoteAmount: UInt64,
        payWithDeep: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_base_quantity_in",
            arguments: [
                ["poolKey": poolKey, "targetQuoteAmount": targetQuoteAmount, "payWithDeep": payWithDeep]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getQuoteQuantityIn(
        sender: String,
        poolKey: String,
        targetBaseAmount: UInt64,
        payWithDeep: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_quote_quantity_in",
            arguments: [
                ["poolKey": poolKey, "targetBaseAmount": targetBaseAmount, "payWithDeep": payWithDeep]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Deep Token Requirement

    public func getOrderDeepRequired(
        sender: String,
        poolKey: String,
        baseQuantity: UInt64,
        price: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_order_deep_required",
            arguments: [
                ["poolKey": poolKey, "baseQuantity": baseQuantity, "price": price]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getPoolDeepPrice(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_order_deep_price",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Pool Trade Params

    public func getPoolTradeParamsNext(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::pool_trade_params_next",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getPoolBookParams(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::pool_book_params",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Pool Metadata

    public func isWhitelisted(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::whitelisted",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func isStablePool(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::stable_pool",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func isRegisteredPool(
        sender: String,
        poolKey: String,
        registryId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::registered_pool",
            arguments: [
                ["poolKey": poolKey, "registryId": registryId]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Helper Methods

    private func call(_ method: String, params params: [Any?] = []) async throws -> [String: Any] {
        guard let client = client else {
            throw DeepBookError.invalidResponse
        }
        return try await client.call(method, params: params)
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