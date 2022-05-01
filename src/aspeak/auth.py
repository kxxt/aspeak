import requests
import base64
import json
import re

from . import errors
from .urls import TRAIL_URL


def _get_auth_token() -> str:
    """
    Get a trial auth token from the trial webpage.
    """
    r = requests.get(TRAIL_URL)
    if r.status_code != 200:
        raise errors.TokenRetrievalError(status_code=r.status_code)
    text = r.text

    # We don't need bs4, because a little of regex is enough.

    match = re.search(r'\s+var\s+localizedResources\s+=\s+\{((.|\n)*?)\};', text, re.M)
    retrieval_error = errors.TokenRetrievalError(message='Could not extract token from webpage.',
                                                 status_code=r.status_code)
    if match is None:
        raise retrieval_error
    token = re.search(r'\s+token:\s*"([^"]+)"', match.group(1), re.M)
    if token is None:
        raise retrieval_error
    return token.group(1)


def _parse_info_from_token(token: str) -> dict:
    """
    Parse the region from the token.
    """
    _, content, _ = token.split('.')
    # The token is base64 encoded, so we need to decode it.
    # :joy: Actually, GitHub Copilot provides the following line, and it works!
    json_string = base64.b64decode(content + '=' * (-len(content) % 4))
    return json.loads(json_string)


def get_token_info_dict() -> dict:
    """
    Get an auth token and its associated information in a dict.
    """
    token = _get_auth_token()
    info = _parse_info_from_token(token)
    info['token'] = token
    return info
