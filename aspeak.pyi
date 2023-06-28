from typing import Any, Optional

class AudioFormat:
    def __init__(
        self, container: str = "mp3", quality: int = 0, use_closest: bool = False
    ) -> None:
        """
        Create an audio format object.

        Args:
            container (Optional[str]): The container format for the audio. It can be either 'mp3', 'wav', 'ogg' or 'webm'.
            quality (Optional[int]): The quality for the audio. Defaults to 0.
            use_closest (Optional[bool]): Whether to use the closest quality if the specified quality does not exist. Defaults to False.
        """
    AmrWb16000Hz: AudioFormat
    Audio16Khz128KBitRateMonoMp3: AudioFormat
    Audio16Khz16Bit32KbpsMonoOpus: AudioFormat
    Audio16Khz32KBitRateMonoMp3: AudioFormat
    Audio16Khz64KBitRateMonoMp3: AudioFormat
    Audio24Khz160KBitRateMonoMp3: AudioFormat
    Audio24Khz16Bit24KbpsMonoOpus: AudioFormat
    Audio24Khz16Bit48KbpsMonoOpus: AudioFormat
    Audio24Khz48KBitRateMonoMp3: AudioFormat
    Audio24Khz96KBitRateMonoMp3: AudioFormat
    Audio48Khz192KBitRateMonoMp3: AudioFormat
    Audio48Khz96KBitRateMonoMp3: AudioFormat
    Ogg16Khz16BitMonoOpus: AudioFormat
    Ogg24Khz16BitMonoOpus: AudioFormat
    Ogg48Khz16BitMonoOpus: AudioFormat
    Raw16Khz16BitMonoPcm: AudioFormat
    Raw16Khz16BitMonoTrueSilk: AudioFormat
    Raw22050Hz16BitMonoPcm: AudioFormat
    Raw24Khz16BitMonoPcm: AudioFormat
    Raw24Khz16BitMonoTrueSilk: AudioFormat
    Raw44100Hz16BitMonoPcm: AudioFormat
    Raw48Khz16BitMonoPcm: AudioFormat
    Raw8Khz16BitMonoPcm: AudioFormat
    Raw8Khz8BitMonoALaw: AudioFormat
    Raw8Khz8BitMonoMULaw: AudioFormat
    Riff16Khz16BitMonoPcm: AudioFormat
    Riff22050Hz16BitMonoPcm: AudioFormat
    Riff24Khz16BitMonoPcm: AudioFormat
    Riff44100Hz16BitMonoPcm: AudioFormat
    Riff48Khz16BitMonoPcm: AudioFormat
    Riff8Khz16BitMonoPcm: AudioFormat
    Riff8Khz8BitMonoALaw: AudioFormat
    Riff8Khz8BitMonoMULaw: AudioFormat
    Webm16Khz16BitMonoOpus: AudioFormat
    Webm24Khz16Bit24KbpsMonoOpus: AudioFormat
    Webm24Khz16BitMonoOpus: AudioFormat

class Role:
    Girl: Role
    Boy: Role
    YoungAdultFemale: Role
    YoungAdultMale: Role
    OlderAdultFemale: Role
    OlderAdultMale: Role
    SeniorFemale: Role
    SeniorMale: Role

class SpeechService:
    def __init__(self, audio_format: AudioFormat, **options: Any) -> None:
        """
        Create a speech service object.

        Args:
            audio_format (AudioFormat): The audio format for the output audio.

        Kwargs:
            endpoint (Optional[str]): The endpoint for the speech service. You must specify this if do not specify the region.
            region (Optional[str]): The region for the speech service.
            mode (Optional[str]): The mode for the speech service. It can be either 'rest' or 'websocket'. Defaults to 'rest'.
                                  In websocket mode, the websocket connection will be established when this object is created.
            key (Optional[str]): The subscription key for the speech service.
            token (Optional[str]): The auth token for the speech service.
            proxy (Optional[str]): The proxy for the speech service. Only http/socks5 proxy servers are supported by now.
            headers (Optional[Iterable[Tuple[str, str]]]): Additional request headers.
        """
    def speak_text(self, text: str, **options: Any) -> None:
        """
        Synthesize text to speech and output to speaker.

        Args:
            text (str): The text to synthesize.

        Kwargs:
            pitch (Optional[str]): The pitch for the speech.
            rate (Optional[str]): The rate for the speech.
            locale (Optional[str]): The locale for the speech.
            voice (Optional[str]): The voice to be used. It takes precedence over locale.
            style (Optional[str]): Speech style.
            style_degree (Optional[float]): Speech style degree. It can be a float number between 0.01 and 2.
            role (Optional[Role]): Speech role.
        """
    def synthesize_text(self, text: str, **options: Any) -> Optional[bytes]:
        """
        Synthesize text to speech.

        Args:
            text (str): The text to synthesize.

        Kwargs:
            output (Optional[str]): The output file path. If this argument is not specified, the audio data will be returned.
            pitch (Optional[str]): The pitch for the speech.
            rate (Optional[str]): The rate for the speech.
            locale (Optional[str]): The locale for the speech.
            voice (Optional[str]): The voice to be used. It takes precedence over locale.
            style (Optional[str]): Speech style.
            style_degree (Optional[float]): Speech style degree. It can be a float number between 0.01 and 2.
            role (Optional[Role]): Speech role.
        """
    def speak_ssml(self, ssml: str) -> None:
        """
        Synthesize SSML to speech and output to speaker.

        Args:
            ssml (str): The SSML to synthesize.
        """
    def synthesize_ssml(self, ssml: str, **options: Any) -> Optional[bytes]:
        """
        Synthesize SSML to speech.

        Args:
            ssml (str): The SSML to synthesize.

        Kwargs:
            output (Optional[str]): The output file path. If this argument is not specified, the audio data will be returned.
        """
