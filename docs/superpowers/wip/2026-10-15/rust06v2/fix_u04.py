import sys

p = "parts/unit04.src.md"
s = open(p, encoding="utf-8").read()
BT = "`"
BS = chr(92)
reps = [
    (BT + "bad request: invalid query value: invalid digit found in string (at " + BS + BT + "page" + BS + BT + ")" + BT,
     BT + BT + "bad request: invalid query value: invalid digit found in string (at " + BT + "page" + BT + ")" + BT + BT),
    (BT + "missing field " + BS + BT + "page" + BS + BT + BT,
     BT + BT + "missing field " + BT + "page" + BT + " " + BT + BT),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a}")
    s = s.replace(a, b)
if (BS + BT) in s:
    sys.exit("まだ \\` が残っている")
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("ok")
