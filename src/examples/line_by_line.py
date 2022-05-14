from aspeak import SpeechServiceProvider, pure_text_to_speech
from azure.cognitiveservices.speech.audio import AudioOutputConfig
from functools import partial

provider = SpeechServiceProvider()
output = AudioOutputConfig(use_default_speaker=True)

tts = partial(pure_text_to_speech, provider, output)

if __name__ == "__main__":
    try:
        while True:
            tts(input("Enter text to speak: "))
    except KeyboardInterrupt:
        print("\nExiting...")
