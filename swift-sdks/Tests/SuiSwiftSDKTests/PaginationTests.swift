import XCTest
@testable import SuiSwiftSDK

final class PaginationTests: XCTestCase {
    func testCollectItemsAcrossPages() async throws {
        let items = try await SuiPagination.collectItems(fetchPage: { cursor in
            if cursor == nil {
                return [
                    "data": [["id": 1], ["id": 2]],
                    "hasNextPage": true,
                    "nextCursor": "page-2"
                ]
            }

            return [
                "data": [["id": 3]],
                "hasNextPage": false,
                "nextCursor": NSNull()
            ]
        })

        XCTAssertEqual(items.count, 3)
        XCTAssertEqual(items[0]["id"] as? Int, 1)
        XCTAssertEqual(items[2]["id"] as? Int, 3)
    }

    func testCollectItemsRespectsMaxItems() async throws {
        let items = try await SuiPagination.collectItems(
            fetchPage: { _ in
                ["data": [["id": 1], ["id": 2], ["id": 3]], "hasNextPage": false]
            },
            maxItems: 2
        )

        XCTAssertEqual(items.count, 2)
    }
}
