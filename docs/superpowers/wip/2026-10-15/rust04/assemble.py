"""Assemble the volume from parts, expanding placeholders.

<!--CODE:rel/path.rs-->          -> ```rust  (file content)
<!--FILE:lang:rel/path-->        -> ```lang  (file content)
<!--OUT:name-->                  -> ```text  (out/name.txt)
"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARTS = ["cover.md", "intro.md"] + [f"unit{n:02d}.md" for n in range(1, 11)] + ["outro.md"]


def fence(lang: str, text: str) -> str:
    text = text.replace("\r\n", "\n").rstrip("\n")
    return f"```{lang}\n{text}\n```"


def expand(md: str) -> str:
    def code(m):
        return fence("rust", (HERE / m.group(1)).read_text(encoding="utf-8"))

    def file_(m):
        return fence(m.group(1), (HERE / m.group(2)).read_text(encoding="utf-8"))

    def out(m):
        return fence("text", (HERE / "out" / f"{m.group(1)}.txt").read_text(encoding="utf-8"))

    md = re.sub(r"^<!--CODE:([^>]+?)-->$", code, md, flags=re.M)
    md = re.sub(r"^<!--FILE:([a-z]+):([^>]+?)-->$", file_, md, flags=re.M)
    md = re.sub(r"^<!--OUT:([^>]+?)-->$", out, md, flags=re.M)
    return md


def main():
    target = Path(sys.argv[1])
    chunks = []
    for name in PARTS:
        p = HERE / "parts" / name
        if p.exists():
            chunks.append(expand(p.read_text(encoding="utf-8")).strip("\n"))
        else:
            print(f"missing part: {name}", file=sys.stderr)
    text = "\n\n".join(chunks) + "\n"
    left = re.findall(r"<!--(CODE|FILE|OUT):", text)
    if left:
        raise SystemExit(f"unexpanded placeholders: {len(left)}")
    target.write_text(text, encoding="utf-8", newline="\n")
    print(f"wrote {target} ({text.count(chr(10))} lines)")


if __name__ == "__main__":
    main()
