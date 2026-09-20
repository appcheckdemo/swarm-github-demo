"""Group strings by their anagram signatures."""

from collections import defaultdict


def group_anagrams(words):
    """Return sorted groups of strings that are anagrams of one another."""
    groups = defaultdict(list)
    for word in words:
        groups[tuple(sorted(word))].append(word)

    result = [sorted(group) for group in groups.values()]
    return sorted(result)


if __name__ == "__main__":
    assert group_anagrams([]) == []
    assert group_anagrams(["word"]) == [["word"]]
    assert group_anagrams(["eat", "tea", "ate"]) == [["ate", "eat", "tea"]]
    assert group_anagrams(["bat", "tab", "cat"]) == [["bat", "tab"], ["cat"]]
    assert group_anagrams(["Tea", "eat"]) == [["Tea"], ["eat"]]
    assert group_anagrams(["a", "a", "A"]) == [["A"], ["a", "a"]]
    assert group_anagrams(["abc", "bca", "cab", "xyz", "zyx"]) == [
        ["abc", "bca", "cab"],
        ["xyz", "zyx"],
    ]
    assert group_anagrams(["", ""]) == [["", ""]]
    assert group_anagrams((word for word in ["ab", "ba"])) == [["ab", "ba"]]
    print("OK")
