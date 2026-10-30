"""「UNIT 05で」のように、UNIT の番号のすぐ後ろに、かな・漢字が続いている所に、半角スペースを入れる。
使い方: python fix_unit_space.py
"""
import re
from pathlib import Path

REPO = Path("C:/Users/oltot/Documents/git-projects/def/Rust")
SCRATCH = Path(__file__).resolve().parent
PAT = re.compile(r"(UNIT \d\d)(?=[ぁ-んァ-ヶ一-龠])")
paths = list(REPO.glob("[0-9][0-9]_*.md"))
for d in SCRATCH.glob("rust[0-9][0-9]"):
    paths += list((d / "parts").glob("*.md"))
total = 0
for p in sorted(paths):
    s = p.read_bytes().decode("utf-8")
    s2, n = PAT.subn(lambda m: m.group(1) + " ", s)
    if n:
        p.write_bytes(s2.encode("utf-8"))
        total += n
        print(f"{n:3} {p.name} ({p.parent.parent.name if p.parent.name == 'parts' else 'Rust'})")
print("合計", total)
