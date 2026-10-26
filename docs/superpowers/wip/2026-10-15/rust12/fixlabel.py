import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
bs = chr(92)
old = "（ここで ." + chr(10) + "ustnote.exe を実行）"
new = "（ここで ." + bs + "rustnote.exe を実行）"
assert t.count(old) == 1
t = t.replace(old, new)
open(p, "w", encoding="utf-8", newline="").write(t)
print("fixed")
