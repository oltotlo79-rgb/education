"""巻の UNIT 10 まで書き終えたとき、巻頭・巻末の「準備中」のお知らせを外す。

使い方: python finish_volume.py <巻のフォルダ>
- intro.md と outro.md の「> **お知らせ:** この版には …」の段落を消す
- 「この版に収録した UNIT 01〜10 で、」を「この巻の UNIT 01〜10 で、」にする
- 表に「（準備中）」が残っていないかを確かめる
"""
import re
import sys
from pathlib import Path

vol = Path(sys.argv[1])
for name in ["intro.md", "outro.md"]:
    p = vol / "parts" / name
    s = p.read_text(encoding="utf-8")
    s2, k = re.subn(r"\n> \*\*お知らせ:\*\* この版には UNIT 01[^\n]*\n", "\n", s)
    s2 = s2.replace("この版に収録した UNIT 01〜10 で、", "この巻の UNIT 01〜10 で、")
    s2 = re.sub(r"\n{3,}", "\n\n", s2)
    p.write_text(s2, encoding="utf-8", newline="\n")
    print(name, "お知らせを消した:", k)
    left = [line for line in s2.splitlines() if "準備中" in line]
    for line in left:
        print("  まだ「準備中」:", line[:120])
