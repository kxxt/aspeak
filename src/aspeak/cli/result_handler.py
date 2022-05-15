import sys
import azure.cognitiveservices.speech as speechsdk

from .constants import COLOR_CLEAR, COLOR_RED


def handle_result(r: speechsdk.SpeechSynthesisResult):
    if r.reason == speechsdk.ResultReason.SynthesizingAudioCompleted:
        exit(0)
    elif r.reason == speechsdk.ResultReason.Canceled:
        cancellation_details = r.cancellation_details
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Speech synthesis canceled: {cancellation_details.reason}",
              file=sys.stderr)
        if cancellation_details.reason == speechsdk.CancellationReason.Error:
            print(cancellation_details.error_details, file=sys.stderr)
        exit(2)
    else:
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Unexpected result reason: {r.reason}", file=sys.stderr)
        exit(3)
