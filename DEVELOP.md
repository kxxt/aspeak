# `aspeak` Python API

You can use `aspeak` as a Python library, or as a command line tool.

For documentations & examples about the command line interface, see [README.md](README.md).

You can see examples in [`src/examples/`](src/examples) directory.

## Quick Start

```python
import sys

from azure.cognitiveservices.speech import ResultReason
from aspeak import SpeechToSpeakerService

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
import azure.cognitiveservices.speech as speechsdk

audio_format = speechsdk.SpeechSynthesisOutputFormat.Webm24Khz16BitMonoOpus
```

### SpeechService

All

## Deprecated API

### `pure_text_to_speech`

This function is used to synthesize the speech directly from the text, without conversion to SSML.

```python
def pure_text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str,
                        locale: Optional[str] = None, voice: Optional[str] = None,
                        use_async: bool = False,
                        audio_format: Union[
                            AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None)
    -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:


...
```

- `locale` format: e.g. `en-US`, `zh-CN`
- `voice` format: e.g. `en-US-JennyNeural`, execute `aspeak -L` to see available voices.
- `use_async` : set it to `True` for using non-blocking (asynchronous) audio synthesizer
- `audio_format`: See [Custom Audio Format](#custom-audio-format)

If you specify the `voice`, there is no need to specify the `locale`.

### `text_to_speech`

This function is used to synthesize the speech from the text,
with conversion to SSML, which provides more options to customize.

```python
def text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str, voice: str,
                   rate: Union[str, float] = 0.0, pitch: Union[str, float] = 0.0, style: str = "general",
                   style_degree: Optional[float] = None,
                   role: Optional[str] = None,
                   use_async: bool = False,
                   audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None)
    -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:


...
```

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
- `use_async` : set it to `True` for using non-blocking (asynchronous) audio synthesizer
- `audio_format`: See [Custom Audio Format](#custom-audio-format)

### `ssml_to_speech`

This function is used to synthesize the speech from the SSML. Using SSML directly is the most flexible approach.

```python
def ssml_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, ssml: str,
                   audio_format: Union[AudioFormat, FileFormat, speechsdk.SpeechSynthesisOutputFormat, None],
                   use_async: bool = False)
    -> Union[speechsdk.SpeechSynthesisResult, speechsdk.ResultFuture]:


...
```

