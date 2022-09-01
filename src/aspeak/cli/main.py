import sys
from functools import partial

import azure.cognitiveservices.speech as speechsdk

from ..api import SpeechToSpeakerService, SpeechToOneFileService
from ..ssml import create_ssml
from .voices import list_voices
from .utils import list_qualities_and_formats
from ..quality import QUALITIES
from .parser import parser
from .validation import validate_quality, get_ineffective_args_for_listing, has_text_options
from .result_handler import handle_result


def read_file(args):
    if not hasattr(args, 'file') or args.file == '-':
        return sys.stdin.read()
    with open(args.file, 'r', encoding=args.encoding) as file:
        return file.read()


def preprocess_text(text, args):
    """
    Preprocess text.
    :param text: plain text
    :param args: args
    :return: (is_ssml, text_or_ssml)
    """
    if has_text_options(args):
        if args.voice is None:
            parser.error('Voice must be specified when using options for --text')
        pitch = args.pitch if hasattr(args, 'pitch') else 0.0
        rate = args.rate if hasattr(args, 'rate') else 0.0
        voice = args.voice
        style = args.style if hasattr(args, 'style') else 'general'
        role = args.role if hasattr(args, 'role') else None
        style_degree = args.style_degree if hasattr(args, 'style_degree') else None
        ssml = create_ssml(text, voice, rate, pitch, style, style_degree, role)
        return True, ssml
    return False, text


def speech_function_selector(speech_service, preprocessed):
    is_ssml, text_or_ssml = preprocessed
    if is_ssml:
        return speech_service.ssml_to_speech(text_or_ssml)
    return speech_service.pure_text_to_speech(text_or_ssml)


def main_ssml(speech, args):
    if hasattr(args, 'rate') or hasattr(args, 'pitch') or hasattr(args, 'style'):
        parser.error(
            'You can only use text options with --text. Please set these settings in your SSML.')
    if args.ssml is None:
        # --ssml is provided but empty
        result = speech.ssml_to_speech(read_file(args))
    else:
        # --ssml is provided and not empty
        if hasattr(args, 'file'):
            parser.error('You can only specify one input source.')
        result = speech.ssml_to_speech(args.text)
    return result


def main_text(speech, args):
    if args.text is None:
        # --text is provided but empty
        result = speech_function_selector(speech, preprocess_text(read_file(args), args))
    else:
        # --text is provided and not empty
        if hasattr(args, 'file'):
            parser.error('You can only specify one input source.')
        result = speech_function_selector(speech, preprocess_text(args.text, args))
    return result


def main_list_qualities_and_formats(args):
    ineffective_args = get_ineffective_args_for_listing(args)
    if hasattr(args, 'locale'):
        parser.error('--locale can not be used with --list-qualities-and-formats')
    if hasattr(args, 'voice'):
        parser.error('--voice can not be used with --list-qualities-and-formats')
    if ineffective_args:
        parser.error(f"You can't use argument(s) {ineffective_args} with --list-qualities-and-formats.")
    list_qualities_and_formats()


def main_list_voices(args):
    ineffective_args = get_ineffective_args_for_listing(args)
    if ineffective_args:
        parser.error(f"You can't use argument(s) {ineffective_args} with --list-voices.")
    list_voices(args)


def setup_defaults(args):
    args.locale = args.locale if hasattr(args, 'locale') else None
    args.voice = args.voice if hasattr(args, 'voice') else None
    args.quality = args.quality if hasattr(args, 'quality') else 0
    args.encoding = args.encoding if hasattr(args, 'encoding') else 'utf-8'

    args.file_type = "wav"  # The output file format
    for ext in ["mp3", "ogg", "webm"]:
        # mp3, ogg, webm are only supported when outputting to file
        if args.output_path is None and getattr(args, ext):
            parser.error(f"{ext} format is only supported when outputting to a file.")
        if getattr(args, ext):
            args.file_type = ext
    if args.file_type == "wav":
        # Set --wav to true in case that no format argument is provided
        args.wav = True


def main():
    args = parser.parse_args()

    if args.list_qualities_and_formats:
        main_list_qualities_and_formats(args)
        return

    if args.list_voices:
        main_list_voices(args)
        return

    setup_defaults(args)
    validate_quality(args, parser)

    if hasattr(args, 'format'):
        audio_format = getattr(speechsdk.SpeechSynthesisOutputFormat, args.format)
    else:
        audio_format = QUALITIES[args.file_type][args.quality]

    if args.output_path is None:
        create_speech = SpeechToSpeakerService
    else:
        create_speech = partial(SpeechToOneFileService, args.output_path)

    speech = create_speech(args.locale, args.voice, audio_format)

    if hasattr(args, 'ssml'):
        result = main_ssml(speech, args)
    elif hasattr(args, 'text'):
        result = main_text(speech, args)
    else:
        # Neither --text nor --ssml is provided, pretend --text is provided and empty
        result = speech_function_selector(speech, preprocess_text(read_file(args), args))
    handle_result(result)
