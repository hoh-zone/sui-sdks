"""Walrus client errors."""


class WalrusError(Exception):
    """Base error for walrus package."""


class WalrusClientError(WalrusError):
    """Client-level walrus failure."""


class InconsistentBlobError(WalrusClientError):
    """Blob contents do not match metadata."""
