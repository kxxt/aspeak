import sys

import azure.cognitiveservices.speech as speechsdk

from .constants import COLOR_CLEAR, COLOR_RED


def handle_result(result: speechsdk.SpeechSynthesisResult):
    if result.reason == speechsdk.ResultReason.SynthesizingAudioCompleted:
        sys.exit(0)
    elif result.reason == speechsdk.ResultReason.Canceled:
        cancellation_details = result.cancellation_details
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Speech synthesis canceled: {cancellation_details.reason}",
              file=sys.stderr)
        if cancellation_details.reason == speechsdk.CancellationReason.Error:
            print(cancellation_details.error_details, file=sys.stderr)
        sys.exit(2)
    else:
        print(f"{COLOR_RED}Error{COLOR_CLEAR}: Unexpected result reason: {result.reason}", file=sys.stderr)
        sys.exit(3)
