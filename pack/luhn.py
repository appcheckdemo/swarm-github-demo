"""Small, standard-library-only Luhn validation helpers."""


def _digits(number):
    if not isinstance(number, str):
        return None
    value = "".join(c for c in number if not c.isspace() and c != "-")
    return value if value and value.isdigit() else None


def luhn(number):
    """Return whether *number* is a valid Luhn number.

    Spaces and hyphens are accepted as display separators; other characters,
    including an empty value, make the number invalid.
    """
    value = _digits(number)
    if value is None:
        return False
    total = sum((lambda x: x - 9 if x > 9 else x)
                (int(d) * (2 if i % 2 else 1))
                for i, d in enumerate(reversed(value)))
    return total % 10 == 0


def check_digit(number):
    """Return the integer Luhn check digit for a number prefix."""
    value = _digits(number)
    if value is None:
        raise ValueError("number must contain only digits and separators")
    total = sum((lambda x: x - 9 if x > 9 else x)
                (int(d) * (2 if i % 2 == 0 else 1))
                for i, d in enumerate(reversed(value)))
    return (-total) % 10


if __name__ == "__main__":
    assert not luhn("")
    assert not luhn("   - ")
    assert luhn("0")
    assert luhn("18")
    assert not luhn("19")
    assert luhn("7992-7398 713")
    assert not luhn("7992-7398 714")
    assert check_digit("7992739871") == 3
    assert check_digit("0") == 0
    assert check_digit(" 123  ") == 0
    assert not luhn("12A3")
    print("OK")
