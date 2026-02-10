import Foundation

public protocol JSONRPCTransport {
    func request(method: String, params: [Any?]) async throws -> Any
}
