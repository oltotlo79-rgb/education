"""1つの UNIT を書き終えたとき、巻頭（intro.md）・巻末（outro.md）・assemble.py を更新する。

使い方: python finish_unit.py <巻のフォルダ> <UNIT 番号>
  <巻のフォルダ>/parts/unitNN.meta.md を読む。形式:

    title: UNIT の題（巻頭の表と同じ）
    ---checklist
    - ☐ できるようになったこと
    ---glossary
    | 用語 | 意味 |

何度実行しても同じ結果になるように、済んでいる所は飛ばす。
"""
import re
import sys
from pathlib import Path

vol = Path(sys.argv[1])
n = int(sys.argv[2])
nn = f"{n:02}"
meta = (vol / "parts" / f"unit{nn}.meta.md").read_text(encoding="utf-8").replace("\r\n", "\n")
head, rest = meta.split("---checklist\n", 1)
checklist, glossary = rest.split("---glossary\n", 1)
title = re.search(r"^title: (.+)$", head, re.M).group(1).strip()
checklist = checklist.strip("\n")
glossary = glossary.strip("\n")
prev = "UNIT 01" if n == 2 else f"UNIT 01〜{n - 1:02}"
now = f"UNIT 01〜{nn}"


def write(p, s):
    p.write_text(s, encoding="utf-8", newline="\n")


# ---- intro.md
p = vol / "parts" / "intro.md"
s = p.read_text(encoding="utf-8")
row = re.compile(r"^(\| UNIT " + nn + r" \| .*?)（準備中）( \|.*)$", re.M)
s, k = row.subn(lambda m: m.group(1) + m.group(2), s)
print("intro: 表の（準備中）を外した" if k else "intro: 表は済み")
if n < 10:
    pat = re.compile(r"この版には UNIT 01(?:〜\d\d)? を収録しています。UNIT \d\d(?:〜10)? は準備中で")
    rest = "UNIT 10" if n == 9 else f"UNIT {n + 1:02}〜10"
    s, k = pat.subn(f"この版には {now} を収録しています。{rest} は準備中で", s)
    print("intro: お知らせを更新" if k else "intro: お知らせが見つからない（手で直す）")
else:
    print("intro: 最後の UNIT。お知らせは手で直す")
write(p, s)

# ---- outro.md
p = vol / "parts" / "outro.md"
s = p.read_text(encoding="utf-8")
if f"**UNIT {nn}: " not in s:
    s = s.replace(f"この版に収録した {prev} で、", f"この版に収録した {now} で、", 1)
    block = f"**UNIT {nn}: {title}**\n\n{checklist}\n\n"
    i = s.find("> **お知らせ:**")
    if i < 0:
        i = s.find("## 用語ミニ辞典")
    s = s[:i] + block + s[i:]
    if n < 10:
        notice = re.compile(r"(> \*\*お知らせ:\*\* この版には )UNIT 01(?:〜\d\d)?( を収録しています。)UNIT " + nn + r"（.*?）・(UNIT " + f"{n + 1:02}" + "（)")
        s, k = notice.subn(lambda m: m.group(1) + now + m.group(2) + m.group(3), s)
        print("outro: お知らせを更新" if k else "outro: お知らせが見つからない（手で直す）")
    s = s.replace(f"この巻（{prev}）に出てきた用語です。", f"この巻（{now}）に出てきた用語です。", 1)
    s = s.rstrip("\n") + f"\n\n### UNIT {nn} の用語\n\n| 用語 | 意味 |\n|---|---|\n{glossary}\n"
    write(p, s)
    print("outro: チェックリストと用語を足した")
else:
    print("outro: 済み")

# ---- assemble.py
p = vol / "assemble.py"
s = p.read_text(encoding="utf-8")
s2 = re.sub(r"range\(1, \d+\)", f"range(1, {n + 1})", s)
if s2 != s:
    write(p, s2)
    print(f"assemble: range(1, {n + 1})")
