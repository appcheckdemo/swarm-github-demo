"""Anagram utilities."""

def is_anagram(first, second):
    normalize = lambda value: sorted(c.lower() for c in value if c.isalnum())
    return normalize(first) == normalize(second)

if __name__ == "__main__":
    assert is_anagram("listen", "silent")
    print("OK")
