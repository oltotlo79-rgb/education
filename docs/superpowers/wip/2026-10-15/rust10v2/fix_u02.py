import sys
p = "parts/unit02.src.md"
s = open(p, encoding="utf-8").read()
reps = [
 ("- 成功したときの 303 は、`return Err(see_other(…).into())` と、**エラーの形**で返しています。Topcoat では、リダイレクトを「画面を作るのをやめて、別の返事をする」ものとして、`Err` で投げる決まりだからです（第6巻 UNIT 06（準備中））。",
  "- 成功したときの 303 は、`return Err(see_other(…).into())` と、**エラーの形**で返しています。Topcoat の `see_other` は `router::error` にある「エラーの形の返事」で、`Err` で返すと、画面を作るのをやめて、303 の返事になります（第6巻 UNIT 06（準備中））。"),
 ("- 入力欄には `maxlength=\"80\"` も付いているので、ふつうのブラウザでは、そもそも 81 文字目を打ち込めません。それでもサーバーで検査するのは、ブラウザの決まりは、テストのように直接 POST すれば、いくらでも外せるからです。",
  "- 入力欄には `maxlength=\"80\"` も付いているので、ブラウザは、80 を超えて打ち込めないようにします（HTML の決まりでは UTF-16 の単位で数えるので、絵文字などでは、サーバーの数え方とずれます。第11巻 UNIT 01）。それでもサーバーで検査するのは、ブラウザの決まりは、テストのように直接 POST すれば、いくらでも外せるからです。"),
 ("- **学べること:** `String` のまま渡すと、`input.title` の持ち主が `parse` に移ってしまい、あとで `NoteFormState::from_input(&input.title, …)` で入力を出し直すことができなくなります。",
  "- **学べること:** もし `parse` が `String` を受け取る形だったら、`input.title` の持ち主が `parse` に移ってしまい、あとで `NoteFormState::from_input(&input.title, …)` で入力を出し直すことができなくなります。"),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a[:40]}")
    s = s.replace(a, b)
open(p, "w", encoding="utf-8").write(s)
print("ok")
