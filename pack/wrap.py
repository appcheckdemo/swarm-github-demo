"""Small word-wrapping helper using only the Python standard library."""


def wrap(text, width):
    """Return *text* wrapped on word boundaries at the requested width.

    Whitespace is normalized to single spaces.  A word longer than ``width``
    is kept intact, so that line may exceed the requested width.
    """
    if not isinstance(text, str):
        raise TypeError("text must be a string")
    if isinstance(width, bool) or not isinstance(width, int) or width < 1:
        raise ValueError("width must be a positive integer")

    lines = []
    current = []
    length = 0
    for word in text.split():
        needed = len(word) if not current else length + 1 + len(word)
        if current and needed > width:
            lines.append(" ".join(current))
            current = [word]
            length = len(word)
        else:
            current.append(word)
            length = needed
    if current:
        lines.append(" ".join(current))
    return "\n".join(lines)


if __name__ == "__main__":
    assert wrap("", 5) == ""
    assert wrap("   \t  ", 5) == ""
    assert wrap("one two three", 7) == "one two\nthree"
    assert wrap("one two", 7) == "one two"  # exact boundary
    assert wrap("  one\n two\tthree  ", 20) == "one two three"
    assert wrap("superlong", 3) == "superlong"
    assert wrap("a bb ccc", 3) == "a\nbb\nccc"
    assert wrap("four", 4) == "four"
    try:
        wrap("text", 0)
    except ValueError:
        pass
    else:
        raise AssertionError("zero width should fail")
    try:
        wrap("text", "4")
    except ValueError:
        pass
    else:
        raise AssertionError("non-integer width should fail")
    print("OK")
