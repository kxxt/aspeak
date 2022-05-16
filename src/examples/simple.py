from azure.cognitiveservices.speech.audio import AudioOutputConfig
from azure.cognitiveservices.speech import ResultReason

from aspeak import SpeechServiceProvider, text_to_speech, AudioFormat, FileFormat

provider = SpeechServiceProvider()
output = AudioOutputConfig(use_default_speaker=True)

if __name__ == "__main__":
    try:
        result = text_to_speech(provider, output, "Hello World! I am using aspeak to synthesize speech.",
                                voice="en-US-JennyNeural", rate="+10%", pitch="-5%", style="cheerful",
                                audio_format=AudioFormat(FileFormat.WAV, 1))
        if result.reason != ResultReason.SynthesizingAudioCompleted:
            raise Exception("Failed to synthesize speech.")
    except:
        print("Error occurred while synthesizing speech.")
