# `aspeak` Python API

You can use `aspeak` as a Python library, or as a command line tool.

For documentations & examples about the command line interface, see [README.md](README.md).

You can see examples in [`src/examples/`](src/examples) directory.

## Quick Start

```python
import sys

from aspeak import SpeechToSpeakerService, ResultReason

if __name__ == '__main__':
    try:
        # We need to create a SpeechService instance first.
        # We can use the same instance through the entire lifecycle of the application.
        speech = SpeechToSpeakerService()

        # Call the `text_to_speech` function to synthesize the speech.
        result = speech.text_to_speech('Hello world!', voice='en-US-JennyNeural', style='excited')
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            print("Failed to synthesize speech!", file=sys.stderr)
    except:
        print("Error occurred!", file=sys.stderr)
```

## API

### Custom Audio Format

**Attention:** When outputing to default speaker, using a non-wav format may lead to white noises.

You can specify a custom audio format in the following ways:

1. Specify a file format and use the default quality setting.

```python
from aspeak import FileFormat

audio_format = FileFormat.WAV
```

2. Specify a file format and a quality setting.
    - `quality` is an integer.
    - The default quality level is 0. You can increase/decrease the quality level.
    - To get available quality levels, execute `aspeak -Q`.

```python
from aspeak import AudioFormat, FileFormat

audio_format = AudioFormat(FileFormat.WAV, quality=1)
```

3. (For expert) You can use formats defined in `speechsdk.SpeechSynthesisOutputFormat`.

```python
from aspeak import SpeechSynthesisOutputFormat

audio_format = SpeechSynthesisOutputFormat.Webm24Khz16BitMonoOpus
```

### SpeechService

All speech services inherit from `SpeechServiceBase`.

They all have simlar constructors. You can provide `locale`, `voice` and `audio_format` to them.

If you do not use the `pure_text_to_speech` method, you can ignore `locale` and `voice` parameter
and set `voice` parameter of method `text_to_speech`.

#### Methods

##### `pure_text_to_speech` and `pure_text_to_speech_async`

The above two methods accepts a `text` parameter, which is the text to be synthesized.

These two methods use the voice and locale specified in the constructor.


##### `text_to_speech` and `text_to_speech_async`

The above two methods provide rich text-to-speech features by transforming the text
into ssml internally.

They accept the following parameters:

- `voice` format: e.g. `en-US-JennyNeural`, execute `aspeak -L` to see available voices.
- `rate`: The speaking rate of the voice.
    - You can use a float value or a valid string value.
    - If you use a float value (say `0.5`), the value will be multiplied by 100% and become `50.00%`.
    - Common string values include: `x-slow`, `slow`, `medium`, `fast`, `x-fast`, `default`.
    - You can also use percentage values directly (converted to a string): `"+10%"`.
    - You can also use a relative float value (converted to a string), `"1.2"`:
        - According to
          the [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-prosody)
          ,
        - A relative value, expressed as a number that acts as a multiplier of the default.
        - For example, a value of 1 results in no change in the rate. A value of 0.5 results in a halving of the rate. A
          value of 3 results in a tripling of the rate.
- `pitch`: The pitch of the voice.
    - You can use a float value or a valid string value.
    - If you use a float value (say `-0.5`), the value will be multiplied by 100% and become `-50.00%`.
    - Common string values include: `x-low`, `low`, `medium`, `high`, `x-high`, `default`.
    - You can also use percentage values directly (converted to a string): `"+10%"`.
    - You can also use a relative value wrapped in a string, (e.g. `"-2st"` or `"+80Hz"`):
        - According to
          the [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-prosody)
          ,
        - A relative value, expressed as a number preceded by "+" or "-" and followed by "Hz" or "st" that specifies an
          amount to change the pitch.
        - The "st" indicates the change unit is semitone, which is half of a tone (a half step) on the standard diatonic
          scale.
    - You can also use an absolute value: e.g. `"600Hz"`
- `style`: The style of the voice.
    - You can get a list of available styles for a specific voice by executing `aspeak -L -v <VOICE_ID>`
    - The default value is `general`.
- `style_degree`: The degree of the style.
    - According to the
      [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-speaking-styles)
      , style degree specifies the intensity of the speaking style.
      It is a floating point number between 0.01 and 2, inclusive.
    - At the time of writing, style degree adjustments are supported for Chinese (Mandarin, Simplified) neural voices.
- `role`: The role of the voice.
    - According to the
      [Azure documentation](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-synthesis-markup?tabs=csharp#adjust-speaking-styles)
      , `role` specifies the speaking role-play. The voice acts as a different age and gender, but the voice name isn't
      changed.
    - At the time of writing, role adjustments are supported for these Chinese (Mandarin, Simplified) neural voices:
      `zh-CN-XiaomoNeural`, `zh-CN-XiaoxuanNeural`, `zh-CN-YunxiNeural`, and `zh-CN-YunyeNeural`.

#### `ssml_to_speech` and `ssml_to_speech_async`

The above two methods accepts a `ssml` parameter, which is the text to be synthesized.

Currently, there are three implementations:

#### Implementations

##### SpeechToSpeakerService

Outputs to system speakers.

You can set the desired speaker using parameter `device_name`. 

##### SpeechToFileService

Saves the speech to file.

You need to pass `path` parameter when doing speech synthesis.

##### SpeechToOneFileService

This is the speech service that the CLI is using. It is almost useless for other purposes.

This service outputs to a specific file which is specified when constructing the service.

DO NOT use this service unless you know what you are doing!

#### Extend aspeak by writing your own `SpeechService`

You can write your own speech service by inheriting from `SpeechServiceBase`.

Read [our code](src/aspeak/api/api.py) to see how to implement it.
