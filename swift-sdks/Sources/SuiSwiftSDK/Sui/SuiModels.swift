import Foundation

public struct SuiCoinObject: Codable {
    public let coinType: String?
    public let coinObjectID: String?
    public let version: String?
    public let digest: String?
    public let balance: String?
    public let previousTransaction: String?

    enum CodingKeys: String, CodingKey {
        case coinType
        case coinObjectID = "coinObjectId"
        case version
        case digest
        case balance
        case previousTransaction
    }
}

public struct SuiCoinPage: Codable {
    public let data: [SuiCoinObject]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiBalance: Codable {
    public let coinType: String?
    public let coinObjectCount: Int?
    public let totalBalance: String?
    public let lockedBalance: [String: String]?
}

public struct SuiCheckpointSummary: Codable {
    public let digest: String?
    public let sequenceNumber: String?
    public let epoch: String?
    public let timestampMs: String?

    enum CodingKeys: String, CodingKey {
        case digest
        case sequenceNumber
        case epoch
        case timestampMs
    }
}

public struct SuiCheckpointPage: Codable {
    public let data: [SuiCheckpointSummary]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiNameServiceNamesPage: Codable {
    public let data: [String]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiEpochSummary: Codable {
    public let epoch: String?
    public let validators: [String]?
    public let epochTotalTransactions: String?
    public let firstCheckpointID: String?
    public let endOfEpochInfo: [String: String]?

    enum CodingKeys: String, CodingKey {
        case epoch
        case validators
        case epochTotalTransactions
        case firstCheckpointID = "firstCheckpointId"
        case endOfEpochInfo
    }
}

public struct SuiEpochPage: Codable {
    public let data: [SuiEpochSummary]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiObjectResponse: Codable {
    public let data: SuiJSONObject?
    public let error: SuiJSONObject?
}

public struct SuiObjectPage: Codable {
    public let data: [SuiObjectResponse]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiDynamicFieldPage: Codable {
    public let data: [SuiJSONObject]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiEventPage: Codable {
    public let data: [SuiJSONObject]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiTransactionBlockPage: Codable {
    public let data: [SuiJSONObject]
    public let nextCursor: String?
    public let hasNextPage: Bool
}

public struct SuiEpochMetricsPage: Codable {
    public let data: [SuiJSONObject]
    public let nextCursor: String?
    public let hasNextPage: Bool
}
