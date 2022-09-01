from typing import Union, Optional
from functools import wraps

from .format import parse_format, AudioFormat, FileFormat
from ..ssml import create_ssml
from ..urls import ENDPOINT_URL
import azure.cognitiveservices.speech as speechsdk


def _parse_kwargs(**kwargs):
    voice = kwargs.get('voice', None)
    pitch = kwargs.get('pitch', 0.0)
    rate = kwargs.get('rate', 0.0)
    style = kwargs.get('style', 'general')
    style_degree = kwargs.get('style_degree', None)
    role = kwargs.get('role', None)
    return voice, rate, pitch, style, style_degree, role


class SpeechServiceBase:
    def __init__(self, locale: Optional[str] = None, voice: Optional[str] = None,
                 output: speechsdk.audio.AudioOutputConfig = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None
                 ):
        self._config = speechsdk.SpeechConfig(endpoint=ENDPOINT_URL)
        self._output = output
        if locale is not None:
            self._config.speech_synthesis_language = locale
        if voice is not None:
            self._config.speech_synthesis_voice_name = voice
        if audio_format is not None:
            self._config.set_speech_synthesis_output_format(parse_format(audio_format))
        self._synthesizer = speechsdk.SpeechSynthesizer(self._config, self._output)

    def pure_text_to_speech(self, text, **kwargs):
        return self._synthesizer.speak_text(text)

    def pure_text_to_speech_async(self, text, **kwargs):
        return self._synthesizer.speak_text_async(text)

    def ssml_to_speech(self, ssml, **kwargs):
        return self._synthesizer.speak_ssml(ssml)

    def ssml_to_speech_async(self, ssml, **kwargs):
        return self._synthesizer.speak_ssml(ssml)

    def text_to_speech(self, text, **kwargs):
        ssml = create_ssml(text, *_parse_kwargs(**kwargs))
        return self._synthesizer.speak_ssml(ssml)

    def text_to_speech_async(self, text, **kwargs):
        ssml = create_ssml(text, *_parse_kwargs(**kwargs))
        return self._synthesizer.speak_ssml_async(ssml)


class SpeechToSpeakerService(SpeechServiceBase):
    def __init__(self, locale: str = None, voice: str = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None,
                 device_name: Union[str, None] = None):
        if device_name is None:
            output = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
        else:
            output = speechsdk.audio.AudioOutputConfig(device_name=device_name)
        super().__init__(locale, voice, output, audio_format)


class SpeechToFileService(SpeechServiceBase):
    def __init__(self, locale: Optional[str] = None, voice: Optional[str] = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None):
        super().__init__(locale, voice, None, audio_format)

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls.pure_text_to_speech = cls._setup_synthesizer_for_file(cls.pure_text_to_speech)
        cls.pure_text_to_speech_async = cls._setup_synthesizer_for_file(cls.pure_text_to_speech_async)
        cls.text_to_speech = cls._setup_synthesizer_for_file(cls.text_to_speech)
        cls.text_to_speech_async = cls._setup_synthesizer_for_file(cls.text_to_speech_async)
        cls.ssml_to_speech = cls._setup_synthesizer_for_file(cls.ssml_to_speech)
        cls.ssml_to_speech_async = cls._setup_synthesizer_for_file(cls.ssml_to_speech_async)

    @staticmethod
    def _setup_synthesizer_for_file(fn):
        @wraps(fn)
        def wrapper(self, text, **kwargs):
            self._setup_synthesizer(kwargs['path'])
            return fn(self, text, **kwargs)

        return wrapper

    def _setup_synthesizer(self, file_path: str):
        self._output = speechsdk.audio.AudioOutputConfig(filename=file_path)
        self._synthesizer = speechsdk.SpeechSynthesizer(self._config, self._output)


class SpeechToOneFileService(SpeechServiceBase):
    def __init__(self, output_path: str, locale: Optional[str] = None, voice: Optional[str] = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None):
        output = speechsdk.audio.AudioOutputConfig(filename=output_path)
        super().__init__(locale, voice, output, audio_format)
