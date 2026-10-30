import sys

p = "parts/unit05.src.md"
s = open(p, encoding="utf-8").read()
BT = "`"
a = "ふつうの関数の中では、" + BT + "cx =>" + BT + " を書かないと、" + BT + "view!" + BT + " が文脈を見つけられずにエラーになります（UNIT 03 のエラー2の前の版で、実際に " + BT + "cannot find value __cx" + BT + " というエラーが出ました）。"
b = "ふつうの関数の中では、" + BT + "cx =>" + BT + " を書かないと、" + BT + "view!" + BT + " が文脈を見つけられずに、" + BT + BT + "cannot find value " + BT + "__cx" + BT + " in this scope" + BT + BT + "（" + BT + "__cx" + BT + " という値が見つかりません）というエラーになります。"
n = s.count(a)
if n != 1:
    sys.exit(f"count {n}")
open(p, "w", encoding="utf-8", newline="\n").write(s.replace(a, b))
print("ok")
