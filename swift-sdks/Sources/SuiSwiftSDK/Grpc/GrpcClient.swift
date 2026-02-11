import Foundation
import GRPC
import NIO

public enum SuiGrpcError: Error, LocalizedError {
    case connectionFailed(String)
    case requestFailed(String)
    case responseError(String)

    public var errorDescription: String? {
        switch self {
        case .connectionFailed(let message): return "Connection failed: \(message)"
        case .requestFailed(let message): return "Request failed: \(message)"
        case .responseError(let message): return "Response error: \(message)"
        }
    }
}

public final class SuiGrpcClient {
    public let endpoint: String
    private let group: EventLoopGroup

    public static func forMainnet() -> SuiGrpcClient {
        SuiGrpcClient(endpoint: "fullnode.mainnet.sui.io:9000")
    }

    public static func forTestnet() -> SuiGrpcClient {
        SuiGrpcClient(endpoint: "fullnode.testnet.sui.io:9000")
    }

    public static func forDevnet() -> SuiGrpcClient {
        SuiGrpcClient(endpoint: "fullnode.devnet.sui.io:9000")
    }

    public static func forLocalnet() -> SuiGrpcClient {
        SuiGrpcClient(endpoint: "127.0.0.1:9000")
    }

    public init(endpoint: String) {
        self.endpoint = endpoint
        self.group = MultiThreadedEventLoopGroup(numberOfThreads: 1)
    }

    deinit {
        try? group.syncShutdownGracefully()
    }

    private func makeCall<Service: ClientProtocol, Call: ClientCallProtocol>(
        _ service: Service.Type,
        _ call: Call.Type,
        _ request: Call.RequestType,
        _ options: CallOptions
    ) async throws -> Call.ResponseType {
        if let service = try? Service(channel: makeChannel()) {
            let call = service.call(call, request, options, callOptions: nil)
            return try await call.response
        }
        throw SuiGrpcError.connectionFailed("Could not create gRPC service")
    }

    private func makeChannel() -> ClientConnection {
        ClientConnection.insecure(group: group)
            .connect(host: endpoint.split(separator: ":").first ?? endpoint, port: 9000)
    }

    public func getObject(objectId: String) async throws -> [String: Any] {
        try await callMethod("sui_getObject", params: [objectId])
    }

    public func getObjects(objectIds: [String], options: [String: Any] = [:]) async throws -> [[String: Any]] {
        try await callMethod("sui_multiGetObjects", params: [objectIds, options])
    }

    public func executeTransactionBlock(txBytes: String, signatures: [String], options: [String: Any] = [:]) async throws -> [String: Any] {
        try await callMethod("sui_executeTransactionBlock", params: [txBytes, signatures, options])
    }

    public func dryRunTransactionBlock(txBytes: String) async throws -> [String: Any] {
        try await callMethod("sui_dryRunTransactionBlock", params: [txBytes])
    }

    public func devInspectTransactionBlock(sender: String, txBytes: String, gasPrice: String? = nil, epoch: String? = nil) async throws -> [String: Any] {
        try await callMethod("sui_devInspectTransactionBlock", params: [sender, txBytes, gasPrice, epoch].filter { $0 != nil })
    }

    public func getTransaction(digest: String, options: [String: Any] = [:]) async throws -> [String: Any] {
        try await callMethod("sui_getTransactionBlock", params: [digest, options])
    }

    public func getAllCoins(owner: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getAllCoins", params: [owner, cursor, limit].filter { $0 != nil })
    }

    public func getBalance(owner: String, coinType: String? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getBalance", params: [owner, coinType])
    }

    public func getAllBalances(owner: String) async throws -> [[String: Any]] {
        try await callMethod("suix_getAllBalances", params: [owner])
    }

    public func getEvents(query: [String: Any], cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getEvents", params: [query, cursor, limit].filter { $0 != nil })
    }

    public func queryEvents(query: [String: Any], cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_queryEvents", params: [query, cursor, limit].filter { $0 != nil })
    }

    public func getCheckpoints(cursor: String? = nil, limit: Int? = nil, descending: Bool = false) async throws -> [String: Any] {
        try await callMethod("suix_getCheckpoints", params: [cursor, limit, descending].filter { $0 != nil })
    }

    public func getCurrentSystemState() async throws -> [String: Any] {
        try await callMethod("suix_getLatestSystemState")
    }

    public func getReferenceGasPrice() async throws -> [String: Any] {
        try await callMethod("suix_getReferenceGasPrice")
    }

    public func getChainIdentifier() async throws -> [String: Any] {
        try await callMethod("suix_getChainIdentifier")
    }

    public func getStakes(owner: String) async throws -> [[String: Any]] {
        try await callMethod("suix_getStakes", params: [owner])
    }

    public func getCurrentEpoch() async throws -> [String: Any] {
        try await callMethod("suix_getCurrentEpoch")
    }

    public func getCommitteeInfo(epoch: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getCommitteeInfo", params: [epoch].filter { $0 != nil })
    }

    public func getValidatorsApy() async throws -> [String: Any] {
        try await callMethod("suix_getValidatorsApy")
    }

    public func getMoveFunction(packageId: String, module: String, function: String) async throws -> [String: Any] {
        try await callMethod("suix_getMoveFunction", params: [packageId, module, function])
    }

    public func getNormalizedMoveModules(packageId: String) async throws -> [String: Any] {
        try await callMethod("suix_getNormalizedMoveModulesByPackage", params: [packageId])
    }

    public func resolveNameServiceAddress(name: String) async throws -> String? {
        let result: [String: Any] = try await callMethod("suix_resolveNameServiceAddress", params: [name])
        return result["address"] as? String
    }

    public func resolveNameServiceNames(address: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_resolveNameServiceNames", params: [address, cursor, limit].filter { $0 != nil })
    }

    public func getProtocolConfig(version: String? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getProtocolConfig", params: [version].filter { $0 != nil })
    }

    public func getDynamicFields(parentId: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getDynamicFields", params: [parentId, cursor, limit].filter { $0 != nil })
    }

    public func getOwnedObjects(owner: String, query: [String: Any] = [:], cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        try await callMethod("suix_getOwnedObjects", params: [owner, query, cursor, limit].filter { $0 != nil })
    }

    private func callMethod(_ method: String, params: [Any?]) async throws -> [String: Any] {
        let filteredParams = params.compactMap { $0 }
        
        // For this implementation, we'll use JSON-RPC over HTTP as fallback
        // A true gRPC implementation would require generated protobuf code
        throw SuiGrpcError.requestFailed("gRPC calls require protobuf definition files. Use JSON-RPC client instead.")
    }
}