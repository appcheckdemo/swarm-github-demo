"""Small, standard-library-only byte-size conversions."""
import re

UNITS = ("B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB")
_SIZE = re.compile(r"^([0-9]+(?:\.[0-9]+)?)\s*(B|KiB|MiB|GiB|TiB|PiB|EiB|ZiB|YiB)$",
                   re.I)


def humansize(value):
    """Format a non-negative byte count using binary units."""
    if isinstance(value, bool) or not isinstance(value, (int, float)) or value < 0:
        raise ValueError("byte count must be non-negative")
    unit = 0
    while value >= 1024 and unit < len(UNITS) - 1:
        value /= 1024
        unit += 1
    return f"{value:.1f} {UNITS[unit]}" if unit else f"{int(value)} B"


def human_to_bytes(text):
    """Parse a binary-unit size, returning the nearest whole byte."""
    if not isinstance(text, str):
        raise ValueError("size must be text")
    match = _SIZE.fullmatch(text.strip())
    if not match:
        raise ValueError("invalid human size")
    number, unit = match.groups()
    index = next(i for i, name in enumerate(UNITS) if name.lower() == unit.lower())
    return int(float(number) * 1024 ** index + 0.5)


bytes_to_human = humansize
parse_humansize = human_to_bytes


if __name__ == "__main__":
    assert humansize(0) == "0 B"
    assert humansize(1536) == "1.5 KiB"
    assert humansize(3 * 1024 ** 2) == "3.0 MiB"
    assert humansize(1024 ** 8) == "1.0 YiB"
    assert human_to_bytes(" 1.5 KiB ") == 1536
    assert human_to_bytes("3.0 MiB") == 3 * 1024 ** 2
    assert human_to_bytes("0.5 KiB") == 512
    assert human_to_bytes("1 B") == 1
    try:
        human_to_bytes("")
    except ValueError:
        pass
    else:
        raise AssertionError("empty input must be rejected")
    print("OK")
