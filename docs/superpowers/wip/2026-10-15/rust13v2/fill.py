"""parts/unitNN.src.md の目印を、実際のファイルと実出力で置き換えて parts/unitNN.md を作る。
目印: @@FILE:パス@@ / @@OUT:名前@@ / @@OUTHEAD:名前:N@@ / @@OUTTAIL:名前:N@@ / @@OUTGREP:名前:正規表現@@ / @@LOCK:パッケージ名@@
"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
name = sys.argv[1]
src = (HERE / "parts" / f"{name}.src.md").read_text(encoding="utf-8")


def out_lines(key):
    lines = (HERE / "out" / f"{key}.txt").read_text(encoding="utf-8").replace("\r\n", "\n").split("\n")
    while lines and (lines[-1].strip() == "" or lines[-1].startswith("exit=")):
        lines.pop()
    return lines


def lock_block(pkg):
    text = (HERE / "rustnote" / "Cargo.lock").read_text(encoding="utf-8").replace("\r\n", "\n")
    start = text.index(f'[[package]]\nname = "{pkg}"\n')
    end = text.index("\n\n", start)
    return text[start:end]


def sub(m):
    kind, arg = m.group(1), m.group(2)
    if kind == "FILE":
        return (HERE / arg).read_text(encoding="utf-8").replace("\r\n", "\n").rstrip("\n")
    if kind == "OUT":
        return "\n".join(out_lines(arg))
    if kind == "LOCK":
        return lock_block(arg)
    key, _, opt = arg.partition(":")
    lines = out_lines(key)
    if kind == "OUTHEAD":
        return "\n".join(lines[: int(opt)])
    if kind == "OUTTAIL":
        return "\n".join(lines[-int(opt):])
    if kind == "OUTGREP":
        return "\n".join(l for l in lines if re.search(opt, l))
    raise SystemExit(f"unknown marker {kind}")


doc = re.sub(r"@@([A-Z]+):([^@]+)@@", sub, src)
left = re.findall(r"@@[A-Z]+:[^@]+@@", doc)
if left:
    sys.exit(f"残った目印: {left}")
(HERE / "parts" / f"{name}.md").write_text(doc, encoding="utf-8", newline="\n")
print(f"wrote {name}.md", doc.count("\n"), "lines")
