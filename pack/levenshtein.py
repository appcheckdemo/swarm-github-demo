"""Compute insertion, deletion, and substitution edit distance."""


def levenshtein(a, b):
    """Return the minimum number of single-character edits from *a* to *b*."""
    if len(a) < len(b):
        a, b = b, a
    previous = list(range(len(b) + 1))
    for i, left in enumerate(a, 1):
        current = [i]
        for j, right in enumerate(b, 1):
            current.append(min(
                current[-1] + 1,
                previous[j] + 1,
                previous[j - 1] + (left != right),
            ))
        previous = current
    return previous[-1]


def distance(a, b):
    """Compatibility alias for :func:`levenshtein`."""
    return levenshtein(a, b)


if __name__ == "__main__":
    assert levenshtein("", "") == 0
    assert levenshtein("", "abc") == 3
    assert levenshtein("abc", "") == 3
    assert levenshtein("abc", "abc") == 0
    assert levenshtein("abc", "abd") == 1
    assert levenshtein("abc", "ab") == 1
    assert levenshtein("ab", "abc") == 1
    assert levenshtein("kitten", "sitting") == 3
    assert levenshtein("flaw", "lawn") == 2
    assert distance("book", "back") == 2
    print("OK")
