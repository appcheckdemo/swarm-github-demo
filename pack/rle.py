"""Run-length encoding.

The format is ``count`` followed by one character.  Since a character that
is a digit (or a backslash) is prefixed by ``\\``, the decimal count is always
unambiguous: for example, ``"111"`` encodes as ``"3\\1"``.
"""


def encode(text):
    """Return the run-length encoding of *text*."""
    if not isinstance(text, str):
        raise TypeError("text must be a string")
    if not text:
        return ""
    out, start = [], 0
    for i in range(1, len(text) + 1):
        if i == len(text) or text[i] != text[start]:
            char = text[start]
            out.append(str(i - start) + ("\\" if char in "0123456789\\" else "") + char)
            start = i
    return "".join(out)


def decode(encoded):
    """Decode *encoded*, rejecting malformed run strings."""
    if not isinstance(encoded, str):
        raise TypeError("encoded must be a string")
    out, i = [], 0
    while i < len(encoded):
        if not encoded[i].isdigit() or encoded[i] == "0":
            raise ValueError("run must start with a positive count")
        j = i
        while j < len(encoded) and encoded[j].isdigit():
            j += 1
        count = int(encoded[i:j])
        if j == len(encoded):
            raise ValueError("missing run character")
        if encoded[j] == "\\":
            j += 1
            if j == len(encoded) or encoded[j] not in "0123456789\\":
                raise ValueError("invalid escape")
        out.append(encoded[j] * count)
        i = j + 1
    return "".join(out)


if __name__ == "__main__":
    assert encode("") == ""
    assert encode("aaab") == "3a1b"
    assert decode("3a1b") == "aaab"
    assert encode("111") == r"3\1"
    assert decode(r"3\1") == "111"
    assert encode(r"\\") == r"2\\"
    assert decode(r"2\\") == r"\\"
    assert len(decode(encode("x" * 1000))) == 1000
    for bad in ("", "0a", "2", "2\\x", "2\\"):
        if bad == "":
            continue
        try:
            decode(bad)
        except ValueError:
            pass
        else:
            raise AssertionError(bad)
    print("OK")
