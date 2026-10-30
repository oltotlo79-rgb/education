"""curl -i の生の出力から、__Host-session=値 を取り出す（無ければ空）。"""
import re, sys

raw = open(sys.argv[1], encoding="utf-8", errors="replace").read()
m = re.search(r"(?im)^set-cookie: (__Host-session=[^;\r\n]*)", raw)
print(m.group(1) if m else "")
