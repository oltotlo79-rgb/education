title: 設定と起動の順番
---checklist
- ☐ src/config.rs が環境変数から Config を作る決まり（既定値・Turso ならトークン・空のトークン）を、実際の結果で説明できる
- ☐ Secret 型が {:?} でトークンを隠し、{} では表示させない仕組みと、expose() の役目を言える
- ☐ src/main.rs の起動の順番を言え、どの段で止まると DB のファイルが残るかを、本物の実行ファイルで確かめられる
- ☐ .env は、すでにある環境変数を上書きしないことを確かめられる
- ☐ src/lib.rs の pub mod を消したときの E0432 と、edition 2024 の set_var の E0133 を読める
---glossary
| 環境変数 | プログラムの外から渡す名前と値の組。RustNote は DATABASE_URL などを読む |
| `.env` | 開発のとき、環境変数をまとめて書いておくファイル。すでにある環境変数は上書きしない |
| `Config` | RustNote の設定（DB の URL と、Turso のトークン） |
| `Secret` | 秘密の値を包む型。`{:?}` では `Secret(********)`、`{}` は使えない |
| `expose()` | `Secret` の中身を取り出す、わざと目立つ名前のメソッド |
| `anyhow::Context` | エラーに「何をしようとして失敗したか」を付ける。main の `Error:`・`Caused by:` の表示になる |
| `pub mod` | ファイルを、クレートの部品（モジュール）にする宣言 |
