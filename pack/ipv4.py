"""IPv4 parsing and formatting."""
import ipaddress

def is_valid(address):
    try:
        ipaddress.IPv4Address(address); return True
    except ipaddress.AddressValueError:
        return False

def to_int(address):
    return int(ipaddress.IPv4Address(address))

def from_int(value):
    return str(ipaddress.IPv4Address(value))

if __name__ == "__main__":
    assert to_int("127.0.0.1") == 2130706433 and is_valid("127.0.0.1")
    print("OK")
