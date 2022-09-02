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
    """
    A base class that provides speech service
    """

    def __init__(self, locale: Optional[str] = None, voice: Optional[str] = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None,
                 output: speechsdk.audio.AudioOutputConfig = None
                 ):
        """
        :param locale: The locale of the voice, optional.
        :param voice: The voice name, optional.
        :param output: An instance of AudioOutputConfig.
        :param audio_format: The audio format, optional.
        """
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
        """
        Supported keyword arguments:
        voice: The voice name.
        rate: The speaking rate, optional. See the documentation for more details.
        pitch: The speaking pitch, optional. See the documentation for more details.
        style: The speaking style, optional. See the documentation for more details.
        style_degree: The speaking style degree, optional. It only works for some Chinese voices.
        role: The speaking role, optional. It only works for some Chinese voices.
        path: Output file path. Only works with SpeechService classes that support it.
        """
        ssml = create_ssml(text, *_parse_kwargs(**kwargs))
        return self._synthesizer.speak_ssml(ssml)

    def text_to_speech_async(self, text, **kwargs):
        """
        Supported keyword arguments:
        voice: The voice name.
        rate: The speaking rate, optional. See the documentation for more details.
        pitch: The speaking pitch, optional. See the documentation for more details.
        style: The speaking style, optional. See the documentation for more details.
        style_degree: The speaking style degree, optional. It only works for some Chinese voices.
        role: The speaking role, optional. It only works for some Chinese voices.
        path: Output file path. Only works with SpeechService classes that support it.
        """
        ssml = create_ssml(text, *_parse_kwargs(**kwargs))
        return self._synthesizer.speak_ssml_async(ssml)


class SpeechToSpeakerService(SpeechServiceBase):
    """
    Speech service that outputs to speakers
    """

    def __init__(self, locale: str = None, voice: str = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None,
                 device_name: Union[str, None] = None):
        """
        :param locale: The locale of the voice, optional.
        :param voice: The voice name, optional.
        :param audio_format: The audio format, optional.
        :param device_name: Device name of the speaker, optional.
        """
        if device_name is None:
            output = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
        else:
            output = speechsdk.audio.AudioOutputConfig(device_name=device_name)
        super().__init__(locale, voice, audio_format, output)


def _setup_synthesizer_for_file(fn):
    @wraps(fn)
    def wrapper(self, text, **kwargs):
        self._setup_synthesizer(kwargs['path'])
        return fn(self, text, **kwargs)

    return wrapper


class SpeechToFileService(SpeechServiceBase):
    """
    Speech service that outputs to files
    """

    def __init__(self, locale: Optional[str] = None, voice: Optional[str] = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None):
        """
        :param locale: The locale of the voice, optional.
        :param voice: The voice name, optional.
        :param audio_format: The audio format, optional.
        """
        super().__init__(locale, voice, audio_format, None)

    pure_text_to_speech = _setup_synthesizer_for_file(SpeechServiceBase.pure_text_to_speech)
    pure_text_to_speech_async = _setup_synthesizer_for_file(SpeechServiceBase.pure_text_to_speech_async)
    text_to_speech = _setup_synthesizer_for_file(SpeechServiceBase.text_to_speech)
    text_to_speech_async = _setup_synthesizer_for_file(SpeechServiceBase.text_to_speech_async)
    ssml_to_speech = _setup_synthesizer_for_file(SpeechServiceBase.ssml_to_speech)
    ssml_to_speech_async = _setup_synthesizer_for_file(SpeechServiceBase.ssml_to_speech_async)

    def _setup_synthesizer(self, file_path: str):
        self._output = speechsdk.audio.AudioOutputConfig(filename=file_path)
        self._synthesizer = speechsdk.SpeechSynthesizer(self._config, self._output)


class SpeechToOneFileService(SpeechServiceBase):
    """
    Speech service that outputs to a specific file, which can't be changed during runtime.
    """

    def __init__(self, output_path: str, locale: Optional[str] = None, voice: Optional[str] = None,
                 audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None):
        """
        :param output_path: The path of output file
        :param locale: The locale of the voice, optional.
        :param voice: The voice name, optional.
        :param audio_format: The audio format, optional.
        """
        output = speechsdk.audio.AudioOutputConfig(filename=output_path)
        super().__init__(locale, voice, audio_format, output)
