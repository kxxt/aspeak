class AspeakError(Exception):
    """
    Base class for all Aspeak errors.
    """


class TokenRetrievalError(AspeakError):
    """
    Error raised when the trial token cannot be retrieved.
    """

    def __init__(self, status_code, message="Failed to retrieve token"):
        super().__init__(message)
        self.status_code = status_code
        self.message = message
