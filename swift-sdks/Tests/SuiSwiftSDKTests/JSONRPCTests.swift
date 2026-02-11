import Foundation
import XCTest
@testable import SuiSwiftSDK

final class JSONRPCTests: XCTestCase {
    func testNetworkURLs() {
        XCTAssertEqual(SuiNetwork.mainnet.fullnodeURL.absoluteString, "https://fullnode.mainnet.sui.io:443")
        XCTAssertEqual(SuiNetwork.testnet.fullnodeURL.absoluteString, "https://fullnode.testnet.sui.io:443")
        XCTAssertEqual(SuiNetwork.devnet.fullnodeURL.absoluteString, "https://fullnode.devnet.sui.io:443")
        XCTAssertEqual(SuiNetwork.localnet.fullnodeURL.absoluteString, "http://127.0.0.1:9000")
    }

    func testHTTPTransportSuccess() async throws {
        let session = makeMockedSession {
            XCTAssertEqual($0.httpMethod, "POST")

            let payload = try XCTUnwrap($0.httpBody)
            let json = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(json?["jsonrpc"] as? String, "2.0")
            XCTAssertEqual(json?["method"] as? String, "rpc.discover")

            return (
                200,
                ["jsonrpc": "2.0", "id": 1, "result": ["info": ["version": "1.0"]]]
            )
        }

        let transport = HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        let result = try await transport.request(method: "rpc.discover", params: [])
        let dict = try XCTUnwrap(result as? [String: Any])
        let info = try XCTUnwrap(dict["info"] as? [String: Any])
        XCTAssertEqual(info["version"] as? String, "1.0")
    }

    func testHTTPTransportJSONRPCError() async throws {
        let session = makeMockedSession {
            return (200, ["jsonrpc": "2.0", "id": 1, "error": ["code": -32601, "message": "Not found"]])
        }

        let transport = HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)

        do {
            _ = try await transport.request(method: "unknown", params: [])
            XCTFail("expected error")
        } catch let error as JSONRPCServerError {
            XCTAssertEqual(error.code, -32601)
            XCTAssertEqual(error.message, "Not found")
        }
    }

    func testClientMethodCalls() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            let method = body?["method"] as? String
            switch method {
            case "sui_getObject":
                return (200, ["jsonrpc": "2.0", "id": 1, "result": ["data": ["objectId": "0x1"]]])
            case "suix_getReferenceGasPrice":
                return (200, ["jsonrpc": "2.0", "id": 2, "result": "1000"])
            default:
                return (200, ["jsonrpc": "2.0", "id": 1, "result": [:]])
            }
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        let object = try await client.getObject(objectID: "0x1")
        let data = try XCTUnwrap(object["data"] as? [String: Any])
        XCTAssertEqual(data["objectId"] as? String, "0x1")

        let gasPrice = try await client.getReferenceGasPrice()
        XCTAssertEqual(gasPrice, "1000")
    }

    func testDevInspectTransactionBlockCall() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            let method = body?["method"] as? String
            XCTAssertEqual(method, "sui_devInspectTransactionBlock")
            return (200, ["jsonrpc": "2.0", "id": 1, "result": ["effects": ["status": "success"]]])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        let response = try await client.devInspectTransactionBlock(
            sender: "0x2",
            transactionBlock: Data([0x1, 0x2]).base64EncodedString()
        )
        let effects = try XCTUnwrap(response["effects"] as? [String: Any])
        XCTAssertEqual(effects["status"] as? String, "success")
    }

    func testGetOwnedObjectsTypedDecode() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            let method = body?["method"] as? String
            XCTAssertEqual(method, "suix_getOwnedObjects")
            return (
                200,
                [
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        "data": [
                            ["data": ["objectId": "0xabc"]]
                        ],
                        "nextCursor": NSNull(),
                        "hasNextPage": false,
                    ],
                ]
            )
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        let page = try await client.getOwnedObjectsTyped(owner: "0x2")
        XCTAssertEqual(page.data.count, 1)
        XCTAssertEqual(page.hasNextPage, false)
    }

    func testDryRunTransactionBlockBytesEncodesBase64() async throws {
        let expectedBase64 = Data([0xAA, 0xBB, 0xCC]).base64EncodedString()
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "sui_dryRunTransactionBlock")
            let params = try XCTUnwrap(body?["params"] as? [Any])
            XCTAssertEqual(params.first as? String, expectedBase64)
            return (200, ["jsonrpc": "2.0", "id": 1, "result": ["effects": [:]]])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        _ = try await client.dryRunTransactionBlock(transactionBlockBytes: [0xAA, 0xBB, 0xCC])
    }

    func testGetEventsAliasUsesSuiGetEvents() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "sui_getEvents")
            return (200, ["jsonrpc": "2.0", "id": 1, "result": []])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )
        let events = try await client.getEvents(transactionDigest: "11111111111111111111111111111111")
        XCTAssertEqual(events.count, 0)
    }

    func testGetStakesByIdsAliasUsesSuixGetStakesByIds() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "suix_getStakesByIds")
            return (200, ["jsonrpc": "2.0", "id": 1, "result": []])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )
        let stakes = try await client.getStakesByIds(stakedSuiIDs: ["0x5"])
        XCTAssertEqual(stakes.count, 0)
    }

    func testGetRpcApiVersionAlias() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "rpc.discover")
            return (200, ["jsonrpc": "2.0", "id": 1, "result": ["info": ["version": "9.9.9"]]])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )
        let version = try await client.getRpcApiVersion()
        XCTAssertEqual(version, "9.9.9")
    }

    func testExecuteTransactionBlockDataSingleSignature() async throws {
        let txData = Data([0x01, 0x02, 0x03, 0x04])
        let txBase64 = txData.base64EncodedString()
        let signature = "AA=="
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "sui_executeTransactionBlock")
            let params = try XCTUnwrap(body?["params"] as? [Any])
            XCTAssertEqual(params[0] as? String, txBase64)
            XCTAssertEqual(params[1] as? [String], [signature])
            return (200, ["jsonrpc": "2.0", "id": 1, "result": ["digest": "11111111111111111111111111111111"]])
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )
        let response = try await client.executeTransactionBlock(
            transactionBlockData: txData,
            signature: signature
        )
        XCTAssertEqual(response["digest"] as? String, "11111111111111111111111111111111")
    }

    func testQueryEventsOrderAscendingMapsToFalse() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "suix_queryEvents")
            let params = try XCTUnwrap(body?["params"] as? [Any])
            XCTAssertEqual(params[3] as? Bool, false)
            return (
                200,
                [
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        "data": [],
                        "nextCursor": NSNull(),
                        "hasNextPage": false,
                    ],
                ]
            )
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        let page = try await client.queryEvents(query: [:], order: .ascending)
        XCTAssertEqual(page["hasNextPage"] as? Bool, false)
    }

    func testGetCheckpointsOrderDescendingMapsToTrue() async throws {
        let session = makeMockedSession {
            let payload = try XCTUnwrap($0.httpBody)
            let body = try JSONSerialization.jsonObject(with: payload) as? [String: Any]
            XCTAssertEqual(body?["method"] as? String, "sui_getCheckpoints")
            let params = try XCTUnwrap(body?["params"] as? [Any])
            XCTAssertEqual(params[2] as? Bool, true)
            return (
                200,
                [
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        "data": [],
                        "nextCursor": NSNull(),
                        "hasNextPage": false,
                    ],
                ]
            )
        }

        let client = SuiClient(
            endpoint: URL(string: "https://example.com")!,
            transport: HTTPJSONRPCTransport(url: URL(string: "https://example.com")!, session: session)
        )

        let page = try await client.getCheckpoints(order: .descending)
        XCTAssertEqual(page["hasNextPage"] as? Bool, false)
    }
}

private typealias MockResponse = (status: Int, json: Any)

private func makeMockedSession(handler: @escaping (URLRequest) throws -> MockResponse) -> URLSession {
    MockURLProtocol.requestHandler = handler
    let config = URLSessionConfiguration.ephemeral
    config.protocolClasses = [MockURLProtocol.self]
    return URLSession(configuration: config)
}

private final class MockURLProtocol: URLProtocol {
    static var requestHandler: ((URLRequest) throws -> MockResponse)?

    override class func canInit(with request: URLRequest) -> Bool { true }
    override class func canonicalRequest(for request: URLRequest) -> URLRequest { request }

    override func startLoading() {
        guard let handler = Self.requestHandler else {
            fatalError("MockURLProtocol.requestHandler not set")
        }

        do {
            let response = try handler(request)
            let data = try JSONSerialization.data(withJSONObject: response.json)
            let httpResponse = HTTPURLResponse(
                url: request.url!,
                statusCode: response.status,
                httpVersion: nil,
                headerFields: ["Content-Type": "application/json"]
            )!
            client?.urlProtocol(self, didReceive: httpResponse, cacheStoragePolicy: .notAllowed)
            client?.urlProtocol(self, didLoad: data)
            client?.urlProtocolDidFinishLoading(self)
        } catch {
            client?.urlProtocol(self, didFailWithError: error)
        }
    }

    override func stopLoading() {}
}
