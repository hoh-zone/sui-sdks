import Foundation

// Advanced Query Operations for Swift SDK (P2)
public class DeepBookAdvancedQueries {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Account Queries

    public func getAccount(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::account",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getLockedBalance(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::locked_balance",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getAccountOrderDetails(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::get_account_order_details",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func accountExists(
        sender: String,
        poolKey: String,
        managerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::account_exists",
            arguments: [
                ["poolKey": poolKey, "managerKey": managerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Pool Metadata Queries

    public func getQuorum(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::quorum",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getPoolID(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::id",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func getBalanceManagerIDs(
        sender: String,
        owner: String,
        registryId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::registry::get_balance_manager_ids",
            arguments: [
                ["owner": owner, "registryId": registryId]
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