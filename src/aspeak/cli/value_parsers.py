def try_parse_float(s: str):
    try:
        return True, float(s)
    except ValueError:
        return False, None


def error(s: str):
    return ValueError('Invalid value: ' + s)


def pitch(s: str):
    if s.endswith('Hz') and try_parse_float(s[:-2])[0]:
        # 1. Absolute value: 400Hz
        # 2. Relative value: +10Hz, -20Hz
        return s
    if s.endswith('%') and try_parse_float(s[:-1])[0]:
        # Percentage values
        return s
    if s.endswith('st') and try_parse_float(s[:-2])[0] and s[0] in {'+', '-'}:
        # Relative value: +1st, -2st
        return s
    if (result := try_parse_float(s)) and result[0]:
        return result[1]
    if s in {'default', 'x-low', 'low', 'medium', 'high', 'x-high'}:
        return s
    raise error(s)


def rate(s: str):
    return s


# noinspection PyShadowingBuiltins
def format(s: str):
    return s
