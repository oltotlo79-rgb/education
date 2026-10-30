import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
rep("Mutex` で包んでいます（第3巻・第5巻）。", "Mutex` で包んでいます（第3巻 UNIT 10・第5巻 UNIT 05）。")
rep("`routes` が知っているのは、**`NoteRepository` という約束（trait）だけ**で、その中身が libSQL かどうかは知りません。",
    "`routes` が知っているのは、**`NoteRepository` という約束（trait）だけ**で、その中身が libSQL かどうかは知りません（`dyn` で使う trait は第4巻 UNIT 04、リポジトリの形は第7巻 UNIT 09）。")
open(p, "w", encoding="utf-8", newline="").write(t)
