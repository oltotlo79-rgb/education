"""docker build --progress=plain のログから、段の見出し・CACHED・DONE・送った中身の大きさ・ERROR だけを取り出す（重なりは1回だけ）。
使い方: python steps12.py ログ
"""
import re
import sys

seen = set()
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    line = line.rstrip("\r\n")
    keep = (
        re.match(r"^#\d+ \[", line)
        or re.match(r"^#\d+ (CACHED|DONE [0-9.]+s)$", line)
        or re.search(r"transferring context: [0-9]", line)
        or re.search(r"\bERROR\b", line)
    )
    if keep and line not in seen:
        seen.add(line)
        print(line)
