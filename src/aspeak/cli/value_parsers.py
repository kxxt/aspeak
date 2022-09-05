import azure.cognitiveservices.speech as speechsdk


def try_parse_float(arg: str):
    try:
        return True, float(arg)
    except ValueError:
        return False, None


def error(arg: str):
    return ValueError('Invalid value: ' + arg)


def pitch(arg: str):
    if arg.endswith('Hz') and try_parse_float(arg[:-2])[0]:
        # 1. Absolute value: 400Hz
        # 2. Relative value: +10Hz, -20Hz
        return arg
    if arg.endswith('%') and try_parse_float(arg[:-1])[0]:
        # Percentage values
        return arg
    if arg.endswith('st') and try_parse_float(arg[:-2])[0] and arg[0] in {'+', '-'}:
        # Relative value: +1st, -2st
        return arg
    is_float, value = try_parse_float(arg)
    if is_float:
        return value
    if arg in {'default', 'x-low', 'low', 'medium', 'high', 'x-high'}:
        return arg
    raise error(arg)


def rate(arg: str):
    if arg.endswith('%') and try_parse_float(arg[:-1])[0]:
        # Percentage values
        return arg
    if arg in {"default", "x-slow", "slow", "medium", "fast", "x-fast"}:
        # enum values
        return arg
    is_float, value = try_parse_float(arg)
    if is_float:
        # float values that will be converted to percentages
        return value
    if arg.endswith('f') and try_parse_float(arg[:-1])[0]:
        # raw float values
        return arg[:-1]
    raise error(arg)


# `format` will appear in the cli error messages, so we need to keep this name, although it shallows the builtin.
# noinspection PyShadowingBuiltins
# pylint: disable=redefined-builtin
def format(arg: str):
    if arg in speechsdk.SpeechSynthesisOutputFormat.__members__:
        return arg
    raise error(arg)
