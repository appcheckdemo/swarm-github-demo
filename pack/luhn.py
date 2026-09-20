"""Small, dependency-free helpers for the Luhn checksum."""


def _digits(value):
    """Return ASCII digits, or ``None`` when value is not digit-only."""
    if not isinstance(value, str):
        return None
    value = value.replace(" ", "")
    return value if value and all("0" <= char <= "9" for char in value) else None


def is_valid(number):
    """Return whether *number* has a valid Luhn checksum.

    Spaces are formatting and are ignored; empty, one-digit, and other
    non-digit inputs are invalid.
    """
    digits = _digits(number)
    if digits is None or len(digits) < 2:
        return False
    total = 0
    for index, char in enumerate(reversed(digits)):
        digit = int(char)
        if index % 2:
            digit = digit * 2
            if digit > 9:
                digit -= 9
        total += digit
    return total % 10 == 0


def check_digit(prefix):
    """Return the digit which makes a non-empty digit-only *prefix* valid."""
    if not isinstance(prefix, str) or not prefix or not all(
            "0" <= char <= "9" for char in prefix):
        raise ValueError("prefix must be a non-empty digit-only string")
    for digit in range(10):
        if is_valid(prefix + str(digit)):
            return digit
    raise AssertionError("unreachable")


if __name__ == "__main__":
    assert is_valid("4539 1488 0343 6467")
    assert not is_valid("4539 1488 0343 6468")
    assert not is_valid("")
    assert not is_valid("7")
    assert not is_valid("12a4")
    assert not is_valid("12\t34")
    assert check_digit("7992739871") == 3
    assert check_digit("0") == 0  # shortest usable prefix
    try:
        check_digit("12 34")
    except ValueError:
        pass
    else:
        raise AssertionError("spaces must not be accepted in a prefix")
    print("OK")
