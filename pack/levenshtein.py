"""Standard-library-only Levenshtein distance."""


def levenshtein(left, right):
    """Return the minimum insertions, deletions, and substitutions needed."""
    if left == right:
        return 0
    if len(left) < len(right):
        left, right = right, left
    if not right:
        return len(left)

    previous = list(range(len(right) + 1))
    for left_index, left_char in enumerate(left, 1):
        current = [left_index]
        for right_index, right_char in enumerate(right, 1):
            insertion = current[right_index - 1] + 1
            deletion = previous[right_index] + 1
            substitution = previous[right_index - 1] + (left_char != right_char)
            current.append(min(insertion, deletion, substitution))
        previous = current
    return previous[-1]


if __name__ == "__main__":
    assert levenshtein("", "") == 0
    assert levenshtein("", "abc") == 3
    assert levenshtein("abc", "") == 3
    assert levenshtein("cat", "cats") == 1
    assert levenshtein("cats", "cat") == 1
    assert levenshtein("cat", "cut") == 1
    assert levenshtein("kitten", "sitting") == 3
    assert levenshtein("flaw", "lawn") == levenshtein("lawn", "flaw")
    assert levenshtein("a" * 100, "a" * 99 + "b") == 1
    print("OK")

