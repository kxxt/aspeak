from typing import Union, Optional

import azure.cognitiveservices.speech as speechsdk

from .provider import SpeechServiceProvider
from .format import AudioFormat, FileFormat, parse_format
from ..ssml import create_ssml


# pylint: disable=too-many-arguments
def pure_text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str,
                        locale: Optional[str] = None, voice: Optional[str] = None,
                        use_async: bool = False,
                        audio_format: Union[
                            AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:
    """
    Execute a text-to-speech request without SSML.
    :param provider: An instance of SpeechServiceProvider.
    :param output: An instance of AudioOutputConfig.
    :param text: The text to be synthesized.
    :param locale: The locale of the voice, optional.
    :param voice: The voice name, optional.
    :param use_async: Use non-blocking (asynchronous) audio synthesizer, optional.
    :param audio_format: The audio format, optional.
    :return: result either of type SpeechSynthesisResult or ResultFuture.
    """
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    if locale is not None:
        cfg.speech_synthesis_language = locale
    if voice is not None:
        cfg.speech_synthesis_voice_name = voice
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.text_to_speech(text, cfg, output, use_async=use_async)


# pylint: disable=too-many-arguments
def text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str, voice: str,
                   rate: Union[str, float] = 0.0, pitch: Union[str, float] = 0.0, style: str = "general",
                   style_degree: Optional[float] = None,
                   role: Optional[str] = None,
                   use_async: bool = False,
                   audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:
    """
    Execute a text-to-speech request with generated SSML from text and various options.
    :param provider: An instance of SpeechServiceProvider.
    :param output: An instance of AudioOutputConfig.
    :param text: The text to be synthesized.
    :param voice: The voice name.
    :param rate: The speaking rate, optional. See the documentation for more details.
    :param pitch: The speaking pitch, optional. See the documentation for more details.
    :param style: The speaking style, optional. See the documentation for more details.
    :param style_degree: The speaking style degree, optional. It only works for some Chinese voices.
    :param role: The speaking role, optional. It only works for some Chinese voices.
    :param use_async: Use non-blocking (asynchronous) audio synthesizer, optional.
    :param audio_format: The audio format, optional.
    :return: result either of type SpeechSynthesisResult or ResultFuture.
    """
    ssml = create_ssml(text, voice, rate, pitch, style, style_degree, role)
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.ssml_to_speech(ssml, cfg, output, use_async=use_async)


def ssml_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, ssml: str,
                   audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None],
                   use_async: bool = False) \
        -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:
    """
    Execute a text-to-speech request with SSML.
    :param provider: An instance of SpeechServiceProvider.
    :param output: An instance of AudioOutputConfig.
    :param ssml: The SSML to be synthesized.
    :param use_async: Use non-blocking (asynchronous) audio synthesizer, optional.
    :param audio_format: The audio format, optional.
    :return: result either of type SpeechSynthesisResult or ResultFuture.
    """
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.ssml_to_speech(ssml, cfg, output, use_async=use_async)
