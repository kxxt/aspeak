from typing import Union
from xml.sax.saxutils import escape


def create_ssml(text: str, voice: Union[str, None], rate: float, pitch: float, style: str = "general",
                style_degree: Union[float, None] = None, role: Union[str, None] = None) -> str:
    """
    Create SSML for text to be spoken.
    """
    style_degree_text = f'styledegree="{style_degree}"' if style_degree is not None else ""
    role_text = f'role="{role}"' if role is not None else ""
    ssml = '<speak xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="http://www.w3.org/2001/mstts" ' \
           'xmlns:emo="http://www.w3.org/2009/10/emotionml" version="1.0" xml:lang="en-US"> '
    ssml += f'<voice name="{voice}">' if voice is not None else '<voice>'
    ssml += f'<mstts:express-as style="{style}" {style_degree_text} {role_text}>'
    ssml += f'<prosody rate="{round(rate * 100)}%" pitch="{round(pitch * 100)}%">'
    ssml += escape(text)
    ssml += '</prosody></mstts:express-as></voice></speak>'
    return ssml
