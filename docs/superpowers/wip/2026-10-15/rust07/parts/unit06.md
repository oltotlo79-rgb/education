## UNIT 06: migration と schema version

### この単元のゴール

- migration（マイグレーション）が何のためにあるかを説明できる。「表の形の変更を、番号付きの手順として記録し、順に適用する」
- `schema_migrations` 表で「どこまで適用したか」を記録し、**まだ適用していない分だけ**を1つずつ transaction の中で適用するプログラムを読める
- 行がある本番の表に列を足すときの落とし穴（`NOT NULL` と `DEFAULT`）を知り、安全な書き方ができる
- `ALTER TABLE` でできない変更を、「表の作り直し」で行う手順を説明できる

### たとえ話でつかむ

家の増築や改装をするとき、工務店は**工事台帳**を付けます。

- 工事には「第1期: 基礎」「第2期: 1階」「第3期: 屋根」「第4期: ベランダ増設」と、**番号と順番**があります。
- 台帳には「第3期まで完了」と記録してあります。だから次に来た職人は、第1期からやり直したりせず、**第4期から**始めます。
- 一度終わった工事の記録を、後から書き換えてはいけません。「第2期の内容を変えたい」なら、新しく「第5期: 1階の改装」を足します。
- 住人がいる家（本番）の工事は、空き地（開発用）の工事とは違う注意が必要です。家具を置いたまま床を張り替えることはできません。

migration は、DB の表の形（スキーマ）についての、この工事台帳です。`migrations/` フォルダに番号付きの SQL ファイルを並べ、DB の中の `schema_migrations` 表に「どの番号まで適用したか」を記録します。

> **用語:** スキーマ（schema：DB にどんな表があり、それぞれどんな列と制約を持つか、という「形」の定義）

<!-- -->

> **用語:** migration（マイグレーション：スキーマを変える SQL を、番号付きの手順として管理し、まだ適用していないものだけを順に適用する仕組み）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-061" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="130" y="22" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">アプリが持つ migration（ファイル）</text>
<text x="590" y="22" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DB の schema_migrations（記録）</text>
<rect x="20" y="36" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="130" y="57" font-size="12" fill="#0f172a" text-anchor="middle">0001_users.sql</text>
<rect x="20" y="76" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="130" y="97" font-size="12" fill="#0f172a" text-anchor="middle">0002_notes.sql</text>
<rect x="20" y="116" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="130" y="137" font-size="12" fill="#0f172a" text-anchor="middle">0003_sessions.sql</text>
<rect x="20" y="156" width="220" height="32" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="130" y="177" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">0004_notes_pinned.sql（新しい）</text>
<rect x="480" y="36" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="590" y="57" font-size="12" fill="#0f172a" text-anchor="middle">0001_users（適用済み）</text>
<rect x="480" y="76" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="590" y="97" font-size="12" fill="#0f172a" text-anchor="middle">0002_notes（適用済み）</text>
<rect x="480" y="116" width="220" height="32" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="590" y="137" font-size="12" fill="#0f172a" text-anchor="middle">0003_sessions（適用済み）</text>
<rect x="480" y="156" width="220" height="32" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="590" y="177" font-size="12" fill="#94a3b8" text-anchor="middle">（まだ記録が無い）</text>
<line x1="240" y1="52" x2="476" y2="52" stroke="#10b981" stroke-width="1.5" stroke-dasharray="4 3"/>
<line x1="240" y1="92" x2="476" y2="92" stroke="#10b981" stroke-width="1.5" stroke-dasharray="4 3"/>
<line x1="240" y1="132" x2="476" y2="132" stroke="#10b981" stroke-width="1.5" stroke-dasharray="4 3"/>
<text x="360" y="125" font-size="11" fill="#047857" text-anchor="middle">記録があるので飛ばす</text>
<rect x="250" y="206" width="220" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="228" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">BEGIN</text>
<text x="360" y="246" font-size="11" fill="#0f172a" text-anchor="middle">0004 の SQL ＋ 記録の INSERT</text>
<text x="360" y="262" font-size="11" fill="#0f172a" text-anchor="middle">COMMIT（失敗なら ROLLBACK）</text>
<path d="M130,188 V237 H246" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-061)"/>
<path d="M470,237 H590 V192" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-061)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: ファイルと記録を比べ、記録の無い migration だけを、1つずつ transaction の中で適用する。SQL と記録は同じ transaction なので、「SQL だけ適用されて記録が無い」状態にならない</div>
</div>

### まず動かす最小コード

#### 例6-1: 未適用の migration だけを適用するプログラム

`sql-lab` の `migrations/` フォルダには、RustNote と同じ3つのファイル（`0001_users.sql`・`0002_notes.sql`・`0003_sessions.sql`）があります。UNIT 01 で `--setup` に使ったものです。これらを順に適用するプログラムを作ります。

> **▼ このコードがやること（先に日本語で）:** 3つの migration の SQL を、`include_str!` で実行ファイルに埋め込みます。DB ファイル `u06.db` を開き、記録用の表 `schema_migrations` が無ければ作ります。記録に無い migration だけを、1つずつ transaction の中で「SQL を実行 → 記録を追加 → 確定」します。失敗したら、その migration を取り消して、エラーと原因を表示します。

```rust
use libsql::{Builder, Connection, Transaction};
use thiserror::Error;

// 1つのマイグレーション（版の名前と SQL）
struct Migration {
    version: &'static str,
    sql: &'static str,
}

// アプリが持つマイグレーションの一覧（古い順）。
// SQL ファイルは include_str! でコンパイル時に実行ファイルへ埋め込む
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: "0001_users",
        sql: include_str!("../../migrations/0001_users.sql"),
    },
    Migration {
        version: "0002_notes",
        sql: include_str!("../../migrations/0002_notes.sql"),
    },
    Migration {
        version: "0003_sessions",
        sql: include_str!("../../migrations/0003_sessions.sql"),
    },
];

#[derive(Debug, Error)]
enum MigrateError {
    #[error("schema_migrations を準備できません")]
    Prepare(#[source] libsql::Error),
    #[error("マイグレーション {version} に失敗しました")]
    Apply {
        version: &'static str,
        #[source]
        source: libsql::Error,
    },
}

// まだ適用していないマイグレーションだけを、1つずつトランザクションの中で適用する。
// 戻り値は、今回新しく適用した版の名前の一覧
async fn migrate(conn: &Connection) -> Result<Vec<&'static str>, MigrateError> {
    // 適用済みの版を記録する表（無ければ作る）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        (),
    )
    .await
    .map_err(MigrateError::Prepare)?;

    let applied = applied_versions(conn)
        .await
        .map_err(MigrateError::Prepare)?;

    let mut newly_applied = Vec::new();
    for migration in MIGRATIONS {
        if applied.iter().any(|v| v == migration.version) {
            continue;
        }
        let apply_error = |source| MigrateError::Apply {
            version: migration.version,
            source,
        };
        let tx = conn.transaction().await.map_err(apply_error)?;
        match apply(&tx, migration).await {
            // 全部成功したときだけ確定する
            Ok(()) => tx.commit().await.map_err(apply_error)?,
            // 途中で失敗したら、その版の変更をすべて取り消す
            Err(source) => {
                tx.rollback().await.map_err(apply_error)?;
                return Err(apply_error(source));
            }
        }
        newly_applied.push(migration.version);
    }
    Ok(newly_applied)
}

// schema_migrations に記録された版の名前を読む
async fn applied_versions(conn: &Connection) -> Result<Vec<String>, libsql::Error> {
    let mut rows = conn
        .query("SELECT version FROM schema_migrations ORDER BY version", ())
        .await?;
    let mut versions = Vec::new();
    while let Some(row) = rows.next().await? {
        versions.push(row.get::<String>(0)?);
    }
    Ok(versions)
}

// 1つのマイグレーションの SQL を実行し、適用済みとして記録する
async fn apply(tx: &Transaction, migration: &Migration) -> Result<(), libsql::Error> {
    tx.execute_batch(migration.sql).await?;
    tx.execute(
        "INSERT INTO schema_migrations (version) VALUES (?1)",
        [migration.version],
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let db = Builder::new_local("u06.db")
        .build()
        .await
        .expect("DB を開けません");
    let conn = db.connect().expect("接続できません");

    match migrate(&conn).await {
        Ok(applied) if applied.is_empty() => println!("新しく適用したもの: なし"),
        Ok(applied) => println!("新しく適用したもの: {applied:?}"),
        Err(error) => {
            // エラーの原因（source）をたどって全部表示する
            println!("エラー: {error}");
            let mut source = std::error::Error::source(&error);
            while let Some(cause) = source {
                println!("  原因: {cause}");
                source = cause.source();
            }
        }
    }
    let applied = applied_versions(&conn).await.expect("読めません");
    println!("適用済みの版: {applied:?}");
}
```

`src/bin/u06_migrate.rs` として保存し、**2回続けて**実行します。

```powershell
cargo run -q --bin u06_migrate
cargo run -q --bin u06_migrate
```

**▼ 実行結果**（1回目）

```text
新しく適用したもの: ["0001_users", "0002_notes", "0003_sessions"]
適用済みの版: ["0001_users", "0002_notes", "0003_sessions"]
```

**▼ 実行結果**（2回目）

```text
新しく適用したもの: なし
適用済みの版: ["0001_users", "0002_notes", "0003_sessions"]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: include_str! で SQL を実行ファイルに埋め込む

- `include_str!("../../migrations/0001_users.sql")` は、**コンパイルのときに**ファイルを読み、その中身を `&'static str`（プログラムが終わるまで有効な文字列）として埋め込むマクロです。パスは、このコードが書かれたファイル（`src/bin/u06_migrate.rs`）から見た場所です。
- 埋め込んでおけば、本番のサーバーに `migrations/` フォルダを一緒に置かなくても、実行ファイル1つで migration ができます。ファイルが無ければ、コンパイルの時点でエラーになるので（「よくあるエラー」で見ます）、置き忘れにも気づけます。

##### 解説2: 記録の表 schema_migrations

- `version` 列が主キーなので、同じ版を2回記録することはできません。`applied_at` には、適用した日時が自動で入ります。
- `CREATE TABLE IF NOT EXISTS` なので、何回実行しても大丈夫です。2回目の実行では、3つとも記録にあったので「なし」になりました。**何度実行しても同じ結果になる**ことを冪等（べきとう）と呼び、起動のたびに migration を実行するアプリには欠かせない性質です。

##### 解説3: SQL と記録を、同じ transaction で

- `apply` は、migration の SQL（`execute_batch` で複数の文をまとめて実行）と、`schema_migrations` への記録を、**同じ transaction の中で**行います。
- もし SQL だけ成功して記録が失敗したら、次の起動で同じ SQL がもう一度流れ、`table … already exists` で止まってしまいます。同じ transaction にしておけば、「両方成功」か「両方取り消し」のどちらかにしかなりません。UNIT 05 で学んだ transaction が、ここで役に立っています。
- 失敗したときは `tx.rollback()` で取り消し、どの版で失敗したかを `MigrateError::Apply` に入れて返します。`#[source]` を付けた元のエラーは、`main` の `source()` のループで「原因」として表示されます（第4巻 UNIT 05 で学んだ、エラーの原因のたどり方です）。

### もう一歩: 実務に近い形

#### 例6-2: 行がある DB に列を足す（成功する場合・失敗する場合）

アプリを公開した後で、「ノートをピン留めしたい」「ノートに色を付けたい」という機能を足すことになりました。次の2つの migration を、`migrations/` に追加します。

```sql
-- 0004: ノートを「ピン留め」できるようにする
-- 既にある行には DEFAULT の 0 が入る
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
```

```sql
-- 0005: タグの表を作り、ノートに色の列を足す（わざと間違いを入れてある）
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL;
```

`ALTER TABLE 表 ADD COLUMN 列の定義` は、すでにある表に列を1つ足す SQL です。

> **▼ このコードがやること（先に日本語で）:** 例6-1 で作った `u06.db` に、会員1人とノート1冊を入れて「アプリを使い始めた」状態にします。`MIGRATIONS` に 0004 を足して実行し、既にあったノートの `pinned` が 0 になったことを確かめます。次に 0005 を足して実行すると失敗し、0005 の途中で作った `tags` 表も取り消されていることを確かめます。

まず、データを入れる SQL（`sql/u06_data.sql`）を `--db u06.db` で、ファイルの DB に対して実行します。

```sql
-- アプリを使い始めた（会員とノートができた）
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a');
INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト');
SELECT id, title, done FROM notes;
```

```powershell
cargo run -q --bin sql -- --db u06.db sql/u06_data.sql
```

```text
sql> INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト')
OK（変更: 1 行）

sql> SELECT id, title, done FROM notes
+----+--------------+------+
| id | title        | done |
+----+--------------+------+
| 1  | 買い物リスト | 0    |
+----+--------------+------+
(1 行)
```

次に、`u06_migrate.rs` の `MIGRATIONS` の最後に、0004 の1件を書き足して実行します。

```rust
    Migration {
        version: "0004_notes_pinned",
        sql: include_str!("../../migrations/0004_notes_pinned.sql"),
    },
```

```powershell
cargo run -q --bin u06_migrate
```

```text
新しく適用したもの: ["0004_notes_pinned"]
適用済みの版: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned"]
```

既にあったノートの `pinned` を見てみます（`sql/u06_after4.sql`: `SELECT id, title, done, pinned FROM notes;`）。

```powershell
cargo run -q --bin sql -- --db u06.db sql/u06_after4.sql
```

```text
sql> SELECT id, title, done, pinned FROM notes
+----+--------------+------+--------+
| id | title        | done | pinned |
+----+--------------+------+--------+
| 1  | 買い物リスト | 0    | 0      |
+----+--------------+------+--------+
(1 行)
```

続けて、0005 の1件も `MIGRATIONS` に書き足して実行します。

```powershell
cargo run -q --bin u06_migrate
```

```text
エラー: マイグレーション 0005_tags に失敗しました
  原因: SQLite failure: `Cannot add a NOT NULL column with default value NULL`
適用済みの版: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned"]
```

0005 の途中（1文目）で作った `tags` 表は残っているでしょうか。`sql/u06_check.sql` で確かめます。

```sql
-- 0005 の途中で作った tags の表は残っているか？
SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name;
-- 記録された版
SELECT version FROM schema_migrations ORDER BY version;
```

```powershell
cargo run -q --bin sql -- --db u06.db sql/u06_check.sql
```

```text
sql> SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name
+-------------------+
| name              |
+-------------------+
| notes             |
| schema_migrations |
| sessions          |
| sqlite_sequence   |
| users             |
+-------------------+
(5 行)

sql> SELECT version FROM schema_migrations ORDER BY version
+-------------------+
| version           |
+-------------------+
| 0001_users        |
| 0002_notes        |
| 0003_sessions     |
| 0004_notes_pinned |
+-------------------+
(4 行)
```

##### 解説4: DEFAULT があれば、既にある行にも値が入る

- 0004 の `pinned` には `DEFAULT 0` があるので、既にあったノートにも 0 が入り、`NOT NULL` を守れました。
- 0005 の `color TEXT NOT NULL` には `DEFAULT` がありません。既にあるノートの `color` は「値が無い（NULL）」になるしかなく、`NOT NULL` と矛盾するので、`Cannot add a NOT NULL column with default value NULL`（DEFAULT が NULL の NOT NULL の列は足せません）で断られました。
- 0005 は transaction の中で適用していたので、1文目で作った `tags` 表も**一緒に取り消され**、記録も 0004 までのままです。DB は「0004 まで適用済み」のきれいな状態に保たれています。0005 のファイルを直せば、次の起動で 0005 だけがやり直されます（練習問題1）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-062" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="250" y="16" width="220" height="40" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="41" font-size="12" fill="#0f172a" text-anchor="middle">ALTER TABLE notes ADD COLUMN color TEXT NOT NULL</text>
<line x1="300" y1="56" x2="190" y2="84" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-062)"/>
<line x1="420" y1="56" x2="530" y2="84" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-062)"/>
<rect x="20" y="88" width="320" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="112" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">空の DB（開発・テスト）</text>
<text x="180" y="136" font-size="12" fill="#0f172a" text-anchor="middle">notes に行が 0 件</text>
<text x="180" y="156" font-size="12" fill="#0f172a" text-anchor="middle">→ 値を埋める行が無いので通る</text>
<text x="180" y="182" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">成功（5 件とも適用）</text>
<rect x="380" y="88" width="320" height="110" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="540" y="112" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">行がある DB（本番）</text>
<text x="540" y="136" font-size="12" fill="#0f172a" text-anchor="middle">既にあるノートの color は？</text>
<text x="540" y="156" font-size="12" fill="#0f172a" text-anchor="middle">→ NULL しか入れられず NOT NULL と矛盾</text>
<text x="540" y="182" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">失敗（0005 を取り消し）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 同じ migration でも、空の DB では通り、行がある DB では失敗することがある。開発で成功しても本番で失敗する典型例</div>
</div>

##### 解説5: 開発では成功するのに、本番で失敗する

上の図の左側を確かめてみましょう。`u06.db` を消して（空の DB から）、0001〜0005 の5件を書いた `u06_migrate` を実行すると、こうなります。

```powershell
Remove-Item u06.db
cargo run -q --bin u06_migrate
```

```text
新しく適用したもの: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned", "0005_tags"]
適用済みの版: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned", "0005_tags"]
```

**5件とも成功しました**。空の表なら、値を埋める行が1つも無いので、`DEFAULT` が無くても矛盾しないのです。開発中は毎回まっさらな DB で試すことが多いので、この間違いに気づかないまま本番に出してしまいがちです。migration は、**本番と同じように行が入っている DB**（本番のコピーや、練習用データを入れた DB）でも試してから出しましょう。

#### 例6-3: ALTER TABLE でできない変更は「表の作り直し」

SQLite の `ALTER TABLE` でできるのは、列の追加・列名の変更・表名の変更・列の削除など、限られた変更だけです。たとえば「既にある列に、後から `CHECK` を足す」ことはできません。そのときは、**新しい形の表を作って中身を写し、入れ替える**という手順を踏みます。

> **▼ このコードがやること（先に日本語で）:** 本文に「10,000 文字まで」の `CHECK` を足すために、transaction の中で、新しい形の表 `notes_new` を作り、中身を写し、古い表を消し、新しい表の名前を `notes` に変え、索引を作り直します。最後に、件数が同じことと、新しい `CHECK` が効いていることを確かめます。

```sql
-- 本文に「10,000 文字まで」の CHECK を足したい（ALTER TABLE では後から CHECK を足せない）
BEGIN;
-- 1) 新しい形の表を、別の名前で作る
CREATE TABLE notes_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    body TEXT NOT NULL DEFAULT '' CHECK (length(body) <= 10000),
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)),
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1))
);
-- 2) 中身を写す
INSERT INTO notes_new (id, user_id, title, body, done, version, created_at, updated_at, pinned)
SELECT id, user_id, title, body, done, version, created_at, updated_at, pinned FROM notes;
-- 3) 古い表を消し、新しい表の名前を変える
DROP TABLE notes;
ALTER TABLE notes_new RENAME TO notes;
-- 4) 索引を作り直す（古い表と一緒に消えている）
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
COMMIT;
-- 5) 確かめる: 件数が同じで、新しい CHECK が効いている
SELECT COUNT(*) AS notes FROM notes;
UPDATE notes SET body = printf('%10001s', '') WHERE id = 1;
```

`sql/u06_rebuild.sql` として保存し、0001〜0004 の migration と練習用データを用意してから実行します。

```powershell
cargo run -q --bin sql -- --setup migrations/0001_users.sql --setup migrations/0002_notes.sql --setup migrations/0003_sessions.sql --setup migrations/0004_notes_pinned.sql --setup sql/seed.sql sql/u06_rebuild.sql
```

**▼ 実行結果**（準備の5行は省略）

```text
sql> BEGIN
OK

sql> CREATE TABLE notes_new ( …
OK

sql> INSERT INTO notes_new (id, user_id, title, body, done, version, created_at, updated_at, pinned) …
OK（変更: 8 行）

sql> DROP TABLE notes
OK

sql> ALTER TABLE notes_new RENAME TO notes
OK

sql> CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)
OK

sql> COMMIT
OK

sql> SELECT COUNT(*) AS notes FROM notes
+-------+
| notes |
+-------+
| 8     |
+-------+
(1 行)

sql> UPDATE notes SET body = printf('%10001s', '') WHERE id = 1
エラー: SQLite failure: `CHECK constraint failed: length(body) <= 10000`
```

##### 解説6: 作り直しの注意点

- 全部を `BEGIN` 〜 `COMMIT` の中で行うので、途中で失敗しても元の `notes` 表は残ります。8 行を写して、件数も同じです。
- `printf('%10001s', '')` は「10,001 文字の空白」を作る式で、新しい `CHECK` に断られました。
- 古い表を `DROP` すると、**その表の索引も一緒に消えます**。作り直しを忘れると、UNIT 04 で見たように一覧が遅くなります。
- 写す前に、**既にあるデータが新しい制約を守っているか**を確かめてください。10,001 文字以上の本文が1件でもあれば、2) の `INSERT` が `CHECK` 違反で失敗し、transaction 全体を取り消すことになります。

> **ヒント:** 本番の DB に migration を流す前には、必ず backup（バックアップ）を取ります。transaction で守られていても、「SQL は成功したが、意図と違う変更だった」場合は元に戻せないからです。backup の取り方と戻し方は UNIT 10 で学びます。

### よくあるエラーと読み方

#### エラー1・2: 列の追加でよく出るエラー

```sql
-- ① 記録を見ずに 0004 をもう一度流してしまう
ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));
-- ② 行がある表に、DEFAULT の無い NOT NULL の列を足す
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL;
-- ③ 直し方: DEFAULT を付ける（既にある行にはその値が入る）
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL DEFAULT 'none';
SELECT id, title, pinned, color FROM notes WHERE user_id = 2;
```

```powershell
cargo run -q --bin sql -- --setup migrations/0001_users.sql --setup migrations/0002_notes.sql --setup migrations/0003_sessions.sql --setup migrations/0004_notes_pinned.sql --setup sql/seed.sql sql/u06_errors.sql
```

```text
sql> ALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1))
エラー: SQLite failure: `duplicate column name: pinned`

sql> ALTER TABLE notes ADD COLUMN color TEXT NOT NULL
エラー: SQLite failure: `Cannot add a NOT NULL column with default value NULL`

sql> ALTER TABLE notes ADD COLUMN color TEXT NOT NULL DEFAULT 'none'
OK

sql> SELECT id, title, pinned, color FROM notes WHERE user_id = 2
+----+------------+--------+-------+
| id | title      | pinned | color |
+----+------------+--------+-------+
| 8  | ジムの予定 | 0      | none  |
| 5  | 買い物     | 0      | none  |
| 3  | 旅行の計画 | 0      | none  |
+----+------------+--------+-------+
(3 行)
```

| メッセージ | 何が起きたか | 直し方 |
|---|---|---|
| `duplicate column name: pinned` | 同じ列をもう一度足そうとした。migration を記録なしで手で流したときによく起きる | 手で流さず、migration の仕組みに任せる。記録（`schema_migrations`）を確かめる |
| `Cannot add a NOT NULL column with default value NULL` | 行がある表に、`DEFAULT` の無い `NOT NULL` の列を足そうとした | `DEFAULT` を付ける。または「NULL を許して追加 → 値を埋める → 作り直しで NOT NULL にする」と段階に分ける |

③の最後の `SELECT` は、`ORDER BY` が無いので 8・5・3 の順に出ました。UNIT 02 で学んだとおり、`ORDER BY` を書かない順番は DB の都合で決まります（ここでは、`user_id` で探すのに使った索引の順です）。

#### エラー3: include_str! のパスの間違い

`include_str!` のパスを1段間違えると、コンパイルの時点でエラーになります。

```rust
// include_str! のパスを1段間違える
const SQL: &str = include_str!("../migrations/0001_users.sql");

fn main() {
    println!("{SQL}");
}
```

```text
error: couldn't read `src\bin\../migrations/0001_users.sql`: 指定されたパスが見つかりません。 (os error 3)
 --> src\bin\u06_err_path.rs:2:19
  |
2 | const SQL: &str = include_str!("../migrations/0001_users.sql");
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: there is a file with the same name in a different directory
  |
2 | const SQL: &str = include_str!("../../migrations/0001_users.sql");
  |                                    +++

error: could not compile `sql-lab` (bin "u06_err_path") due to 1 previous error
```

- **何が起きたか:** `couldn't read` は「読めなかった」。パスは、このファイルがある `src\bin\` から数えるので、`../migrations` は `src\migrations` を指してしまいました。
- **直し方:** `help:` が「別のフォルダに同じ名前のファイルがあります」と、`../../` に直す案を示してくれています。`src\bin\` から2段上がると、プロジェクトのフォルダです。RustNote のように `src/db.rs` から読むなら、1段上がる `../migrations/…` が正解になります。

### RustNote ではここで使う

RustNote の `Rust/rustnote/src/db.rs` の `migrate` 関数は、例6-1 の `migrate` とほとんど同じ形です（`MIGRATIONS` の一覧、`schema_migrations` の表、版ごとの transaction）。違いは、`Db` から接続を作る点だけです。

アプリの起動時（`src/main.rs`）に `migrate` を呼び、新しく適用した版があれば「マイグレーションを適用しました: …」と表示してから、Web サーバーを起動します。だから、新しい migration ファイルを足して公開し直すだけで、本番の DB の形も自動でそろいます。

ただし、実務では次の点に注意が必要です。

- **Turso Cloud では `PRAGMA user_version` が読み取り専用です。** SQLite には、DB の版の番号をしまっておく `user_version` という場所があり、migration の記録に使う道具もあります。Turso Cloud ではこれを書き換えられないので、RustNote のように**記録用の表を自分で持つ**方法が確実です。
- **アプリを複数台で同時に起動すると、migration がぶつかることがあります。** 2台が同時に「0004 はまだだ」と判断して、両方が適用しようとするからです（片方は記録の主キーの重複か、`duplicate column name` で失敗します）。台数を増やすときは、migration を「公開の前に1回だけ実行する手順」に分けるのが定石です。
- **本番に流す前に backup を取ります**（UNIT 10）。

### 練習問題

**問題1:** 例6-2 で失敗した 0005 を直してください。`color` の列に `DEFAULT 'none'` を付けて、行がある `u06.db`（0004 まで適用済み）で `u06_migrate` を実行し、0005 だけが適用されること、既にあったノートの `color` が `none` になることを確かめます。

**問題2:** 同僚が「0005 が本番で失敗したので、0005 のファイルを直して出し直しました。ついでに、もう本番で適用済みの 0002 のファイルにも、コメントを1行足しておきました」と言っています。0005 を直したことは問題ありませんが、0002 を書き換えたことには問題があります。何が問題でしょうか。

**問題3:** 例6-1 のプログラムで、`MIGRATIONS` の中の `"0004_notes_pinned"` という版の名前を、うっかり `"0004_pinned"` に書き換えてしまいました。0004 まで適用済みの DB で実行すると、どうなるでしょうか。

### 解答と解説

**解答1:** `migrations/0005_tags.sql` の最後の行を次のように直します。

```sql
ALTER TABLE notes ADD COLUMN color TEXT NOT NULL DEFAULT 'none';
```

0004 まで適用済みでノートが1冊ある `u06.db` で実行します（例6-2 の手順で作った状態）。

```powershell
cargo run -q --bin u06_migrate
cargo run -q --bin sql -- --db u06.db sql/u06_q1.sql
```

```text
新しく適用したもの: ["0005_tags"]
適用済みの版: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned", "0005_tags"]
```

```text
sql> SELECT id, title, pinned, color FROM notes
+----+--------------+--------+-------+
| id | title        | pinned | color |
+----+--------------+--------+-------+
| 1  | 買い物リスト | 0      | none  |
+----+--------------+--------+-------+
(1 行)
```

0005 は前回の失敗で取り消され、記録も無かったので、直したファイルで最初からやり直されました。0001〜0004 は記録があるので飛ばされています。

**解答2:** 適用済みの migration は、記録があるので**二度と実行されません**。0002 のファイルを書き換えても、本番の DB には反映されず、「ファイルの内容」と「本番の DB の実際の形」が食い違います。新しく開発を始めた人の DB（0002 の新しい内容で作られる）と本番の DB で形が違う、という気づきにくい事故の元になります。コメントだけなら害は無いように見えますが、「適用済みのファイルは書き換えない。変えたいときは新しい番号のファイルを足す」という決まりを例外なく守ることが大切です。0005 は失敗して取り消され、どの DB にも適用されていないので、直してかまいません。

**解答3:** 記録にある `"0004_notes_pinned"` と名前が違うので、`"0004_pinned"` は「まだ適用していない」と判断され、0004 の SQL がもう一度流れます。`pinned` の列はすでにあるので、エラーになります。

```text
エラー: マイグレーション 0004_pinned に失敗しました
  原因: SQLite failure: `duplicate column name: pinned`
適用済みの版: ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned"]
```

版の名前は、記録と照らし合わせるための**鍵**です。一度公開したら、ファイルの中身と同じく、名前も変えてはいけません。

### まとめ

- migration は、スキーマの変更を番号付きのファイルとして管理し、`schema_migrations` の記録と比べて、まだの分だけを適用する仕組みです。SQL と記録は同じ transaction で適用します。
- 行がある表に `NOT NULL` の列を足すときは `DEFAULT` が必要です。空の DB で成功しても本番で失敗することがあるので、データ入りの DB でも試します。`ALTER TABLE` でできない変更は、transaction の中で表を作り直します。
- 適用済みの migration のファイルと名前は書き換えません。変えたいときは、新しい番号のファイルを足します。
