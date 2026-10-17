"""Replace hand-copied code/output blocks in a part file with placeholders.

Compares ```rust blocks with snips/**/*.rs and ```text blocks with the 1.97 outputs in old197/.
Prints blocks that could not be matched.
usage: python convert.py parts/unit01.md
"""
import re
import sys
from pathlib import Path

R = Path(__file__).resolve().parent
S = R / "snips"
OLD = R / "old197"


def norm(t):
    return t.strip("\n")


def strip_status(text, drop_compiling):
    out = []
    for line in text.split("\n"):
        s = line.strip()
        if s.startswith("Finished ") or s.startswith("Running "):
            continue
        if drop_compiling and s.startswith("Compiling "):
            continue
        out.append(line)
    return "\n".join(out).strip("\n")


codes = {}
for f in S.glob("u*/*.rs"):
    codes.setdefault(norm(f.read_text(encoding="utf-8")), f"{f.parent.name}/{f.name}")

outs = {}
for f in OLD.glob("u*/*.out"):
    key = f"{f.parent.name}/{f.stem}"
    o = norm(f.read_text(encoding="utf-8"))
    errf = f.with_suffix(".err")
    e = errf.read_text(encoding="utf-8") if errf.exists() else ""
    if o:
        outs.setdefault(o, ("OUT", key))
    full = strip_status(e, False)
    body = strip_status(e, True)
    if full and "error" in full:
        outs.setdefault(full, ("ERR", key))
    if body:
        run = (o + "\n" + body).strip("\n") if o else body
        outs.setdefault(run, ("RUN", key))

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
unmatched = []


def repl(m):
    lang, body = m.group(1), m.group(2)
    b = norm(body)
    if lang == "rust" and b in codes:
        return f"```rust\n@@CODE {codes[b]}@@\n```"
    if lang == "text" and b in outs:
        kind, key = outs[b]
        return f"```text\n@@{kind} {key}@@\n```"
    if lang in ("rust", "text"):
        first = b.split("\n")[0][:70]
        unmatched.append(f"{lang}: {first}")
    return m.group(0)


new = re.sub(r"```(\w+)\n(.*?)\n```", repl, text, flags=re.S)
path.with_suffix(".ph.md").write_text(new, encoding="utf-8", newline="\n")
print(path.name, "unmatched:", len(unmatched))
for u in unmatched:
    print("  ", u)
