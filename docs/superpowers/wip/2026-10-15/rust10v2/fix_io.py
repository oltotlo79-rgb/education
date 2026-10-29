import sys
def edit(p, reps):
    s = open(p, encoding="utf-8").read()
    for a, b in reps:
        n = s.count(a)
        if n != 1:
            sys.exit(f"{p}: count {n}: {a[:50]}")
        s = s.replace(a, b)
    open(p, "w", encoding="utf-8").write(s)

edit("parts/intro.md", [
 ("| UNIT 02 | 作成のフォーム・入力の検査・PRG（準備中） |", "| UNIT 02 | 作成のフォーム・入力の検査・PRG |"),
 ("> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02〜10 は準備中で、", "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03〜10 は準備中で、"),
])

checklist02 = """
**UNIT 02: 作成のフォーム・入力の検査・PRG**

- ☐ `POST /notes` が、`require_user` → `NewNote::parse` → 保存して 303、または保存せずに 400、と分かれることを説明できる
- ☐ 成功の返事が `303 See Other` と `Location` とお知らせのクッキーだけで、画面は GET で取りに行くこと（PRG）を、テストで確かめられる
- ☐ 再読み込みではノートが増えないのに、同じ POST をもう一度送ると増えることと、二重送信の防ぎ方を言える
- ☐ 400 で出し直すとき、入力を残し、項目ごとのエラーを全部まとめ、フォームを開いたまま返す理由を説明できる
- ☐ `NewNote::parse` が、両方の検査をしてから結果をまとめる理由を言える
- ☐ `String` を `&str` の場所に渡したときの `E0308` と、フォームの `name` が違うときの 400 を読める
- ☐ 前後の空白・改行の `\r\n`・タグや引用符の入ったタイトルが、どう保存され、どう表示されるかを確かめられる
"""
edit("parts/outro.md", [
 ("この版に収録した UNIT 01 で、", "この版に収録した UNIT 01〜02 で、"),
 ("- ☐ 件数の SQL にだけ持ち主の条件を忘れると、件数から他人のノートの数が分かってしまうことを説明できる\n",
  "- ☐ 件数の SQL にだけ持ち主の条件を忘れると、件数から他人のノートの数が分かってしまうことを説明できる\n" + checklist02),
 ("> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（作成のフォーム・入力の検査・PRG）・UNIT 03", "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03"),
 ("この巻（UNIT 01）に出てきた用語です。", "この巻（UNIT 01〜02）に出てきた用語です。"),
])

glossary02 = """
### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| PRG（Post/Redirect/Get） | POST が成功したら、画面を返さずに 303 で別の URL へ移し、ブラウザに GET で開き直させる形 |
| 303 See Other | 「次は、この URL を GET で取りに来て」という返事。`Location` ヘッダーに移動先が入る |
| 400 Bad Request | 送られてきた内容に問題がある、という返事。RustNote は、入力を残した画面と一緒に返す |
| 入力の検査（バリデーション） | 送られてきた値が決まりに合っているかを確かめること。合わなければ保存しない |
| `NoteInputErrors` | 作成・編集の検査で見つかった問題を、項目ごと（`title`・`body`）に入れる型 |
| `NoteFormState` | フォームに出す値とエラー。`from_input` で入力を残し、`with_errors` でエラーの文を入れる |
| お知らせ（フラッシュ） | 移った先で一度だけ出すメッセージ。RustNote はクッキー `flash` で受け渡す |
| 二重送信 | 同じフォームの POST が2回届くこと。PRG では防げない |
| 冪等性（べきとうせい） | 同じ操作を何回くり返しても、1回したときと結果が変わらない性質 |
| `&quot;` | HTML の属性の値の中で、`"` を、ただの文字として書く書き方 |
| XSS | 利用者の入力に書かれたプログラムが、ほかの人の画面で動いてしまう弱さ |
| E0308 | 型が合わないエラー（ここでは `&str` の場所に `String` を渡した） |
"""
s = open("parts/outro.md", encoding="utf-8").read()
s = s.rstrip("\n") + "\n" + glossary02
open("parts/outro.md", "w", encoding="utf-8").write(s)

edit("assemble.py", [("range(1, 2)", "range(1, 3)")])
print("ok")
