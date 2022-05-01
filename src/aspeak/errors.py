class ASpeakError(Exception):
    pass


class TokenRetrievalError(ASpeakError):
    def __init__(self, status_code, message="Failed to retrieve token"):
        super(TokenRetrievalError, self).__init__(message)
        self.status_code = status_code
        self.message = message
