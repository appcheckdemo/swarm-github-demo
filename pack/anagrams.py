"""Deterministic anagram helpers.

Matching is case-sensitive (``"A"`` and ``"a"`` differ), and duplicate input
words are retained as duplicate entries in their group.
"""


def are_anagrams(left, right):
    """Return whether two strings contain the same characters."""
    return sorted(left) == sorted(right)


def group_anagrams(words):
    """Return sorted anagram groups, with deterministic group ordering."""
    groups = {}
    for word in words:
        groups.setdefault(tuple(sorted(word)), []).append(word)
    result = [sorted(group) for group in groups.values()]
    return sorted(result)


if __name__ == "__main__":
    assert group_anagrams([]) == []
    assert group_anagrams(["solo"]) == [["solo"]]
    assert are_anagrams("listen", "silent")
    assert not are_anagrams("A", "a")
    assert group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"]) == [
        ["ate", "eat", "tea"], ["bat"], ["nat", "tan"]
    ]
    assert group_anagrams(["ab", "ba", "ab"]) == [["ab", "ab", "ba"]]
    assert group_anagrams(["b", "a", "c"]) == [["a"], ["b"], ["c"]]
    assert group_anagrams(["", "", "a"]) == [["", ""], ["a"]]
    assert group_anagrams(["Ab", "bA", "ab"]) == [["Ab", "bA"], ["ab"]]
    print("OK")
