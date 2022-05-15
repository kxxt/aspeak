import argparse
import azure.cognitiveservices.speech as speechsdk
import sys

from .synthesizer import Synthesizer
from .ssml import create_ssml
from .cli.voices import format_voice, get_voice_list
from .formats import get_available_formats
from .quality import QUALITIES
from .cli import parser


def read_file(args):
    if not hasattr(args, 'file') or args.file == '-':
        return sys.stdin.read()
    with open(args.file, 'r', encoding=args.encoding) as f:
        return f.read()


def ineffective_args_for_listing(args):
    result = [option for option in
              {'pitch', 'rate', 'style', 'role', 'style_degree', 'quality', 'format', 'encoding', 'file', 'text',
               'ssml'} if hasattr(args, option)]
    if args.output_path is not None:
        result.append('output_path')
    return ', '.join(result)


def has_text_options(args):
    return any(hasattr(args, option) for option in {'pitch', 'rate', 'style', 'role', 'style_degree'})


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
        voice = args.voice if hasattr(args, 'voice') else None
        style = args.style if hasattr(args, 'style') else 'general'
        role = args.role if hasattr(args, 'role') else None
        style_degree = args.style_degree if hasattr(args, 'style_degree') else None
        ssml = create_ssml(text, voice, rate, pitch, style, style_degree, role)
        return True, ssml
    return False, text


def speech_function_selector(synthesizer, preprocessed):
    is_ssml, text_or_ssml = preprocessed
    if is_ssml:
        return synthesizer.ssml_to_speech(text_or_ssml)
    else:
        return synthesizer.text_to_speech(text_or_ssml)


def list_voices(synthesizer, args):
    voices = get_voice_list(synthesizer._token)
    if hasattr(args, 'voice'):
        voices = [v for v in voices if v["ShortName"] == args.voice]
    if hasattr(args, 'locale'):
        voices = [v for v in voices if v['Locale'] == args.locale]
    for v in voices:
        print(format_voice(v))


def list_qualities_and_formats():
    print('Available qualities:')
    for ext, info in QUALITIES.items():
        print(f"Qualities for {ext}:")
        for k, v in info.items():
            print(f"{k:2}: {v.name}")
    print()
    formats = get_available_formats()
    print("Available formats:")
    for f in formats:
        print(f'- {f}')


def validate_quality(args, parser):
    if not hasattr(args, 'quality'):
        return
    if hasattr(args, 'format') and args.quality != 0:
        parser.error("You can't use --quality with --format.")
    for ext in {"mp3", "ogg", "wav", "webm"}:
        if getattr(args, ext) and args.quality not in QUALITIES[ext]:
            parser.error(f"Invalid quality {args.quality} for {ext}.")


COLOR_RED = '\033[91m'
COLOR_CLEAR = '\033[0m'


def handle_result(r: speechsdk.SpeechSynthesisResult):
    if r.reason == speechsdk.ResultReason.SynthesizingAudioCompleted:
        exit(0)
    elif r.reason == speechsdk.ResultReason.Canceled:
        cancellation_details = r.cancellation_details
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Speech synthesis canceled: {cancellation_details.reason}",
              file=sys.stderr)
        if cancellation_details.reason == speechsdk.CancellationReason.Error:
            print(cancellation_details.error_details, file=sys.stderr)
        exit(2)
    else:
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Unexpected result reason: {r.reason}", file=sys.stderr)
        exit(3)


def main():
    args = parser.parse_args()

    if args.list_qualities_and_formats:
        ineffective_args = ineffective_args_for_listing(args)
        if hasattr(args, 'locale'):
            parser.error('--locale can not be used with --list-qualities-and-formats')
        if hasattr(args, 'voice'):
            parser.error('--voice can not be used with --list-qualities-and-formats')
        if ineffective_args:
            parser.error(f"You can't use argument(s) {ineffective_args} with --list-qualities-and-formats.")
        list_qualities_and_formats()
        return

    if args.list_voices:
        ineffective_args = ineffective_args_for_listing(args)
        if ineffective_args:
            parser.error(f"You can't use argument(s) {ineffective_args} with --list-voices.")
        list_voices(Synthesizer(), args)
        return

    if args.output_path is None:
        audio_config = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
    else:
        audio_config = speechsdk.audio.AudioOutputConfig(filename=args.output_path)
    locale = args.locale if hasattr(args, 'locale') else 'en-US'
    voice = args.voice if hasattr(args, 'voice') else None
    args.quality = args.quality if hasattr(args, 'quality') else 0
    args.encoding = args.encoding if hasattr(args, 'encoding') else 'utf-8'

    file_ext = "wav"  # The output file format
    for ext in {"mp3", "ogg", "webm"}:
        # mp3, ogg, webm are only supported when outputting to file
        if args.output_path is None and getattr(args, ext):
            parser.error(f"{ext} format is only supported when outputting to a file.")
        if getattr(args, ext):
            file_ext = ext
    if file_ext == "wav":
        # Set --wav to true in case that no format argument is provided
        args.wav = True

    validate_quality(args, parser)

    if hasattr(args, 'format'):
        audio_format = getattr(speechsdk.SpeechSynthesisOutputFormat, args.format)
    else:
        audio_format = QUALITIES[file_ext][args.quality]

    try:
        synthesizer = Synthesizer(audio_config, locale, voice, audio_format)
        if hasattr(args, 'ssml'):
            if hasattr(args, 'rate') or hasattr(args, 'pitch') or hasattr(args, 'style'):
                parser.error(
                    'You can only use text options with --text. Please set these settings in your SSML.')
            if args.ssml is None:
                # --ssml is provided but empty
                handle_result(synthesizer.ssml_to_speech(read_file(args)))
            else:
                # --ssml is provided and not empty
                if hasattr(args, 'file'):
                    parser.error('You can only specify one input source.')
                handle_result(synthesizer.ssml_to_speech(args.text))
        elif hasattr(args, 'text'):
            if args.text is None:
                # --text is provided but empty
                handle_result(speech_function_selector(synthesizer, preprocess_text(read_file(args), args)))
            else:
                # --text is provided and not empty
                if hasattr(args, 'file'):
                    parser.error('You can only specify one input source.')
                handle_result(speech_function_selector(synthesizer, preprocess_text(args.text, args)))
        else:
            # Neither --text nor --ssml is provided, pretend --text is provided and empty
            handle_result(speech_function_selector(synthesizer, preprocess_text(read_file(args), args)))
    except Exception as e:
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: {e}")
        exit(4)


if __name__ == '__main__':
    main()
