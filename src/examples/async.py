from azure.cognitiveservices.speech import ResultReason, audio
from aspeak import SpeechServiceProvider, text_to_speech, AspeakError

if __name__ == "__main__":
    try:
        output = audio.AudioOutputConfig(use_default_speaker=True)
        provider = SpeechServiceProvider()
        result = text_to_speech(provider, output, "Hello World! I am using aspeak to synthesize speech.",
                                use_async=True,
                                voice="en-US-JennyNeural", rate="+10%", pitch="-5%", style="cheerful")
        print("Synthesis started and the workflow isn't blocked.")
        result = result.get()  # Wait for the synthesis to complete
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            print("Failed to synthesize speech.")
    except AspeakError:
        print("Error occurred while synthesizing speech.")
