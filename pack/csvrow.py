"""Split one CSV row without dependencies.

An empty input is an empty row (``[]``).  Empty fields, including fields after
the final comma, are retained.  Quotes may surround a field and doubled quotes
inside them represent one quote.  A stray quote, an unterminated quote, or
anything following a closing quote other than a comma is malformed and raises
``ValueError``.
"""


def parse_row(row):
    """Return the fields in *row*, accepting standard CSV quoting."""
    if not row:
        return []
    fields, field, quoted, closed = [], [], False, False
    index = 0
    while index < len(row):
        char = row[index]
        if quoted:
            if char == '"':
                if index + 1 < len(row) and row[index + 1] == '"':
                    field.append('"')
                    index += 1
                else:
                    quoted, closed = False, True
            else:
                field.append(char)
        elif closed:
            if char == ',':
                fields.append(''.join(field))
                field, closed = [], False
            else:
                raise ValueError("characters after closing quote")
        elif char == ',':
            fields.append(''.join(field))
            field = []
        elif char == '"':
            if field:
                raise ValueError("quote in unquoted field")
            quoted = True
        else:
            field.append(char)
        index += 1
    if quoted:
        raise ValueError("unterminated quote")
    fields.append(''.join(field))
    return fields


if __name__ == "__main__":
    assert parse_row("") == []
    assert parse_row(",") == ["", ""]
    assert parse_row("a,b,c") == ["a", "b", "c"]
    assert parse_row('"a,b",c') == ["a,b", "c"]
    assert parse_row('"say ""hi""",x') == ['say "hi"', "x"]
    assert parse_row("one,,three,") == ["one", "", "three", ""]
    assert parse_row('""') == [""]
    assert parse_row('"a",') == ["a", ""]
    for malformed in ('"open', 'bad"field', '"ok"tail'):
        try:
            parse_row(malformed)
        except ValueError:
            pass
        else:
            raise AssertionError("malformed quote accepted")
    print("OK")
