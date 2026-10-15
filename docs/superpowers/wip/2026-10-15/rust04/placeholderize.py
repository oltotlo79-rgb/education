"""Replace pasted code/output blocks in a part with placeholders (when they match real files)."""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def norm(s: str) -> str:
    return "\n".join(line.rstrip() for line in s.replace("\r\n", "\n").strip("\n").split("\n"))


def main():
    part = HERE / "parts" / sys.argv[1]
    md = part.read_text(encoding="utf-8")
    sources = {}
    for p in list((HERE).rglob("*.rs")):
        if "target" in p.parts:
            continue
        sources[norm(p.read_text(encoding="utf-8"))] = p.relative_to(HERE).as_posix()
    outs = {p.stem: p.read_text(encoding="utf-8") for p in (HERE / "out").glob("*.txt")}

    def repl(m):
        lang, body = m.group(1), m.group(2)
        if lang == "rust":
            key = norm(body)
            if key in sources:
                return f"<!--CODE:{sources[key]}-->"
            return m.group(0)
        if lang == "text":
            mm = re.search(r"(?:debug\\|src\\bin\\|examples\\)([A-Za-z0-9_]+)\.(?:exe|rs)", body)
            if mm and mm.group(1) in outs:
                return f"<!--OUT:{mm.group(1)}-->"
            for name, txt in outs.items():
                if norm(txt) == norm(body):
                    return f"<!--OUT:{name}-->"
            return m.group(0)
        return m.group(0)

    new = re.sub(r"^```(rust|text)\n(.*?)\n```$", repl, md, flags=re.M | re.S)
    part.write_text(new, encoding="utf-8", newline="\n")
    remaining = re.findall(r"^```(rust|text)\n(.*?)\n```$", new, flags=re.M | re.S)
    print(f"placeholders: CODE={new.count('<!--CODE:')} OUT={new.count('<!--OUT:')}; remaining literal rust/text blocks: {len(remaining)}")
    for lang, body in remaining:
        print("  -", lang, body.split(chr(10))[0][:70])


if __name__ == "__main__":
    main()
