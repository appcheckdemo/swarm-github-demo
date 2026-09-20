"""Small ASCII Caesar cipher."""


def shift(text, amount):
    """Return *text* shifted by *amount* positions, preserving case."""
    amount %= 26
    result = []
    for char in text:
        if "A" <= char <= "Z":
            char = chr((ord(char) - ord("A") + amount) % 26 + ord("A"))
        elif "a" <= char <= "z":
            char = chr((ord(char) - ord("a") + amount) % 26 + ord("a"))
        result.append(char)
    return "".join(result)


if __name__ == "__main__":
    assert shift("", 0) == ""
    assert shift("abc", 0) == "abc"
    assert shift("abc", 26) == "abc"
    assert shift("Hello, Z!", 3) == "Khoor, C!"
    assert shift("Khoor, C!", -3) == "Hello, Z!"
    assert shift("xyz XYZ", 1) == "yza YZA"
    assert shift("a", -1) == "z"
    assert shift("123 !@#", 100) == "123 !@#"
    assert shift("éÅ", 5) == "éÅ"
    print("OK")
