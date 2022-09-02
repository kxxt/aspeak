from typing import Union
from deprecated import deprecated
import azure.cognitiveservices.speech as speechsdk


class SpeechServiceProvider:
    """
    The SpeechServiceProvider class is a service provider for Azure Cognitive Services Text-to-speech
    that automatically renews trial auth tokens.
    """
    @deprecated(version="3.0.0.dev2", reason="The old API is deprecated. Please upgrade to new API. ")
    def get_synthesizer(self, cfg: speechsdk.SpeechConfig,
                        output: speechsdk.audio.AudioOutputConfig) -> speechsdk.SpeechSynthesizer:
        return speechsdk.SpeechSynthesizer(speech_config=cfg, audio_config=output)

    @deprecated(version="3.0.0.dev2", reason="The old API is deprecated. Please upgrade to new API. ")
    def text_to_speech(self, text: str, cfg: speechsdk.SpeechConfig,
                       output: speechsdk.audio.AudioOutputConfig,
                       use_async: bool = False) -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:
        synthesizer = self.get_synthesizer(cfg, output)
        if use_async:
            return synthesizer.speak_text_async(text)
        return synthesizer.speak_text(text)

    @deprecated(version="3.0.0.dev2", reason="The old API is deprecated. Please upgrade to new API. ")
    def ssml_to_speech(self, ssml: str, cfg: speechsdk.SpeechConfig,
                       output: speechsdk.audio.AudioOutputConfig,
                       use_async: bool = False) -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:
        synthesizer = self.get_synthesizer(cfg, output)
        if use_async:
            return synthesizer.speak_ssml_async(ssml)
        return synthesizer.speak_ssml(ssml)
