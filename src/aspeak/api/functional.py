from .provider import SpeechServiceProvider


def text_to_speech(provider: SpeechServiceProvider, text: str) -> None:
    pass


async def text_to_speech_async(provider: SpeechServiceProvider, text: str) -> None:
    pass


def ssml_to_speech(provider: SpeechServiceProvider, ssml: str) -> None:
    pass


async def ssml_to_speech_async(provider: SpeechServiceProvider, ssml: str) -> None:
    pass
