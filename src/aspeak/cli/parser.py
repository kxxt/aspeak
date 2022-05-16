import argparse
from ..version import version
from .range import Range
# pylint: disable=redefined-builtin
from .value_parsers import pitch, rate, format

parser = argparse.ArgumentParser(
    description='This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you',
    prog='aspeak')
group = parser.add_mutually_exclusive_group()
group.add_argument('-V', '--version', action='version', version=f'%(prog)s {version}')
group.add_argument('-L', '--list-voices', action='store_true',
                   help='list available voices, you can combine this argument with -v and -l', dest='list_voices')
group.add_argument('-Q', '--list-qualities-and-formats', action='store_true',
                   help='list available qualities and formats',
                   dest='list_qualities_and_formats')
subgroup = group.add_mutually_exclusive_group()
subgroup.add_argument('-t', '--text', help='Text to speak. Left blank when reading from file/stdin',
                      dest='text', nargs='?', default=argparse.SUPPRESS)
subgroup.add_argument('-s', '--ssml', help='SSML to speak. Left blank when reading from file/stdin',
                      dest='ssml', nargs='?', default=argparse.SUPPRESS)
text_group = parser.add_argument_group('Options for --text')
text_group.add_argument('-p', '--pitch', help='Set pitch, default to 0', dest='pitch',
                        type=pitch, default=argparse.SUPPRESS)
text_group.add_argument('-r', '--rate', help='Set speech rate, default to 0', dest='rate',
                        type=rate, default=argparse.SUPPRESS)
text_group.add_argument('-S', '--style', help='Set speech style, default to "general"', dest='style',
                        default=argparse.SUPPRESS)
text_group.add_argument('-R', '--role',
                        help='Specifies the speaking role-play. This only works for some Chinese voices!',
                        dest='role', type=str, default=argparse.SUPPRESS,
                        choices=['Girl', 'Boy', 'YoungAdultFemale', 'YoungAdultMale', 'OlderAdultFemale',
                                 'OlderAdultMale', 'SeniorFemale', 'SeniorMale'])
text_group.add_argument('-d', '--style-degree', dest='style_degree', type=float, default=argparse.SUPPRESS,
                        help='Specifies the intensity of the speaking style.'
                             'This only works for some Chinese voices!', choices=[Range(0.01, 2)])
parser.add_argument('-f', '--file', help='Text/SSML file to speak, default to `-`(stdin)', dest='file',
                    default=argparse.SUPPRESS)
parser.add_argument('-e', '--encoding', help='Text/SSML file encoding, default to "utf-8"(Not for stdin!)',
                    dest='encoding', default=argparse.SUPPRESS)
parser.add_argument('-o', '--output', help='Output file path, wav format by default', dest='output_path', default=None)
format_group = parser.add_mutually_exclusive_group()
format_group.add_argument('--mp3', help='Use mp3 format for output. (Only works when outputting to a file)',
                          action='store_true', dest='mp3')
format_group.add_argument('--ogg', help='Use ogg format for output. (Only works when outputting to a file)',
                          action='store_true', dest='ogg')
format_group.add_argument('--webm', help='Use webm format for output. (Only works when outputting to a file)',
                          action='store_true', dest='webm')
format_group.add_argument('--wav', help='Use wav format for output', action='store_true', dest='wav')
format_group.add_argument('-F', '--format', help='Set output audio format (experts only)', dest='format', type=format,
                          default=argparse.SUPPRESS)
parser.add_argument('-l', '--locale', help='Locale to use, default to en-US', dest='locale', default=argparse.SUPPRESS)
parser.add_argument('-v', '--voice', help='Voice to use', dest='voice', default=argparse.SUPPRESS)
parser.add_argument('-q', '--quality', help='Output quality, default to 0', dest='quality', type=int,
                    default=argparse.SUPPRESS)
# pylint: disable=line-too-long
parser.usage = '''usage: aspeak [-h] [-V | -L | -Q | [-t [TEXT] [-p PITCH] [-r RATE] [-S STYLE] [-R ROLE] [-d STYLE_DEGREE] | -s [SSML]]]
              [-f FILE] [-e ENCODING] [-o OUTPUT_PATH] [-l LOCALE] [-v VOICE]
              [--mp3 [-q QUALITY] | --ogg [-q QUALITY] | --webm [-q QUALITY] | --wav [-q QUALITY] | -F FORMAT] 
'''
parser.epilog = 'Attention: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes ' \
                'and the program will not report an error. Please refer to the documentation for other limitations at' \
                ' https://github.com/kxxt/aspeak/blob/main/README.md#limitations'
