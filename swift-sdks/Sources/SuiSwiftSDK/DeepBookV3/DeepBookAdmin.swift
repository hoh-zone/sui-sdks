import Foundation

// Admin Operations for Swift SDK (P3)
public class DeepBookAdmin {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Admin Operations

    public func createPoolAdmin(
        sender: String,
        baseCoinKey: String,
        quoteCoinKey: String,
        tickSize: Double,
        lotSize: Double,
        minSize: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::create_pool_admin",
            arguments: [
                [
                    "baseCoinKey": baseCoinKey,
                    "quoteCoinKey": quoteCoinKey,
                    "tickSize": tickSize,
                    "lotSize": lotSize,
                    "minSize": minSize
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func setTickSize(
        sender: String,
        poolKey: String,
        tickSize: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::set_tick_size",
            arguments: [["poolKey": poolKey, "tickSize": tickSize]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func setLotSize(
        sender: String,
        poolKey: String,
        lotSize: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::set_lot_size",
            arguments: [["poolKey": poolKey, "lotSize": lotSize]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func setBasePricePoint(
        sender: String,
        poolKey: String,
        price: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::set_base_price_point",
            arguments: [["poolKey": poolKey, "price": price]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func setQuotePricePoint(
        sender: String,
        poolKey: String,
        price: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::set_quote_price_point",
            arguments: [["poolKey": poolKey, "price": price]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func updatePoolAllowedVersions(
        sender: String,
        poolKey: String,
        registryId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::update_pool_allowed_versions",
            arguments: [["poolKey": poolKey, "registryId": registryId]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func withdrawAll(
        sender: String,
        poolKey: String,
        recipient: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::withdraw_all",
            arguments: [["poolKey": poolKey, "recipient": recipient]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func removePool(
        sender: String,
        poolKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::remove_pool",
            arguments: [["poolKey": poolKey]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func burnDeep(
        sender: String,
        poolKey: String,
        treasuryId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "\((client?.config.deepbookPackage ?? ""))::pool::burn_deep",
            arguments: [["poolKey": poolKey, "treasuryId": treasuryId]]
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