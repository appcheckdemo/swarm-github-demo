"""Format and parse binary byte sizes."""

import re
from decimal import Decimal, InvalidOperation

_UNITS = ("B", "KiB", "MiB", "GiB", "TiB")
_PATTERN = re.compile(r"^(\d+)(?:\.(\d))? (B|KiB|MiB|GiB|TiB)$")


def format_size(bytes_count):
    """Return *bytes_count* in a compact binary unit."""
    if isinstance(bytes_count, bool) or not isinstance(bytes_count, int):
        raise ValueError("byte count must be a non-negative integer")
    if bytes_count < 0:
        raise ValueError("byte count must be non-negative")
    unit = 0
    value = bytes_count
    while unit < len(_UNITS) - 1 and value >= 1024:
        value /= 1024
        unit += 1
    if unit == 0:
        return f"{bytes_count} B"
    return f"{value:.1f} {_UNITS[unit]}"


def parse_size(text):
    """Parse a canonical size emitted by :func:`format_size`."""
    if not isinstance(text, str):
        raise ValueError("size must be text")
    match = _PATTERN.fullmatch(text)
    if not match or (match.group(2) is None) != (match.group(3) == "B"):
        raise ValueError("malformed size")
    number, fraction, unit = match.groups()
    try:
        value = Decimal(number + ("." + fraction if fraction else ""))
        result = value * (1024 ** _UNITS.index(unit))
    except (InvalidOperation, ValueError):
        raise ValueError("malformed size") from None
    if result != result.to_integral_value():
        raise ValueError("size is not a whole number of bytes")
    return int(result)


def humansize(size):
    """Backward-compatible name for :func:`format_size`."""
    return format_size(size)


if __name__ == "__main__":
    assert format_size(0) == "0 B"
    assert format_size(1023) == "1023 B"
    assert format_size(1024) == "1.0 KiB"
    assert format_size(1536) == "1.5 KiB"
    assert format_size(3145728) == "3.0 MiB"
    assert parse_size("0 B") == 0
    assert parse_size("1.5 KiB") == 1536
    assert parse_size("3.0 MiB") == 3145728
    try:
        parse_size("")
    except ValueError:
        pass
    else:
        raise AssertionError("empty input must be rejected")
    print("OK")
