"""
aspeak is a CLI tool as well as a Python library that
uses trial auth token of Azure Cognitive Services to do speech synthesis for you.
"""

# Re-export some common types to simplify imports on the user side
from azure.cognitiveservices.speech import SpeechSynthesisOutputFormat, \
    ResultReason, CancellationReason, ResultFuture
from .api import *
