"""unit04.src.md の @@FILE:パス@@ と @@OUT:名前@@ を、実際のファイルと実出力で置き換えて unit04.md を作る。"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
src = (HERE / "parts" / "unit04.src.md").read_text(encoding="utf-8")

def file_text(m):
    return (HERE / m.group(1)).read_text(encoding="utf-8").replace("\r\n", "\n").rstrip("\n")

def out_text(m):
    lines = (HERE / "out4" / f"{m.group(1)}.txt").read_text(encoding="utf-8").replace("\r\n", "\n").split("\n")
    while lines and (lines[-1].strip() == "" or lines[-1].startswith("exit=")):
        lines.pop()
    return "\n".join(lines)

doc = re.sub(r"@@FILE:([^@]+)@@", file_text, src)
doc = re.sub(r"@@OUT:([^@]+)@@", out_text, doc)
left = re.findall(r"@@[A-Z]+:[^@]+@@", doc)
if left:
    sys.exit(f"残った目印: {left}")
(HERE / "parts" / "unit04.md").write_text(doc, encoding="utf-8", newline="\n")
print("wrote unit04.md", doc.count("\n"), "行")
