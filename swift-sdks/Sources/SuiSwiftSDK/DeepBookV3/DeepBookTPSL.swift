import Foundation

// Take Profit / Stop Loss Operations for Swift SDK
public class DeepBookTPSL {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Conditional Orders

    public func addConditionalOrder(
        sender: String,
        marginManagerKey: String,
        poolKey: String,
        targetPrice: UInt64,
        quantity: UInt64,
        isBid: Bool,
        priceDirection: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::add_conditional_order",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "poolKey": poolKey,
                    "targetPrice": targetPrice,
                    "quantity": quantity,
                    "isBid": isBid,
                    "priceDirection": priceDirection
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelConditionalOrder(
        sender: String,
        marginManagerKey: String,
        orderId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::cancel_conditional_order",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "orderId": orderId
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelAllConditionalOrders(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::cancel_all_conditional_orders",
            arguments: [["marginManagerKey": marginManagerKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func executeConditionalOrders(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::execute_conditional_orders",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Take Profit

    public func setTakeProfit(
        sender: String,
        marginManagerKey: String,
        targetPrice: UInt64,
        quantity: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::set_take_profit",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "targetPrice": targetPrice,
                    "quantity": quantity
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelTakeProfit(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::cancel_take_profit",
            arguments: [["marginManagerKey": marginManagerKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Stop Loss

    public func setStopLoss(
        sender: String,
        marginManagerKey: String,
        targetPrice: UInt64,
        quantity: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::set_stop_loss",
            arguments: [
                [
                    "marginManagerKey": marginManagerKey,
                    "targetPrice": targetPrice,
                    "quantity": quantity
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func cancelStopLoss(
        sender: String,
        marginManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::margin_manager::cancel_stop_loss",
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