from azure.cognitiveservices.speech import SpeechSynthesisOutputFormat

QUALITIES = {
    'wav': {
        -2: SpeechSynthesisOutputFormat.Riff8Khz16BitMonoPcm,
        -1: SpeechSynthesisOutputFormat.Riff16Khz16BitMonoPcm,
        0: SpeechSynthesisOutputFormat.Riff24Khz16BitMonoPcm,
        1: SpeechSynthesisOutputFormat.Riff24Khz16BitMonoPcm,
    },
    'mp3': {
        -4: SpeechSynthesisOutputFormat.Audio16Khz32KBitRateMonoMp3,
        -3: SpeechSynthesisOutputFormat.Audio16Khz64KBitRateMonoMp3,
        -2: SpeechSynthesisOutputFormat.Audio16Khz128KBitRateMonoMp3,
        -1: SpeechSynthesisOutputFormat.Audio24Khz48KBitRateMonoMp3,
        0: SpeechSynthesisOutputFormat.Audio24Khz96KBitRateMonoMp3,
        1: SpeechSynthesisOutputFormat.Audio24Khz160KBitRateMonoMp3,
        2: SpeechSynthesisOutputFormat.Audio48Khz96KBitRateMonoMp3,
        3: SpeechSynthesisOutputFormat.Audio48Khz192KBitRateMonoMp3,
    },
    'ogg': {
        -1: SpeechSynthesisOutputFormat.Ogg16Khz16BitMonoOpus,
        0: SpeechSynthesisOutputFormat.Ogg24Khz16BitMonoOpus,
        1: SpeechSynthesisOutputFormat.Ogg48Khz16BitMonoOpus,
    },
    'webm': {
        -1: SpeechSynthesisOutputFormat.Webm16Khz16BitMonoOpus,
        0: SpeechSynthesisOutputFormat.Webm24Khz16BitMonoOpus,
        1: SpeechSynthesisOutputFormat.Webm24Khz16Bit24KbpsMonoOpus,
    }
}
