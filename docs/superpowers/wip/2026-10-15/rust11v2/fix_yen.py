import re
import sys

p = "parts/unit02.src.md"
s = open(p, encoding="utf-8").read()
mono = '<tspan font-family="Consolas, monospace">\\</tspan>'
span = '<span style="font-family:Consolas,monospace;">\\</span>'
reps = [
    ("LIKE ?2 ESCAPE '\\'（SQL・元のまま）", "LIKE ?2 ESCAPE '" + mono + "'（SQL・元のまま）"),
    ("「100\\」を含む → 0 件", "「100" + mono + "」を含む → 0 件"),
    ("SQL が \\ を「ただの \\」として読むので", "SQL が " + span + " を「ただの " + span + "」として読むので"),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a}")
    s = s.replace(a, b)
open(p, "w", encoding="utf-8", newline="\n").write(s)

# 3つの巻の UNIT 02 で、コードの外にある \ を探す（PDF で ¥ に見えるおそれ）
for vol in ["rust08", "rust10", "rust11"]:
    path = f"../{vol}/parts/unit02.md"
    fence = False
    for no, line in enumerate(open(path, encoding="utf-8"), 1):
        if line.startswith("```"):
            fence = not fence
            continue
        if fence:
            continue
        plain = re.sub(r"`[^`]*`", "", line)
        plain = plain.replace(mono, "").replace(span, "")
        if "\\" in plain:
            print(vol, no, plain.strip()[:100])
print("ok")
