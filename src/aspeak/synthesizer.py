import azure.cognitiveservices.speech as speechsdk
import requests

from .token import Token
from .urls import voice_list_url


class Synthesizer:
    def __init__(self):
        self._current_token = None
        pass

    def _token(self):
        if self._current_token is None or self._current_token.expired():
            self._current_token = Token.new()
        return self._current_token

    def _base_speech_config(self):
        return speechsdk.SpeechConfig(auth_token=self._token().token, region=self._token().region)

    def get_voice_list(self):
        r = requests.get(voice_list_url(self._token().region),
                         headers={'Authorization': 'Bearer ' + self._token().token})
        return r.json()

    def text_to_speech(self, text):
        pass

    def text_to_wav(self, text, filename):
        pass

    def ssml_to_wav(self, ssml, filename):
        pass

    def ssml_to_speech(self, ssml):
        pass
