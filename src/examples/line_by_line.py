from sys import stderr

from aspeak import SpeechToSpeakerService, ResultReason

if __name__ == "__main__":
    try:
        speech = SpeechToSpeakerService()
        while True:
            result = speech.pure_text_to_speech(input("Enter text to speak: "))
            if result.reason != ResultReason.SynthesizingAudioCompleted:
                print("Error occurred. Please try again.", file=stderr)
    except KeyboardInterrupt:
        print("\nExiting...")
    except Exception as e:
        print("\nUnexpected error:", e, file=stderr)
