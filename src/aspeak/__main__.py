import argparse
import azure.cognitiveservices.speech as speechsdk
import sys

from . import Synthesizer

parser = argparse.ArgumentParser(description='A simple text-to-speech client using azure TTS API(trial).', prog='aspeak')
parser.add_argument('-v', '--version', action='version', version='%(prog)s 0.1')
parser.add_argument('-t', '--text', help='Text to speak.', dest='text', default=None)
parser.add_argument('-s', '--ssml', help='SSML to speak.', dest='ssml', default=None)
parser.add_argument('-o', '--output', help='Output wav file path', dest='output_path', default=None)

if __name__ == '__main__':
    args = parser.parse_args()
    if args.output_path is None:
        audio_config = speechsdk.audio.AudioOutputConfig(use_default_speaker=True)
    else:
        audio_config = speechsdk.audio.AudioOutputConfig(filename=args.output_path)
    synthesizer = Synthesizer(audio_config)
    if args.text is not None and args.ssml is not None:
        parser.error('`--text` and `--ssml` are mutually exclusive.')
    if args.ssml is not None:
        synthesizer.ssml_to_speech(args.ssml)
    elif args.text is not None:
        synthesizer.text_to_speech(args.text)
    else:
        synthesizer.text_to_speech(sys.stdin.read())
