from time import time

from .auth import _parse_info_from_token, _get_auth_token


class Token:
    def __init__(self, token=None):
        if token is None:
            token = _get_auth_token()
        self.token = token
        info = _parse_info_from_token(token)
        self.region = info['region']
        self.expires = info['exp']

    def expired(self):
        # 5 seconds safe margin
        return self.expires < time() - 5

    def renew(self):
        self.__init__(_get_auth_token())

    def __repr__(self):
        return f'Token({self.token})'

    @classmethod
    def from_string(cls, token):
        return cls(token)
