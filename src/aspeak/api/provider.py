import azure.cognitiveservices.speech as speechsdk

from ..token import Token


class SpeechServiceProvider:
    """
    The SpeechServiceProvider class is a service provider for Azure Cognitive Services Text-to-speech
    that automatically renews trial auth tokens.
    """

    def __init__(self):
        self._current_token = Token()

    @property
    def token(self) -> Token:
        """
        Returns the current valid token instance.
        :return: token instance of type Token
        """
        if self._expired:
            self.renew()
        return self._current_token

    @property
    def _expired(self) -> bool:
        return self._current_token.expired()

    def renew(self) -> None:
        """
        Manually renew the current token. Usually you do not need to call this method.
        :return: None
        """
        self._current_token.renew()

    def text_to_speech(self, text: str, cfg: speechsdk.SpeechConfig,
                       output: speechsdk.audio.AudioOutputConfig) -> speechsdk.SpeechSynthesisResult:
        if self._expired:
            self.renew()
        return speechsdk.SpeechSynthesizer(speech_config=cfg, audio_config=output).speak_text(text)

    def ssml_to_speech(self, ssml: str, cfg: speechsdk.SpeechConfig,
                       output: speechsdk.audio.AudioOutputConfig) -> speechsdk.SpeechSynthesisResult:
        if self._expired:
            self.renew()
        return speechsdk.SpeechSynthesizer(speech_config=cfg, audio_config=output).speak_ssml(ssml)
