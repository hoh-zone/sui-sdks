import Foundation

// Balance Manager Operations for Swift SDK
public class DeepBookBalanceManager {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Balance Manager Operations

    public func createManager(
        sender: String,
        ownerAddress: String? = nil
    ) async throws -> [String: Any] {
        let target = "balance_manager::new"
        var arguments: [[String: Any]] = []
        
        if let owner = ownerAddress {
            arguments = [["ownerAddress": owner]]
        }
        
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::" + target,
            arguments: arguments
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func deposit(
        sender: String,
        managerKey: String,
        coinKey: String,
        amount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::deposit",
            arguments: [
                ["managerKey": managerKey, "coinKey": coinKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func withdraw(
        sender: String,
        managerKey: String,
        coinKey: String,
        amount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::withdraw",
            arguments: [
                ["managerKey": managerKey, "coinKey": coinKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func withdrawAll(
        sender: String,
        managerKey: String,
        coinKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::withdraw_all",
            arguments: [
                ["managerKey": managerKey, "coinKey": coinKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func createCap(
        sender: String,
        managerKey: String,
        capType: String
    ) async throws -> [String: Any] {
        let target = "balance_manager::create_" + capType + "_cap"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::" + target,
            arguments: [["managerKey": managerKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getManagerBalance(
        sender: String,
        managerKey: String,
        coinKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::balance",
            arguments: [
                ["managerKey": managerKey, "coinKey": coinKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Referral Operations

    public func createReferral(
        sender: String,
        poolKey: String,
        multiplier: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::deepbook::create_referral",
            arguments: [
                ["poolKey": poolKey, "multiplier": multiplier]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func setReferrer(
        sender: String,
        referralCode: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::deepbook::set_referrer",
            arguments: [["referralCode": referralCode]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Trade Cap Methods

    public func createTradeCap(
        sender: String,
        managerKey: String,
        trader: String,
        limit: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::create_trade_cap",
            arguments: [
                ["managerKey": managerKey, "trader": trader, "limit": limit]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func updateTradeCap(
        sender: String,
        capId: String,
        newLimit: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::update_trade_cap",
            arguments: [["capId": capId, "newLimit": newLimit]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func addTraderToCap(
        sender: String,
        capId: String,
        trader: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::add_trader",
            arguments: [["capId": capId, "trader": trader]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func removeTraderFromCap(
        sender: String,
        capId: String,
        trader: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::balance_manager::remove_trader",
            arguments: [["capId": capId, "trader": trader]]
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