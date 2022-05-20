from functools import partial
from sys import stderr

from azure.cognitiveservices.speech.audio import AudioOutputConfig
from azure.cognitiveservices.speech import ResultReason

from aspeak import SpeechServiceProvider, pure_text_to_speech, AspeakError

output = AudioOutputConfig(use_default_speaker=True)

if __name__ == "__main__":
    try:
        provider = SpeechServiceProvider()
        tts = partial(pure_text_to_speech, provider, output)
        while True:
            result = tts(input("Enter text to speak: "))
            if result.reason != ResultReason.SynthesizingAudioCompleted:
                print("Error occurred. Please try again.", file=stderr)
    except KeyboardInterrupt:
        print("\nExiting...")
    except AspeakError as e:
        print("\nUnexpected error:", e, file=stderr)
