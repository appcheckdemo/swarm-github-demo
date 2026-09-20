"""Run-length encoding."""

def encode(text):
    if not text: return ""
    result, count, previous = [], 0, text[0]
    for char in text + "\0":
        if char == previous:
            count += 1
        else:
            result.append((count, previous)); previous, count = char, 1
    return result

def decode(runs):
    return "".join(char * count for count, char in runs)

if __name__ == "__main__":
    assert decode(encode("aaabbc")) == "aaabbc"
    print("OK")
