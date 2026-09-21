"""A small, standard-library-only Caesar cipher."""


def caesar(text, shift=0):
    """Shift ASCII letters by *shift*, preserving case and other characters."""
    shift %= 26
    result = []
    for char in text:
        if "A" <= char <= "Z":
            result.append(chr((ord(char) - ord("A") + shift) % 26 + ord("A")))
        elif "a" <= char <= "z":
            result.append(chr((ord(char) - ord("a") + shift) % 26 + ord("a")))
        else:
            result.append(char)
    return "".join(result)


if __name__ == "__main__":
    assert caesar("") == ""
    assert caesar("abc", 0) == "abc"
    assert caesar("abc", 26) == "abc"
    assert caesar("xyz", 3) == "abc"
    assert caesar("XYZ", 3) == "ABC"
    assert caesar("bcd", -1) == "abc"
    assert caesar("Hello, World! 123", 5) == "Mjqqt, Btwqi! 123"
    assert caesar("aZ", 52) == "aZ"
    assert caesar("é_[]", 10) == "é_[]"
    print("OK")

