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


# pylint: disable=too-few-public-methods
class AudioFormat:
    """
    Class for audio format. You can also use FileFormat enum or speechsdk.SpeechSynthesisOutputFormat.
    """

    def __init__(self, file_format: FileFormat, quality: int = 0):
        """
        Initialize an instance of AudioFormat from the file format and quality.
        :param file_format: Enum of type FileFormat
        :param quality: Quality of the audio, execute `aspeak -Q` to see the available qualities for each file format.
        """
        self._format = QUALITIES[file_format.value][quality]

    @property
    def format(self) -> speechsdk.SpeechSynthesisOutputFormat:
        """
        Get the underlying format.
        :return: audio format of type speechsdk.SpeechSynthesisOutputFormat
        """
        return self._format


def parse_format(audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None]) \
        -> speechsdk.SpeechSynthesisOutputFormat:
    if isinstance(audio_format, AudioFormat):
        return audio_format.format
    if isinstance(audio_format, FileFormat):
        return QUALITIES[audio_format.value][0]
    if isinstance(audio_format, speechsdk.SpeechSynthesisOutputFormat):
        return audio_format
    if audio_format is None:
        return QUALITIES['wav'][0]
    raise ValueError(f'Invalid audio format: {audio_format}')
