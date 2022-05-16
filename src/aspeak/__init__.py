"""
aspeak is a CLI tool as well as a Python library that
uses trial auth token of Azure Cognitive Services to do speech synthesis for you.
"""

from .auth import get_token_info_dict
from .token import Token
from .api import *
from .errors import AspeakError, TokenRetrievalError
