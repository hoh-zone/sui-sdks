import Foundation

public struct JSONRPCServerError: Error {
    public let code: Int
    public let message: String
    public let data: Any?

    public init(code: Int, message: String, data: Any? = nil) {
        self.code = code
        self.message = message
        self.data = data
    }
}

public struct HTTPStatusError: Error, Equatable {
    public let statusCode: Int
    public let body: String

    public init(statusCode: Int, body: String) {
        self.statusCode = statusCode
        self.body = body
    }
}

public struct JSONRPCMalformedResponseError: Error, Equatable {
    public let reason: String

    public init(reason: String) {
        self.reason = reason
    }
}
