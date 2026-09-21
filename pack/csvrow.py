"""CSV row utility stub."""


def parse_row(row):
    """Return a one-item placeholder row."""
    return [row]


if __name__ == "__main__":
    assert parse_row("OK") == ["OK"]
    print("OK")
