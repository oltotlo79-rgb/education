# RustNote

Rust の Web フレームワーク **Topcoat 0.11** と **Turso / libSQL** で作った、ログイン付きのメモアプリです。
教材「Rust バックエンド」シリーズで、少しずつ作りながら学ぶサンプルの完成形です。

## できること

- メールアドレスとパスワードで **登録・ログイン・ログアウト**（パスワードは Argon2id でハッシュ化して保存）
- ログインした人だけが、**自分のノート**を一覧・作成・詳細表示・編集・完了の切り替え・削除できる
- 一覧は **キーワード検索**（`?q=`）と **ページ送り**（1ページ 20 件、`?page=`）に対応
- 編集は **楽観的ロック**（`version`）付き。ほかの画面で先に保存されていたら上書きせずに 409 で知らせる
- フォームの送信が成功したら **303 で GET の画面へ移動**（PRG）し、「保存しました」などを一度だけ表示する
- 他人のノートは「見つからない」（404）として扱う
- `GET /health` でヘルスチェック（DB に問い合わせて確認）

## 使っている技術と版

| 項目 | 版 |
|---|---|
| Rust | 1.99 以上（動作確認は 1.99.1。`rust-toolchain.toml` で 1.99.1 を指定） |
| Topcoat（Web フレームワーク） | 0.11.0（`topcoat-cli` も 0.11.0） |
| libsql（Turso / libSQL クライアント） | 0.10.0 |
| argon2（パスワードのハッシュ化） | 0.6.0 |
| tokio / thiserror / anyhow / async-trait / serde / dotenvy | `Cargo.toml` を参照（版は `=` で固定） |

## 準備

1. **Rust**（rustup）を入れる。このフォルダで `cargo` を実行すると、`rust-toolchain.toml` により Rust 1.99.1 が自動で使われます。
2. **topcoat CLI** を入れる（開発サーバーとアセットの束を作るのに使います）。

   ```powershell
   cargo install topcoat-cli --version 0.11.0 --locked
   ```

3. **Windows（ARM64）の場合は LLVM（clang）も必要**です。libsql が使う `ring`（暗号のライブラリ）のビルドに clang を使うためです。

   ```powershell
   winget install LLVM.LLVM
   ```

   インストール後、`clang --version` が動くこと（`C:\Program Files\LLVM\bin` が PATH にあること）を確かめてください。x64 の Windows・macOS・Linux では追加の準備は要りません。

## 起動する（開発）

PowerShell で、このフォルダ（`Rust/rustnote`）に移動して実行します。

```powershell
# 環境変数の見本をコピーする（中身はそのままで動く）
Copy-Item .env.example .env

# 開発サーバー: ビルド → CSS の束を作る → 起動。ファイルを保存すると作り直す
topcoat dev
```

ブラウザで <http://127.0.0.1:3000> を開きます。最初の起動で `rustnote.db`（ローカルの DB ファイル）が作られ、マイグレーションが適用されます。

`cargo run` だけで起動するときは、先に CSS の束（`target/debug/assets/`）を作ってください。束が無いと「アセットの束（assets/）がありません」と表示して止まります。

```powershell
topcoat asset bundle
cargo run
```

> **ログインのクッキーについて:** ログイン状態は `__Host-session` という `Secure` 付きのクッキーで保たれます。`Secure` は本来 HTTPS 専用ですが、`127.0.0.1` / `localhost` は安全な接続とみなされるため、開発中の `http://127.0.0.1:3000` でもログインできます（curl 8.18 で確認）。

## 環境変数

| 名前 | 意味 | 既定値 |
|---|---|---|
| `HOST` | 待ち受けるアドレス（`topcoat::start` が読む） | `127.0.0.1`（Fly.io では `0.0.0.0`） |
| `PORT` | 待ち受けるポート | `3000`（Fly.io では `8080`） |
| `DATABASE_URL` | DB の場所。ローカルファイルのパス、または Turso の URL（`libsql://...`） | `rustnote.db` |
| `DATABASE_AUTH_TOKEN` | Turso の認証トークン。`libsql://` / `https://` の URL のときは必須 | なし |

`.env` があれば起動時に読み込みます（`dotenvy`）。`.env` は秘密の値を書くファイルなので Git に入れません（`.gitignore` 済み）。

## URL の一覧

| メソッドと URL | 内容 | 成功したとき |
|---|---|---|
| `GET /` | トップ（ログイン中なら一覧へ） | 200 / 303 → `/notes` |
| `GET /health` | ヘルスチェック | 200 `{"status":"ok"}`（DB に届かなければ 503） |
| `GET /signup`・`POST /signup` | 登録フォーム・登録 | 303 → `/notes` |
| `GET /login`・`POST /login` | ログインフォーム・ログイン | 303 → `/notes` |
| `POST /logout` | ログアウト | 303 → `/login` |
| `GET /notes` | 一覧（`?q=` 検索、`?page=` ページ） | 200 |
| `POST /notes` | 作成 | 303 → `/notes/{id}` |
| `GET /notes/{id}` | 詳細 | 200 |
| `GET /notes/{id}/edit`・`POST /notes/{id}/edit` | 編集フォーム・保存 | 303 → `/notes/{id}` |
| `POST /notes/{id}/toggle` | 完了 ⇔ 未完了 | 303 → `/notes/{id}` |
| `POST /notes/{id}/delete` | 削除 | 303 → `/notes` |

ログインが必要な URL に未ログインで来ると、303 で `/login` へ移動します。

エラーのときのステータスコード:

| 場面 | ステータス |
|---|---|
| 入力の検査に失敗（空のタイトル、形式の違うメールなど） | 400（入力を残してフォームを再表示） |
| ログインに失敗 | 401 |
| 他のサイトから送られた POST（CSRF） | 403（Topcoat の OriginPolicy が止める） |
| 存在しない・他人のノート、数字でない ID、存在しない URL | 404 |
| 登録済みのメール、ほかの画面で先に保存された（version が古い） | 409 |
| DB の故障など | 500（画面には詳細を出さず、サーバーのログに書く） |

## テスト

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

| 種類 | 場所 | 内容 |
|---|---|---|
| ユニットテスト | `src/*.rs` の `#[cfg(test)] mod tests` | ドメインの型（`NoteTitle`・`Email`・`NoteId` など）、エラーの対応表、設定 |
| リポジトリの結合テスト | `tests/repository.rs` | 一時フォルダの本物の libSQL ファイルに対する読み書き（持ち主ごとの分離、楽観的ロック、検索とページ送り、セッションの期限） |
| HTTP のテスト | `tests/http.rs` | `Router::handle` にリクエストを直接渡す（ポートを開かない）。未ログインのリダイレクト、空タイトルの 400、他人のノートの 404、PRG の 303、409、CSRF の 403 など |

テストは DB を一時フォルダに作るので、`rustnote.db` には触りません。

## フォルダの構成

```text
rustnote/
├── Cargo.toml / Cargo.lock / rust-toolchain.toml / rustfmt.toml
├── .env.example        環境変数の見本
├── migrations/         テーブルを作る SQL（起動時に未適用の分だけ適用）
│   ├── 0001_users.sql
│   ├── 0002_notes.sql
│   └── 0003_sessions.sql
├── assets/style.css    画面のスタイル（asset! で配信）
├── src/
│   ├── main.rs         起動（設定 → アセット → DB → マイグレーション → 待ち受け）
│   ├── lib.rs          部品の一覧
│   ├── config.rs       環境変数から設定（Config）を読む
│   ├── domain.rs       検証済みの型（UserId・NoteId・Email・Password・NoteTitle・NoteBody・Note・NewNote …）
│   ├── error.rs        RepoError・AppError と HTTP ステータスの対応
│   ├── db.rs           DB への接続とマイグレーション
│   ├── repository.rs   リポジトリの入口
│   ├── repository/     notes.rs・users.rs・sessions.rs（trait と libSQL 版の実装）
│   ├── state.rs        AppState（Arc<dyn NoteRepository + Send + Sync> など）
│   ├── auth.rs         パスワード・ログイン・ログイン中のユーザー
│   ├── flash.rs        一度だけ出すメッセージ
│   ├── views.rs        画面の部品とエラー画面
│   ├── routes.rs       ルーター・レイアウト・トップページ
│   └── routes/         URL ごとの処理（ファイルの並び＝URL の並び）
├── tests/              common/mod.rs・repository.rs・http.rs
├── Dockerfile / .dockerignore / fly.toml
└── ci.yml              GitHub Actions の設定例（.github/workflows/ にコピーして使う）
```

## 公開する（Fly.io + Turso）の流れ

> この手順は設定ファイルとしては用意してありますが、作者の手元では Docker のビルドと Fly.io への公開は実行していません（未検証）。

1. Turso に DB を作り、URL とトークンを手に入れる。

   ```powershell
   turso db create rustnote
   turso db show rustnote --url
   turso db tokens create rustnote
   ```

2. `fly.toml` の `app` を自分用の名前に変え、アプリを作る（デプロイはまだしない）。

   ```powershell
   fly launch --no-deploy
   ```

3. DB の URL とトークンを秘密の値として渡す。

   ```powershell
   fly secrets set DATABASE_URL=libsql://rustnote-<組織名>.turso.io DATABASE_AUTH_TOKEN=<トークン>
   ```

4. デプロイする。`Dockerfile` の中で `cargo build --release` と `topcoat asset bundle --release` が行われ、実行ファイルと `assets/` だけが小さなイメージに入ります。アプリは `0.0.0.0:8080` で待ち受け、Fly.io は `GET /health` で動作を確かめます。

   ```powershell
   fly deploy
   ```

起動時に、まだ適用していないマイグレーションが Turso にも自動で適用されます。

## 設計のメモ

- **ミドルウェアではなく関数:** ログインが必要なページは、処理の最初で `require_user(cx)` を呼びます。
- **ログイン中のユーザーは1リクエストで1回だけ調べる:** `#[memoize(as_ref)]` で、レイアウトとページから呼ばれても DB への問い合わせは1回です。
- **セッション:** Topcoat の `session::start` がトークンを発行し、クッキー（`__Host-session`、`Secure`・`HttpOnly`・`SameSite=Lax`）に入れます。アプリはトークンの SHA-256 だけを `sessions` テーブルに保存します。
- **CSRF:** Topcoat の OriginPolicy（既定で有効）が、他のサイトからの POST を 403 で止めます。状態を変える操作はすべて POST です。
- **エスケープ:** `view!` の `(式)` は自動で HTML エスケープされます。
- **DB の接続:** 処理のたびに `Db::connect` で接続を作ります（ローカルファイルなら開くだけ、Turso なら HTTP の準備だけなので軽い）。
