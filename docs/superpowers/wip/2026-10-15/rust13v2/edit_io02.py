import sys
from pathlib import Path

HERE = Path(sys.argv[1])


def patch(name, pairs):
    p = HERE / "parts" / name
    t = p.read_text(encoding="utf-8")
    for old, new in pairs:
        assert t.count(old) == 1, (name, old[:60])
        t = t.replace(old, new)
    p.write_text(t, encoding="utf-8", newline="\n")


patch(
    "intro.md",
    [
        ("| UNIT 02 | 表を作る SQL と DB の入口（準備中） |", "| UNIT 02 | 表を作る SQL と DB の入口 |"),
        (
            "> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02〜10 と、",
            "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03〜10 と、",
        ),
    ],
)

unit02_check = """
**UNIT 02: 表を作る SQL と DB の入口**

- ☐ `migrations/` の3つの SQL ファイルが作る表・索引・制約と、記録簿の `schema_migrations` を言える
- ☐ `db::migrate` が、未適用の版だけを古い順に、1つずつトランザクションの中で適用し、SQL の実行と記録を同じトランザクションに入れる理由を説明できる
- ☐ `migrate` を2回かけても2回目は何もしない（冪等である）ことを確かめられる
- ☐ WAL（ファイルに記録される）と、`busy_timeout`・`foreign_keys`（接続ごと）の違いを、実際の値で説明できる
- ☐ マイグレーションが途中で失敗したとき、作りかけの表と索引が `rollback` で残らないことと、直したあとに失敗した版だけが流れることを確かめられる
- ☐ 適用済みの SQL ファイルを書き換えてはいけない理由を、古い DB と新しい DB の食い違いで説明し、変更を新しい版として足せる
- ☐ `EXPLAIN QUERY PLAN` で、一覧の SQL が索引を使っているかを確かめられる
"""

patch(
    "outro.md",
    [
        ("この版に収録した UNIT 01 で、", "この版に収録した UNIT 01〜02 で、"),
        (
            "- ☐ `git check-ignore -v` で、ファイルが `.gitignore` のどの決まりで外されるかを確かめられる\n",
            "- ☐ `git check-ignore -v` で、ファイルが `.gitignore` のどの決まりで外されるかを確かめられる\n" + unit02_check,
        ),
        (
            "> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（表を作る SQL と DB の入口）・UNIT 03（",
            "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03（",
        ),
        ("この巻（UNIT 01）に出てきた用語です。", "この巻（UNIT 01〜02）に出てきた用語です。"),
    ],
)

p = HERE / "parts" / "outro.md"
t = p.read_text(encoding="utf-8").rstrip("\n")
t += """

### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| マイグレーション | 表の作り（スキーマ）を変える手順を、番号順のファイルに分けて書いたもの |
| `schema_migrations` | 適用済みのマイグレーションの版の名前と時刻を記録する表 |
| トランザクション | いくつかの変更を、全部やるか全部やめるかの1まとまりにする仕組み。`commit` で確定、`rollback` で取り消す |
| 冪等 | 同じ操作を何回くり返しても、1回したときと同じ結果になること |
| `include_str!` | ファイルの中身を、コンパイルのときに文字列として実行ファイルに埋め込むマクロ |
| WAL（`journal_mode = wal`） | 変更をまず別のファイル（`-wal`）に書く方式。読み込みと書き込みが重なっても待ちにくい。ファイルに記録される |
| `busy_timeout` | ほかの接続が書き込み中のとき、何ミリ秒まで待つか。接続ごとの設定 |
| `PRAGMA foreign_keys` | 外部キーの確かめを ON にする SQLite の設定。SQLite では接続ごとに入れる |
| `sqlite_sequence` | `AUTOINCREMENT` の次の番号を、SQLite が自動で記録する表 |
| `EXPLAIN QUERY PLAN` | SQL を、DB がどう実行するつもりか（索引を使うか、全部読むか）を表示する |
| `SQLITE_CANTOPEN`（14） | DB のファイルを開けなかったときのエラーの番号。フォルダが無いときなど |
"""
p.write_text(t + "\n", encoding="utf-8", newline="\n")
print("ok")
