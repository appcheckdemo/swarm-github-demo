"""Format and parse binary byte sizes."""
import re
from decimal import Decimal, InvalidOperation, ROUND_HALF_UP

_UNITS = ("B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB")
_SIZES = {unit: 1024 ** power for power, unit in enumerate(_UNITS)}


def format_size(bytes_value):
    """Return a non-negative integer byte count in a binary unit."""
    if isinstance(bytes_value, bool) or not isinstance(bytes_value, int):
        raise TypeError("bytes_value must be a non-negative integer")
    if bytes_value < 0:
        raise ValueError("bytes_value must not be negative")
    power = 0
    while power + 1 < len(_UNITS) and bytes_value >= 1024 ** (power + 1):
        power += 1
    if power == 0:
        return "%d B" % bytes_value
    return "%.1f %s" % (bytes_value / (1024 ** power), _UNITS[power])


def parse_size(text):
    """Parse a binary size such as ``1.5 KiB`` and return whole bytes."""
    if not isinstance(text, str) or not text.strip():
        raise ValueError("size must not be empty")
    match = re.fullmatch(r"([+]?(?:\d+(?:\.\d*)?|\.\d+))\s*(B|KiB|MiB|GiB|TiB|PiB|EiB)", text.strip())
    if not match:
        raise ValueError("malformed size")
    try:
        value = Decimal(match.group(1)) * _SIZES[match.group(2)]
    except InvalidOperation as error:
        raise ValueError("malformed size") from error
    return int(value.to_integral_value(rounding=ROUND_HALF_UP))


if __name__ == "__main__":
    assert format_size(0) == "0 B"
    assert format_size(1023) == "1023 B"
    assert format_size(1024) == "1.0 KiB"
    assert format_size(1536) == "1.5 KiB"
    assert format_size(3 * 1024 ** 2) == "3.0 MiB"
    assert parse_size("0 B") == 0
    assert parse_size("1.5 KiB") == 1536
    assert parse_size("2 MiB") == 2 * 1024 ** 2
    assert parse_size("1.5 B") == 2
    for bad in ("", "-1 B", "2 KB", "KiB", "1 MiB junk"):
        try:
            parse_size(bad)
        except ValueError:
            pass
        else:
            raise AssertionError("accepted %r" % bad)
    try:
        format_size(-1)
    except ValueError:
        pass
    else:
        raise AssertionError("accepted negative bytes")
    print("OK")
