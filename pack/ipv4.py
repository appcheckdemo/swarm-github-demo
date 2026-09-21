"""Small, standard-library-only dotted-quad IPv4 helpers."""


def _octets(address):
    if not isinstance(address, str):
        raise ValueError("IPv4 address must be a string")
    parts = address.split(".")
    if len(parts) != 4 or any(not p or not p.isascii() or not p.isdigit()
                              for p in parts):
        raise ValueError("invalid IPv4 address")
    if any(len(p) > 1 and p[0] == "0" for p in parts):
        raise ValueError("leading zero in IPv4 address")
    values = [int(p) for p in parts]
    if any(value > 255 for value in values):
        raise ValueError("IPv4 octet out of range")
    return values


def is_ipv4(address):
    """Return whether *address* is a canonical dotted-quad IPv4 string."""
    try:
        _octets(address)
    except ValueError:
        return False
    return True


def ipv4_to_int(address):
    """Convert a canonical dotted-quad address to an unsigned 32-bit int."""
    values = _octets(address)
    return (values[0] << 24) | (values[1] << 16) | (values[2] << 8) | values[3]


def int_to_ipv4(value):
    """Convert an unsigned 32-bit integer to its dotted-quad spelling."""
    if type(value) is not int or not 0 <= value <= 0xFFFFFFFF:
        raise ValueError("IPv4 integer must be an unsigned 32-bit integer")
    return ".".join(str((value >> shift) & 255) for shift in (24, 16, 8, 0))


if __name__ == "__main__":
    assert is_ipv4("0.0.0.0")
    assert is_ipv4("255.255.255.255")
    assert ipv4_to_int("0.0.0.0") == 0
    assert ipv4_to_int("255.255.255.255") == 4294967295
    assert int_to_ipv4(0) == "0.0.0.0"
    assert int_to_ipv4(4294967295) == "255.255.255.255"
    assert int_to_ipv4(ipv4_to_int("192.168.1.7")) == "192.168.1.7"
    assert not is_ipv4("")
    assert not is_ipv4("1.2.3")
    assert not is_ipv4("01.2.3.4")
    assert not is_ipv4("256.0.0.1")
    for bad in (-1, 4294967296, True, "1"):
        try:
            int_to_ipv4(bad)
        except ValueError:
            pass
        else:
            raise AssertionError(bad)
    print("OK")
