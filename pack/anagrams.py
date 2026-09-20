"""Small anagram helpers.

Anagrams are compared case-insensitively and non-alphanumeric characters are
ignored.  ``group`` preserves duplicate words, while sorting words and groups
makes its result deterministic.
"""

from collections import defaultdict


def _key(word):
    """Return the canonical, case-insensitive spelling of *word*."""
    return tuple(sorted(c.lower() for c in word if c.isalnum()))


def is_anagram(first, second):
    """Return whether two strings contain the same significant characters."""
    return _key(first) == _key(second)


def group(words):
    """Return sorted lists of anagrams from *words*.

    Input order does not affect the result.  Empty input returns ``[]`` and
    repeated words remain repeated in their group.
    """
    groups = defaultdict(list)
    for word in words:
        groups[_key(word)].append(word)
    return sorted(sorted(values) for values in groups.values())


if __name__ == "__main__":
    assert group([]) == []
    assert group(["solo"]) == [["solo"]]
    assert group(["eat", "tea", "ate"]) == [["ate", "eat", "tea"]]
    assert group(["tan", "ate", "nat", "bat"]) == [
        ["ate"], ["bat"], ["nat", "tan"]
    ]
    assert group(["Eat", "tea", "ATE"]) == [["ATE", "Eat", "tea"]]
    assert group(["ab", "ba", "ab"]) == [["ab", "ab", "ba"]]
    assert group(["a-b", "ba"]) == [["a-b", "ba"]]
    assert is_anagram("listen", "silent")
    assert not is_anagram("hello", "world")
    print("OK")
