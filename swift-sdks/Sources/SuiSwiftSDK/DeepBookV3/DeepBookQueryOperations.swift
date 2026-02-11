import Foundation

// Order Book Query Operations for Swift SDK
public class DeepBookQueryOperations {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Order Book Queries

    public func getLevel2Range(
        sender: String,
        poolKey: String,
        priceLow: UInt64,
        priceHigh: UInt64,
        isBid: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? "") + "::deepbook::get_level2_range")",            arguments: [
                ["poolKey": poolKey, "priceLow": priceLow, "priceHigh": priceHigh, "isBid": isBid]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getLevel2TicksFromMid(
        sender: String,
        poolKey: String,
        ticks: Int,
        isBid: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? "") + "::deepbook::get_level2_ticks_from_mid")",
            arguments: [
                ["poolKey": poolKey, "ticks": ticks, "isBid": isBid]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func accountOpenOrders(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? "") + "::deepbook::account_open_orders")",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getOrder(accountKey: String) async throws -> [String: Any] {
        let options: [String: Any] = ["showContent": true]
        
        return try await call("sui_getObject", params: [
            "objectId": accountKey,
            "options": options
        ])
    }

    public func getOrders(accountKeys: [String]) async throws -> [String: Any] {
        let options: [String: Any] = ["showContent": true]
        
        return try await call("sui_multiGetObjects", params: [accountKeys, options])
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