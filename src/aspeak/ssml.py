from typing import Optional, Union
from xml.sax.saxutils import escape


# pylint: disable=too-many-arguments
def create_ssml(text: str, voice: Optional[str], rate: Union[float, str], pitch: Union[float, str],
                style: str = "general", style_degree: Optional[float] = None, role: Optional[str] = None) -> str:
    """
    Create SSML for text to be spoken.
    """
    style_degree_text = f'styledegree="{style_degree}"' if style_degree is not None else ""
    role_text = f'role="{role}"' if role is not None else ""
    if isinstance(rate, str):
        rate_text = f'rate="{rate}"'
    else:
        rate_text = f'rate="{round(rate * 100, 2)}%"'
    if isinstance(pitch, str):
        pitch_text = f'pitch="{pitch}"'
    else:
        pitch_text = f'pitch="{round(pitch * 100, 2)}%"'
    ssml = '<speak xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="http://www.w3.org/2001/mstts" ' \
           'xmlns:emo="http://www.w3.org/2009/10/emotionml" version="1.0" xml:lang="en-US"> '
    ssml += f'<voice name="{voice}">' if voice is not None else '<voice>'
    ssml += f'<mstts:express-as style="{style}" {style_degree_text} {role_text}>'
    ssml += f'<prosody {rate_text} {pitch_text}>'
    ssml += escape(text)
    ssml += '</prosody></mstts:express-as></voice></speak>'
    return ssml
