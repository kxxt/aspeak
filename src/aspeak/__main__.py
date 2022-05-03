import argparse
import azure.cognitiveservices.speech as speechsdk
import sys

from . import Synthesizer
from .ssml import create_ssml
from .voices import format_voice

parser = argparse.ArgumentParser(
    description='This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you',
    prog='aspeak')
group = parser.add_mutually_exclusive_group()
group.add_argument('-V', '--version', action='version', version='%(prog)s 1.1.1')
group.add_argument('-L', '--list-voices', action='store_true',
                   help='list available voices, you can combine this argument with -v and -l', dest='list_voices')
subgroup = group.add_mutually_exclusive_group()
subgroup.add_argument('-t', '--text', help='Text to speak. Left blank when reading from file/stdin',
                      dest='text', nargs='?', default=argparse.SUPPRESS)
subgroup.add_argument('-s', '--ssml', help='SSML to speak. Left blank when reading from file/stdin',
                      dest='ssml', nargs='?', default=argparse.SUPPRESS)
text_group = parser.add_argument_group('Options for --text')
text_group.add_argument('-p', '--pitch', help='Set pitch, default to 0', dest='pitch',
                        type=float, default=argparse.SUPPRESS)
text_group.add_argument('-r', '--rate', help='Set speech rate, default to 0.04', dest='rate',
                        type=float, default=argparse.SUPPRESS)
text_group.add_argument('-S', '--style', help='Set speech style, default to "general"', dest='style',
                        default=argparse.SUPPRESS)
parser.add_argument('-f', '--file', help='Text/SSML file to speak, default to `-`(stdin)', dest='file',
                    default=argparse.SUPPRESS)
parser.add_argument('-o', '--output', help='Output wav file path', dest='output_path', default=None)
parser.add_argument('-l', '--locale', help='Locale to use, default to en-US', dest='locale', default=argparse.SUPPRESS)
parser.add_argument('-v', '--voice', help='Voice to use', dest='voice', default=argparse.SUPPRESS)


def read_file(args):
    if not hasattr(args, 'file') or args.file == '-':
        return sys.stdin.read()
    with open(args.file, 'r') as f:
        return f.read()


def preprocess_text(text, args):
    """
    Preprocess text.
    :param text: plain text
    :param args: args
    :return: (is_ssml, text_or_ssml)
    """
    if hasattr(args, 'pitch') or hasattr(args, 'rate') or hasattr(args, 'style'):
        if args.voice is None:
            parser.error('Voice must be specified when using pitch or rate.')
        pitch = args.pitch if hasattr(args, 'pitch') else 0.0
        rate = args.rate if hasattr(args, 'rate') else 0.04
        voice = args.voice if hasattr(args, 'voice') else None
        style = args.style if hasattr(args, 'style') else 'general'
        ssml = create_ssml(text, voice, rate, pitch, style)
        return True, ssml
    return False, text


def speech_function_selector(synthesizer, preprocessed):
    is_ssml, text_or_ssml = preprocessed
    if is_ssml:
        return synthesizer.ssml_to_speech(text_or_ssml)
    else:
        return synthesizer.text_to_speech(text_or_ssml)


def list_voices(synthesizer, args):
    voices = synthesizer.get_voice_list()
    if hasattr(args, 'voice'):
        voices = [v for v in voices if v["ShortName"] == args.voice]
    if hasattr(args, 'locale'):
        voices = [v for v in voices if v['Locale'] == args.locale]
    for v in voices:
        print(format_voice(v))


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
    if args.output_path is None:
        audio_config = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
    else:
        audio_config = speechsdk.audio.AudioOutputConfig(filename=args.output_path)
    locale = args.locale if hasattr(args, 'locale') else 'en-US'
    voice = args.voice if hasattr(args, 'voice') else None
    try:
        synthesizer = Synthesizer(audio_config, locale, voice)
        if args.list_voices:
            list_voices(synthesizer, args)
            return
        if hasattr(args, 'ssml'):
            if hasattr(args, 'rate') or hasattr(args, 'pitch') or hasattr(args, 'style'):
                parser.error(
                    'You can only use --rate/--pitch/--style with --text. Please set these settings in your SSML.')
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
            handle_result(synthesizer.text_to_speech(read_file(args)))
    except Exception as e:
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: {e}")
        exit(4)


if __name__ == '__main__':
    main()
