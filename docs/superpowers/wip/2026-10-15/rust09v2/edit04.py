import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)

rep("""- Rust では、`src/lib.rs` に `pub mod routes;` と書くと、`src/routes.rs` が `routes` モジュールになり、`src/routes/` フォルダの下のファイルは、その子のモジュールになります（第4巻 UNIT 06）。""",
"""- Rust では、`src/lib.rs` に `pub mod routes;` と書くと、`src/routes.rs` が `routes` モジュールになります。`src/routes.rs` の中に `pub mod health;` と書くと、`src/routes/health.rs` がその子のモジュールになります（第4巻 UNIT 06）。つまり、`src/routes/` の下のファイルは、どれも `routes` の一部です。""")
rep("""- このテストは、RustNote の関数を1つも呼んでいません。ファイルを文字として読んでいるだけなので、0.65 秒ほどで終わります。""",
"""- このテストは、RustNote の関数を1つも呼んでいません。ファイルを文字として読んでいるだけなので、1秒もかからずに終わります。""")
rep("""UNIT 01 の受入条件の「障害」の枠に、「DB が故障していたら、500 で『時間をおいてもう一度お試しください』とだけ出す」がありました。本物の DB をわざと壊すのは大変ですが、保存先を差し替えれば簡単に確かめられます。""",
"""UNIT 01 の練習問題の解答では、「障害」の枠に AC-15「DB が故障しているときに保存すると、500 で『時間をおいてもう一度お試しください』とだけ出す」を書きました。本物の DB をわざと壊すのは大変ですが、保存先を差し替えれば簡単に確かめられます。ここでは、まず一覧の画面で確かめます（保存の場面は問3で確かめます）。""")
rep("""RustNote では、件数は `search` が返す `NotePage` の `total` に入っているので、新しいメソッドも要りません。""",
"""RustNote では、`search` が返す `NotePage` の `total_count`（条件に合うノートの総数）に件数が入っているので、新しいメソッドも要りません。""")
rep("""になります（第6巻）。URL を見れば""", """になります。URL を見れば""")
rep("""`TagName::parse` は、画面もDB も無くても""", """`TagName::parse` は、画面も DB も無くても""")
rep("""- `impl NoteRepository for BrokenNotes` では、`insert`・`find`・`search`・`update`・`toggle_done`・`delete` の6つを、全部書く必要があります。1つでも足りないと、コンパイルエラー（E0046: 足りないメソッドがある）になります。""",
"""- `impl NoteRepository for BrokenNotes` では、`insert`・`find`・`search`・`update`・`toggle_done`・`delete` の6つを、全部書く必要があります。1つでも足りないと、`error[E0046]: not all trait items implemented, missing: `delete``（約束のメソッドが全部はそろっていません。`delete` がありません）のようなコンパイルエラーになります。""")

old_q3 = t[t.index("**問3の解答:**"):t.index("### まとめ")]
new_q3 = """**問3の解答:**

`tests/u04_q3.rs` です。例4-3の `BrokenNotes` と同じように、どの操作も失敗させます。そのうえで、**呼ばれたメソッドの名前を記録**しておき、最後に「`insert` だけが呼ばれた」ことも確かめます。

```rust
@@FILE:book/rustnote/tests/u04_q3.rs@@
```

```powershell
cargo test --test u04_q3 -- --nocapture
```

```text
@@OUT:u04_q3@@
```

- `POST /notes -> 500 Internal Server Error` になり、画面には決まった文だけが出ました。フォームの `title=` の後ろは、「秘密のメモ」を URL エンコードした文字です。入力した内容が、エラー画面にそのまま出てしまわないことも確かめています。
- 呼ばれたメソッドは `["insert"]` だけでした。作成の処理は、検査に合格したらすぐ `insert` を呼び、その失敗をそのまま 500 にしています。ログの `insert に失敗` も、それを裏付けています。
- `calls` は `Mutex<Vec<…>>` です。`&self` しか受け取らないメソッドの中から記録を書き足すために、`Mutex` で包んでいます（第3巻・第5巻）。ロックを持ったまま `.await` をしないので、標準ライブラリの `Mutex` で十分です。
- `state.notes = notes.clone();` では、`Arc<RecordingBrokenNotes>` を複製して、`Arc<dyn NoteRepository + Send + Sync>` として入れています。中身は1つなので、テストの側に残した `notes` から、同じ記録を読めます。

> **寄り道: 「呼ばれないはず」に unreachable! を書くと、どうなるか:** 記録を取る代わりに、呼ばれないはずのメソッドに `unreachable!("search は呼ばれないはず")` と書きたくなるかもしれません。ところが、ページの処理の中で起きた panic は、テストの失敗にはなりません。`search` を呼ぶ一覧のページ（`GET /notes`）を、すべてのメソッドに `unreachable!` を書いた保存先で開いてみた結果です（`tests/u04_panic.rs`。コードは例4-3と同じ形なので省きます）。

```text
@@OUT:u04_panic@@
```

- panic のメッセージは出ていますが、テストは `ok` です。Topcoat のルーター（`Router::handle`）は、ページの処理の中で起きた panic を受け止めて、500 の返事に変えるからです。1つのリクエストの panic で、サーバー全体が止まらないようにするためです。
- このときの本文は `"internal server error"` の1行だけで、RustNote のエラー画面（レイアウトや「トップページへ戻る」）は出ません。アクセスログの `GET /notes -> ...` の行も出ていません。レイアウトとアクセスログの処理ごと、途中で捨てられたからです。
- ですから、状態コードが 500 かどうかだけを確かめるテストでは、panic に気づけません。問3のテストのように、**本文と、呼ばれたメソッドの記録**まで確かめておくと、思ったとおりの道を通ったかが分かります。

"""
t = t.replace(old_q3, new_q3)
open(p, "w", encoding="utf-8", newline="").write(t)
