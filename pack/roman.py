"""Convert canonical Roman numerals in the range 1 through 3999."""

_VALUES = ((1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
           (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
           (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"))


def to_roman(number):
    """Return the canonical Roman numeral for an integer from 1 to 3999."""
    if isinstance(number, bool) or not isinstance(number, int) or not 1 <= number <= 3999:
        raise ValueError("number must be an integer from 1 through 3999")
    result = []
    for value, symbol in _VALUES:
        count, number = divmod(number, value)
        result.append(symbol * count)
    return "".join(result)


def from_roman(text):
    """Return the integer represented by a canonical Roman numeral."""
    if not isinstance(text, str) or not text:
        raise ValueError("Roman numeral must be non-empty text")
    remaining, result = text, 0
    for value, symbol in _VALUES:
        while remaining.startswith(symbol):
            result += value
            remaining = remaining[len(symbol):]
    if remaining or to_roman(result) != text:
        raise ValueError("malformed or non-canonical Roman numeral")
    return result


# Preserve the scaffold's original name for callers that used it.
roman = to_roman


if __name__ == "__main__":
    assert to_roman(1) == "I"
    assert to_roman(4) == "IV"
    assert to_roman(3999) == "MMMCMXCIX"
    assert to_roman(1984) == "MCMLXXXIV"
    assert from_roman("I") == 1
    assert from_roman("XL") == 40
    assert from_roman("MMMCMXCIX") == 3999
    assert from_roman("MCMLXXXIV") == 1984
    for invalid in (0, 4000, "", "IIII", "IC", "iv"):
        try:
            from_roman(invalid) if isinstance(invalid, str) else to_roman(invalid)
        except ValueError:
            pass
        else:
            raise AssertionError(invalid)
    print("OK")
