"""Small CSV-row parser with explicit malformed-input handling."""
import csv
import io


def split_row(line):
    """Return fields in *line*; raise ValueError for malformed quoting.

    Quotes are allowed only at the beginning of a field.  Inside a quoted
    field, two quotes represent one quote; after a closing quote only a comma
    or the end of the row is valid.
    """
    if line == "":
        return []
    fields, field, i = [], [], 0
    while i < len(line):
        if line[i] == '"':
            if field:
                raise ValueError("quote in unquoted field")
            i += 1
            while True:
                if i == len(line):
                    raise ValueError("unclosed quote")
                if line[i] != '"':
                    field.append(line[i])
                    i += 1
                elif i + 1 < len(line) and line[i + 1] == '"':
                    field.append('"')
                    i += 2
                else:
                    i += 1
                    if i < len(line) and line[i] != ',':
                        raise ValueError("characters after closing quote")
                    break
        else:
            while i < len(line) and line[i] != ',':
                if line[i] == '"':
                    raise ValueError("quote in unquoted field")
                field.append(line[i])
                i += 1
        fields.append(''.join(field))
        field = []
        if i < len(line):
            i += 1
    if line.endswith(','):
        fields.append('')
    return fields


def parse(row):
    return split_row(row)


def format_row(fields):
    output = io.StringIO()
    csv.writer(output, lineterminator="").writerow(fields)
    return output.getvalue()


if __name__ == "__main__":
    assert split_row("") == []
    assert split_row("one,two") == ["one", "two"]
    assert split_row("one,,three,") == ["one", "", "three", ""]
    assert split_row('"one, two",three') == ["one, two", "three"]
    assert split_row('"say ""hi""",ok') == ['say "hi"', "ok"]
    assert split_row('""') == [""]
    assert split_row(",") == ["", ""]
    assert split_row(" plain ,text") == [" plain ", "text"]
    for bad in ('"unfinished', 'a"b', '"a"b', '"a""b'):
        try:
            split_row(bad)
        except ValueError:
            pass
        else:
            raise AssertionError("accepted malformed row: " + bad)
    print("OK")
