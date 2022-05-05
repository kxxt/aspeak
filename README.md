# :speaking_head: aspeak

[![GitHub stars](https://img.shields.io/github/stars/kxxt/aspeak)](https://github.com/kxxt/aspeak/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/kxxt/aspeak)](https://github.com/kxxt/aspeak/issues)
[![GitHub forks](https://img.shields.io/github/forks/kxxt/aspeak)](https://github.com/kxxt/aspeak/network)
[![GitHub license](https://img.shields.io/github/license/kxxt/aspeak)](https://github.com/kxxt/aspeak/blob/main/LICENSE)
[![PyPI version](https://badge.fury.io/py/aspeak.svg)](https://badge.fury.io/py/aspeak)

<a href="https://github.com/kxxt/aspeak/graphs/contributors" alt="Contributors">
    <img src="https://img.shields.io/github/contributors/kxxt/aspeak" />
</a>
<a href="https://github.com/kxxt/aspeak/pulse" alt="Activity">
    <img src="https://img.shields.io/github/commit-activity/m/kxxt/aspeak" />
</a>


A simple text-to-speech client using azure TTS API(trial). :laughing:

**TL;DR**: This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you.

You can try the Azure TTS API online: https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech

## Installation

```sh
$ pip install --upgrade aspeak
```

## Usage

```
usage: aspeak [-h] [-V | -L | [-t [TEXT] | -s [SSML]]] [-p PITCH] [-r RATE] [-S STYLE] [-f FILE] [-e ENCODING] [-o OUTPUT_PATH] [-l LOCALE] [-v VOICE]

This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you

options:
  -h, --help            show this help message and exit
  -V, --version         show program's version number and exit
  -L, --list-voices     list available voices, you can combine this argument with -v and -l
  -t [TEXT], --text [TEXT]
                        Text to speak. Left blank when reading from file/stdin
  -s [SSML], --ssml [SSML]
                        SSML to speak. Left blank when reading from file/stdin
  -f FILE, --file FILE  Text/SSML file to speak, default to `-`(stdin)
  -e ENCODING, --encoding ENCODING
                        Text/SSML file encoding, default to "utf-8"(Not for stdin!)
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        Output wav file path
  -l LOCALE, --locale LOCALE
                        Locale to use, default to en-US
  -v VOICE, --voice VOICE
                        Voice to use

Options for --text:
  -p PITCH, --pitch PITCH
                        Set pitch, default to 0
  -r RATE, --rate RATE  Set speech rate, default to 0.04
  -S STYLE, --style STYLE
                        Set speech style, default to "general"
```

- If you don't specify `-o`, we will use your default speaker.
- If you don't specify `-t` or `-s`, we will assume `-t` is provided.
- You must specify voice if you want to use `-p` or `-r` option.

### Examples

#### Speak "Hello, world!" to default speaker.

```sh
$ aspeak -t "Hello, world!"
```

#### List all available voices.

```sh
$ aspeak -L
```

#### List all available voices for Chinese.

```sh
$ aspeak -L -l zh-CN
```

#### Get information about a voice.

```sh
$ aspeak -L -v en-US-SaraNeural
```

<details>

<summary>
    Output
</summary>

```
Microsoft Server Speech Text to Speech Voice (en-US, SaraNeural)
Display Name: Sara
Local Name: Sara @ en-US
Locale: English (United States)
Gender: Female
ID: en-US-SaraNeural
Styles: ['cheerful', 'angry', 'sad']
Voice Type: Neural
Status: GA
```

</details>

#### Save synthesized speech to a file.

```sh
$ aspeak -t "Hello, world!" -o output.wav
```

#### Read text from file and speak it.

```sh
$ cat input.txt | aspeak
```

or

```sh
$ aspeak -f input.txt
```

with custom encoding:

```sh
$ aspeak -f input.txt -e gbk
```

#### Read from stdin and speak it.

```sh
$ aspeak
```

or (more verbose)

```sh
$ aspeak -f -
```

maybe you prefer:

```sh
$ aspeak -l zh-CN << EOF
我能吞下玻璃而不伤身体。
EOF
```

#### Speak Chinese.

```sh
$ aspeak -t "你好，世界！" -l zh-CN
```

#### Use a custom voice.

```sh
$ aspeak -t "你好，世界！" -v zh-CN-YunjianNeural
```

#### Custom pitch, rate and style

```sh
$ aspeak -t "你好，世界！" -v zh-CN-XiaoxiaoNeural -p 1.5 -r 0.5 -S sad
```

## About This Application

- I found Azure TTS can synthesize nearly authentic human voice, which is very interesting :laughing:.
- I wrote this program to learn Azure Cognitive Services.
- And I use this program daily, because `espeak` and `festival` outputs terrible :fearful: audio.
    - But I respect :raised_hands: their maintainers' work, both are good open source software and they can be used
      off-line.
- I hope you like it :heart:.

## Alternative Applications

- https://github.com/skygongque/tts
- https://github.com/LuckyHookin/edge-TTS-record/
