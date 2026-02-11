import Foundation

public enum TransactionCommandType: String {
    case moveCall
    case transferObjects
    case splitCoins
    case mergeCoins
    case publish
    case upgrade
    case makeMoveVec
    case transferSui
    case stakeCoin
    case unstakeCoin
    case requestAddStake
    case requestWithdrawStake
}

public enum UpgradePolicy: Int {
    case compatible = 0
    case additive = 1
    case depOnly = 2
}

public enum SelfMatchingOptions: Int {
    case allow = 0
    case cancelmaker = 1
    case cancellaker = 2
}

public enum OrderType: Int {
    case noRestriction = 0
    case immediateOrCancel = 1
    case fillOrKill = 2
    case postOnly = 3
}

public enum IntentScope: Int {
    case transactionData = 0
    case personalMessage = 1
    case transaction = 3
}

public final class Transaction {
    public private(set) var commands: [[String: Any]] = []
    public private(set) var sender: String?
    public private(set) var gasPrice: UInt64?
    public private(set) var gasData: GasData?

    public init() {}

    public static func fromBytes(_ bytes: [UInt8]) -> Transaction {
        Transaction()
    }

    public static func fromData(_ data: Data) -> Transaction {
        Transaction()
    }

    public func setSender(_ address: String) -> Self {
        self.sender = address
        return self
    }

    public func setGasPrice(_ price: UInt64) -> Self {
        self.gasPrice = price
        return self
    }

    public func setGasData(_ gasData: GasData) -> Self {
        self.gasData = gasData
        return self
    }

    public func addCommand(_ command: [String: Any]) -> Self {
        commands.append(command)
        return self
    }

    public func moveCall(_ target: String, arguments: [Any] = [], typeArguments: [String] = []) -> Self {
        let command: [String: Any] = [
            "MoveCall": [
                "target": target,
                "arguments": arguments,
                "typeArguments": typeArguments
            ]
        ]
        return addCommand(command)
    }

    public func transferObjects(_ objects: [Any], to address: String) -> Self {
        let command: [String: Any] = [
            "TransferObjects": [
                "objects": objects,
                "address": address
            ]
        ]
        return addCommand(command)
    }

    public func splitCoins(_ coin: Any, amounts: [UInt64]) -> Self {
        let command: [String: Any] = [
            "SplitCoins": [
                "coin": coin,
                "amounts": amounts.map { ["address": "0x2::sui::SUI", "value": String($0)] }
            ]
        ]
        return addCommand(command)
    }

    public func mergeCoins(_ destination: Any, sources: [Any]) -> Self {
        let command: [String: Any] = [
            "MergeCoins": [
                "destination": destination,
                "sources": sources
            ]
        ]
        return addCommand(command)
    }

    public func publish(_ modules: [String], dependencies: [String]? = nil, upgradeCap: String? = nil) -> Self {
        var command: [String: Any] = [
            "Publish": [
                "modules": modules
            ]
        ]
        if let deps = dependencies {
            command["Publish"] = deps
        }
        if let cap = upgradeCap {
            command["Publish"] = cap
        }
        return addCommand(command)
    }

    public func upgrade(
        _ packageId: String,
        modules: [String],
        dependencies: [String]? = nil,
        ticket: String? = nil,
        policy: UpgradePolicy? = nil
    ) -> Self {
        let command: [String: Any] = [
            "Upgrade": [
                "package": packageId,
                "modules": modules,
                "dependencies": dependencies ?? [],
                "ticket": ticket ?? "",
                "policy": policy?.rawValue ?? 0
            ]
        ]
        return addCommand(command)
    }

    public func makeMoveVec(_ type: String, objects: [Any]) -> Self {
        let command: [String: Any] = [
            "MakeMoveVec": [
                "type": type,
                "objects": objects
            ]
        ]
        return addCommand(command)
    }

    public func transferSui(_ amount: UInt64, to address: String) -> Self {
        let command: [String: Any] = [
            "TransferSui": [
                "amount": amount,
                "address": address
            ]
        ]
        return addCommand(command)
    }

    public func stakeCoin(_ coin: String, validator: String) -> Self {
        let command: [String: Any] = [
            "StakeCoin": [
                "coin": coin,
                "validator": validator
            ]
        ]
        return addCommand(command)
    }

    public func unstakeCoin(_ coin: String) -> Self {
        let command: [String: Any] = [
            "UnstakeCoin": [
                "coin": coin
            ]
        ]
        return addCommand(command)
    }

    public func requestAddStake(_ coin: String, validator: String) -> Self {
        let command: [String: Any] = [
            "RequestAddStake": [
                "coin": coin,
                "validator": validator
            ]
        ]
        return addCommand(command)
    }

    public func requestWithdrawStake(_ coin: String) -> Self {
        let command: [String: Any] = [
            "RequestWithdrawStake": [
                "coin": coin
            ]
        ]
        return addCommand(command)
    }

    private var commandIndex: Int = 0

    private func nextIndex() -> Int {
        defer { commandIndex += 1 }
        return commandIndex
    }

    private func makeReturn(_ value: Any) -> [String: Any] {
        [
            "Return": [
                "index": nextIndex(),
                "use": value
            ]
        ]
    }
}

public struct GasData {
    public var budget: UInt64
    public var price: UInt64
    public var payment: [String]

    public init(budget: UInt64 = 10000000, price: UInt64 = 1000, payment: [String] = []) {
        self.budget = budget
        self.price = price
        self.payment = payment
    }
}