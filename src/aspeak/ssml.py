from typing import Union
from xml.sax.saxutils import escape


def create_ssml(text: str, voice: Union[str, None], rate: float, pitch: float, style: str = "general"):
    """
    Create SSML for text to be spoken.
    """
    ssml = '<speak xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="http://www.w3.org/2001/mstts" ' \
           'xmlns:emo="http://www.w3.org/2009/10/emotionml" version="1.0" xml:lang="en-US"> '
    ssml += f'<voice name="{voice}">' if voice is not None else '<voice>'
    ssml += f'<mstts:express-as style="{style}">'
    ssml += f'<prosody rate="{round(rate * 100)}%" pitch="{round(pitch * 100)}%">'
    ssml += escape(text)
    ssml += '</prosody></mstts:express-as></voice></speak>'
    return ssml
