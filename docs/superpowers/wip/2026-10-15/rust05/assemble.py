"""第5巻の部品（parts/）を連結し、図番号を振ってリポジトリの Rust/ に書き出す。"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = Path("C:/Users/oltot/Documents/git-projects/def/Rust/05_Rustバックエンド_async_並行処理_ジョブ_運用.md")
ORDER = ["intro.md"] + [f"unit{n:02}.md" for n in range(1, 11)] + ["outro.md"]

texts = []
for name in ORDER:
    path = HERE / "parts" / name
    if not path.exists():
        sys.exit(f"missing: {path}")
    texts.append(path.read_text(encoding="utf-8").replace("\r\n", "\n").strip("\n"))
doc = "\n\n".join(texts) + "\n"

numbered = re.findall(r"図5-\d+", doc)
if numbered:
    print("warning: 番号付きの図の参照が残っている:", sorted(set(numbered))[:20])

count = 0


def renumber(_m: re.Match) -> str:
    global count
    count += 1
    return f"図5-{count}:"


doc = re.sub(r"図5-\?:", renumber, doc)
left = doc.count("図5-?")
print(f"図: {count} 枚に番号を付けた。残った 図5-? = {left}")
OUT.write_text(doc, encoding="utf-8", newline="\n")
print(f"wrote {OUT} ({doc.count(chr(10))} 行)")
