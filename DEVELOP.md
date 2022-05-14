# `aspeak` Python API

You can use `aspeak` as a Python library, or as a command line tool.

For documentations & examples about the command line interface, see [README.md](README.md).

## Quick Start

```python
from azure.cognitiveservices.speech.audio import AudioOutputConfig
from aspeak import SpeechServiceProvider, text_to_speech

# We need to create a `SpeechServiceProvider` instance first.
# We can use the same instance through the entire lifecycle of the application.

provider = SpeechServiceProvider()

# We need to create an `AudioOutputConfig` instance to configure 
# where the audio output should be sent.

# You can use the default speaker.
output = AudioOutputConfig(use_default_speaker=True)
# Or you can specify the output file.
# output = AudioOutputConfig(filename='output.wav')
# Or you can specify the output stream.
# output = AudioOutputConfig(stream=stream)

if __name__ == '__main__':
    # Call the `text_to_speech` function to synthesize the speech.
    text_to_speech(provider, output, 'Hello world!', 'en-US-JennyNeural')
```

## API

### `pure_text_to_speech`

This function is used to synthesize the speech directly from the text, without conversion to SSML.

```python
def pure_text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str,
                        locale: Union[str, None] = None, voice: Union[str, None] = None,
                        audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> speechsdk.SpeechSynthesisResult:
    ...
```
- `locale` format: e.g. `en-US`, `zh-CN`
- `voice` format: e.g. `en-US-JennyNeural`, execute `aspeak -L` to see available voices.
- `audio_format`: See [AudioFormat](#AudioFormat)

If you specify the `voice`, there is no need to specify the `locale`.

### `text_to_speech`

This function is used to synthesize the speech from the text,
with conversion to SSML, which provides more options to customize.

```python
def text_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, text: str, voice: str,
                   rate: Union[str, float] = 0.0, pitch: Union[str, float] = 0.0, style: str = "general",
                   style_degree: Union[float, None] = None,
                   role: Union[str, None] = None,
                   audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None] = None) \
        -> speechsdk.SpeechSynthesisResult:
    ...
```

### `ssml_to_speech`

This function is used to synthesize the speech from the SSML. Using SSML directly is the most flexible approach.

```python
def ssml_to_speech(provider: SpeechServiceProvider, output: speechsdk.audio.AudioOutputConfig, ssml: str,
                   audio_format: Union[AudioFormat, speechsdk.SpeechSynthesisOutputFormat, None]) \
        -> speechsdk.SpeechSynthesisResult:
    ...
```

