import Foundation

public enum SuiPagination {
    public static func collectItems(
        fetchPage: (_ cursor: String?) async throws -> [String: Any],
        startCursor: String? = nil,
        maxItems: Int? = nil
    ) async throws -> [[String: Any]] {
        var cursor = startCursor
        var emitted = 0
        var output: [[String: Any]] = []

        while true {
            let page = try await fetchPage(cursor)
            let data = try extractData(page)

            for item in data {
                output.append(item)
                emitted += 1
                if let maxItems, emitted >= maxItems {
                    return output
                }
            }

            let hasNextPage = page["hasNextPage"] as? Bool ?? false
            if !hasNextPage {
                return output
            }

            let nextCursor = page["nextCursor"] as? String
            if nextCursor == nil || nextCursor == cursor {
                return output
            }
            cursor = nextCursor
        }
    }

    private static func extractData(_ page: [String: Any]) throws -> [[String: Any]] {
        guard let data = page["data"] else {
            return []
        }
        guard let items = data as? [[String: Any]] else {
            throw JSONRPCMalformedResponseError(reason: "paginated response has non-array data")
        }
        return items
    }
}
