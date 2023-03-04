# :speaking_head: aspeak

[![GitHub stars](https://img.shields.io/github/stars/kxxt/aspeak)](https://github.com/kxxt/aspeak/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/kxxt/aspeak)](https://github.com/kxxt/aspeak/issues)
[![GitHub forks](https://img.shields.io/github/forks/kxxt/aspeak)](https://github.com/kxxt/aspeak/network)
[![GitHub license](https://img.shields.io/github/license/kxxt/aspeak)](https://github.com/kxxt/aspeak/blob/main/LICENSE)

<a href="https://github.com/kxxt/aspeak/graphs/contributors" alt="Contributors">
    <img src="https://img.shields.io/github/contributors/kxxt/aspeak" />
</a>
<a href="https://github.com/kxxt/aspeak/pulse" alt="Activity">
    <img src="https://img.shields.io/github/commit-activity/m/kxxt/aspeak" />
</a>

A simple text-to-speech client for Azure TTS API. :laughing:

You can try the Azure TTS API online: https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech

## Note

Starting from version 4.0.0, `aspeak` is rewritten in rust. The old python version is available at the `python` branch.

## Installation

### Download from GitHub Releases (Recommended for most users)

Download the latest release from [here](https://github.com/kxxt/aspeak/releases/latest).

After downloading, extract the archive and you will get a binary executable file.

You can put it in a directory that is in your `PATH` environment variable so that you can run it from anywhere.

### Install from PyPI

Installing from PyPI will also install the python binding of `aspeak` for you. Check [Library Usage#Python](#Python) for more information on using the python binding.

```bash
pip install -U aspeak
```

Now the prebuilt wheels are only available for x86_64 architecture.
Due to some technical issues, I haven't uploaded the source distribution to PyPI yet.
So to build wheel from source, you need to follow the instructions in [Install from Source](#Install-from-Source).

Because of manylinux compatibility issues, the wheels for linux are not available on PyPI. (But you can still build them from source.)

### Install from Source

#### CLI Only

The easiest way to install `aspeak` is to use cargo:

```bash
cargo install aspeak
```

#### Python Wheel

To build the python wheel, you need to install `maturin` first:

```bash
pip install maturin
```

After cloning the repository and `cd` into the directory
, you can build the wheel by running:

```bash
maturin build --release --strip -F python --bindings pyo3 --interpreter python --manifest-path Cargo.toml --out dist-pyo3
maturin build --release --strip --bindings  bin --interpreter python --manifest-path Cargo.toml --out dist-bin
bash merge-wheel.bash
```

If everything goes well, you will get a wheel file in the `dist` directory.

## Usage

Run `aspeak help` to see the help message.

Run `aspeak help <subcommand>` to see the help message of a subcommand.

### Configuration

You can configure `aspeak` by creating a profile. Run the following command to create a profile:

```sh
$ aspeak config init
```

To edit the profile, run:

```sh
$ aspeak config edit
```

If you have trouble running the above command, you can edit the profile manually:

Fist get the path of the profile by running:

```sh
$ aspeak config where
```

Then edit the file with your favorite text editor.

The profile is a TOML file. The default profile looks like this:

Check the comments in the config file for more information about available options.

```toml
# Profile for aspeak
# GitHub: https://github.com/kxxt/aspeak

# Output verbosity
# 0   - Default
# 1   - Verbose
# The following output verbosity levels are only supported on debug build
# 2   - Debug
# >=3 - Trace
verbosity = 0

#
# Authentication configuration
#

[auth]
# Endpoint for TTS
# endpoint = "wss://eastus.api.speech.microsoft.com/cognitiveservices/websocket/v1"

# Alternatively, you can specify the region if you are using official endpoints
# region = "eastus"

# Azure Subscription Key
# key = "YOUR_KEY"

# Authentication Token
# token = "Your Authentication Token"

# Extra http headers (for experts)
# headers = [["X-My-Header", "My-Value"], ["X-My-Header2", "My-Value2"]]

#
# Configuration for text subcommand
#

[text]
# Voice to use. Note that it takes precedence over the locale
# voice = "en-US-JennyNeural"
# Locale to use
locale = "en-US"
# Rate
rate = 0
# Pitch
pitch = 0
# Role
role = "Boy"
# Style, "general" by default
style = "general"
# Style degree, a floating-point number between 0.1 and 2.0
# style_degree = 1.0

#
# Output Configuration
#

[output]
# Container Format, Only wav/mp3/ogg/webm is supported.
container = "wav"
# Audio Quality. Run `aspeak list-qualities` to see available qualities.
#
# If you choose a container format that does not support the quality level you specified here, 
# we will automatically select the closest level for you.
quality = 0
# Audio Format(for experts). Run `aspeak list-formats` to see available formats.
# Note that it takes precedence over container and quality!
# format = "audio-16khz-128kbitrate-mono-mp4"
```

### Pitch and Rate

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

### Examples

#### Speak "Hello, world!" to default speaker.

```sh
$ aspeak text "Hello, world"
```

#### SSML to Speech

```sh
$ aspeak ssml << EOF 
<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'><voice name='en-US-JennyNeural'>Hello, world!</voice></speak>
EOF
```

#### List all available voices.

```sh
$ aspeak list-voices
```

#### List all available voices for Chinese.

```sh
$ aspeak list-voices -l zh-CN
```

#### Get information about a voice.

```sh
$ aspeak list-voices -v en-US-SaraNeural
```

<details>

<summary>
    Output
</summary>

```
Microsoft Server Speech Text to Speech Voice (en-US, SaraNeural)
Display name: Sara
Local name: Sara @ en-US
Locale: English (United States)
Gender: Female
ID: en-US-SaraNeural
Voice type: Neural
Status: GA
Sample rate: 48000Hz
Words per minute: 157
Styles: ["angry", "cheerful", "excited", "friendly", "hopeful", "sad", "shouting", "terrified", "unfriendly", "whispering"]
```

</details>

#### Save synthesized speech to a file.

```sh
$ aspeak text "Hello, world" -o output.wav
```

If you prefer mp3/ogg/webm, you can use `-c mp3`/`-c ogg`/`-c webm` option.

```sh
$ aspeak text "Hello, world" -o output.mp3 -c mp3
$ aspeak text "Hello, world" -o output.ogg -c ogg
$ aspeak text "Hello, world" -o output.webm -c webm
```

#### List available quality levels

```sh
$ aspeak list-qualities
```

<details>

<summary>Output</summary>

```
Qualities for MP3:
  3: audio-48khz-192kbitrate-mono-mp3
  2: audio-48khz-96kbitrate-mono-mp3
 -3: audio-16khz-64kbitrate-mono-mp3
  1: audio-24khz-160kbitrate-mono-mp3
 -2: audio-16khz-128kbitrate-mono-mp3
 -4: audio-16khz-32kbitrate-mono-mp3
 -1: audio-24khz-48kbitrate-mono-mp3
  0: audio-24khz-96kbitrate-mono-mp3

Qualities for WAV:
 -2: riff-8khz-16bit-mono-pcm
  1: riff-24khz-16bit-mono-pcm
  0: riff-24khz-16bit-mono-pcm
 -1: riff-16khz-16bit-mono-pcm

Qualities for OGG:
  0: ogg-24khz-16bit-mono-opus
 -1: ogg-16khz-16bit-mono-opus
  1: ogg-48khz-16bit-mono-opus

Qualities for WEBM:
  0: webm-24khz-16bit-mono-opus
 -1: webm-16khz-16bit-mono-opus
  1: webm-24khz-16bit-24kbps-mono-opus
```

</details>

#### List available audio formats (For expert users)

```sh
$ aspeak list-formats
```

<details>

<summary>Output</summary>

```
amr-wb-16000hz
audio-16khz-128kbitrate-mono-mp3
audio-16khz-16bit-32kbps-mono-opus
audio-16khz-32kbitrate-mono-mp3
audio-16khz-64kbitrate-mono-mp3
audio-24khz-160kbitrate-mono-mp3
audio-24khz-16bit-24kbps-mono-opus
audio-24khz-16bit-48kbps-mono-opus
audio-24khz-48kbitrate-mono-mp3
audio-24khz-96kbitrate-mono-mp3
audio-48khz-192kbitrate-mono-mp3
audio-48khz-96kbitrate-mono-mp3
ogg-16khz-16bit-mono-opus
ogg-24khz-16bit-mono-opus
ogg-48khz-16bit-mono-opus
raw-16khz-16bit-mono-pcm
raw-16khz-16bit-mono-truesilk
raw-22050hz-16bit-mono-pcm
raw-24khz-16bit-mono-pcm
raw-24khz-16bit-mono-truesilk
raw-44100hz-16bit-mono-pcm
raw-48khz-16bit-mono-pcm
raw-8khz-16bit-mono-pcm
raw-8khz-8bit-mono-alaw
raw-8khz-8bit-mono-mulaw
riff-16khz-16bit-mono-pcm
riff-22050hz-16bit-mono-pcm
riff-24khz-16bit-mono-pcm
riff-44100hz-16bit-mono-pcm
riff-48khz-16bit-mono-pcm
riff-8khz-16bit-mono-pcm
riff-8khz-8bit-mono-alaw
riff-8khz-8bit-mono-mulaw
webm-16khz-16bit-mono-opus
webm-24khz-16bit-24kbps-mono-opus
webm-24khz-16bit-mono-opus
```

</details>

#### Increase/Decrease audio qualities

```sh
# Less than default quality.
$ aspeak text "Hello, world" -o output.mp3 -c mp3 -q=-1
# Best quality for mp3
$ aspeak text "Hello, world" -o output.mp3 -c mp3 -q=3
```

#### Read text from file and speak it.

```sh
$ cat input.txt | aspeak text
```

or

```sh
$ aspeak text -f input.txt
```

with custom encoding:

```sh
$ aspeak text -f input.txt -e gbk
```

#### Read from stdin and speak it.

```sh
$ aspeak text
```

maybe you prefer:

```sh
$ aspeak text -l zh-CN << EOF
我能吞下玻璃而不伤身体。
EOF
```

#### Speak Chinese.

```sh
$ aspeak text "你好，世界！" -l zh-CN
```

#### Use a custom voice.

```sh
$ aspeak text "你好，世界！" -v zh-CN-YunjianNeural
```

#### Custom pitch, rate and style

```sh
$ aspeak text "你好，世界！" -v zh-CN-XiaoxiaoNeural -p 1.5 -r 0.5 -S sad
$ aspeak text "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=-10% -r=+5% -S cheerful
$ aspeak text "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=+40Hz -r=1.2f -S fearful
$ aspeak text "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=high -r=x-slow -S calm
$ aspeak text "你好，世界！" -v zh-CN-XiaoxiaoNeural -p=+1st -r=-7% -S lyrical
```

### Advanced Usage

#### Use a custom audio format for output

**Note**: Some audio formats are not supported when you are outputting to speaker.

```sh
$ aspeak text "Hello World" -F riff-48khz-16bit-mono-pcm -o high-quality.wav
```

## Library Usage

### Python

The new version of `aspeak` is written in Rust, and the Python binding is provided by PyO3.

Here is a simple example:

```python
from aspeak import SpeechService

service = SpeechService()
service.connect()
service.speak_text("Hello, world")
```

First you need to create a `SpeechService` instance.

When creating a `SpeechService` instance, you can specify the following parameters:

- `audio_format`: The audio format of the output audio. Default is `AudioFormat.Riff24KHz16BitMonoPcm`.
  - You can get an audio format by providing a container format and a quality level: `AudioFormat("mp3", 2)`.
- `endpoint`: The endpoint of the speech service. We will use a trial endpoint by default.
- `region`: Alternatively, you can specify the region of the speech service instead of typing the boring endpoint url.
- `subscription_key`: The subscription key of the speech service. **This option doesn't work yet**
- `token`: The auth token for the speech service. 
- `headers`: Additional HTTP headers for the speech service.

Then you need to call `connect()` to connect to the speech service.

After that, you can call `speak_text()` to speak the text or `speak_ssml()` to speak the SSML.
Or you can call `synthesize_text()` or `synthesize_ssml()` to get the audio data.

For `synthesize_text()` and `synthesize_ssml()`, if you provide an `output`, the audio data will be written to that file and the function will return `None`. Otherwise, the function will return the audio data.

Here are the common options for `speak_text()` and `synthesize_text()`:

- `locale`: The locale of the voice. Default is `en-US`.
- `voice`: The voice name. Default is `en-US-JennyNeural`.
- `rate`: The speaking rate of the voice. It must be a string that fits the requirements as documented in this section: [Pitch and Rate](#pitch-and-rate)
- `pitch`: The pitch of the voice. It must be a string that fits the requirements as documented in this section: [Pitch and Rate](#pitch-and-rate)
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


### Rust

Add `aspeak` to your `Cargo.toml`:

```bash
$ cargo add aspeak
```

Then follow the [documentation](https://docs.rs/aspeak) of `aspeak` crate.
