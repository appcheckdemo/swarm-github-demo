"""Compute Levenshtein edit distances using only the standard library."""


def distance(left: str, right: str) -> int:
    """Return the minimum number of single-character edits between strings."""
    if len(left) < len(right):
        left, right = right, left
    previous = list(range(len(right) + 1))
    for left_index, left_char in enumerate(left, 1):
        current = [left_index]
        for right_index, right_char in enumerate(right, 1):
            current.append(min(
                current[-1] + 1,
                previous[right_index] + 1,
                previous[right_index - 1] + (left_char != right_char),
            ))
        previous = current
    return previous[-1]


def levenshtein(left: str, right: str) -> int:
    """Compatibility name for :func:`distance`."""
    return distance(left, right)


if __name__ == "__main__":
    assert distance("", "") == 0
    assert distance("a", "a") == 0
    assert distance("", "abc") == 3
    assert distance("kitten", "sitting") == 3
    assert distance("flaw", "lawn") == 2
    assert distance("abc", "ab") == 1
    assert distance("abc", "axc") == 1
    assert distance("Saturday", "Sunday") == 3
    assert levenshtein("book", "back") == 2
    print("OK")
