import Foundation

public enum SuiClientWaitError: Error, Equatable {
    case timeout
    case missingTransactionDigest
}

public final class SuiClient {
    public let network: SuiNetwork?
    public let endpoint: URL
    private let transport: JSONRPCTransport

    public init(endpoint: URL, transport: JSONRPCTransport? = nil) {
        self.network = nil
        self.endpoint = endpoint
        self.transport = transport ?? HTTPJSONRPCTransport(url: endpoint)
    }

    public init(network: SuiNetwork, transport: JSONRPCTransport? = nil) {
        self.network = network
        self.endpoint = network.fullnodeURL
        self.transport = transport ?? HTTPJSONRPCTransport(url: network.fullnodeURL)
    }

    @discardableResult
    public func call(method: String, params: [Any?] = []) async throws -> Any {
        try await transport.request(method: method, params: params)
    }

    public func getRPCAPIVersion() async throws -> String? {
        let discovered = try await discoverRPCAPI()
        let info = discovered["info"] as? [String: Any]
        return info?["version"] as? String
    }

    public func getRpcApiVersion() async throws -> String? {
        try await getRPCAPIVersion()
    }

    public func discoverRPCAPI() async throws -> [String: Any] {
        let result = try await call(method: "rpc.discover")
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "rpc.discover result type mismatch")
        }
        return value
    }

    public func getObject(objectID: String, options: [String: Any] = [:]) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(objectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "sui_getObject", params: [objectID, options])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getObject result type mismatch")
        }
        return value
    }

    public func getObjects(objectIDs: [String], options: [String: Any] = [:]) async throws -> [[String: Any]] {
        var out: [[String: Any]] = []
        out.reserveCapacity(objectIDs.count)
        for objectID in objectIDs {
            out.append(try await getObject(objectID: objectID, options: options))
        }
        return out
    }

    public func multiGetObjects(objectIDs: [String], options: [String: Any] = [:]) async throws -> [[String: Any]] {
        for objectID in objectIDs {
            if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(objectID)) {
                throw SuiValidationError.invalidSuiObjectID
            }
        }
        if Set(objectIDs).count != objectIDs.count {
            throw SuiValidationError.duplicateObjectIDs
        }
        let result = try await call(method: "sui_multiGetObjects", params: [objectIDs, options])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "sui_multiGetObjects result type mismatch")
        }
        return value
    }

    public func dryRunTransactionBlock(txBytesBase64: String) async throws -> [String: Any] {
        let result = try await call(method: "sui_dryRunTransactionBlock", params: [txBytesBase64])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_dryRunTransactionBlock result type mismatch")
        }
        return value
    }

    public func dryRunTransactionBlock(transactionBlockBytes: [UInt8]) async throws -> [String: Any] {
        let txBase64 = Data(transactionBlockBytes).base64EncodedString()
        return try await dryRunTransactionBlock(txBytesBase64: txBase64)
    }

    public func dryRunTransactionBlock(transactionBlockData: Data) async throws -> [String: Any] {
        return try await dryRunTransactionBlock(transactionBlockBytes: [UInt8](transactionBlockData))
    }

    public func devInspectTransactionBlock(
        sender: String,
        transactionBlock: String,
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(sender)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(
            method: "sui_devInspectTransactionBlock",
            params: [sender, transactionBlock, gasPrice, epoch]
        )
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_devInspectTransactionBlock result type mismatch")
        }
        return value
    }

    public func devInspectTransactionBlock(
        sender: String,
        transactionBlockBytes: [UInt8],
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> [String: Any] {
        let txBase64 = Data(transactionBlockBytes).base64EncodedString()
        return try await devInspectTransactionBlock(
            sender: sender,
            transactionBlock: txBase64,
            gasPrice: gasPrice,
            epoch: epoch
        )
    }

    public func devInspectTransactionBlock(
        sender: String,
        transactionBlockData: Data,
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> [String: Any] {
        return try await devInspectTransactionBlock(
            sender: sender,
            transactionBlockBytes: [UInt8](transactionBlockData),
            gasPrice: gasPrice,
            epoch: epoch
        )
    }

    public func getCoins(
        owner: String,
        coinType: String? = nil,
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getCoins", params: [owner, coinType, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getCoins result type mismatch")
        }
        return value
    }

    public func getAllCoins(owner: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getAllCoins", params: [owner, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getAllCoins result type mismatch")
        }
        return value
    }

    public func getBalance(owner: String, coinType: String? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getBalance", params: [owner, coinType])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getBalance result type mismatch")
        }
        return value
    }

    public func getAllBalances(owner: String) async throws -> [[String: Any]] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getAllBalances", params: [owner])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getAllBalances result type mismatch")
        }
        return value
    }

    public func getCoinMetadata(coinType: String) async throws -> [String: Any]? {
        let result = try await call(method: "suix_getCoinMetadata", params: [coinType])
        if result is NSNull {
            return nil
        }
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getCoinMetadata result type mismatch")
        }
        return value
    }

    public func getTotalSupply(coinType: String) async throws -> [String: Any] {
        let result = try await call(method: "suix_getTotalSupply", params: [coinType])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getTotalSupply result type mismatch")
        }
        return value
    }

    public func getMoveFunctionArgTypes(
        package packageID: String,
        module: String,
        function: String
    ) async throws -> [[String: Any]] {
        let result = try await call(method: "sui_getMoveFunctionArgTypes", params: [packageID, module, function])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getMoveFunctionArgTypes result type mismatch")
        }
        return value
    }

    public func getNormalizedMoveModulesByPackage(package packageID: String) async throws -> [String: Any] {
        let result = try await call(method: "sui_getNormalizedMoveModulesByPackage", params: [packageID])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getNormalizedMoveModulesByPackage result type mismatch")
        }
        return value
    }

    public func getNormalizedMoveModule(package packageID: String, module: String) async throws -> [String: Any] {
        let result = try await call(method: "sui_getNormalizedMoveModule", params: [packageID, module])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getNormalizedMoveModule result type mismatch")
        }
        return value
    }

    public func getNormalizedMoveFunction(
        package packageID: String,
        module: String,
        function: String
    ) async throws -> [String: Any] {
        let result = try await call(method: "sui_getNormalizedMoveFunction", params: [packageID, module, function])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getNormalizedMoveFunction result type mismatch")
        }
        return value
    }

    public func getNormalizedMoveStruct(
        package packageID: String,
        module: String,
        `struct`: String
    ) async throws -> [String: Any] {
        let result = try await call(method: "sui_getNormalizedMoveStruct", params: [packageID, module, `struct`])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getNormalizedMoveStruct result type mismatch")
        }
        return value
    }

    public func getOwnedObjects(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getOwnedObjects", params: [owner, query, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getOwnedObjects result type mismatch")
        }
        return value
    }

    public func getDynamicFields(parentObjectID: String, cursor: String? = nil, limit: Int? = nil) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(parentObjectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "suix_getDynamicFields", params: [parentObjectID, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getDynamicFields result type mismatch")
        }
        return value
    }

    public func getDynamicFieldObject(parentObjectID: String, name: [String: Any]) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(parentObjectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "suix_getDynamicFieldObject", params: [parentObjectID, name])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getDynamicFieldObject result type mismatch")
        }
        return value
    }

    public func getEventsByTransaction(transactionDigest: String) async throws -> [[String: Any]] {
        if !SuiTypes.isValidTransactionDigest(transactionDigest) {
            throw SuiValidationError.invalidTransactionDigest
        }
        let result = try await call(method: "sui_getEvents", params: [transactionDigest])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getEvents result type mismatch")
        }
        return value
    }

    public func getEvents(transactionDigest: String) async throws -> [[String: Any]] {
        try await getEventsByTransaction(transactionDigest: transactionDigest)
    }

    public func queryEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_queryEvents", params: [query, cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_queryEvents result type mismatch")
        }
        return value
    }

    public func queryEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> [String: Any] {
        return try await queryEvents(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending
        )
    }

    public func queryTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_queryTransactionBlocks", params: [query, cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_queryTransactionBlocks result type mismatch")
        }
        return value
    }

    public func queryTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> [String: Any] {
        return try await queryTransactionBlocks(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending
        )
    }

    public func getCheckpoints(cursor: String? = nil, limit: Int? = nil, descendingOrder: Bool = false) async throws -> [String: Any] {
        let result = try await call(method: "sui_getCheckpoints", params: [cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getCheckpoints result type mismatch")
        }
        return value
    }

    public func getCheckpoints(cursor: String? = nil, limit: Int? = nil, order: SuiOrder) async throws -> [String: Any] {
        return try await getCheckpoints(cursor: cursor, limit: limit, descendingOrder: order.isDescending)
    }

    public func getTransactionBlock(digest: String, options: [String: Any] = [:]) async throws -> [String: Any] {
        if !SuiTypes.isValidTransactionDigest(digest) {
            throw SuiValidationError.invalidTransactionDigest
        }
        let result = try await call(method: "sui_getTransactionBlock", params: [digest, options])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getTransactionBlock result type mismatch")
        }
        return value
    }

    public func multiGetTransactionBlocks(
        digests: [String],
        options: [String: Any] = [:]
    ) async throws -> [[String: Any]] {
        for digest in digests {
            if !SuiTypes.isValidTransactionDigest(digest) {
                throw SuiValidationError.invalidTransactionDigest
            }
        }
        if Set(digests).count != digests.count {
            throw SuiValidationError.duplicateTransactionDigests
        }
        let result = try await call(method: "sui_multiGetTransactionBlocks", params: [digests, options])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "sui_multiGetTransactionBlocks result type mismatch")
        }
        return value
    }

    public func tryGetPastObject(
        objectID: String,
        version: Int,
        options: [String: Any] = [:]
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(objectID)) {
            throw SuiValidationError.invalidSuiObjectID
        }
        let result = try await call(method: "sui_tryGetPastObject", params: [objectID, version, options])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_tryGetPastObject result type mismatch")
        }
        return value
    }

    public func executeTransactionBlock(
        transactionBlock: String,
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        let result = try await call(
            method: "sui_executeTransactionBlock",
            params: [transactionBlock, signatures, options, requestType]
        )
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_executeTransactionBlock result type mismatch")
        }
        return value
    }

    public func executeTransactionBlock(
        transactionBlock: String,
        signature: String,
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        try await executeTransactionBlock(
            transactionBlock: transactionBlock,
            signatures: [signature],
            options: options,
            requestType: requestType
        )
    }

    public func executeTransactionBlock(
        transactionBlockBytes: [UInt8],
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        let txBase64 = Data(transactionBlockBytes).base64EncodedString()
        return try await executeTransactionBlock(
            transactionBlock: txBase64,
            signatures: signatures,
            options: options,
            requestType: requestType
        )
    }

    public func executeTransactionBlock(
        transactionBlockBytes: [UInt8],
        signature: String,
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        return try await executeTransactionBlock(
            transactionBlockBytes: transactionBlockBytes,
            signatures: [signature],
            options: options,
            requestType: requestType
        )
    }

    public func executeTransactionBlock(
        transactionBlockData: Data,
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        return try await executeTransactionBlock(
            transactionBlockBytes: [UInt8](transactionBlockData),
            signatures: signatures,
            options: options,
            requestType: requestType
        )
    }

    public func executeTransactionBlock(
        transactionBlockData: Data,
        signature: String,
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> [String: Any] {
        return try await executeTransactionBlock(
            transactionBlockData: transactionBlockData,
            signatures: [signature],
            options: options,
            requestType: requestType
        )
    }

    public func signAndExecuteTransaction(
        transactionBlockBytes: [UInt8],
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> [String: Any] {
        let signed = try signer.signTransaction(bytes: transactionBlockBytes)
        let executed = try await executeTransactionBlock(
            transactionBlock: Data(transactionBlockBytes).base64EncodedString(),
            signatures: [signed.signature],
            options: options,
            requestType: requestType
        )

        if !waitForConfirmation {
            return executed
        }

        guard let digest = transactionDigest(fromExecuteResponse: executed) else {
            throw SuiClientWaitError.missingTransactionDigest
        }

        return try await waitForTransaction(
            digest: digest,
            options: options,
            timeoutMs: waitTimeoutMs,
            pollIntervalMs: waitPollIntervalMs
        )
    }

    public func signAndExecuteTransaction(
        transactionBlockBase64: String,
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> [String: Any] {
        guard let txBytes = Data(base64Encoded: transactionBlockBase64) else {
            throw JSONRPCMalformedResponseError(reason: "transactionBlockBase64 is not valid base64")
        }
        return try await signAndExecuteTransaction(
            transactionBlockBytes: [UInt8](txBytes),
            signer: signer,
            options: options,
            requestType: requestType,
            waitForConfirmation: waitForConfirmation,
            waitTimeoutMs: waitTimeoutMs,
            waitPollIntervalMs: waitPollIntervalMs
        )
    }

    public func signAndExecuteTransaction(
        transactionBlockData: Data,
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> [String: Any] {
        return try await signAndExecuteTransaction(
            transactionBlockBytes: [UInt8](transactionBlockData),
            signer: signer,
            options: options,
            requestType: requestType,
            waitForConfirmation: waitForConfirmation,
            waitTimeoutMs: waitTimeoutMs,
            waitPollIntervalMs: waitPollIntervalMs
        )
    }

    public func getLatestSuiSystemState() async throws -> [String: Any] {
        let result = try await call(method: "suix_getLatestSuiSystemState")
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getLatestSuiSystemState result type mismatch")
        }
        return value
    }

    public func getLatestCheckpointSequenceNumber() async throws -> String {
        let result = try await call(method: "sui_getLatestCheckpointSequenceNumber")
        guard let value = result as? String else {
            throw JSONRPCMalformedResponseError(reason: "sui_getLatestCheckpointSequenceNumber result type mismatch")
        }
        return value
    }

    public func getCheckpoint(checkpointID: String) async throws -> [String: Any] {
        let result = try await call(method: "sui_getCheckpoint", params: [checkpointID])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getCheckpoint result type mismatch")
        }
        return value
    }

    public func getCommitteeInfo(epoch: String? = nil) async throws -> [String: Any] {
        let result = try await call(method: "suix_getCommitteeInfo", params: [epoch])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getCommitteeInfo result type mismatch")
        }
        return value
    }

    public func getNetworkMetrics() async throws -> [String: Any] {
        let result = try await call(method: "suix_getNetworkMetrics", params: [])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getNetworkMetrics result type mismatch")
        }
        return value
    }

    public func getAddressMetrics() async throws -> [String: Any] {
        let result = try await call(method: "suix_getLatestAddressMetrics", params: [])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getLatestAddressMetrics result type mismatch")
        }
        return value
    }

    public func getEpochMetrics(
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool? = nil
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_getEpochMetrics", params: [cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getEpochMetrics result type mismatch")
        }
        return value
    }

    public func getEpochMetrics(
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> [String: Any] {
        return try await getEpochMetrics(cursor: cursor, limit: limit, descendingOrder: order.isDescending)
    }

    public func getAllEpochAddressMetrics(descendingOrder: Bool? = nil) async throws -> [String: Any] {
        let result = try await call(method: "suix_getAllEpochAddressMetrics", params: [descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getAllEpochAddressMetrics result type mismatch")
        }
        return value
    }

    public func getEpochs(
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool? = nil
    ) async throws -> [String: Any] {
        let result = try await call(method: "suix_getEpochs", params: [cursor, limit, descendingOrder])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getEpochs result type mismatch")
        }
        return value
    }

    public func getEpochs(
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> [String: Any] {
        return try await getEpochs(cursor: cursor, limit: limit, descendingOrder: order.isDescending)
    }

    public func getMoveCallMetrics() async throws -> [String: Any] {
        let result = try await call(method: "suix_getMoveCallMetrics", params: [])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getMoveCallMetrics result type mismatch")
        }
        return value
    }

    public func getCurrentEpoch() async throws -> [String: Any] {
        let result = try await call(method: "suix_getCurrentEpoch", params: [])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getCurrentEpoch result type mismatch")
        }
        return value
    }

    public func getTotalTransactionBlocks() async throws -> String {
        let result = try await call(method: "sui_getTotalTransactionBlocks")
        guard let value = result as? String else {
            throw JSONRPCMalformedResponseError(reason: "sui_getTotalTransactionBlocks result type mismatch")
        }
        return value
    }

    public func getStakes(owner: String) async throws -> [[String: Any]] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(owner)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_getStakes", params: [owner])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getStakes result type mismatch")
        }
        return value
    }

    public func getStakesByIDs(stakedSuiIDs: [String]) async throws -> [[String: Any]] {
        for id in stakedSuiIDs {
            if !SuiTypes.isValidSuiObjectID(SuiTypes.normalizeSuiObjectID(id)) {
                throw SuiValidationError.invalidSuiObjectID
            }
        }
        let result = try await call(method: "suix_getStakesByIds", params: [stakedSuiIDs])
        guard let value = result as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getStakesByIds result type mismatch")
        }
        return value
    }

    public func getStakesByIds(stakedSuiIDs: [String]) async throws -> [[String: Any]] {
        try await getStakesByIDs(stakedSuiIDs: stakedSuiIDs)
    }

    public func getValidatorsApy() async throws -> [String: Any] {
        let result = try await call(method: "suix_getValidatorsApy")
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_getValidatorsApy result type mismatch")
        }
        return value
    }

    public func getChainIdentifier() async throws -> String {
        do {
            let result = try await call(method: "sui_getChainIdentifier")
            guard let value = result as? String else {
                throw JSONRPCMalformedResponseError(reason: "sui_getChainIdentifier result type mismatch")
            }
            return value
        } catch {
            // Fallback for older RPCs: derive from checkpoint-0 digest (TS SDK compatibility path).
            let checkpoint = try await getCheckpoint(checkpointID: "0")
            guard let digest = checkpoint["digest"] as? String else {
                throw JSONRPCMalformedResponseError(reason: "sui_getCheckpoint missing digest for chain identifier fallback")
            }
            guard let bytes = Base58.decode(digest), bytes.count >= 4 else {
                throw JSONRPCMalformedResponseError(reason: "invalid checkpoint digest for chain identifier fallback")
            }
            return bytes.prefix(4).map { String(format: "%02x", $0) }.joined()
        }
    }

    public func resolveNameServiceAddress(name: String) async throws -> String? {
        let result = try await call(method: "suix_resolveNameServiceAddress", params: [name])
        if result is NSNull {
            return nil
        }
        guard let value = result as? String else {
            throw JSONRPCMalformedResponseError(reason: "suix_resolveNameServiceAddress result type mismatch")
        }
        return value
    }

    public func resolveNameServiceNames(
        address: String,
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> [String: Any] {
        if !SuiTypes.isValidSuiAddress(SuiTypes.normalizeSuiAddress(address)) {
            throw SuiValidationError.invalidSuiAddress
        }
        let result = try await call(method: "suix_resolveNameServiceNames", params: [address, cursor, limit])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "suix_resolveNameServiceNames result type mismatch")
        }
        return value
    }

    public func resolveNameServiceNames(
        address: String,
        cursor: String? = nil,
        limit: Int? = nil,
        format: String
    ) async throws -> [String: Any] {
        var result = try await resolveNameServiceNames(address: address, cursor: cursor, limit: limit)
        guard var data = result["data"] as? [String] else {
            return result
        }
        data = data.map { normalizeSuiNSName($0, format: format) }
        result["data"] = data
        return result
    }

    public func getProtocolConfig(version: String? = nil) async throws -> [String: Any] {
        let result = try await call(method: "sui_getProtocolConfig", params: [version])
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_getProtocolConfig result type mismatch")
        }
        return value
    }

    public func verifyZkLoginSignature(
        bytes: String,
        signature: String,
        intentScope: String,
        author: String
    ) async throws -> [String: Any] {
        let result = try await call(
            method: "sui_verifyZkLoginSignature",
            params: [bytes, signature, intentScope, author]
        )
        guard let value = result as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "sui_verifyZkLoginSignature result type mismatch")
        }
        return value
    }

    public func getPackage(packageID: String) async throws -> [String: Any] {
        try await getObject(
            objectID: packageID,
            options: [
                "showType": true,
                "showOwner": true,
                "showPreviousTransaction": true,
                "showDisplay": false,
                "showContent": true,
                "showBcs": true,
                "showStorageRebate": true,
            ]
        )
    }

    public func waitForTransaction(
        digest: String,
        options: [String: Any] = [:],
        timeoutMs: Int = 60_000,
        pollIntervalMs: Int = 2_000
    ) async throws -> [String: Any] {
        let deadline = Date().addingTimeInterval(Double(timeoutMs) / 1000.0)
        while Date() < deadline {
            do {
                return try await getTransactionBlock(digest: digest, options: options)
            } catch {
                try await Task.sleep(nanoseconds: UInt64(max(1, pollIntervalMs)) * 1_000_000)
            }
        }
        throw SuiClientWaitError.timeout
    }

    private func transactionDigest(fromExecuteResponse response: [String: Any]) -> String? {
        if let digest = response["digest"] as? String {
            return digest
        }
        if let effects = response["effects"] as? [String: Any],
           let digest = effects["transactionDigest"] as? String {
            return digest
        }
        return nil
    }

    private func normalizeSuiNSName(_ name: String, format: String) -> String {
        let lower = name.lowercased()
        switch format {
        case "dot":
            if lower.contains("@") {
                let parts = lower.split(separator: "@", maxSplits: 1).map(String.init)
                if parts.count == 2 {
                    let labels = parts[0]
                    let domain = parts[1]
                    return labels.isEmpty ? "\(domain).sui" : "\(labels).\(domain).sui"
                }
            }
            return lower
        case "at":
            if lower.hasSuffix(".sui") {
                let base = String(lower.dropLast(4))
                let parts = base.split(separator: ".").map(String.init)
                if parts.count == 1 {
                    return "@\(parts[0])"
                }
                if parts.count > 1 {
                    return "\(parts.dropLast().joined(separator: "."))@\(parts.last ?? "")"
                }
            }
            return lower
        default:
            return lower
        }
    }

    public func allCoins(
        owner: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getAllCoins(owner: owner, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allOwnedObjects(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getOwnedObjects(owner: owner, query: query, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allDynamicFields(
        parentObjectID: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getDynamicFields(parentObjectID: parentObjectID, cursor: pageCursor, limit: limit)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.queryEvents(query: query, cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allEvents(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        return try await allEvents(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending,
            maxItems: maxItems
        )
    }

    public func allTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.queryTransactionBlocks(
                    query: query,
                    cursor: pageCursor,
                    limit: limit,
                    descendingOrder: descendingOrder
                )
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allTransactionBlocks(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        return try await allTransactionBlocks(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending,
            maxItems: maxItems
        )
    }

    public func allCheckpoints(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getCheckpoints(cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allCheckpoints(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        return try await allCheckpoints(
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending,
            maxItems: maxItems
        )
    }

    public func allEpochMetrics(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool? = nil,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getEpochMetrics(cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allEpochMetrics(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        return try await allEpochMetrics(
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending,
            maxItems: maxItems
        )
    }

    public func allEpochs(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool? = nil,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        try await SuiPagination.collectItems(
            fetchPage: { pageCursor in
                try await self.getEpochs(cursor: pageCursor, limit: limit, descendingOrder: descendingOrder)
            },
            startCursor: cursor,
            maxItems: maxItems
        )
    }

    public func allEpochs(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        return try await allEpochs(
            cursor: cursor,
            limit: limit,
            descendingOrder: order.isDescending,
            maxItems: maxItems
        )
    }

    public func getReferenceGasPrice() async throws -> String {
        let result = try await call(method: "suix_getReferenceGasPrice")
        guard let value = result as? String else {
            throw JSONRPCMalformedResponseError(reason: "suix_getReferenceGasPrice result type mismatch")
        }
        return value
    }
}
