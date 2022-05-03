TRAIL_URL = 'https://azure.microsoft.com/en-us/services/cognitive-services/text-to-speech/'


def voice_list_url(region: str) -> str:
    return f'https://{region}.tts.speech.microsoft.com/cognitiveservices/voices/list'
