## UNIT 07: migration・seed・テスト用の DB

### この単元のゴール

- 本番・開発・テストの3つの DB に、**同じマイグレーション**（`db::MIGRATIONS`）で表を作る理由と、`schema_migrations` で「どこまで適用したか」を確かめる方法を説明できる
- 開発用の見本のデータ（seed）を、**アプリと同じ関数**（`hash_password`・リポジトリ）で入れ、何度動かしても同じ状態になり、本番の DB には入らないようにできる
- テストが、開発の DB に触らずに、テストごとの一時的な DB で動くことを、ファイルのハッシュで確かめられる
- SQL で直接 seed を入れたとき（ログインが 500）・マイグレーションを呼び忘れたとき（`no such table`）のエラーを読める

### たとえ話でつかむ

チェーンのお店の、売り場の作り方を考えます。

- 本店・練習用の店・新人研修の店の3つがあります。どの店も、**本部の同じ設計図**（棚の配置）で作ります。設計図には「第1版: 入口の棚」「第2版: 冷蔵の棚」のように番号が付いていて、各店は「どの版まで工事したか」を壁の記録に書いています。
- 練習用の店には、**見本の商品**を並べます。見本は、本物と同じ仕入れの手続きで入れます（値札の付け方も同じ）。手で値札を書くと、レジで読めないからです。見本を並べる係は、「ここは本店ではないか」を最初に確かめます。本店に見本が並んだら大変です。
- 研修の店は、研修の**たびに新しく作り**、終わったら壊します。前の研修の商品が残っていると、研修の結果が変わってしまうからです。練習用の店には、研修の人は入りません。

| チェーンのお店 | RustNote の DB |
|---|---|
| 本部の同じ設計図 | `migrations/0001〜0003`（`db::MIGRATIONS`）。どの DB も、これで表を作る |
| 壁の工事の記録 | `schema_migrations` の表（適用した版と日時） |
| 本店 | 本番の DB（Turso）。マイグレーションだけ。見本は入れない |
| 練習用の店と見本の商品 | 開発の DB（`dev.db`）と seed（`examples/u07_seed.rs`） |
| 本物と同じ仕入れの手続き | `hash_password` とリポジトリの `create`・`insert` |
| 研修の店（毎回つくって壊す） | テストの DB（`common::test_db()` の一時フォルダ） |

> **用語:** seed（シード：開発や画面の確かめのために、最初から入れておく見本のデータ。「種」という意味）

<!-- -->

> **用語:** 冪等（べきとう：同じ操作を何回くり返しても、1回したときと同じ状態になること。seed は冪等にしておくと、気軽に動かせる）

### 図で理解する

3つの DB と、それぞれに入るものです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0701" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="220" y="14" width="280" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="38" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">db::MIGRATIONS（同じ SQL）</text>
<text x="360" y="58" font-size="11" fill="#0f172a" text-anchor="middle">0001_users・0002_notes・0003_sessions</text>
<rect x="20" y="120" width="210" height="120" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="125" y="144" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">本番（Turso）</text>
<text x="125" y="168" font-size="11" fill="#0f172a" text-anchor="middle">起動のときに migrate</text>
<text x="125" y="188" font-size="11" fill="#0f172a" text-anchor="middle">データは、本物の利用者だけ</text>
<text x="125" y="212" font-size="11" fill="#b91c1c" text-anchor="middle">seed は断る（is_remote）</text>
<rect x="255" y="120" width="210" height="120" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="144" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">開発（dev.db）</text>
<text x="360" y="168" font-size="11" fill="#0f172a" text-anchor="middle">migrate のあとに seed</text>
<text x="360" y="188" font-size="11" fill="#0f172a" text-anchor="middle">alice・bob と見本のノート</text>
<text x="360" y="212" font-size="11" fill="#047857" text-anchor="middle">何度 seed しても同じ</text>
<rect x="490" y="120" width="210" height="120" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="144" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">テスト（一時フォルダ）</text>
<text x="595" y="168" font-size="11" fill="#0f172a" text-anchor="middle">テストごとに新しく作って migrate</text>
<text x="595" y="188" font-size="11" fill="#0f172a" text-anchor="middle">データは、そのテストが入れたものだけ</text>
<text x="595" y="212" font-size="11" fill="#c2410c" text-anchor="middle">終わるとフォルダごと消える</text>
<line x1="300" y1="70" x2="140" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0701)"/>
<line x1="360" y1="70" x2="360" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0701)"/>
<line x1="420" y1="70" x2="580" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0701)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 3つの DB。表の形は、どれも同じマイグレーションで作る。入るデータは混ぜない。見本のデータは開発の DB にだけ、テストのデータはテストの一時的な DB にだけ入れる</div>
</div>

seed のプログラムの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0702" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="130" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="75" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">Config</text>
<text x="75" y="64" font-size="11" fill="#0f172a" text-anchor="middle">is_remote() なら止める</text>
<rect x="160" y="20" width="120" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="220" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">db::migrate</text>
<text x="220" y="64" font-size="11" fill="#0f172a" text-anchor="middle">表を最新に</text>
<rect x="300" y="20" width="130" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="365" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">hash_password</text>
<text x="365" y="64" font-size="11" fill="#0f172a" text-anchor="middle">アプリと同じ Argon2id</text>
<rect x="450" y="20" width="130" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="515" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">users.create</text>
<text x="515" y="64" font-size="11" fill="#0f172a" text-anchor="middle">リポジトリで作る</text>
<rect x="600" y="20" width="110" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="655" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">notes.insert</text>
<text x="655" y="64" font-size="11" fill="#0f172a" text-anchor="middle">見本のノート</text>
<line x1="140" y1="48" x2="156" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0702)"/>
<line x1="280" y1="48" x2="296" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0702)"/>
<line x1="430" y1="48" x2="446" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0702)"/>
<line x1="580" y1="48" x2="596" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0702)"/>
<rect x="450" y="110" width="260" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="580" y="134" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">UniqueViolation（もういる）</text>
<text x="580" y="154" font-size="11" fill="#0f172a" text-anchor="middle">→ 飛ばす（何度動かしても同じ状態）</text>
<line x1="515" y1="76" x2="530" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0702)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: seed の流れ。本番ではないことを確かめ、本番と同じマイグレーションと、アプリと同じ関数でデータを入れる。もういるユーザーは飛ばすので、何度動かしてもよい</div>
</div>

### まず動かす最小コード

#### 例7-1: マイグレーションの状態を見て、見本のデータを入れる

2つの小さなプログラムを、コピーした `rustnote` の `examples` に置きます。1つ目は、アプリが持つマイグレーションと、DB に記録された適用済みの版を並べて表示します。

```rust
//! 第9巻 UNIT 07: アプリが持つマイグレーションと、DB に記録された適用済みの版を並べて表示する。
//! 実行: $env:DATABASE_URL = "dev.db"; cargo run --example u07_status

use rustnote::{
    config::Config,
    db::{Db, MIGRATIONS},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let db = Db::open(&config).await?;
    let conn = db.connect().await?;

    // DB に記録された版（schema_migrations が無ければ、まだ1つも適用していない）
    let mut applied = Vec::new();
    if let Ok(mut rows) = conn
        .query(
            "SELECT version, applied_at FROM schema_migrations ORDER BY version",
            (),
        )
        .await
    {
        while let Some(row) = rows.next().await? {
            applied.push((row.get::<String>(0)?, row.get::<String>(1)?));
        }
    }

    println!("{}", config.database_url);
    for migration in MIGRATIONS {
        match applied
            .iter()
            .find(|(version, _)| version == migration.version)
        {
            Some((_, at)) => println!("  {}  適用済み（{at}）", migration.version),
            None => println!("  {}  まだ", migration.version),
        }
    }
    Ok(())
}
```

2つ目が、seed です。

> **▼ このコードがやること（先に日本語で）:** 設定を読み、リモートの DB なら止めます。DB を開き、本番と同じ `db::migrate` で表を作ります。alice と bob を、アプリと同じ `hash_password` でパスワードをハッシュにしてから、リポジトリの `create` で作り、それぞれの見本のノートを `insert` で入れます。もういるユーザーは飛ばします。

```rust
//! 第9巻 UNIT 07: 開発用の見本のデータ（seed）を入れる。本番の DB には入れない。
//! 実行: $env:DATABASE_URL = "dev.db"; cargo run --example u07_seed

use rustnote::{
    auth::hash_password,
    config::Config,
    db::{self, Db},
    domain::{Email, NewNote, Password},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

/// 見本のユーザーと、そのノートのタイトル
const SAMPLES: &[(&str, &[&str])] = &[
    (
        "alice@example.com",
        &["牛乳を買う", "本を返す", "歯医者の予約"],
    ),
    ("bob@example.com", &["会議の資料を作る"]),
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    // ネットワークの向こうの DB（本番の Turso など）には、見本のデータを入れない
    if config.is_remote() {
        anyhow::bail!(
            "{} はリモートの DB です。見本のデータは、手元の DB にだけ入れます",
            config.database_url
        );
    }
    let db = Db::open(&config).await?;
    // 本番と同じマイグレーションで、表を作る（済んでいれば何もしない）
    let applied = db::migrate(&db).await?;
    println!("適用したマイグレーション: {applied:?}");

    let users = LibsqlUserRepository::new(db.clone());
    let notes = LibsqlNoteRepository::new(db.clone());
    // パスワードは、アプリと同じ関数でハッシュにする（SQL に直接書かない）
    let password = Password::parse("correct horse battery")?;

    for (email, titles) in SAMPLES {
        let hash = hash_password(&password).await?;
        match users.create(&Email::parse(email)?, &hash).await {
            Ok(user) => {
                for title in *titles {
                    // 見本のタイトルは、ここで決めた正しい値なので、検査に落ちることはない
                    let note = NewNote::parse(title, "").expect("見本のタイトルが不正です");
                    notes.insert(user.id, &note).await?;
                }
                println!("{email} を作り、ノートを {} 件入れました", titles.len());
            }
            // もういれば飛ばす（何度動かしても、同じ状態になる）
            Err(RepoError::UniqueViolation(_)) => println!("{email} はもういるので、飛ばします"),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
```

```text
### $env:DATABASE_URL = "dev.db"; cargo run --example u07_status（まだ何もない）
dev.db
  0001_users  まだ
  0002_notes  まだ
  0003_sessions  まだ
### cargo run --example u07_seed
適用したマイグレーション: ["0001_users", "0002_notes", "0003_sessions"]
alice@example.com を作り、ノートを 3 件入れました
bob@example.com を作り、ノートを 1 件入れました
### cargo run --example u07_status
dev.db
  0001_users  適用済み（2026-10-30 12:51:39）
  0002_notes  適用済み（2026-10-30 12:51:39）
  0003_sessions  適用済み（2026-10-30 12:51:39）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 状態を見て、seed で表と見本を入れる

- 最初の `u07_status` は、3つとも「まだ」でした（`dev.db` のファイルは、開いた時点で空のまま作られます）。
- `u07_seed` は、まず3つのマイグレーションを適用し、alice（ノート3件）と bob（ノート1件）を作りました。もう一度 `u07_status` を見ると、3つとも「適用済み」と、適用した日時（UTC）が出ます。
- マイグレーションは、アプリの起動（UNIT 06）と同じ `db::migrate` です。seed のために別の `CREATE TABLE` を書かないので、**表の形が、アプリとずれることがありません**。

##### 解説2: 見本も、アプリと同じ関数で作る

- パスワードは `hash_password` で Argon2id のハッシュにし、ユーザーは `users.create`、ノートは `notes.insert` で入れています。アプリの登録や作成と、まったく同じ道を通るので、`NewNote::parse` の決まり（1〜80 文字）も、`Email::parse` の決まり（小文字にそろえる）も、そのまま効きます（エラー1）。
- 入れた見本で、アプリを触れます。`dev.db` で起動し、alice でログインして一覧を開きます。

```powershell
$env:DATABASE_URL = "dev.db"
cargo run
```

```text
### curl.exe -i -d "email=alice%40example.com&password=correct+horse+battery" http://127.0.0.1:3000/login
HTTP/1.1 303 See Other
location: /notes
set-cookie: flash=logged_in; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
content-length: 0
（set-cookie: __Host-session=… は省いています）

### curl.exe -s -H "Cookie: （ログインで受け取った __Host-session=…）" http://127.0.0.1:3000/notes の、ノートの行
<a href="/notes/3">歯医者の予約</a>
<a href="/notes/2">本を返す</a>
<a href="/notes/1">牛乳を買う</a>
```

- 見本のパスワード（`correct horse battery`）でログインでき、alice の3件のノートが、新しい順に並びました（第10巻 UNIT 01）。画面を作り込むときや、見た目を確かめるとき（UNIT 09）に、毎回、手で登録しなくて済みます。

### もう一歩: 実務に近い形

#### 例7-2: テストの DB は、開発の DB に触らない

テストは、`tests/common/mod.rs` の `test_db()` で、テストごとに一時フォルダの DB を作ります（第11巻 UNIT 02）。本当に開発の `dev.db` に触らないかを、ファイルのハッシュ（中身から計算した値。中身が1バイトでも変われば変わる）で確かめます。

```powershell
(Get-FileHash dev.db).Hash
cargo test --test repository
(Get-FileHash dev.db).Hash
```

```text
### (Get-FileHash dev.db).Hash（テストの前）
F8D8455C9DD32B27E663F05DF2E7E1A8534B063458F8736B02391B1F4618C7D4
### cargo test --test repository
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
### (Get-FileHash dev.db).Hash（テストのあと）
F8D8455C9DD32B27E663F05DF2E7E1A8534B063458F8736B02391B1F4618C7D4
```

- テストの前とあとで、`dev.db` のハッシュは同じでした。8つのテストは、それぞれ自分の一時的な DB で、ユーザーやノートを作っては消していましたが、`dev.db` には1バイトも触っていません。
- テストが `DATABASE_URL` を読まずに、自分で一時フォルダを作るので、`.env` に何が書いてあっても、テストの結果は変わりません。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 見本のユーザーを、SQL で直接入れた

seed を早く書きたくて、`INSERT INTO users` の SQL で、パスワードをそのまま入れてしまった場合です（`errs07/e07_rawsql.rs`）。

```rust
//! 見本のユーザーを、SQL で直接入れてしまった（パスワードをそのまま書いた）
use rustnote::{config::Config, db::{self, Db}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::open(&Config::from_env()?).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES ('carol@example.com', 'correct horse battery')",
        (),
    )
    .await?;
    println!("carol@example.com を入れました");
    Ok(())
}
```

seed は成功します。そのあとで、carol でログインしようとすると、こうなります。

```text
### $env:DATABASE_URL = "dev.db"; cargo run --example e07_rawsql
carol@example.com を入れました
### curl.exe -i -d "email=carol%40example.com&password=correct+horse+battery" http://127.0.0.1:3000/login
HTTP/1.1 500 Internal Server Error
content-length: 615

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>RustNote</title><link rel="stylesheet" href="/_topcoat/assets/style-e9b983efabcfc6c3.css"></head><body><header class="site-header"><a class="brand" href="/">RustNote</a><nav><a href="/login">ログイン</a><a href="/signup">新規登録</a></nav></header><main class="container"><section class="error-page"><h1>500 Internal Server Error</h1><p>時間をおいてもう一度お試しください</p><p><a href="/">トップページへ戻る</a></p></section></main></body></html>
```

**▼ サーバーの窓の表示**

```text
RustNote を起動しました: http://127.0.0.1:3000
[error] サーバーの中でエラーが起きました <- invalid encoding
POST /login -> 500 Internal Server Error (18.0418ms)
```

- **何が起きたか:** ログインが **500** になり、窓には `invalid encoding` が出ました。`password_hash` の列に、Argon2id の PHC 文字列ではなく、パスワードそのものが入っているので、照合のときに読めなかったのです（第8巻 UNIT 02 のエラー2と同じです）。
- **学べること:** SQL で直接入れると、**アプリの決まりを通らない**データができます。パスワードのハッシュ・メールアドレスのそろえ方・タイトルの長さなど、アプリの関数が守っている決まりを、seed も同じ関数で守ります。おまけに、パスワードをそのまま DB に入れてしまうことにもなります。

#### エラー2: マイグレーションを呼ばずに、見本を入れようとした

新しい DB に、`db::migrate` を呼ばずに、ユーザーを作ろうとした場合です（`errs07/e07_nomigrate.rs`。抜き出しています）。

```rust
    let db = Db::open(&Config::from_env()?).await?;
    // db::migrate(&db).await?; を書き忘れた
    let users = LibsqlUserRepository::new(db.clone());
```

```text
### $env:DATABASE_URL = "empty.db"（新しい DB）; cargo run --example e07_nomigrate
Error: DB の操作に失敗しました

Caused by:
    SQLite failure: `no such table: users`
```

- **何が起きたか:** 「`users` という表がありません」（`no such table: users`）です。DB のファイルは開けましたが、中は空なので、表がまだありません。
- **直し方:** 何かを入れる前に、`db::migrate(&db).await?` を呼びます。アプリの起動・seed・テストの `test_db()` の、どの入口でも最初に呼んでいます。

### RustNote ではここで使う

- マイグレーションは、`migrations/` の3つの SQL ファイルを、`src/db.rs` の `MIGRATIONS` に `include_str!` で埋め込み、`db::migrate` が古い順に、まだ適用していないものだけを流します（第13巻 UNIT 02）。アプリの起動（`main.rs`）とテスト（`tests/common/mod.rs` の `test_db`）が、同じ `migrate` を呼びます。
- RustNote には、seed のプログラムは入っていません。この単元の `u07_seed.rs` は、`examples` に置く形の例です。`examples` に置けば、本番の実行ファイル（`cargo build --release --bin rustnote`）には入りません。
- テストの DB は、`tempfile::tempdir()` で作る一時フォルダの中です（`tests/common/mod.rs`）。開発の `rustnote.db`・`dev.db` とも、本番の Turso とも、混ざりません。

### 練習問題

**問1:** 例7-1の `u07_seed` を、同じ `dev.db` で、もう一度動かすと、どうなるでしょうか。

**問2:** `$env:DATABASE_URL` に Turso の URL（`libsql://rustnote-example.turso.io`）を入れて `u07_seed` を動かすと、どうなるでしょうか。トークンは仮の値を入れます。

**問3:** 例7-2で、テストが `dev.db` に触らないことを確かめました。もし、テストが `DATABASE_URL` を読んで、そこの DB を使う作りだったら、何が困るでしょうか。

### 解答と解説

**問1の解答:**

```text
### （もう一度）cargo run --example u07_seed
適用したマイグレーション: []
alice@example.com はもういるので、飛ばします
bob@example.com はもういるので、飛ばします
```

- マイグレーションは `[]`（何も適用しない）、alice と bob は「もういるので、飛ばします」でした。ユーザーを作るときの `UniqueViolation`（同じメールアドレスがある）を、エラーにせずに飛ばしているからです。
- 何度動かしても同じ状態になる（冪等）ので、「入れたかどうか分からない」ときも、気軽に動かせます。見本を入れ直したいときは、`dev.db` を消してから動かします（開発の DB なので、消してよいものです）。

**問2の解答:**

```text
### $env:DATABASE_URL = "libsql://rustnote-example.turso.io"; $env:DATABASE_AUTH_TOKEN = "dummy"; cargo run --example u07_seed
Error: libsql://rustnote-example.turso.io はリモートの DB です。見本のデータは、手元の DB にだけ入れます
```

- 「リモートの DB です。見本のデータは、手元の DB にだけ入れます」で、**接続する前に**止まりました。`Config::is_remote()`（UNIT 06）で、URL の形を見て断っています。
- 本番に見本のデータが入ると、知らない「alice」が本物の利用者に見えたり、見本のパスワードで誰でもログインできたりします。**入れてはいけない場所を、プログラムが確かめる**ようにしておけば、うっかりの事故を防げます。

**問3の解答:**

- 開発中の `.env` に本番の Turso の URL を書いていたら、`cargo test` が**本番の DB** にテストのユーザーを作ったり、消したりしてしまいます。テストの結果も、DB の中身しだいで変わります（第11巻 UNIT 02 のエラー2）。
- RustNote のテストは、`DATABASE_URL` を読まずに、テストごとに一時フォルダの DB を作るので、どちらも起きません。テストの入口は、**本番の設定とつながらない**ようにしておくのが安全です。

### まとめ

- 本番・開発・テストの DB は、どれも同じ `db::MIGRATIONS` で表を作る。`schema_migrations` に適用済みの版が記録され、`migrate` は新しいものだけを流す。何かを入れる前に、必ず `migrate` を呼ぶ。
- seed は、アプリと同じ関数（`hash_password`・リポジトリ）で入れ、もういるものは飛ばして冪等にし、リモートの DB では動かない。SQL で直接入れると、アプリの決まりを通らないデータができる。
- テストは、テストごとの一時的な DB で動き、開発の DB にも本番の DB にも触らない。
