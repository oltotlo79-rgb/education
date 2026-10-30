title: ドメインの型とエラーの分類
---checklist
- ☐ 検査済みの型の中身が private で、parse を通らずには作れない（E0423）ことと、UserId と NoteId を取り違えられない（E0308）ことを説明できる
- ☐ タイトル・本文・メール・パスワード・ID の数え方とそろえ方を、例4-3の結果（NUL・絵文字・全角）で言える
- ☐ NewNote::parse が問題を全部集める理由を、? で書いた版と比べて説明できる
- ☐ RepoError と AppError の2つの層、#[from] と From による ? の変換（無いと E0277）、status_code と public_message の役目を言える
- ☐ NoteQuery の pub な項目で page: 0 を作ると、debug と release でどうなるかを確かめ、直し方を言える
---glossary
| 検査済みの型 | 決まりの検査に合格した値だけが入る型。中身を private にして、parse を通らないと作れないようにする |
| `thiserror` | `#[derive(Error)]` と `#[error("…")]` で、エラーの型とその文を短く書けるようにするクレート |
| `#[from]` | thiserror の印。`From` を作るので、`?` でそのエラーを自動で変換できる |
| `#[source]` | thiserror の印。元のエラーを「原因」として持っておき、ログに原因まで書けるようにする |
| `RepoError` | RustNote の DB の層の失敗（一意制約の違反・version の食い違い・見つからない など） |
| `AppError` | RustNote の、利用者への返事を決める分類。HTTP の状態コードに対応する |
| `public_message` | 画面に出してよい文。500 のときは原因を隠す |
| `FromStr` | `"42".parse::<NoteId>()` のように、文字列から型を作れるようにする trait |
