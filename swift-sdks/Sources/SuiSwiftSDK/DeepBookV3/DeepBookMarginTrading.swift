import Foundation

// Margin Trading Operations for Swift SDK
public class DeepBookMarginTrading {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Margin Pool Operations

    public func deposit(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "deposit_base" : "deposit_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func withdraw(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "withdraw_base" : "withdraw_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func borrow(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "borrow_base" : "borrow_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func repay(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "repay_base" : "repay_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Margin Manager Operations

    public func createManager(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::new",
            arguments: [
                ["poolKey": poolKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func placeLimitOrder(
        sender: String,
        marginManagerKey: String,
        clientOrderId: String,
        price: UInt64,
        quantity: UInt64,
        isBid: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool_proxy::place_limit_order",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "clientOrderId": clientOrderId,
                    "price": price,
                    "quantity": quantity,
                    "isBid": isBid
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func placeMarketOrder(
        sender: String,
        marginManagerKey: String,
        clientOrderId: String,
        quantity: UInt64,
        isBid: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool_proxy::place_market_order",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "clientOrderId": clientOrderId,
                    "quantity": quantity,
                    "isBid": isBid
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Liquidation Methods

    public func forceLiquidate(
        sender: String,
        marginManagerKey: String,
        liquidatorKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::force_liquidate",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "liquidatorKey": liquidatorKey
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func calculateLiquidation(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::calculate_liquidation",
            arguments: [["marginManagerKey": marginManagerKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func isOverCollateralized(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::is_over_collateralized",
            arguments: [["marginManagerKey": marginManagerKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getLiquidationPrice(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::get_liquidation_price",
            arguments: [["marginManagerKey": marginManagerKey]]
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