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


A simple text-to-speech client which enables you to try out Azure's speech synthesis API directly in your terminal. :laughing:

You can try the Azure TTS API online: https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech

## Important Notice

Microsoft made some breaking changes to how the trial page works, which breaks `aspeak < 3.0.0.dev1`.

For old users, to continue to use aspeak, you need to upgrade to v3.0 by executing:

```sh
pip install "aspeak>=3.0"
```

## Installation

```sh
$ pip install -U aspeak
```

## Data Privacy

We don't store your data, and Microsoft doesn't store your data according to information available on
[this page](https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/).

## Limitations

Since we are using Azure Cognitive Services, there are some limitations:

| Quota | Free (F0)<sup>3</sup> |
|--|--|
| Max input length | 1000 characters |
| **Max number of transactions per certain time period per Speech service resource** | |
| Real-time API. Prebuilt neural voices and custom neural voices. | 20 transactions per 60 seconds |
| Adjustable | No<sup>4</sup> |
| **HTTP-specific quotas** | |
| Max audio length produced per request | 10 min |
| Max total number of distinct `<voice>` and `<audio>` tags in SSML | 50 |
| **Websocket specific quotas** | |
| Max audio length produced per turn | 10 min |
| Max total number of distinct `<voice>` and `<audio>` tags in SSML | 50 |
| Max SSML message size per turn | 64 KB |

This table is copied
from [Azure Cognitive Services documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-services-quotas-and-limits#general)

**The 1000 characters limitation was added recently(2022-09-01).**  

And the limitations may be subject to change. The table above might become outdated in the future. Please refer to the
latest [Azure Cognitive Services documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-services-quotas-and-limits#general)
for the latest information.

**Attention**: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes and the program
will not report an error.

## Using `aspeak` as a Python library

See [DEVELOP.md](DEVELOP.md) for more details. You can find examples in `src/examples`.

## Usage

```
usage: aspeak [-h] [-V | -L | -Q | [-t [TEXT] [-p PITCH] [-r RATE] [-S STYLE] [-R ROLE] [-d STYLE_DEGREE] | -s [SSML]]]
              [-f FILE] [-e ENCODING] [-o OUTPUT_PATH] [-l LOCALE] [-v VOICE]
              [--mp3 [-q QUALITY] | --ogg [-q QUALITY] | --webm [-q QUALITY] | --wav [-q QUALITY] | -F FORMAT] 

Try speech synthesis service(Provided by Azure Cognitive Services) in your terminal!

options:
  -h, --help            show this help message and exit
  -V, --version         show program's version number and exit
  -L, --list-voices     list available voices, you can combine this argument with -v and -l
  -Q, --list-qualities-and-formats
                        list available qualities and formats
  -t [TEXT], --text [TEXT]
                        Text to speak. Left blank when reading from file/stdin
  -s [SSML], --ssml [SSML]
                        SSML to speak. Left blank when reading from file/stdin
  -f FILE, --file FILE  Text/SSML file to speak, default to `-`(stdin)
  -e ENCODING, --encoding ENCODING
                        Text/SSML file encoding, default to "utf-8"(Not for stdin!)
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        Output file path, wav format by default
  --mp3                 Use mp3 format for output. (Only works when outputting to a file)
  --ogg                 Use ogg format for output. (Only works when outputting to a file)
  --webm                Use webm format for output. (Only works when outputting to a file)
  --wav                 Use wav format for output
  -F FORMAT, --format FORMAT
                        Set output audio format (experts only)
  -l LOCALE, --locale LOCALE
                        Locale to use, default to en-US
  -v VOICE, --voice VOICE
                        Voice to use
  -q QUALITY, --quality QUALITY
                        Output quality, default to 0

Options for --text:
  -p PITCH, --pitch PITCH
                        Set pitch, default to 0. Valid values include floats(will be converted to percentages), percentages such as 20% and -10%, absolute values like 300Hz, and
                        relative values like -20Hz, +2st and string values like x-low. See the documentation for more details.
  -r RATE, --rate RATE  Set speech rate, default to 0. Valid values include floats(will be converted to percentages), percentages like -20%, floats with postfix "f" (e.g. 2f
                        means doubling the default speech rate), and string values like x-slow. See the documentation for more details.
  -S STYLE, --style STYLE
                        Set speech style, default to "general"
  -R {Girl,Boy,YoungAdultFemale,YoungAdultMale,OlderAdultFemale,OlderAdultMale,SeniorFemale,SeniorMale}, --role {Girl,Boy,YoungAdultFemale,YoungAdultMale,OlderAdultFemale,OlderAdultMale,SeniorFemale,SeniorMale}
                        Specifies the speaking role-play. This only works for some Chinese voices!
  -d {values in range 0.01-2 (inclusive)}, --style-degree {values in range 0.01-2 (inclusive)}
                        Specifies the intensity of the speaking style.This only works for some Chinese voices!

Attention: If the result audio is longer than 10 minutes, the audio will be truncated to 10 minutes and the program will not report an error. Unreasonable high/low values for
pitch and rate will be clipped to reasonable values by Azure Cognitive Services.Please refer to the documentation for other limitations at
https://github.com/kxxt/aspeak/blob/main/README.md#limitations. By the way, we don't store your data, and Microsoft doesn't store your data according to information available on
https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/
```

- If you don't specify `-o`, we will use your default speaker.
- If you don't specify `-t` or `-s`, we will assume `-t` is provided.
- You must specify voice if you want to use special options for `--text`.

### Special Note for Pitch and Rate

- `rate`: The speaking rate of the voice.
  - If you use a float value (say `0.5`), the value will be multiplied by 100% and become `50.00%`.
  - You can use the following values as well: `x-slow`, `slow`, `medium`, `fast`, `x-fast`, `default`.
  - You can also use percentage values directly: `+10%`.
  - You can also use a relative float value (with `f` postfix), `1.2f`: 
    - According to the [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-prosody),
    - A relative value, expressed as a number that acts as a multiplier of the default. 
    - For example, a value of `1f` results in no change in the rate. A value of `0.5f` results in a halving of the rate. A value of `3f` results in a tripling of the rate.
- `pitch`: The pitch of the voice.
  - If you use a float value (say `-0.5`), the value will be multiplied by 100% and become `-50.00%`.
  - You can also use the following values as well: `x-low`, `low`, `medium`, `high`, `x-high`, `default`.
  - You can also use percentage values directly: `+10%`.
  - You can also use a relative value, (e.g. `-2st` or `+80Hz`): 
    - According to the [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-prosody),
    - A relative value, expressed as a number preceded by "+" or "-" and followed by "Hz" or "st" that specifies an amount to change the pitch.
    - The "st" indicates the change unit is semitone, which is half of a tone (a half step) on the standard diatonic scale.
  - You can also use an absolute value: e.g. `600Hz`

**Note**: Unreasonable high/low values will be clipped to reasonable values by Azure Cognitive Services. 


### About Custom Style Degree and Role

According to the
[Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-speaking-styles)
, style degree specifies the intensity of the speaking style.
It is a floating point number between 0.01 and 2, inclusive.

At the time of writing, style degree adjustments are supported for Chinese (Mandarin, Simplified) neural voices.

According to the
[Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-speaking-styles)
, `role` specifies the speaking role-play. The voice acts as a different age and gender, but the voice name isn't
changed.

At the time of writing, role adjustments are supported for these Chinese (Mandarin, Simplified) neural voices:
`zh-CN-XiaomoNeural`, `zh-CN-XiaoxuanNeural`, `zh-CN-YunxiNeural`, and `zh-CN-YunyeNeural`.

### Examples

#### Speak "Hello, world!" to default speaker.

```sh
$ aspeak -t "Hello, world"
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
$ aspeak -t "Hello, world" -o output.wav
```

If you prefer mp3/ogg/webm, you can use `--mp3`/`--ogg`/`--webm` option.

```sh
$ aspeak -t "Hello, world" -o output.mp3 --mp3
$ aspeak -t "Hello, world" -o output.ogg --ogg
$ aspeak -t "Hello, world" -o output.webm --webm
```

#### List available quality levels and formats

```sh
$ aspeak -Q
```

<details>

<summary>Output</summary>

```
Available qualities:
Qualities for wav:
-2: Riff8Khz16BitMonoPcm
-1: Riff16Khz16BitMonoPcm
 0: Riff24Khz16BitMonoPcm
 1: Riff24Khz16BitMonoPcm
Qualities for mp3:
-3: Audio16Khz32KBitRateMonoMp3
-2: Audio16Khz64KBitRateMonoMp3
-1: Audio16Khz128KBitRateMonoMp3
 0: Audio24Khz48KBitRateMonoMp3
 1: Audio24Khz96KBitRateMonoMp3
 2: Audio24Khz160KBitRateMonoMp3
 3: Audio48Khz96KBitRateMonoMp3
 4: Audio48Khz192KBitRateMonoMp3
Qualities for ogg:
-1: Ogg16Khz16BitMonoOpus
 0: Ogg24Khz16BitMonoOpus
 1: Ogg48Khz16BitMonoOpus
Qualities for webm:
-1: Webm16Khz16BitMonoOpus
 0: Webm24Khz16BitMonoOpus
 1: Webm24Khz16Bit24KbpsMonoOpus

Available formats:
- Riff8Khz16BitMonoPcm
- Riff16Khz16BitMonoPcm
- Audio16Khz128KBitRateMonoMp3
- Raw24Khz16BitMonoPcm
- Raw48Khz16BitMonoPcm
- Raw16Khz16BitMonoPcm
- Audio24Khz160KBitRateMonoMp3
- Ogg24Khz16BitMonoOpus
- Audio16Khz64KBitRateMonoMp3
- Raw8Khz8BitMonoALaw
- Audio24Khz16Bit48KbpsMonoOpus
- Ogg16Khz16BitMonoOpus
- Riff8Khz8BitMonoALaw
- Riff8Khz8BitMonoMULaw
- Audio48Khz192KBitRateMonoMp3
- Raw8Khz16BitMonoPcm
- Audio24Khz48KBitRateMonoMp3
- Raw24Khz16BitMonoTrueSilk
- Audio24Khz16Bit24KbpsMonoOpus
- Audio24Khz96KBitRateMonoMp3
- Webm24Khz16BitMonoOpus
- Ogg48Khz16BitMonoOpus
- Riff48Khz16BitMonoPcm
- Webm24Khz16Bit24KbpsMonoOpus
- Raw8Khz8BitMonoMULaw
- Audio16Khz16Bit32KbpsMonoOpus
- Audio16Khz32KBitRateMonoMp3
- Riff24Khz16BitMonoPcm
- Raw16Khz16BitMonoTrueSilk
- Audio48Khz96KBitRateMonoMp3
- Webm16Khz16BitMonoOpus
```

</details>

#### Increase/Decrease audio qualities

```sh
# Less than default quality.
$ aspeak -t "Hello, world" -o output.mp3 --mp3 -q=-1
# Best quality for mp3
$ aspeak -t "Hello, world" -o output.mp3 --mp3 -q=3
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
$ aspeak -t "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=-10% -r=+5% -S cheerful
$ aspeak -t "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=+40Hz -r=1.2f -S fearful
$ aspeak -t "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=high -r=x-slow -S calm
$ aspeak -t "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=+1st -r=-7% -S lyrical
```

### Advanced Usage

#### Use a custom audio format for output

**Note**: When outputing to default speaker, using a non-wav format may lead to white noises.

```sh
$ aspeak -t "Hello World" -F Riff48Khz16BitMonoPcm -o high-quality.wav
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
