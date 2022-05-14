from typing import Union

import azure.cognitiveservices.speech as speechsdk

from .provider import SpeechServiceProvider
from .format import AudioFormat, parse_format
from ..ssml import create_ssml


def pure_text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str,
                        locale: Union[str, None] = None, voice: Union[str, None] = None,
                        audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> speechsdk.SpeechSynthesisResult:
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    if locale is not None:
        cfg.speech_synthesis_language = locale
    if voice is not None:
        cfg.speech_synthesis_voice_name = voice
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.text_to_speech(text, cfg, output)


def text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str, voice: str,
                   rate: Union[str, float] = 0.0, pitch: Union[str, float] = 0.0, style: str = "general",
                   style_degree: Union[float, None] = None,
                   role: Union[str, None] = None,
                   audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> speechsdk.SpeechSynthesisResult:
    ssml = create_ssml(text, voice, rate, pitch, style, style_degree, role)
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.ssml_to_speech(ssml, cfg, output)


def ssml_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, ssml: str,
                   audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None]) \
        -> speechsdk.SpeechSynthesisResult:
    cfg = speechsdk.SpeechConfig(auth_token=provider.token.token, region=provider.token.region)
    cfg.set_speech_synthesis_output_format(parse_format(audio_format))
    return provider.ssml_to_speech(ssml, cfg, output)
