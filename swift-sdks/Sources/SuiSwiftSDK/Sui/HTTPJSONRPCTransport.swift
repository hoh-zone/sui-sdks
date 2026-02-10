import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

public final class HTTPJSONRPCTransport: JSONRPCTransport {
    private let url: URL
    private let session: URLSession
    private var nextID: Int

    public init(url: URL, session: URLSession = .shared) {
        self.url = url
        self.session = session
        self.nextID = 1
    }

    public func request(method: String, params: [Any?]) async throws -> Any {
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let requestID = nextRequestID()
        let payload: [String: Any] = [
            "jsonrpc": "2.0",
            "id": requestID,
            "method": method,
            "params": params.map { $0 ?? NSNull() }
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: payload)

        let (data, response) = try await session.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse else {
            throw JSONRPCMalformedResponseError(reason: "response is not HTTP")
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let body = String(data: data, encoding: .utf8) ?? ""
            throw HTTPStatusError(statusCode: httpResponse.statusCode, body: body)
        }

        let jsonObject = try JSONSerialization.jsonObject(with: data)
        guard let body = jsonObject as? [String: Any] else {
            throw JSONRPCMalformedResponseError(reason: "response body is not object")
        }

        if let error = body["error"] as? [String: Any] {
            let code = error["code"] as? Int ?? -1
            let message = error["message"] as? String ?? "unknown"
            let errorData = error["data"]
            throw JSONRPCServerError(code: code, message: message, data: errorData)
        }

        guard let result = body["result"] else {
            throw JSONRPCMalformedResponseError(reason: "missing result")
        }

        return result
    }

    private func nextRequestID() -> Int {
        defer { nextID += 1 }
        return nextID
    }
}
