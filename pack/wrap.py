"""Word wrapping helpers."""

def wrap(text, width):
    """Wrap *text* into lines no wider than width, preserving words."""
    if width < 1:
        raise ValueError("width must be positive")
    words = text.split()
    lines, line = [], ""
    for word in words:
        if line and len(line) + 1 + len(word) > width:
            lines.append(line); line = word
        elif line:
            line += " " + word
        else:
            line = word
    if line:
        lines.append(line)
    return "\n".join(lines)

if __name__ == "__main__":
    assert wrap("one two three", 7) == "one two\nthree"
    print("OK")
