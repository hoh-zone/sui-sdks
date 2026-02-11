import Foundation

public enum SuiTypedDecodingError: Error, Equatable {
    case invalidJSONRoot
}

public extension SuiClient {
    func discoverRPCAPITyped() async throws -> SuiJSONObject {
        let raw = try await discoverRPCAPI()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getRPCAPIVersionTyped() async throws -> String? {
        try await getRPCAPIVersion()
    }

    func getRpcApiVersionTyped() async throws -> String? {
        try await getRpcApiVersion()
    }

    func getObjectTyped(objectID: String, options: [String: Any] = [:]) async throws -> SuiObjectResponse {
        let raw = try await getObject(objectID: objectID, options: options)
        return try decodeJSON(raw, as: SuiObjectResponse.self)
    }

    func getMoveFunctionArgTypesTyped(
        package packageID: String,
        module: String,
        function: String
    ) async throws -> [SuiJSONObject] {
        let raw = try await getMoveFunctionArgTypes(package: packageID, module: module, function: function)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func getNormalizedMoveModulesByPackageTyped(package packageID: String) async throws -> SuiJSONObject {
        let raw = try await getNormalizedMoveModulesByPackage(package: packageID)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getNormalizedMoveModuleTyped(package packageID: String, module: String) async throws -> SuiJSONObject {
        let raw = try await getNormalizedMoveModule(package: packageID, module: module)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getNormalizedMoveFunctionTyped(
        package packageID: String,
        module: String,
        function: String
    ) async throws -> SuiJSONObject {
        let raw = try await getNormalizedMoveFunction(package: packageID, module: module, function: function)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getNormalizedMoveStructTyped(
        package packageID: String,
        module: String,
        struct structName: String
    ) async throws -> SuiJSONObject {
        let raw = try await getNormalizedMoveStruct(package: packageID, module: module, struct: structName)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getObjectsTyped(objectIDs: [String], options: [String: Any] = [:]) async throws -> [SuiObjectResponse] {
        let raw = try await getObjects(objectIDs: objectIDs, options: options)
        return try decodeJSON(raw, as: [SuiObjectResponse].self)
    }

    func multiGetObjectsTyped(objectIDs: [String], options: [String: Any] = [:]) async throws -> [SuiObjectResponse] {
        let raw = try await multiGetObjects(objectIDs: objectIDs, options: options)
        return try decodeJSON(raw, as: [SuiObjectResponse].self)
    }

    func getOwnedObjectsTyped(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> SuiObjectPage {
        let raw = try await getOwnedObjects(owner: owner, query: query, cursor: cursor, limit: limit)
        return try decodeJSON(raw, as: SuiObjectPage.self)
    }

    func allOwnedObjectsTyped(
        owner: String,
        query: [String: Any] = [:],
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [SuiObjectResponse] {
        let rawItems = try await allOwnedObjects(owner: owner, query: query, cursor: cursor, limit: limit, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiObjectResponse].self)
    }

    func getDynamicFieldsTyped(
        parentObjectID: String,
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> SuiDynamicFieldPage {
        let raw = try await getDynamicFields(parentObjectID: parentObjectID, cursor: cursor, limit: limit)
        return try decodeJSON(raw, as: SuiDynamicFieldPage.self)
    }

    func allDynamicFieldsTyped(
        parentObjectID: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allDynamicFields(parentObjectID: parentObjectID, cursor: cursor, limit: limit, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func getDynamicFieldObjectTyped(parentObjectID: String, name: [String: Any]) async throws -> SuiObjectResponse {
        let raw = try await getDynamicFieldObject(parentObjectID: parentObjectID, name: name)
        return try decodeJSON(raw, as: SuiObjectResponse.self)
    }

    func getEventsByTransactionTyped(transactionDigest: String) async throws -> [SuiJSONObject] {
        let raw = try await getEventsByTransaction(transactionDigest: transactionDigest)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func getEventsTyped(transactionDigest: String) async throws -> [SuiJSONObject] {
        let raw = try await getEvents(transactionDigest: transactionDigest)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func queryEventsTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> SuiEventPage {
        let raw = try await queryEvents(query: query, cursor: cursor, limit: limit, descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiEventPage.self)
    }

    func queryEventsTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> SuiEventPage {
        let raw = try await queryEvents(query: query, cursor: cursor, limit: limit, order: order)
        return try decodeJSON(raw, as: SuiEventPage.self)
    }

    func allEventsTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allEvents(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: descendingOrder,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func allEventsTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allEvents(
            query: query,
            cursor: cursor,
            limit: limit,
            order: order,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func queryTransactionBlocksTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool = false
    ) async throws -> SuiTransactionBlockPage {
        let raw = try await queryTransactionBlocks(query: query, cursor: cursor, limit: limit, descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiTransactionBlockPage.self)
    }

    func queryTransactionBlocksTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> SuiTransactionBlockPage {
        let raw = try await queryTransactionBlocks(query: query, cursor: cursor, limit: limit, order: order)
        return try decodeJSON(raw, as: SuiTransactionBlockPage.self)
    }

    func allTransactionBlocksTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allTransactionBlocks(
            query: query,
            cursor: cursor,
            limit: limit,
            descendingOrder: descendingOrder,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func allTransactionBlocksTyped(
        query: [String: Any],
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allTransactionBlocks(
            query: query,
            cursor: cursor,
            limit: limit,
            order: order,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func getTransactionBlockTyped(digest: String, options: [String: Any] = [:]) async throws -> SuiJSONObject {
        let raw = try await getTransactionBlock(digest: digest, options: options)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func multiGetTransactionBlocksTyped(
        digests: [String],
        options: [String: Any] = [:]
    ) async throws -> [SuiJSONObject] {
        let raw = try await multiGetTransactionBlocks(digests: digests, options: options)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func tryGetPastObjectTyped(
        objectID: String,
        version: Int,
        options: [String: Any] = [:]
    ) async throws -> SuiJSONObject {
        let raw = try await tryGetPastObject(objectID: objectID, version: version, options: options)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func executeTransactionBlockTyped(
        transactionBlock: String,
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await executeTransactionBlock(
            transactionBlock: transactionBlock,
            signatures: signatures,
            options: options,
            requestType: requestType
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func executeTransactionBlockTyped(
        transactionBlockBytes: [UInt8],
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await executeTransactionBlock(
            transactionBlockBytes: transactionBlockBytes,
            signatures: signatures,
            options: options,
            requestType: requestType
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func executeTransactionBlockTyped(
        transactionBlockBytes: [UInt8],
        signature: String,
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await executeTransactionBlock(
            transactionBlockBytes: transactionBlockBytes,
            signature: signature,
            options: options,
            requestType: requestType
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func executeTransactionBlockTyped(
        transactionBlockData: Data,
        signatures: [String],
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await executeTransactionBlock(
            transactionBlockData: transactionBlockData,
            signatures: signatures,
            options: options,
            requestType: requestType
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func executeTransactionBlockTyped(
        transactionBlockData: Data,
        signature: String,
        options: [String: Any] = [:],
        requestType: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await executeTransactionBlock(
            transactionBlockData: transactionBlockData,
            signature: signature,
            options: options,
            requestType: requestType
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func signAndExecuteTransactionTyped(
        transactionBlockBytes: [UInt8],
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> SuiJSONObject {
        let raw = try await signAndExecuteTransaction(
            transactionBlockBytes: transactionBlockBytes,
            signer: signer,
            options: options,
            requestType: requestType,
            waitForConfirmation: waitForConfirmation,
            waitTimeoutMs: waitTimeoutMs,
            waitPollIntervalMs: waitPollIntervalMs
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func signAndExecuteTransactionTyped(
        transactionBlockBase64: String,
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> SuiJSONObject {
        let raw = try await signAndExecuteTransaction(
            transactionBlockBase64: transactionBlockBase64,
            signer: signer,
            options: options,
            requestType: requestType,
            waitForConfirmation: waitForConfirmation,
            waitTimeoutMs: waitTimeoutMs,
            waitPollIntervalMs: waitPollIntervalMs
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func signAndExecuteTransactionTyped(
        transactionBlockData: Data,
        signer: SuiSigner,
        options: [String: Any] = [:],
        requestType: String? = nil,
        waitForConfirmation: Bool = false,
        waitTimeoutMs: Int = 60_000,
        waitPollIntervalMs: Int = 2_000
    ) async throws -> SuiJSONObject {
        let raw = try await signAndExecuteTransaction(
            transactionBlockData: transactionBlockData,
            signer: signer,
            options: options,
            requestType: requestType,
            waitForConfirmation: waitForConfirmation,
            waitTimeoutMs: waitTimeoutMs,
            waitPollIntervalMs: waitPollIntervalMs
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getCheckpointRawTyped(checkpointID: String) async throws -> SuiJSONObject {
        let raw = try await getCheckpoint(checkpointID: checkpointID)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getLatestCheckpointSequenceNumberTyped() async throws -> String {
        try await getLatestCheckpointSequenceNumber()
    }

    func getEpochMetricsTyped(
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool? = nil
    ) async throws -> SuiEpochMetricsPage {
        let raw = try await getEpochMetrics(cursor: cursor, limit: limit, descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiEpochMetricsPage.self)
    }

    func getEpochMetricsTyped(
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> SuiEpochMetricsPage {
        let raw = try await getEpochMetrics(cursor: cursor, limit: limit, order: order)
        return try decodeJSON(raw, as: SuiEpochMetricsPage.self)
    }

    func allEpochMetricsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool? = nil,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allEpochMetrics(cursor: cursor, limit: limit, descendingOrder: descendingOrder, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func allEpochMetricsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [SuiJSONObject] {
        let rawItems = try await allEpochMetrics(cursor: cursor, limit: limit, order: order, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiJSONObject].self)
    }

    func getLatestSuiSystemStateTyped() async throws -> SuiJSONObject {
        let raw = try await getLatestSuiSystemState()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getPackageTyped(packageID: String) async throws -> SuiObjectResponse {
        let raw = try await getPackage(packageID: packageID)
        return try decodeJSON(raw, as: SuiObjectResponse.self)
    }

    func getCommitteeInfoTyped(epoch: String? = nil) async throws -> SuiJSONObject {
        let raw = try await getCommitteeInfo(epoch: epoch)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getNetworkMetricsTyped() async throws -> SuiJSONObject {
        let raw = try await getNetworkMetrics()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getAddressMetricsTyped() async throws -> SuiJSONObject {
        let raw = try await getAddressMetrics()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getCoinMetadataTyped(coinType: String) async throws -> SuiJSONObject? {
        let raw = try await getCoinMetadata(coinType: coinType)
        guard let raw else { return nil }
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getTotalSupplyTyped(coinType: String) async throws -> SuiJSONObject {
        let raw = try await getTotalSupply(coinType: coinType)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getAllEpochAddressMetricsTyped(descendingOrder: Bool? = nil) async throws -> SuiJSONObject {
        let raw = try await getAllEpochAddressMetrics(descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getMoveCallMetricsTyped() async throws -> SuiJSONObject {
        let raw = try await getMoveCallMetrics()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getCurrentEpochTyped() async throws -> SuiJSONObject {
        let raw = try await getCurrentEpoch()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getValidatorsApyTyped() async throws -> SuiJSONObject {
        let raw = try await getValidatorsApy()
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getChainIdentifierTyped() async throws -> String {
        try await getChainIdentifier()
    }

    func getStakesTyped(owner: String) async throws -> [SuiJSONObject] {
        let raw = try await getStakes(owner: owner)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func getStakesByIDsTyped(stakedSuiIDs: [String]) async throws -> [SuiJSONObject] {
        let raw = try await getStakesByIDs(stakedSuiIDs: stakedSuiIDs)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func getStakesByIdsTyped(stakedSuiIDs: [String]) async throws -> [SuiJSONObject] {
        let raw = try await getStakesByIds(stakedSuiIDs: stakedSuiIDs)
        return try decodeJSON(raw, as: [SuiJSONObject].self)
    }

    func getProtocolConfigTyped(version: String? = nil) async throws -> SuiJSONObject {
        let raw = try await getProtocolConfig(version: version)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func resolveNameServiceAddressTyped(name: String) async throws -> String? {
        try await resolveNameServiceAddress(name: name)
    }

    func verifyZkLoginSignatureTyped(
        bytes: String,
        signature: String,
        intentScope: String,
        author: String
    ) async throws -> SuiJSONObject {
        let raw = try await verifyZkLoginSignature(
            bytes: bytes,
            signature: signature,
            intentScope: intentScope,
            author: author
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func devInspectTransactionBlockTyped(
        sender: String,
        transactionBlock: String,
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await devInspectTransactionBlock(
            sender: sender,
            transactionBlock: transactionBlock,
            gasPrice: gasPrice,
            epoch: epoch
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func devInspectTransactionBlockTyped(
        sender: String,
        transactionBlockBytes: [UInt8],
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await devInspectTransactionBlock(
            sender: sender,
            transactionBlockBytes: transactionBlockBytes,
            gasPrice: gasPrice,
            epoch: epoch
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func devInspectTransactionBlockTyped(
        sender: String,
        transactionBlockData: Data,
        gasPrice: String? = nil,
        epoch: String? = nil
    ) async throws -> SuiJSONObject {
        let raw = try await devInspectTransactionBlock(
            sender: sender,
            transactionBlockData: transactionBlockData,
            gasPrice: gasPrice,
            epoch: epoch
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func dryRunTransactionBlockTyped(txBytesBase64: String) async throws -> SuiJSONObject {
        let raw = try await dryRunTransactionBlock(txBytesBase64: txBytesBase64)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func dryRunTransactionBlockTyped(transactionBlockBytes: [UInt8]) async throws -> SuiJSONObject {
        let raw = try await dryRunTransactionBlock(transactionBlockBytes: transactionBlockBytes)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func dryRunTransactionBlockTyped(transactionBlockData: Data) async throws -> SuiJSONObject {
        let raw = try await dryRunTransactionBlock(transactionBlockData: transactionBlockData)
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getReferenceGasPriceTyped() async throws -> String {
        try await getReferenceGasPrice()
    }

    func getTotalTransactionBlocksTyped() async throws -> String {
        try await getTotalTransactionBlocks()
    }

    func waitForTransactionTyped(
        digest: String,
        options: [String: Any] = [:],
        timeoutMs: Int = 60_000,
        pollIntervalMs: Int = 2_000
    ) async throws -> SuiJSONObject {
        let raw = try await waitForTransaction(
            digest: digest,
            options: options,
            timeoutMs: timeoutMs,
            pollIntervalMs: pollIntervalMs
        )
        return try decodeJSON(raw, as: SuiJSONObject.self)
    }

    func getAllCoinsTyped(owner: String, cursor: String? = nil, limit: Int? = nil) async throws -> SuiCoinPage {
        let raw = try await getAllCoins(owner: owner, cursor: cursor, limit: limit)
        return try decodeJSON(raw, as: SuiCoinPage.self)
    }

    func getCoinsTyped(
        owner: String,
        coinType: String? = nil,
        cursor: String? = nil,
        limit: Int? = nil
    ) async throws -> SuiCoinPage {
        let raw = try await getCoins(owner: owner, coinType: coinType, cursor: cursor, limit: limit)
        return try decodeJSON(raw, as: SuiCoinPage.self)
    }

    func allCoinsTyped(
        owner: String,
        cursor: String? = nil,
        limit: Int = 100,
        maxItems: Int? = nil
    ) async throws -> [SuiCoinObject] {
        let rawItems = try await allCoins(owner: owner, cursor: cursor, limit: limit, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiCoinObject].self)
    }

    func getBalanceTyped(owner: String, coinType: String? = nil) async throws -> SuiBalance {
        let raw = try await getBalance(owner: owner, coinType: coinType)
        return try decodeJSON(raw, as: SuiBalance.self)
    }

    func getAllBalancesTyped(owner: String) async throws -> [SuiBalance] {
        let raw = try await getAllBalances(owner: owner)
        return try decodeJSON(raw, as: [SuiBalance].self)
    }

    func getCheckpointsTyped(cursor: String? = nil, limit: Int? = nil, descendingOrder: Bool = false) async throws -> SuiCheckpointPage {
        let raw = try await getCheckpoints(cursor: cursor, limit: limit, descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiCheckpointPage.self)
    }

    func getCheckpointsTyped(cursor: String? = nil, limit: Int? = nil, order: SuiOrder) async throws -> SuiCheckpointPage {
        let raw = try await getCheckpoints(cursor: cursor, limit: limit, order: order)
        return try decodeJSON(raw, as: SuiCheckpointPage.self)
    }

    func allCheckpointsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool = false,
        maxItems: Int? = nil
    ) async throws -> [SuiCheckpointSummary] {
        let rawItems = try await allCheckpoints(
            cursor: cursor,
            limit: limit,
            descendingOrder: descendingOrder,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiCheckpointSummary].self)
    }

    func allCheckpointsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [SuiCheckpointSummary] {
        let rawItems = try await allCheckpoints(
            cursor: cursor,
            limit: limit,
            order: order,
            maxItems: maxItems
        )
        return try decodeJSON(rawItems, as: [SuiCheckpointSummary].self)
    }

    func getCheckpointTyped(checkpointID: String) async throws -> SuiCheckpointSummary {
        let raw = try await getCheckpoint(checkpointID: checkpointID)
        return try decodeJSON(raw, as: SuiCheckpointSummary.self)
    }

    func resolveNameServiceNamesTyped(
        address: String,
        cursor: String? = nil,
        limit: Int? = nil,
        format: String = "dot"
    ) async throws -> SuiNameServiceNamesPage {
        let raw = try await resolveNameServiceNames(address: address, cursor: cursor, limit: limit, format: format)
        return try decodeJSON(raw, as: SuiNameServiceNamesPage.self)
    }

    func getEpochsTyped(
        cursor: String? = nil,
        limit: Int? = nil,
        descendingOrder: Bool? = nil
    ) async throws -> SuiEpochPage {
        let raw = try await getEpochs(cursor: cursor, limit: limit, descendingOrder: descendingOrder)
        return try decodeJSON(raw, as: SuiEpochPage.self)
    }

    func getEpochsTyped(
        cursor: String? = nil,
        limit: Int? = nil,
        order: SuiOrder
    ) async throws -> SuiEpochPage {
        let raw = try await getEpochs(cursor: cursor, limit: limit, order: order)
        return try decodeJSON(raw, as: SuiEpochPage.self)
    }

    func allEpochsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        descendingOrder: Bool? = nil,
        maxItems: Int? = nil
    ) async throws -> [SuiEpochSummary] {
        let rawItems = try await allEpochs(cursor: cursor, limit: limit, descendingOrder: descendingOrder, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiEpochSummary].self)
    }

    func allEpochsTyped(
        cursor: String? = nil,
        limit: Int = 100,
        order: SuiOrder,
        maxItems: Int? = nil
    ) async throws -> [SuiEpochSummary] {
        let rawItems = try await allEpochs(cursor: cursor, limit: limit, order: order, maxItems: maxItems)
        return try decodeJSON(rawItems, as: [SuiEpochSummary].self)
    }
}

private func decodeJSON<T: Decodable>(_ object: Any, as type: T.Type) throws -> T {
    guard JSONSerialization.isValidJSONObject(object) else {
        throw SuiTypedDecodingError.invalidJSONRoot
    }
    let data = try JSONSerialization.data(withJSONObject: object)
    let decoder = JSONDecoder()
    return try decoder.decode(T.self, from: data)
}
