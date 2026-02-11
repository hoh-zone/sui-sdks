"""Seal package error types."""


class SealError(Exception):
    """Base seal error."""


class UserError(SealError):
    pass


class InvalidClientOptionsError(SealError):
    pass


class InvalidThresholdError(SealError):
    pass


class InvalidPackageError(SealError):
    pass


class InvalidCiphertextError(SealError):
    pass


class DecryptionError(SealError):
    pass


class ExpiredSessionKeyError(SealError):
    pass


class TooManyFailedFetchKeyRequestsError(SealError):
    pass


class InvalidKeyServerError(SealError):
    pass
