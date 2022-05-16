from functools import partial
from sys import stderr

from azure.cognitiveservices.speech.audio import AudioOutputConfig
from azure.cognitiveservices.speech import ResultReason

from aspeak import SpeechServiceProvider, pure_text_to_speech, AspeakError

provider = SpeechServiceProvider()
output = AudioOutputConfig(use_default_speaker=True)

tts = partial(pure_text_to_speech, provider, output)

if __name__ == "__main__":
    try:
        while True:
            result = tts(input("Enter text to speak: "))
            if result.reason != ResultReason.SynthesizingAudioCompleted:
                print("Error occurred. Please try again.", file=stderr)
    except KeyboardInterrupt:
        print("\nExiting...")
    except AspeakError as e:
        print("\nUnexpected error:", e, file=stderr)
