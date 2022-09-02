import sys

from azure.cognitiveservices.speech import ResultReason, CancellationReason
from aspeak import SpeechToFileService, AudioFormat, FileFormat

if __name__ == "__main__":
    speech = SpeechToFileService(voice="en-US-JennyNeural", audio_format=AudioFormat(FileFormat.OGG, quality=1))
    texts = ["Put", "here", "the", "sentences", "you", "want"]
    filenames = (f"{i}.ogg" for i in range(len(texts)))
    futures = [speech.text_to_speech_async(text, voice="en-US-JennyNeural", rate="-15%", pitch="-5%", style="cheerful",
                                           path=filename) for
               text, filename in zip(texts, filenames)]
    for future in futures:
        result = future.get()
        if result.reason == ResultReason.SynthesizingAudioCompleted:
            print("C")
            continue
        elif result.reason == ResultReason.Canceled:
            cancellation_details = result.cancellation_details
            print(f"Error: Speech synthesis canceled: {cancellation_details.reason}",
                  file=sys.stderr)
            if cancellation_details.reason == CancellationReason.Error:
                print(cancellation_details.error_details, file=sys.stderr)
        else:
            print(f"Error: Unexpected result reason: {result.reason}", file=sys.stderr)
