import pathlib
import re

P = pathlib.Path(r"C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust00/parts")
for name in ["unit03.md", "unit06.md", "unit09.md"]:
    p = P / name
    t = p.read_text(encoding="utf-8")
    out = []
    pos = 0
    for m in re.finditer(r'<g [^>]*xml:space="preserve"[^>]*>(.*?)</g>', t, re.S):
        out.append(t[pos:m.start(1)])
        inner = m.group(1)
        inner = re.sub(r">([^<]*)<", lambda mm: ">" + mm.group(1).replace(" ", "\u00a0") + "<", inner)
        out.append(inner)
        pos = m.end(1)
    out.append(t[pos:])
    new = "".join(out)
    p.write_text(new, encoding="utf-8", newline="\n")
    print(name, new.count("\u00a0"))
