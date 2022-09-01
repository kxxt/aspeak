from azure.cognitiveservices.speech import ResultReason
from aspeak import SpeechToSpeakerService

if __name__ == "__main__":
    try:
        speech = SpeechToSpeakerService()
        result = speech.text_to_speech("Hello World! I am using aspeak to synthesize speech.",
                                             voice="en-US-JennyNeural", rate="+10%", pitch="-5%", style="cheerful")
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            print("Failed to synthesize speech.")
    except:
        print("Error occurred while synthesizing speech.")
