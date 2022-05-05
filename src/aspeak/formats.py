from azure.cognitiveservices.speech import SpeechSynthesisOutputFormat


def get_available_formats() -> set:
    """
    Returns a set of all available output formats.
    """
    return {
        x.name for x in SpeechSynthesisOutputFormat
        if not x.name.endswith('Siren')  # *Siren is unsupported now
    }
