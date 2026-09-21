"""Deterministic anagram grouping using only the Python standard library."""


def group_anagrams(words):
    """Return sorted groups of anagrams from *words*.

    Words are compared case-sensitively, and duplicate input words are kept.
    Each group and the collection of groups are sorted lexicographically.
    """
    groups = {}
    for word in words:
        groups.setdefault(tuple(sorted(word)), []).append(word)
    result = [sorted(group) for group in groups.values()]
    return sorted(result, key=lambda group: (group[0], group))


def _self_test():
    assert group_anagrams([]) == []
    assert group_anagrams(["solo"]) == [["solo"]]
    assert group_anagrams(["eat", "tea", "ate"]) == [["ate", "eat", "tea"]]
    assert group_anagrams(["bat", "tab", "cat"]) == [["bat", "tab"], ["cat"]]
    assert group_anagrams(["ab", "ba", "ab"]) == [["ab", "ab", "ba"]]
    assert group_anagrams(["Tea", "eat", "ate"]) == [["Tea"], ["ate", "eat"]]
    assert group_anagrams(["abc", "bca", "cab", "foo", "oof"]) == [
        ["abc", "bca", "cab"],
        ["foo", "oof"],
    ]
    assert group_anagrams(["", ""]) == [["", ""]]
    assert group_anagrams(["a", "b", "c"]) == [["a"], ["b"], ["c"]]


if __name__ == "__main__":
    _self_test()
    print("OK")
