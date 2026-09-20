"""Small Caesar-cipher implementation using only the standard library."""


def shift(text: str, amount: int) -> str:
    """Shift ASCII letters in *text* by *amount* places.

    Letter case is preserved and characters outside ASCII A-Z/a-z are copied
    unchanged.  Any integer shift, including a negative one, is supported.
    """
    offset = amount % 26
    result = []
    for character in text:
        code = ord(character)
        if 65 <= code <= 90:
            code = 65 + (code - 65 + offset) % 26
        elif 97 <= code <= 122:
            code = 97 + (code - 97 + offset) % 26
        result.append(chr(code))
    return "".join(result)


def caesar(text: str, amount: int) -> str:
    """Backward-compatible name for :func:`shift`."""
    return shift(text, amount)


if __name__ == "__main__":
    assert shift("", 5) == ""
    assert shift("abc", 1) == "bcd"
    assert shift("XYZ", 1) == "YZA"
    assert shift("Z", 1) == "A"
    assert shift("z", 1) == "a"
    assert shift("Hello, World!", 3) == "Khoor, Zruog!"
    assert shift("Same", 26) == "Same"
    assert shift("abc", -1) == "zab"
    assert shift("123 !-", 100) == "123 !-"
    assert caesar("Ab", 25) == "Za"
    print("OK")
