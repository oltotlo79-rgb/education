"""UNIT 10 の出力から、色の制御文字を除き、練習用のコピーのフォルダ名（rustnote_u07）を rustnote にそろえる。"""
import re
from pathlib import Path

out = Path(__file__).resolve().parent / "out"
ansi = re.compile(chr(27) + r"\[[0-9;]*m")
for p in sorted(out.glob("u10_*.txt")):
    s = p.read_text(encoding="utf-8")
    t = ansi.sub("", s).replace("rustnote_u07", "rustnote").replace("targetb", "target")
    if t != s:
        p.write_text(t, encoding="utf-8", newline="\n")
        print("cleaned", p.name)
