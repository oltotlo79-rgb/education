import re
import sys

src, dst = sys.argv[1], sys.argv[2]
t = open(src, encoding="utf-8", errors="replace").read()
BS = chr(92)
# (C:\Users\...\basics) -> (.../basics)
t = re.sub(re.escape("(C:" + BS) + "[^)]*" + re.escape(")"), "(.../basics)", t)
open(dst, "w", encoding="utf-8", newline="\n").write(t)
