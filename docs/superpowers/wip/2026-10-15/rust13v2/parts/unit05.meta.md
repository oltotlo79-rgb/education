title: リポジトリ
---checklist
- ☐ trait と libSQL の実装を組にし、AppState が Arc<dyn … + Send + Sync> で持つ理由と、#[async_trait] が要る理由（無いと E0038）を説明できる
- ☐ notes.rs のどの SQL にも user_id = ? が入っていることを示し、消すと notes_are_isolated_per_owner だけが止めることを確かめられる
- ☐ 楽観的ロックの update が、0 行のときに find で VersionMismatch（409）と NotFound（404）を分ける理由を言える
- ☐ LIKE の % と _ を escape しないと、100 で「1000円」まで当たることを示せる
- ☐ UserRecord（ハッシュ付き）と User の使い分けと、セッションをトークンの SHA-256 で探す仕組みを言える
---glossary
| リポジトリ | データのしまい方・取り出し方を引き受ける部品。ほかの部品は SQL を知らずに頼む |
| `#[async_trait]` | async fn を持つ trait を、dyn で使える形に書き換えるマクロ |
| `Arc<dyn Trait + Send + Sync>` | 実装を実行のときに選び、複数のスレッドで共有して使うための形 |
| 楽観的ロック | 先に鍵をかけず、保存するときに版の番号（version）を比べて、食い違いを見つける方法 |
| `RETURNING` | INSERT・UPDATE の終わりに付けて、書き込んだ行をその場で受け取る SQL の書き方 |
| `ESCAPE '\'` | LIKE の中で、`\` の次の文字（`%`・`_`）を、ただの文字として扱わせる指定 |
| `UserRecord` | パスワードのハッシュ付きのユーザー。ログインの照合のときだけ使う |
| `TokenHash` | セッションのトークンの SHA-256（32 バイト）。DB にはトークンそのものではなく、これを保存する |
