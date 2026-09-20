"""Parse a single, conventional CSV record."""


def split_row(line: str) -> list[str]:
    """Return the fields in *line*, preserving empty fields."""
    fields, field = [], []
    quoted = after_quote = False
    for char in line:
        if quoted:
            if char == '"':
                quoted = False
                after_quote = True
            else:
                field.append(char)
        elif after_quote:
            if char == '"':
                field.append('"')
                quoted = True
                after_quote = False
            elif char == ',':
                fields.append(''.join(field))
                field = []
                after_quote = False
            else:
                raise ValueError("invalid character after closing quote")
        elif char == ',':
            fields.append(''.join(field))
            field = []
        elif char == '"' and not field:
            quoted = True
        else:
            field.append(char)
    if quoted:
        raise ValueError("unterminated quoted field")
    fields.append(''.join(field))
    return fields


if __name__ == "__main__":
    assert split_row("") == [""]
    assert split_row("a,b,c") == ["a", "b", "c"]
    assert split_row(",a,") == ["", "a", ""]
    assert split_row('"a,b",c') == ["a,b", "c"]
    assert split_row('"say ""hi"""') == ['say "hi"']
    assert split_row('a,,""') == ["a", "", ""]
    assert split_row('"",x') == ["", "x"]
    assert split_row('plain"quote') == ['plain"quote']
    try:
        split_row('"unfinished')
    except ValueError:
        pass
    else:
        raise AssertionError("unterminated quote accepted")
    try:
        split_row('"closed"x')
    except ValueError:
        pass
    else:
        raise AssertionError("trailing character accepted")
    print("OK")
