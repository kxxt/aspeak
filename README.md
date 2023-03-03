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

**Please note that the rust rewritten version is experimental and might have bugs!**

## Installation

### Download from GitHub Releases

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

### Examples

#### Speak "Hello, world!" to default speaker.

```sh
$ aspeak text "Hello, world"
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

TODO

```python
from aspeak import SpeechService, AudioFormat

service = SpeechService()
service.connect()
```

### Rust

Add `aspeak` to your `Cargo.toml`:

```bash
$ cargo add aspeak
```

Then you can use it in your code:

TODO
