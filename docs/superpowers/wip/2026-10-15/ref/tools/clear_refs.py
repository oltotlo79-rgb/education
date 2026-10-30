"""書き終えた UNIT への参照から「（準備中）」を外す。

- 各巻の完成した UNIT は、Rust/NN_*.md の巻頭の表（| UNIT NN | 題 |）に「（準備中）」が無いもの
- 「第N巻 UNIT a・b（準備中）」は、N 巻の a と b が両方できていれば外す
- 「UNIT a（準備中）」（巻の番号なし）は、その文書の巻の UNIT として扱う
- 対象: Rust/NN_*.md と、スクラッチの rustNN/parts/*.md（組み立て直しても消えないように）

使い方: python clear_refs.py        （変えたファイルと件数を表示する）
"""
import re
from pathlib import Path

REPO = Path("C:/Users/oltot/Documents/git-projects/def/Rust")
SCRATCH = Path(__file__).resolve().parent

done = {}
for md in sorted(REPO.glob("[0-9][0-9]_*.md")):
    vol = int(md.name[:2])
    text = md.read_text(encoding="utf-8")
    units = set()
    for m in re.finditer(r"^\| UNIT (\d\d) \| ([^|]*)\|", text, re.M):
        if "（準備中）" not in m.group(2):
            units.add(int(m.group(1)))
    done[vol] = units

REF = re.compile(r"(第(\d+)巻 )?UNIT ((?:\d\d)(?:・\d\d)*)（準備中）")


def fix(text, own_vol):
    count = 0

    def repl(m):
        nonlocal count
        vol = int(m.group(2)) if m.group(2) else own_vol
        units = [int(u) for u in m.group(3).split("・")]
        if all(u in done.get(vol, set()) for u in units):
            count += 1
            return m.group(0).replace("（準備中）", "")
        return m.group(0)

    # 表の行（巻頭の UNIT の一覧）は、その巻の進み具合そのものなので触らない
    out = []
    for line in text.split("\n"):
        if re.match(r"^\| UNIT \d\d \|", line):
            out.append(line)
        else:
            out.append(REF.sub(repl, line))
    return "\n".join(out), count


targets = [(md, int(md.name[:2])) for md in REPO.glob("[0-9][0-9]_*.md")]
for d in SCRATCH.glob("rust[0-9][0-9]"):
    for p in (d / "parts").glob("*.md"):
        targets.append((p, int(d.name[4:6])))

total = 0
for path, vol in sorted(targets):
    s = path.read_bytes().decode("utf-8")
    s2, n = fix(s, vol)
    if n:
        path.write_bytes(s2.encode("utf-8"))
        total += n
        print(f"{n:3} {path}")
print("合計", total)
