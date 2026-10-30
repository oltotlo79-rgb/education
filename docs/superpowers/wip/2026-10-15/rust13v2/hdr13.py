"""curl -i の生の出力から、状態の行・location・set-cookie（トークンは先頭6文字だけ）・お知らせの表示を取り出す。"""
import re, sys

raw = open(sys.argv[1], encoding="utf-8", errors="replace").read().replace("\r\n", "\n")
head, _, body = raw.partition("\n\n")
lines = head.split("\n")
print(lines[0])
for line in lines[1:]:
    name = line.split(":", 1)[0].lower()
    if name == "location":
        print(line)
    elif name == "set-cookie":
        print(re.sub(r"(__Host-session=)([^;]{6})[^;]*", r"\1\2…", line))
if lines[0].split()[1] == "200":
    found = re.findall(r'<p class="flash[^"]*"[^>]*>[^<]*</p>', body)
    print("お知らせの表示: " + (found[0] if found else "なし"))
