"""CSV row parsing and serialization."""
import csv
import io

def parse(row):
    return next(csv.reader([row]))

def format_row(fields):
    output = io.StringIO()
    csv.writer(output, lineterminator="").writerow(fields)
    return output.getvalue()

if __name__ == "__main__":
    assert parse(format_row(["one", "two, too"])) == ["one", "two, too"]
    print("OK")
