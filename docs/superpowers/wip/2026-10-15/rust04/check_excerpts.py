"""Check that every literal ```rust / ```toml excerpt in the parts appears in some real source file.

Lines are compared with leading/trailing whitespace stripped; blank lines are ignored.
Usage: python check_excerpts.py [unitNN.md ...]
"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def lines_of(text):
    return [l.strip() for l in text.replace("\r\n", "\n").split("\n") if l.strip()]


def main():
    srcs = []
    for p in HERE.rglob("*"):
        if p.is_file() and p.suffix in (".rs", ".toml") and "target" not in p.parts:
            srcs.append((p, lines_of(p.read_text(encoding="utf-8"))))
    names = sys.argv[1:] or sorted(x.name for x in (HERE / "parts").glob("*.md"))
    bad = 0
    for name in names:
        md = (HERE / "parts" / name).read_text(encoding="utf-8")
        for m in re.finditer(r"^```(rust|toml)\n(.*?)\n```$", md, flags=re.M | re.S):
            ex = lines_of(m.group(2))
            if not ex:
                continue
            found = False
            for p, sl in srcs:
                n = len(ex)
                for i in range(len(sl) - n + 1):
                    if sl[i : i + n] == ex:
                        found = True
                        break
                if found:
                    break
            if not found:
                bad += 1
                line_no = md[: m.start()].count("\n") + 1
                print(f"{name}:{line_no}: excerpt not found: {ex[0][:80]}")
    print(f"done, {bad} unmatched")


if __name__ == "__main__":
    main()
