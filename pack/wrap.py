"""Small, dependency-free word-boundary text wrapper."""


def wrap(text, width=80):
    """Wrap *text* at *width*, without ever splitting a word.

    Whitespace is treated as a separator.  A word longer than ``width`` is
    emitted whole, since preserving the word is more important than the hint.
    """
    if width <= 0:
        raise ValueError("width must be positive")
    words = text.split()
    if not words:
        return ""
    lines = []
    line = words[0]
    for word in words[1:]:
        if len(line) + 1 + len(word) <= width:
            line += " " + word
        else:
            lines.append(line)
            line = word
    lines.append(line)
    return "\n".join(lines)


if __name__ == "__main__":
    assert wrap("") == ""
    assert wrap("   \t\n") == ""
    assert wrap("one two", 7) == "one two"
    assert wrap("one two", 6) == "one\ntwo"
    assert wrap("one two three", 7) == "one two\nthree"
    assert wrap("supercalifragilistic", 5) == "supercalifragilistic"
    assert wrap(" a   b ", 2) == "a\nb"
    assert wrap("short", 80) == "short"
    try:
        wrap("text", 0)
    except ValueError:
        pass
    else:
        raise AssertionError("nonpositive width should fail")
    print("OK")
