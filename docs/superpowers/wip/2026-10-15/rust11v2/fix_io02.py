import sys


def edit(p, reps):
    s = open(p, encoding="utf-8").read()
    for a, b in reps:
        n = s.count(a)
        if n != 1:
            sys.exit(f"{p}: count {n}: {a[:50]}")
        s = s.replace(a, b)
    open(p, "w", encoding="utf-8", newline="\n").write(s)


edit("parts/unit02.src.md", [
    ("`TempDir` を持たずに捨ててしまうと、その場でフォルダが消えて、DB を開けなくなります。",
     "`TempDir` を持っておくのは、テストの間はフォルダを残し、終わったら消すためです。"),
])

edit("parts/intro.md", [
    ("| UNIT 02 | リポジトリの結合テスト（準備中） |", "| UNIT 02 | リポジトリの結合テスト |"),
    ("> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02〜10 は準備中で、",
     "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03〜10 は準備中で、"),
])

checklist02 = """
**UNIT 02: リポジトリの結合テスト**

- ☐ リポジトリを、偽物ではなく本物の DB（一時フォルダの libSQL ファイル）で確かめる理由を、`LIKE` の `ESCAPE` の例で説明できる
- ☐ 準備・操作・確かめの形で、成功・見つからない・他人・版の食い違いを、1つずつ確かめるテストを書ける
- ☐ SQL から `ESCAPE` を消すと、ユニットテストは通ったまま、結合テストだけが失敗することを確かめられる
- ☐ 削除の SQL から持ち主の条件を消したときの失敗を読み、`assert!` に実際の値を出す文を足せる
- ☐ 1つの DB ファイルを使い回したテストが、実行のたびに違う理由（UNIQUE・locked・表がもうある）で失敗することと、`common::test_db()` で直す理由を言える
- ☐ 検索の大文字と小文字・セッションの期限ちょうど・ユーザーを消したときの CASCADE を、本物の DB で確かめられる
"""

edit("parts/outro.md", [
    ("この版に収録した UNIT 01 で、", "この版に収録した UNIT 01〜02 で、"),
    ("- ☐ `#[test]` を async fn に付けたエラーと、結果を確かめないテストが何も守っていないことを説明できる\n",
     "- ☐ `#[test]` を async fn に付けたエラーと、結果を確かめないテストが何も守っていないことを説明できる\n" + checklist02),
    ("> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（リポジトリの結合テスト）・UNIT 03",
     "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03"),
    ("この巻（UNIT 01）に出てきた用語です。", "この巻（UNIT 01〜02）に出てきた用語です。"),
])

glossary02 = """
### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| 結合テスト | いくつかの部品を、本物どうしでつないで確かめるテスト。ここでは、リポジトリと本物の DB |
| Arrange・Act・Assert | テストを、準備する・操作する・確かめるの3つに分けて書く形 |
| 偽物（fake） | テストのために作った、本物の代わりの実装。SQL は動かないので、SQL の間違いには気づけない |
| `common::test_db()` | テストごとに一時フォルダを作り、マイグレーション済みの空の DB を渡す RustNote の関数 |
| `TempDir` | `tempfile` クレートの一時フォルダ。片付けられる（drop される）と、フォルダごと消える |
| `ESCAPE '\\'` | `LIKE` の中で、`\\` のうしろの文字（`%`・`_`）を、ただの文字として扱わせる指定 |
| `VersionMismatch` | 更新しようとした version が、今の version と違ったときのエラー（楽観的ロック） |
| 共有しているもの | DB のファイル・ポート番号・環境変数・今の時刻など、テストどうしで同じものを使う所。ときどき失敗するテストの原因になる |
| `database is locked` | ほかの接続が書いている最中で、DB を使えなかったときの SQLite のエラー |
| `ON DELETE CASCADE` | 参照先の行が消えたら、参照している行も一緒に消す、という表の約束 |
"""

s = open("parts/outro.md", encoding="utf-8").read()
s = s.rstrip("\n") + "\n" + glossary02
open("parts/outro.md", "w", encoding="utf-8", newline="\n").write(s)

edit("assemble.py", [("range(1, 2)", "range(1, 3)")])
print("ok")
