"""Caesar cipher."""

def caesar(text, shift):
    result = []
    for char in text:
        if char.isalpha() and char.isascii():
            start = ord('A') if char.isupper() else ord('a')
            char = chr((ord(char) - start + shift) % 26 + start)
        result.append(char)
    return "".join(result)

if __name__ == "__main__":
    assert caesar("Hello, Z!", 3) == "Khoor, C!"
    print("OK")
