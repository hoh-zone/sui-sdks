"""Storage node error types."""


class StorageNodeError(Exception):
    """Base storage node error."""


class ConnectionTimeoutError(StorageNodeError):
    """Request timed out."""


class StorageNodeAPIError(StorageNodeError):
    """Storage node API returned non-2xx."""

    def __init__(self, status_code: int, message: str = ""):
        super().__init__(f"status={status_code}: {message}".strip())
        self.status_code = status_code


class NotFoundError(StorageNodeAPIError):
    pass


class LegallyUnavailableError(StorageNodeAPIError):
    pass


class UserAbortError(StorageNodeError):
    pass
