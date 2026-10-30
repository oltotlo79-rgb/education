import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
rep("""1つでも足りないと、`error[E0046]: not all trait items implemented, missing: `delete``（約束のメソッドが全部はそろっていません。`delete` がありません）のようなコンパイルエラーになります。""",
"""1つでも足りないと、`error[E0046]: not all trait items implemented`（約束のメソッドが全部はそろっていません）というコンパイルエラーになり、続きの `missing: delete` のように、足りないメソッドの名前も教えてくれます。""")
rep("""> **寄り道: 「呼ばれないはず」に unreachable! を書くと、どうなるか:** 記録を取る代わりに""",
"""#### 寄り道: 呼ばれないはずのメソッドに unreachable! を書くと

記録を取る代わりに""")
open(p, "w", encoding="utf-8", newline="").write(t)
