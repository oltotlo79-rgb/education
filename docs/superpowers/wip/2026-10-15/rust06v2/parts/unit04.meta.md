title: URL の部品（path・query）と Cx
---checklist
- ☐ `module_param!(id: u64, error = not_found)` で区切りを `{id}` にし、`path_param::<Id>(cx)?` で読める
- ☐ 型を書かない `module_param!(name)` は、`%XX` を戻した文字が `&str` で読めることを説明できる
- ☐ `#[query_params]` の構造体でクエリを読み、無い・空の項目が `None` になることと、項目を `Option` にする理由を言える
- ☐ 読めなかったときに 404（`not_found`）と 400（`bad_request`）のどちらを返すかを、理由を付けて選べる
- ☐ `FromStr` を書いた自分の型（1 以上の数だけの `NoteId`）で URL を読み、決まりを型に任せられる
- ☐ `cx` からメソッド・URL・ヘッダーを読む関数を書け、`E0308`（`Result` を受け取り忘れ）と `E0277`（`view!` に入れられない型）を読める
---glossary
| パスパラメーター | URL のパスの中で変わる部分（`/notes/7` の `7`）。Topcoat では `{id}` と書く |
| `module_param!` | モジュールの区切りを `{id}` にし、読むための型（`Id`）を作るマクロ |
| `path_param::<Id>(cx)` | パスパラメーターを読む関数。型を付けたときは `Result` を返す |
| クエリ文字列 | URL の `?` より後ろの、`名前=中身` を `&` でつないだ部分 |
| `#[query_params]` | クエリを読み込む構造体に付ける印。`query_params::<T>(cx)` で読む |
| パーセントエンコーディング | URL に書けない文字を、`%` と2文字の 16 進数で1バイトずつ書く方法（`%E5%A4%AA` など） |
| `error = not_found` / `error = bad_request` | 読めなかったときに、404 / 400 の返事にする指定 |
| `FromStr` | 文字から値を作る trait。`module_param!` は、これで URL の部品を型に変える |
| `Cx` | 今の要求の中身と、アプリで共有する値を持つ入れ物。`cx: &Cx` で受け取る |
| `User-Agent` | 要求を送ったプログラム（ブラウザや curl）の名前を入れるヘッダー |
| E0277 | 必要な trait が実装されていないエラー（ここでは `view!` に入れられない型） |
