# aspeak

A simple text-to-speech client using azure TTS API(trial).

**TL;DR**: This program uses trial auth token of Azure Congnitive Services to do speech synthesis for you.

## Installation

```sh
$ pip install aspeak
```

## Usage

```
usage: aspeak [-h] [-v] [-t TEXT] [-s SSML] [-o OUTPUT_PATH]

A simple text-to-speech client using azure TTS API(trial).

options:
  -h, --help            show this help message and exit
  -v, --version         show program's version number and exit
  -t TEXT, --text TEXT  Text to speak.
  -s SSML, --ssml SSML  SSML to speak.
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        Output wav file path
```

e.g.

```sh
$ aspeak -t "Hello, world!" -o output.wav
```

- If you don't specify `-o`, we will use your default speaker.
- If you don't specify `-t` or `-s`, we will read from stdin until `EOF`.
