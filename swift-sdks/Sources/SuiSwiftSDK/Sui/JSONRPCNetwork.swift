import Foundation

public enum SuiNetwork: String, CaseIterable {
    case mainnet
    case testnet
    case devnet
    case localnet

    public var fullnodeURL: URL {
        switch self {
        case .mainnet:
            return URL(string: "https://fullnode.mainnet.sui.io:443")!
        case .testnet:
            return URL(string: "https://fullnode.testnet.sui.io:443")!
        case .devnet:
            return URL(string: "https://fullnode.devnet.sui.io:443")!
        case .localnet:
            return URL(string: "http://127.0.0.1:9000")!
        }
    }
}
