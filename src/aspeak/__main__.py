import argparse
import azure.cognitiveservices.speech as speechsdk
import sys

from . import Synthesizer

parser = argparse.ArgumentParser(
    description='This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you.',
    prog='aspeak')
group = parser.add_mutually_exclusive_group()
group.add_argument('-V', '--version', action='version', version='%(prog)s 0.3.0')
subgroup = group.add_mutually_exclusive_group()
subgroup.add_argument('-t', '--text', help='Text to speak. Left blank when reading from file/stdin.',
                      dest='text', nargs='?', default=argparse.SUPPRESS)
subgroup.add_argument('-s', '--ssml', help='SSML to speak. Left blank when reading from file/stdin.',
                      dest='ssml', nargs='?', default=argparse.SUPPRESS)
parser.add_argument('-f', '--file', help='Text/SSML file to speak, default to `-`(stdin).', dest='file',
                    default=argparse.SUPPRESS)
parser.add_argument('-o', '--output', help='Output wav file path', dest='output_path', default=None)
parser.add_argument('-l', '--locale', help='Locale to use, default to en-US', dest='locale', default='en-US')
parser.add_argument('-v', '--voice', help='Voice to use.', dest='voice', default=None)


def read_file(args):
    if not hasattr(args, 'file') or args.file == '-':
        return sys.stdin.read()
    with open(args.file, 'r') as f:
        return f.read()


def main():
    args = parser.parse_args()
    if args.output_path is None:
        audio_config = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
    else:
        audio_config = speechsdk.audio.AudioOutputConfig(filename=args.output_path)
    synthesizer = Synthesizer(audio_config, args.locale, args.voice)
    if hasattr(args, 'ssml'):
        if args.ssml is None:
            # --ssml is provided but empty
            synthesizer.ssml_to_speech(read_file(args))
        else:
            # --ssml is provided and not empty
            if hasattr(args, 'file'):
                parser.error('You can only specify one input source.')
            synthesizer.ssml_to_speech(args.text)
    elif hasattr(args, 'text'):
        if args.text is None:
            # --text is provided but empty
            synthesizer.text_to_speech(read_file(args))
        else:
            # --text is provided and not empty
            if hasattr(args, 'file'):
                parser.error('You can only specify one input source.')
            synthesizer.text_to_speech(args.text)
    else:
        # Neither --text nor --ssml is provided, pretend --text is provided and empty
        synthesizer.text_to_speech(read_file(args))


if __name__ == '__main__':
    main()
