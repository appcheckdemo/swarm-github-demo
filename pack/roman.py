"""Conversion between integers and canonical Roman numerals."""

import re

_VALUES = ((1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
           (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
           (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"))
_CANONICAL = re.compile(
    r"M{0,3}(?:CM|CD|D?C{0,3})(?:XC|XL|L?X{0,3})(?:IX|IV|V?I{0,3})"
)


def to_roman(value):
    """Return the canonical Roman numeral for an integer from 1 to 3999."""
    if isinstance(value, bool) or not isinstance(value, int) or not 1 <= value <= 3999:
        raise ValueError("value must be an integer from 1 through 3999")
    result = []
    for number, numeral in _VALUES:
        count, value = divmod(value, number)
        result.append(numeral * count)
    return "".join(result)


def from_roman(text):
    """Return the value of a canonical Roman numeral."""
    if not isinstance(text, str) or not text or not _CANONICAL.fullmatch(text):
        raise ValueError("text must be a canonical Roman numeral")
    total = 0
    previous = 0
    for numeral in reversed(text):
        value = {"I": 1, "V": 5, "X": 10, "L": 50,
                 "C": 100, "D": 500, "M": 1000}[numeral]
        total += -value if value < previous else value
        previous = max(previous, value)
    return total


if __name__ == "__main__":
    assert to_roman(1) == "I"
    assert from_roman("I") == 1
    assert to_roman(3999) == "MMMCMXCIX"
    assert from_roman("MMMCMXCIX") == 3999
    assert to_roman(2024) == "MMXXIV"
    assert from_roman("MCMXC") == 1990
    for bad in (0, 4000, 1.0, True):
        try:
            to_roman(bad)
        except ValueError:
            pass
        else:
            raise AssertionError(bad)
    for bad in ("", "IIII", "IL", "iv", "MMMM"):
        try:
            from_roman(bad)
        except ValueError:
            pass
        else:
            raise AssertionError(bad)
    print("OK")
