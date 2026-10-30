"""curl -i の生の出力を、見やすく取り出す。
使い方: python http13.py <ファイル> <どこを見せるか: none | html | main | header | raw>
- 先頭に、状態の行と、content-type・location・allow・set-cookie（トークンは先頭6文字だけ）を出す
- html は本文全体、main は <main>…</main> の中、header は <header>…</header> を、タグの区切りで改行して出す
"""
import re, sys

raw = open(sys.argv[1], encoding="utf-8", errors="replace").read().replace("\r\n", "\n")
mode = sys.argv[2]
head, _, body = raw.partition("\n\n")
lines = head.split("\n")
print(lines[0])
for line in lines[1:]:
    name = line.split(":", 1)[0].lower()
    if name in ("content-type", "location", "allow"):
        print(line)
    elif name == "set-cookie":
        print(re.sub(r"(__Host-session=)([^;]{6})[^;]*", r"\1\2…", line))


def tags(text):
    return text.replace("><", ">\n<")


if mode == "html":
    print()
    print(tags(body))
elif mode == "main":
    m = re.search(r'<main class="container">(.*)</main>', body, re.S)
    print()
    print(tags(m.group(1)) if m else "（main なし）")
elif mode == "header":
    m = re.search(r"<header.*?</header>", body, re.S)
    print()
    print(tags(m.group(0)) if m else "（header なし）")
elif mode == "raw":
    print()
    print(body)
