"""Small, standard-library-only implementation of the Luhn algorithm."""


def is_valid(number: str) -> bool:
    """Return whether *number* is a non-empty, valid digit string."""
    if not isinstance(number, str) or not number or not all("0" <= c <= "9" for c in number):
        return False
    total = 0
    for position, character in enumerate(reversed(number)):
        digit = int(character)
        if position % 2:
            digit *= 2
            if digit > 9:
                digit -= 9
        total += digit
    return total % 10 == 0


def check_digit(prefix: str) -> int:
    """Return the Luhn check digit to append to a non-empty digit prefix."""
    if not isinstance(prefix, str) or not prefix or not all("0" <= c <= "9" for c in prefix):
        raise ValueError("prefix must be a non-empty digit string")
    total = 0
    for position, character in enumerate(reversed(prefix)):
        digit = int(character)
        if position % 2 == 0:
            digit *= 2
            if digit > 9:
                digit -= 9
        total += digit
    return (-total) % 10


# Keep the scaffold's original entry point useful for callers.
def luhn(number: str) -> bool:
    return is_valid(number)


if __name__ == "__main__":
    assert not is_valid("")
    assert not is_valid("12a3")
    assert is_valid("0")
    assert is_valid("18")
    assert not is_valid("19")
    assert check_digit("1") == 8
    assert check_digit("7992739871") == 3
    assert is_valid("79927398713")
    assert not is_valid("79927398714")
    try:
        check_digit("")
    except ValueError:
        pass
    else:
        raise AssertionError("empty prefix should fail")
    print("OK")
