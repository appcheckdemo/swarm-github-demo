"""Roman numeral conversion."""

_VALUES = ((1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
           (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
           (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"))

def to_roman(number):
    if not 1 <= number <= 3999 or int(number) != number:
        raise ValueError("number must be an integer from 1 through 3999")
    result = []
    for value, numeral in _VALUES:
        count, number = divmod(number, value)
        result.append(numeral * count)
    return "".join(result)

def from_roman(text):
    if not text:
        raise ValueError("empty numeral")
    value, index = 0, 0
    for number, numeral in _VALUES:
        while text[index:index + len(numeral)] == numeral:
            value += number; index += len(numeral)
            if index == len(text):
                break
    if to_roman(value) != text:
        raise ValueError("invalid Roman numeral")
    return value

if __name__ == "__main__":
    assert from_roman(to_roman(2024)) == 2024
    print("OK")
