"""Small run-length encoder.

The payload alphabet deliberately excludes decimal digits.  This keeps the
format unambiguous: every run is a decimal count followed by exactly one
non-digit character.  ``encode`` therefore raises ``ValueError`` for text
containing a digit.
"""


def encode(text: str) -> str:
    """Return *text* as ``count, character`` runs."""
    if not isinstance(text, str):
        raise TypeError("text must be a string")
    if any(character.isdigit() for character in text):
        raise ValueError("digit characters are not supported in the payload")
    if not text:
        return ""

    runs = []
    start = 0
    for index in range(1, len(text) + 1):
        if index == len(text) or text[index] != text[start]:
            runs.append(str(index - start) + text[start])
            start = index
    return "".join(runs)


def decode(encoded: str) -> str:
    """Decode runs, rejecting missing, zero, or malformed counts."""
    if not isinstance(encoded, str):
        raise TypeError("encoded must be a string")
    result = []
    index = 0
    while index < len(encoded):
        count_start = index
        while index < len(encoded) and encoded[index].isdigit():
            index += 1
        if index == count_start or index == len(encoded):
            raise ValueError("run must have a positive count and character")
        count_text = encoded[count_start:index]
        count = int(count_text)
        if count == 0 or count_text[0] == "0":
            raise ValueError("run count must be positive")
        character = encoded[index]
        if character.isdigit():  # defensive: also documents the grammar
            raise ValueError("run character cannot be a digit")
        result.append(character * count)
        index += 1
    return "".join(result)


def rle(text: str) -> str:
    return encode(text)


if __name__ == "__main__":
    assert encode("") == ""
    assert decode("") == ""
    assert encode("aaab") == "3a1b"
    assert decode("3a1b") == "aaab"
    assert encode("xxxxxxxxxx") == "10x"
    assert decode("10x") == "xxxxxxxxxx"
    assert encode("abc") == "1a1b1c"
    assert decode("1a1b1c") == "abc"
    assert rle("book") == "1b2o1k"
    for bad in ("a", "0a", "01a", "2", "1a0b"):
        try:
            decode(bad)
        except ValueError:
            pass
        else:
            raise AssertionError("malformed input was accepted: " + bad)
    try:
        encode("a1")
    except ValueError:
        pass
    else:
        raise AssertionError("digit payload was accepted")
    print("OK")
