from aspeak import SpeechToSpeakerService, ResultReason

if __name__ == "__main__":
    try:
        speech = SpeechToSpeakerService()
        result = speech.text_to_speech_async("Hello World! I am using aspeak to synthesize speech.",
                                             voice="en-US-JennyNeural", rate="+10%", pitch="-5%", style="cheerful")
        print("Synthesis started and the workflow isn't blocked.")
        result = result.get()  # Wait for the synthesis to complete
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            print("Failed to synthesize speech.")
    except:
        print("Error occurred while synthesizing speech.")
