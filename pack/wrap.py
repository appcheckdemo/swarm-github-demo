"""Small, standard-library-only word wrapper."""


def wrap_text(text: str, width: int) -> list[str]:
    """Return *text* wrapped at word boundaries, using at most *width* columns.

    Whitespace separates words and is not retained.  A word wider than the
    requested width is kept intact rather than split.
    """
    if width <= 0:
        raise ValueError("width must be positive")

    words = text.split()
    lines: list[str] = []
    current = ""
    for word in words:
        if len(word) > width:
            if current:
                lines.append(current)
                current = ""
            lines.append(word)
        elif not current:
            current = word
        elif len(current) + 1 + len(word) <= width:
            current += " " + word
        else:
            lines.append(current)
            current = word
    if current:
        lines.append(current)
    return lines


if __name__ == "__main__":
    assert wrap_text("", 5) == []
    assert wrap_text(" \t\n", 5) == []
    assert wrap_text("one two three", 7) == ["one two", "three"]
    assert wrap_text("hello", 5) == ["hello"]
    assert wrap_text("hello", 3) == ["hello"]
    assert wrap_text("a  b\tc", 3) == ["a b", "c"]
    assert wrap_text("small enormousword tiny", 5) == ["small", "enormousword", "tiny"]
    assert wrap_text("a bb ccc", 3) == ["a", "bb", "ccc"]
    try:
        wrap_text("text", 0)
    except ValueError:
        pass
    else:
        raise AssertionError("zero width should fail")
    try:
        wrap_text("text", -1)
    except ValueError:
        pass
    else:
        raise AssertionError("negative width should fail")
    print("OK")
