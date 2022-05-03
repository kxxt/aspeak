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
