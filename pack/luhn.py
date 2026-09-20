"""Luhn checksum validation."""

def is_valid(number):
    raw = str(number).replace(" ", "").replace("-", "")
    if not raw.isdigit():
        return False
    digits = [int(c) for c in raw]
    if len(digits) < 2:
        return False
    total = sum((d * 2 - 9 if d * 2 > 9 else d * 2) if i % 2 == 1 else d
                for i, d in enumerate(reversed(digits)))
    return total % 10 == 0

if __name__ == "__main__":
    assert is_valid("4539 1488 0343 6467")
    print("OK")
