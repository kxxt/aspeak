from azure.cognitiveservices.speech.audio import AudioOutputConfig
from azure.cognitiveservices.speech import ResultReason

from aspeak import SpeechServiceProvider, text_to_speech, AudioFormat, FileFormat, AspeakError

output = AudioOutputConfig(use_default_speaker=True)

if __name__ == "__main__":
    try:
        provider = SpeechServiceProvider()
        result = text_to_speech(provider, output, "Hello World! I am using aspeak to synthesize speech.",
                                voice="en-US-JennyNeural", rate="+10%", pitch="-5%", style="cheerful",
                                audio_format=AudioFormat(FileFormat.WAV, 1))
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            print("Failed to synthesize speech.")
    except AspeakError:
        print("Error occurred while synthesizing speech.")
