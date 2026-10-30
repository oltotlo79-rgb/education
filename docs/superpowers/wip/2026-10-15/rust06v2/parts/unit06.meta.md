title: フォーム・JSON・PRG
---checklist
- ☐ `Form<T>` でフォームの本文を構造体として受け取り、`#[derive(Deserialize)]` と `name` をそろえる理由を言える
- ☐ 成功したら `Err(see_other(…).into())` で 303 に移り、失敗したら `(StatusCode::BAD_REQUEST)` で入力を残して出し直せる
- ☐ `#[route(POST)]` と `Json<T>` で JSON を受け取り、`(StatusCode::CREATED, Json(…))` で 201 と JSON を返せる
- ☐ 形の違う JSON・足りない項目・違う `Content-Type`・大きすぎる本文（既定 2 MiB）が、関数の前に 400・413 で断られることを確かめられる
- ☐ PowerShell 5.1 で JSON を送るとき、ファイルに書いて `--data-binary "@ファイル"` を使う理由を言える
- ☐ `Deserialize` の付け忘れ（E0277）・本文の引数を2つ書いたとき・303 を `Ok` で返したとき（E0308）のエラーを読める
---glossary
| `Form<T>` | フォームの本文（`application/x-www-form-urlencoded`）を、構造体 `T` にして受け取る引数 |
| `Json<T>` | JSON の本文を構造体 `T` にして受け取る引数。返すときは、構造体を JSON にする |
| JSON | `{"title":"milk"}` のように、名前と値の組を書くデータの形。プログラムどうしのやり取りに使う |
| API | プログラムから使うための窓口。ここでは JSON を返す URL |
| `Deserialize` / `Serialize` | serde の trait。本文から構造体を作る／構造体から JSON を作る |
| `see_other` | 303 See Other の返事を作る関数。ページでは `Err(… .into())` で返す |
| 201 Created | 「新しく作りました」という返事 |
| 413 Payload Too Large | 本文が大きすぎて受け取れない、という返事。Topcoat の既定の上限は 2 MiB |
| `Content-Type` | 本文の種類を表すヘッダー。`Json` は `application/json` でなければ断る |
| `100 Continue` | 大きな本文を送る前の「送ってよいですか」に、サーバーが「どうぞ」と答える返事 |
| `BodyLimit` | URL ごとに、受け取る本文の大きさの上限を変えるレイヤー |
