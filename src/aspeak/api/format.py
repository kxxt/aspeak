from enum import Enum
from typing import Union

import azure.cognitiveservices.speech as speechsdk

from ..quality import QUALITIES


class FileFormat(Enum):
    """
    Enum for audio file formats.
    """
    WAV = 'wav'
    MP3 = 'mp3'
    OGG = 'ogg'
    WEBM = 'webm'


class AudioFormat:
    def __init__(self, audio_format: speechsdk.SpeechSynthesisOutputFormat):
        self._format = audio_format

    @property
    def format(self) -> speechsdk.SpeechSynthesisOutputFormat:
        return self._format

    @classmethod
    def from_enum(cls, file_format: FileFormat) -> 'AudioFormat':
        return cls(QUALITIES[file_format.value][0])

    @classmethod
    def from_enum_and_quality(cls, file_format: FileFormat, quality: int) -> 'AudioFormat':
        return cls(QUALITIES[file_format.value][quality])


def parse_format(audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None]) \
        -> speechsdk.SpeechSynthesisOutputFormat:
    if isinstance(audio_format, AudioFormat):
        return audio_format.format
    if isinstance(audio_format, speechsdk.SpeechSynthesisOutputFormat):
        return audio_format
    if audio_format is None:
        return QUALITIES['wav'][0]
    raise ValueError(f'Invalid audio format: {audio_format}')
