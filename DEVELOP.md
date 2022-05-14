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
    text_to_speech(provider, output, 'Hello world!')
```

## API
