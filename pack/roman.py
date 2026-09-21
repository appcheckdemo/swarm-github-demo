"""Canonical Roman numeral conversion for the range 1 through 3999."""

_VALUES = ((1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
           (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
           (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"))


def to_roman(number):
    """Return the canonical Roman representation of an integer."""
    if isinstance(number, bool) or not isinstance(number, int) or not 1 <= number <= 3999:
        raise ValueError("number must be an integer from 1 through 3999")
    result = []
    for value, numeral in _VALUES:
        count, number = divmod(number, value)
        result.append(numeral * count)
    return "".join(result)


def from_roman(numeral):
    """Return the integer represented by a canonical Roman numeral."""
    if not isinstance(numeral, str) or not numeral:
        raise ValueError("numeral must be a non-empty string")
    values = {symbol: value for value, symbol in _VALUES}
    total = 0
    index = 0
    while index < len(numeral):
        value = values.get(numeral[index])
        if value is None:
            raise ValueError("invalid Roman numeral")
        if index + 1 < len(numeral):
            next_value = values.get(numeral[index + 1])
            if next_value is None:
                raise ValueError("invalid Roman numeral")
            if value < next_value:
                total += next_value - value
                index += 2
                continue
        total += value
        index += 1
    if not 1 <= total <= 3999 or to_roman(total) != numeral:
        raise ValueError("non-canonical Roman numeral")
    return total


roman_to_int = from_roman


if __name__ == "__main__":
    assert to_roman(1) == "I"
    assert to_roman(3999) == "MMMCMXCIX"
    assert to_roman(1984) == "MCMLXXXIV"
    assert from_roman("I") == 1
    assert from_roman("MMMCMXCIX") == 3999
    assert from_roman("XLII") == 42
    assert from_roman(to_roman(2468)) == 2468
    for invalid in ("", "IIII", "IC", "MMMM", "ABC"):
        try:
            from_roman(invalid)
        except ValueError:
            pass
        else:
            raise AssertionError(invalid)
    print("OK")
