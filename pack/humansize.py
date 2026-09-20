"""Human-readable byte sizes."""

def naturalsize(number, binary=False, format="%.1f %s"):
    base = 1024.0 if binary else 1000.0
    units = ("B", "KiB", "MiB", "GiB", "TiB") if binary else ("B", "kB", "MB", "GB", "TB")
    sign = "-" if number < 0 else ""
    number = abs(float(number)); index = 0
    while number >= base and index < len(units) - 1:
        number /= base; index += 1
    return sign + format % (number, units[index])

if __name__ == "__main__":
    assert naturalsize(1000) == "1.0 kB"
    print("OK")
