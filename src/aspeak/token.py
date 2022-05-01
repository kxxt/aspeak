from time import time

from .auth import _parse_info_from_token, _get_auth_token


class Token:
    def __init__(self, token):
        self.token = token
        info = _parse_info_from_token(token)
        self.region = info['region']
        self.expires = info['exp']

    def expired(self):
        # 10 seconds safe margin
        return self.expires < time() - 10

    def renew(self):
        self.__init__(_get_auth_token())

    def __repr__(self):
        return f'Token({self.token})'

    @classmethod
    def new(cls):
        return cls(_get_auth_token())
