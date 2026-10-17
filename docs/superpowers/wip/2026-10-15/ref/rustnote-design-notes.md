# RustNote 実装の設計メモ（教材執筆者向け）

実装: `C:/Users/oltot/Documents/git-projects/def/Rust/rustnote/`（main にコミット済み 06d6993）。教材のコードはここから引用し、食い違わせないこと。ビルド・テストは `CARGO_TARGET_DIR` をスクラッチに向けて実行する（リポジトリに target/ を作らない）:
`CARGO_TARGET_DIR="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/ref/rustnote-target" cargo +1.99.0 test`
検証ログ: `scratchpad/ref/rustnote-verify/`（verify-fmt-clippy-test.txt、smoke-log.txt、smoke.sh、release-and-startup-errors.txt、topcoat-dev.log）

## 検証状況
- `cargo fmt --check` / `topcoat fmt --check` / `cargo clippy --all-targets -- -D warnings` / `cargo test`（41件: ユニット19・リポジトリ8・HTTP 14）すべて成功（Rust 1.99.0）。
- 実サーバー + curl のスモークテスト済み（signup→logout→login→作成→一覧・検索→編集→409→toggle→他人から 404×5→cross-site POST 403→削除→logout→古いクッキーで入れない）。release を `HOST=0.0.0.0 PORT=8090` で起動し /health 200。
- **未検証**: Docker ビルド、Fly.io 公開、Turso リモート接続（`Builder::new_remote` のコードはある）、GitHub 上の CI、実ブラウザでの Secure クッキー（http://127.0.0.1）、README の turso / fly コマンド書式。教材ではこれらを「例」と明記する。

## ルーティング
- `src/routes.rs` で `module_router!`。モジュールの並び＝URL の並び。
- `{id}` は `routes/notes/id.rs` の `module_param!(pub id: NoteId, error = not_found)`。生成される型は `Id`、`*path_param::<Id>(cx)?` で読む、リンクは `href!(id::show, id::Id(note.id))`。0・負数・数字以外は 404。
- 仕様の URL に加えて `GET /`（ログイン中なら /notes へ 303）と、存在しない URL を 404 画面にする受け皿。
- フォームの POST はすべて `#[page(POST)]`（`#[route]` は /health のみ）。理由: Topcoat は自作エラー型を 500 にし 409 相当の型も無いので、ページにしてレイアウトの `error_boundary` で `AppError` を取り出し正しいステータスの画面にする。成功時は `Err(see_other(..).into())` で 303。未ログインは 303 で /login（flash「ログインしてください」）。

## 型
UserId / NoteId（i64 newtype、`new` で 1 以上を検査）、Email、Password（Debug で隠す）、NoteTitle（1〜80 文字）、NoteBody（10,000 文字まで）、User、Note（持ち主フィールド名は `owner`）、NewNote、NoteQuery、NotePage、RepoError、AppError、Config（`database_auth_token: Option<Secret>`）、Secret、Db、AppState。定数 MAX_TITLE_CHARS=80、PER_PAGE=20。
- `Secret`（`src/config.rs`）: 秘密の値（Turso のトークン）を包む newtype `pub struct Secret(String)`。第4巻 UNIT 09 と同じ形で、`Debug` は derive せず手で書いて `Secret(********)` と表示し、`Display` は実装しない（`{}` で表示するとコンパイルエラー）。作るのは `Secret::new(String)`、中身は `expose()` で取り出す。使うのは DB に接続する `db.rs` の `Db::open` だけ。`Config` を `{:?}` で表示しても `database_auth_token: Some(Secret(********))` となる（unit test `config::tests::debug_output_hides_the_token` で確認）。

## エラーの対応
| ステータス | 場面 |
|---|---|
| 400 | 入力の検証エラー（入力を残してフォーム再表示）、page が数字でない |
| 401 | ログイン失敗 |
| 403 | 他サイトからの POST（Topcoat の OriginPolicy） |
| 404 | ノートが無い・他人のノート・ID 不正・存在しない URL |
| 409 | 登録済みメール、version の食い違い |
| 500 | 内部エラー（画面には出さずログに原因） |
RepoError→AppError: VersionMismatch→Conflict、NotFound→NotFound、それ以外→Internal。

## セッション・認証
- Topcoat の `session::start` / `stop` / `token_hash`。`sessions.token_hash` は BLOB、`expires_at` は UNIX 秒の INTEGER。ログイン時に古いセッションと期限切れを削除。
- 現在のユーザーは `#[memoize(as_ref)]` で 1 リクエスト 1 回だけ DB 問い合わせ。
- 未登録メールでもダミーのハッシュで照合して応答時間をそろえる（ユーザー列挙対策）。
- フラッシュ: クッキー `flash` に決まったコード（8 種類）だけを入れ、表示する GET ページで読んで消す。

## DB
- 日時は UTC の TEXT（`datetime('now')`）。検索はタイトルと本文の LIKE（% と _ をエスケープ）、更新日時の新しい順。接続は操作ごとに作る。テストは一時フォルダの libSQL ファイル。
- migrations: 0001_users.sql / 0002_notes.sql / 0003_sessions.sql。`schema_migrations` は起動時にコードが作り、未適用分だけを 1 つずつ transaction で適用。

## 第4巻との違い（第4巻の説明を「基礎形」、RustNote を「実務形」として橋渡しする）
1. `NoteRepository` のメソッドは async（`#[async_trait]` 0.1.92 で dyn 対応）。
2. `insert(owner, &NewNote)`、`list_by_owner` の代わりに `search(owner, &NoteQuery) -> NotePage`。
3. `status_code()` は u16 ではなく `StatusCode` を返す。
4. `AppError::Internal` は `Box<dyn Error + Send + Sync>` を持つ。
5. `AppState` は users・sessions・db も持つ。
6. `RepoError` に NotFound・InvalidData・Database(libsql::Error)、`AppError` に InvalidBody・InvalidPassword・Unauthorized を追加。
7. InMemory の Fake は無く、テストは本物の libSQL を一時ファイルで使う。

## その他（topcoat-0.10-reference.md §17 にも追記あり）
自作エラーが 500 になる点と回避策、plain fn の `view!` には `cx =>` が必要、手作り Manifest でのアセットのテスト、`topcoat fmt` で長い文字列が崩れる不具合、curl で日本語が化ける件。
