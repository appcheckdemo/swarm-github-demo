"""Small, strict run-length encoder/decoder (standard library only).

The format is ``<positive decimal count><one non-digit character>`` per run.
Digits are therefore forbidden in source strings, and counts have no leading
zeroes.  These rules make the encoded representation unambiguous.
"""


def encode(value):
    """Encode a string, rejecting digits which would make the format unclear."""
    if not isinstance(value, str):
        raise TypeError("value must be a string")
    if any(character.isdigit() for character in value):
        raise ValueError("source strings may not contain digits")
    if not value:
        return ""
    result = []
    start = 0
    for index in range(1, len(value) + 1):
        if index == len(value) or value[index] != value[start]:
            result.append(str(index - start) + value[start])
            start = index
    return "".join(result)


def decode(value):
    """Decode canonical runs, raising ValueError for malformed input."""
    if not isinstance(value, str):
        raise TypeError("value must be a string")
    result = []
    index = 0
    while index < len(value):
        begin = index
        while index < len(value) and "0" <= value[index] <= "9":
            index += 1
        count_text = value[begin:index]
        if not count_text or count_text[0] == "0":
            raise ValueError("run must start with a positive canonical count")
        if index == len(value) or "0" <= value[index] <= "9":
            raise ValueError("run must have one non-digit character")
        result.append(value[index] * int(count_text))
        index += 1
    return "".join(result)


if __name__ == "__main__":
    assert encode("") == ""
    assert encode("aaab") == "3a1b"
    assert encode("a" * 9) == "9a"  # single-digit boundary
    assert encode("a" * 10) == "10a"  # count width boundary
    assert decode("3a1b") == "aaab"
    assert decode(encode("abccc")) == "abccc"
    assert decode(encode("")) == ""
    assert decode("10a") == "a" * 10
    for malformed in ("a", "0a", "01a", "2", "2a3"):
        try:
            decode(malformed)
        except ValueError:
            pass
        else:
            raise AssertionError("accepted malformed input: " + malformed)
    try:
        encode("a1")
    except ValueError:
        pass
    else:
        raise AssertionError("accepted digit in source")
    print("OK")
