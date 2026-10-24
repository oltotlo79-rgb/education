import sys
BS = chr(92)
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
t = t.replace("..." + BS + "dup_probe" + BS + "rustnote", "..." + BS + "rustnote")
open(p, "w", encoding="utf-8", newline="").write(t)
