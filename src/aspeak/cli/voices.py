import requests
from ..urls import voice_list_url


def format_voice(voice: dict) -> str:
    return f"""{voice["Name"]}
Display Name: {voice["DisplayName"]}
Local Name: {voice["LocalName"]} @ {voice["Locale"]}
Locale: {voice["LocaleName"]}
Gender: {voice["Gender"]}
ID: {voice["ShortName"]}
Styles: {voice.get("StyleList")}
Voice Type: {voice["VoiceType"]}
Status: {voice["Status"]}
"""


def get_voice_list() -> list:
    response = requests.get(voice_list_url(), headers={
                            'Origin': 'https://azure.microsoft.com'})
    return response.json()


def list_voices(args):
    voices = get_voice_list()
    if hasattr(args, 'voice'):
        voices = [v for v in voices if v["ShortName"] == args.voice]
    if hasattr(args, 'locale'):
        voices = [v for v in voices if v['Locale'] == args.locale]
    for voice in voices:
        print(format_voice(voice))
