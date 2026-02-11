import Foundation

// System-level features for Swift SDK (Flash Loans, Governance, Pyth Oracle, DEEP token)
public class DeepBookSystem {
    weak var client: DeepBookV3Client?

    public init(client: DeepBookV3Client) {
        self.client = client
    }

    // MARK: - Flash Loans

    public func borrowFlashLoan(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "borrow_flashloan_base" : "borrow_flashloan_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func returnFlashLoan(
        sender: String,
        poolKey: String,
        amount: UInt64,
        isBase: Bool
    ) async throws -> [String: Any] {
        let function = isBase ? "return_flashloan_base" : "return_flashloan_quote"
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::" + function,
            arguments: [
                ["poolKey": poolKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Governance

    public func createProposal(
        sender: String,
        poolKey: String,
        takerFee: Double,
        makerFee: Double,
        stakeRequired: Double
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::submit_proposal",
            arguments: [
                [
                    "poolKey": poolKey,
                    "takerFee": takerFee,
                    "makerFee": makerFee,
                    "stakeRequired": stakeRequired
                ]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func vote(
        sender: String,
        poolKey: String,
        balanceManagerKey: String,
        proposalId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::vote",
            arguments: [
                ["poolKey": poolKey, "balanceManagerKey": balanceManagerKey, "proposalId": proposalId]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func executeProposal(
        sender: String,
        proposalId: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::execute_proposal",
            arguments: [["proposalId": proposalId]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - DEEP Token Operations

    public func stakeDeep(
        sender: String,
        poolKey: String,
        balanceManagerKey: String,
        amount: UInt64
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::stake",
            arguments: [
                ["poolKey": poolKey, "balanceManagerKey": balanceManagerKey, "amount": amount]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func unstakeDeep(
        sender: String,
        poolKey: String,
        balanceManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::unstake",
            arguments: [
                ["poolKey": poolKey, "balanceManagerKey": balanceManagerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func claimRebates(
        sender: String,
        poolKey: String,
        balanceManagerKey: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::pool::claim_rebates",
            arguments: [
                ["poolKey": poolKey, "balanceManagerKey": balanceManagerKey]
            ]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    // MARK: - Pyth Oracle Integration

    public func getPythPriceUpdate(
        sender: String,
        priceIds: [String]
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "pyth::oracle::get_price_feeds",
            arguments: [["priceIds": priceIds]]
        )

        return try await call("suix_getObject", params: txData)
    }

    public func verifyPythPrice(
        sender: String,
        priceUpdateData: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: "pyth::price_feed::update_price_feeds",
            arguments: [["priceUpdateData": priceUpdateData]]
        )

        return try await call("suix_getObject", params: txData)
    }

    // MARK: - DEBATE Functions

    public func debateVote(
        sender: String,
        debateId: String,
        vote: Bool
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::debate::vote",
            arguments: [["debateId": debateId, "vote": vote]]
        )

        return try await call("suix_devInspectTransactionBlock", params: [sender, txData])
    }

    public func debateCreate(
        sender: String,
        topic: String,
        description: String
    ) async throws -> [String: Any] {
        let txData = buildTransaction(
            target: (client?.config.deepbookPackage ?? "") + "::debate::create",
            arguments: [
                ["topic": topic, "description": description]
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