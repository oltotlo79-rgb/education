import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
rep("この実験で同じ順になったのは、今の SQLite の作り（同じ値なら、行の番号の順に並べる）に、たまたまそろっただけです。",
    "この実験で同じ順になったのは、今の SQLite が、同じ値の行を、たまたま行の番号（id）の大きい順に返したからです。")
rep("@@OUTTAIL:u01_err_borrow:4@@", "@@OUTTAIL:u01_err_borrow:3@@")
rep("（エラーの全文は長いので、最初の2行と最後の4行だけを載せています。", "（エラーの全文は長いので、最初の2行と最後の3行だけを載せています。")
open(p, "w", encoding="utf-8", newline="").write(t)
print("ok")
