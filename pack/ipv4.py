"""Strict dotted-quad IPv4 conversion helpers."""


def _octets(text):
    if not isinstance(text, str):
        return None
    parts = text.split(".")
    if len(parts) != 4:
        return None
    values = []
    for part in parts:
        if not part or any(char < "0" or char > "9" for char in part):
            return None
        if len(part) > 1 and part[0] == "0":
            return None
        value = int(part)
        if value > 255:
            return None
        values.append(value)
    return values


def is_valid(text):
    """Return whether *text* is a strict dotted-quad IPv4 address."""
    return _octets(text) is not None


def to_int(text):
    """Convert an IPv4 address to its network-order integer."""
    values = _octets(text)
    if values is None:
        raise ValueError("invalid IPv4 address")
    return (values[0] << 24) | (values[1] << 16) | (values[2] << 8) | values[3]


def from_int(value):
    """Convert a network-order integer to a dotted-quad IPv4 address."""
    if isinstance(value, bool) or not isinstance(value, int) or not 0 <= value <= 0xFFFFFFFF:
        raise ValueError("IPv4 integer out of range")
    return ".".join(str((value >> shift) & 255) for shift in (24, 16, 8, 0))


if __name__ == "__main__":
    assert not is_valid("")
    assert not is_valid("1.2.3")
    assert not is_valid("01.2.3.4")
    assert not is_valid("1.2.3.256")
    assert not is_valid("1.2.3.4 ")
    assert is_valid("0.0.0.0")
    assert is_valid("255.255.255.255")
    assert to_int("127.0.0.1") == 0x7F000001
    assert to_int("0.0.0.0") == 0
    assert from_int(0) == "0.0.0.0"
    assert from_int(0xFFFFFFFF) == "255.255.255.255"
    assert from_int(to_int("192.168.1.10")) == "192.168.1.10"
    for invalid in ("", "1.2.3", "1.2.3.256"):
        try:
            to_int(invalid)
        except ValueError:
            pass
        else:
            raise AssertionError("invalid address accepted")
    for invalid in (-1, 0x100000000):
        try:
            from_int(invalid)
        except ValueError:
            pass
        else:
            raise AssertionError("invalid integer accepted")
    print("OK")
