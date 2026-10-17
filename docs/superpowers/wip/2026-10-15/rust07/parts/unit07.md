## UNIT 07: libsql Rust SDK の接続とパラメーターの bind

### この単元のゴール

- `Builder` → `Database` → `Connection` の順に DB を開き、`execute`（行を返さない文）と `query`（行を返す文）を使い分けられる
- `rows.next().await?` で行を1つずつ読み、`row.get::<型>(列の番号)` で Rust の値に変換できる。`NULL` は `Option` で受け取れる
- 値を `?1`・`params!`・タプル・配列・名前付きで **bind（バインド）** でき、文字列連結が SQL インジェクションを生む理由を、実際に攻撃を再現して説明できる
- `libsql::Error` の種類（`SqliteFailure`・`NullValue`・`InvalidColumnType` など）と、エラー番号の読み分けができる

### たとえ話でつかむ

役所の窓口で手続きをする場面を考えます。

- **Database（建物）:** 役所の建物そのもの。どこにあるか（手元のファイルか、ネットの向こうか）を決めて、建物を開けます。
- **Connection（窓口）:** 建物の中の窓口。手続きは窓口を通して行います。窓口は何か所も開けられます（UNIT 05 の `a` と `b`）。
- **SQL（申込用紙）:** 「○○の写しをください」という決まった書式の用紙。
- **bind（記入欄）:** 用紙の「氏名」「住所」の**記入欄**に値を書くこと。

ここで、記入欄の使い方がとても大事です。正しい用紙では、氏名の欄に何を書いても、それは「氏名」として扱われます。ところが、用紙を作るときに**氏名を本文に書き込んで**しまうと、「山田太郎。なお、全員分の写しも出すこと」と書かれたとき、係の人がそれを**指示の一部として読んでしまう**かもしれません。これが SQL インジェクションです。値は必ず記入欄（`?1` など）に入れ、SQL の文そのものには混ぜない、というのがこの単元の一番大事な約束です。

> **用語:** bind（バインド：SQL の文の中の `?1` などの場所取り（プレースホルダー）に、値を別に渡して当てはめること。値は SQL の文法として解釈されない）

<!-- -->

> **用語:** SQL インジェクション（SQL injection：入力した文字列が SQL の文の一部として解釈され、作った人の意図と違う SQL が実行されてしまう脆弱性。データの盗み見や全削除につながる）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-071" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="140" height="56" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="44" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Builder</text>
<text x="80" y="64" font-size="11" fill="#475569" text-anchor="middle">new_local("u07.db")</text>
<rect x="190" y="20" width="140" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="260" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Database</text>
<text x="260" y="64" font-size="11" fill="#475569" text-anchor="middle">.build().await?</text>
<rect x="370" y="20" width="150" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="445" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Connection</text>
<text x="445" y="64" font-size="11" fill="#475569" text-anchor="middle">db.connect()?</text>
<line x1="150" y1="48" x2="186" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<line x1="330" y1="48" x2="366" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<rect x="250" y="120" width="210" height="56" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="355" y="144" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">conn.execute(SQL, 値)</text>
<text x="355" y="164" font-size="11" fill="#475569" text-anchor="middle">INSERT・UPDATE・DELETE など</text>
<rect x="500" y="120" width="210" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="144" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">conn.query(SQL, 値)</text>
<text x="605" y="164" font-size="11" fill="#475569" text-anchor="middle">SELECT・RETURNING つきの文</text>
<path d="M445,76 V100 H355 V116" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<path d="M445,100 H605 V116" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<rect x="250" y="200" width="210" height="36" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="355" y="223" font-size="12" fill="#0f172a" text-anchor="middle">u64（変更した行数）</text>
<rect x="500" y="200" width="210" height="36" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="605" y="223" font-size="12" fill="#0f172a" text-anchor="middle">Rows → next() → Row → get::&lt;T&gt;(i)</text>
<line x1="355" y1="176" x2="355" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<line x1="605" y1="176" x2="605" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-071)"/>
<text x="20" y="140" font-size="11" fill="#475569">.await? が付くもの:</text>
<text x="20" y="158" font-size="11" fill="#475569">build・execute・query・next</text>
<text x="20" y="182" font-size="11" fill="#475569">? だけのもの:</text>
<text x="20" y="200" font-size="11" fill="#475569">connect・get</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: libsql の基本の流れ。Builder で Database を作り、Connection を開いて、行を返さない文は execute、行を返す文は query で実行する</div>
</div>

### まず動かす最小コード

#### 例7-1: ファイルの DB に表を作り、1行追加して読み出す

> **▼ このコードがやること（先に日本語で）:** ファイル `u07.db` の DB を開き（無ければ作られます）、接続を作ります。メモの表が無ければ作り、「牛乳を買う」を1行追加して、追加した行数と付いた番号を表示します。最後に、全部の行を番号の順に読み出して表示します。

```rust
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // ① DB を開く（ファイルが無ければ作られる）
    let db = Builder::new_local("u07.db").build().await?;
    // ② 接続（DB と話すための通り道）を作る
    let conn = db.connect()?;

    // ③ 表を作る（2回目に実行してもエラーにならないよう IF NOT EXISTS を付ける）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // ④ ?1 の場所に値を入れて INSERT する
    let changed = conn
        .execute("INSERT INTO memos (text) VALUES (?1)", ["牛乳を買う"])
        .await?;
    println!(
        "追加した行数: {changed} / 新しい id: {}",
        conn.last_insert_rowid()
    );

    // ⑤ 全部の行を読み出す
    let mut rows = conn
        .query("SELECT id, text FROM memos ORDER BY id", ())
        .await?;
    while let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let text: String = row.get(1)?;
        println!("{id}: {text}");
    }
    Ok(())
}
```

`src/bin/u07_first.rs` として保存し、**2回**実行します。

```powershell
cargo run -q --bin u07_first
cargo run -q --bin u07_first
```

**▼ 実行結果**（1回目）

```text
追加した行数: 1 / 新しい id: 1
1: 牛乳を買う
```

**▼ 実行結果**（2回目）

```text
追加した行数: 1 / 新しい id: 2
1: 牛乳を買う
2: 牛乳を買う
```

`u07.db` はファイルなので、プログラムが終わってもデータが残ります。2回目は、1回目の行に続けて id 2 の行が足されました。最初からやり直したいときは、`u07.db` を消してください（`Remove-Item u07.db`）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: Builder → Database → Connection

- `Builder::new_local("u07.db")` は「手元のファイル `u07.db` を DB として使う」設定を作ります。`":memory:"` を渡すと、ファイルを作らずメモリの中だけの DB になります（道具 `sql` の既定です）。リモートやレプリカの Builder は UNIT 08 で学びます。
- `.build().await?` で `Database`（DB そのもの）ができます。ファイルを開く準備に時間がかかることがあるので `.await` が付きます。
- `db.connect()?` で `Connection`（接続）を作ります。SQL は、すべてこの接続を通して実行します。

##### 解説2: execute は「行を返さない文」用

- `conn.execute(SQL, 値)` は、`CREATE TABLE`・`INSERT`・`UPDATE`・`DELETE` のように**行を返さない文**を実行し、**変更した行数（`u64`）**を返します。UNIT 03 の「影響行数」です。
- 渡す値が無いときは `()`（空のタプル）を渡します。
- `conn.last_insert_rowid()` は、この接続で最後に `INSERT` した行の id を返します。RustNote では、代わりに `RETURNING` で行ごと受け取っています（UNIT 03）。

##### 解説3: query と rows.next()

- `conn.query(SQL, 値)` は、`SELECT` のように**行を返す文**を実行し、`Rows`（行の並び）を返します。
- `rows.next().await?` は、次の1行を `Option<Row>` で返します。行があれば `Some(row)`、もう無ければ `None` です。`while let Some(row) = rows.next().await? { … }` で、全部の行を順に処理できます。
- 行は、DB から**少しずつ**取り出されます。だから `next()` にも `.await` が付きます。

##### 解説4: row.get で Rust の型に変換する

- `row.get(0)?` は、0 番目（いちばん左）の列の値を取り出します。列の番号は 0 から数えます。
- 受け取る型は、変数の型（`let id: i64 = …`）から決まります。`row.get::<i64>(0)?` のように、ターボフィッシュで書くこともできます。
- 型が合わなかったり、`NULL` だったりすると `Err` が返ります（「よくあるエラー」で見ます）。

### もう一歩: 実務に近い形

#### 例7-2: 値の渡し方と、値の受け取り方

> **▼ このコードがやること（先に日本語で）:** 練習用データを入れたメモリの DB で、値の渡し方を4通り（`params!` マクロ・タプル・配列・名前付き）試します。続けて、型の分からない値を `Value` として受け取って種類を見分ける方法と、`NULL` を `Option` で受け取る方法を確かめます。

```rust
use libsql::{Builder, Value};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // ① params! マクロ: 型の違う値を並べられる（いちばんよく使う）
    conn.execute(
        "INSERT INTO notes (user_id, title, body) VALUES (?1, ?2, ?3)",
        libsql::params![1, "映画を見る", "土曜の夜"],
    )
    .await?;

    // ② タプル: 値が少ないときに手軽
    conn.execute("UPDATE notes SET done = ?1 WHERE id = ?2", (1, 9))
        .await?;

    // ③ 配列: 同じ型の値だけなら [..] でもよい
    let mut rows = conn
        .query("SELECT id, title, done FROM notes WHERE id = ?1", [9])
        .await?;
    if let Some(row) = rows.next().await? {
        let id = row.get::<i64>(0)?;
        let title = row.get::<String>(1)?;
        // done は 0 / 1 の整数。bool として取り出すこともできる
        let done = row.get::<bool>(2)?;
        println!("③ id = {id}, title = {title}, done = {done}");
    }

    // ④ 名前付き: :owner のように名前で値を渡す（値が多いときに取り違えにくい）
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = :owner AND done = :done",
            libsql::named_params! { ":owner": 1, ":done": 0 },
        )
        .await?;
    if let Some(row) = rows.next().await? {
        println!("④ Alice の未完了: {} 件", row.get::<i64>(0)?);
    }

    // ⑤ 型が決まっていない値は Value として取り出し、match で見分ける
    let mut rows = conn.query("SELECT 42, 'テキスト', NULL, 1.5", ()).await?;
    if let Some(row) = rows.next().await? {
        for i in 0..row.column_count() {
            match row.get_value(i)? {
                Value::Integer(n) => println!("⑤ 列{i}: 整数 {n}"),
                Value::Text(s) => println!("⑤ 列{i}: 文字列 {s}"),
                Value::Null => println!("⑤ 列{i}: NULL"),
                Value::Real(x) => println!("⑤ 列{i}: 小数 {x}"),
                Value::Blob(b) => println!("⑤ 列{i}: バイト列 {} バイト", b.len()),
            }
        }
    }

    // ⑥ NULL かもしれない値は Option で受け取る
    let mut rows = conn.query("SELECT NULL", ()).await?;
    if let Some(row) = rows.next().await? {
        let maybe: Option<String> = row.get(0)?;
        println!("⑥ Option で受け取ると: {maybe:?}");
    }
    Ok(())
}
```

```powershell
cargo run -q --bin u07_params
```

**▼ 実行結果**

```text
③ id = 9, title = 映画を見る, done = true
④ Alice の未完了: 4 件
⑤ 列0: 整数 42
⑤ 列1: 文字列 テキスト
⑤ 列2: NULL
⑤ 列3: 小数 1.5
⑥ Option で受け取ると: None
```

##### 解説5: 値の渡し方は4通り

| 書き方 | 例 | 向いている場面 |
|---|---|---|
| `libsql::params![…]` | `params![1, "映画を見る", "土曜の夜"]` | 型の違う値を並べるとき（いちばんよく使う） |
| タプル | `(1, 9)` | 値が少ないとき |
| 配列 | `[9]`・`["牛乳を買う"]` | 値が全部同じ型のとき |
| `libsql::named_params!{…}` | `{ ":owner": 1, ":done": 0 }` | 値が多く、番号だと取り違えそうなとき |

- `?1`・`?2` の数字は、渡した値の何番目かを表します（1 から数えます）。同じ値を2回使うときは、`?2` を2か所に書けます（RustNote の検索が `title LIKE ?2 … OR body LIKE ?2` と書いているのは、これです）。
- 名前付きでは、SQL に `:owner` と書き、同じ名前で値を渡します。

##### 解説6: 受け取れる型と Value

- `row.get` で受け取れる主な型は、`i64`・`i32`・`f64`・`String`・`bool`・`Vec<u8>`・`Option<T>`・`Value` です。UNIT 01 のストレージクラス（integer・real・text・blob・null）との対応を思い出してください。
- `bool` で受け取ると、0 は `false`、1 は `true` になります。
- `Value` は「どの種類か分からない値」を、そのまま enum で受け取る型です。`match` で種類ごとに処理できます。巻頭の道具 `sql` が、どんな SELECT の結果でも表にできたのは、`row.get_value(i)` で `Value` として受け取っていたからです。
- `NULL` かもしれない列は、`Option<String>` のように `Option` で受け取ります。`NULL` なら `None` になります。

#### 例7-3: SQL インジェクションを再現し、bind で防ぐ

> **▼ このコードがやること（先に日本語で）:** 入力をそのまま SQL の文字列に混ぜる危険な関数と、`?1` に bind する安全な関数を用意します。攻撃者が入力欄に打ち込みそうな文字列で、会員を探す・ノートを消す、の2つを試し、危険な版では全員分が見えたり全部のノートが消えたりすること、安全な版では何も起きないことを確かめます。

```rust
use libsql::{Builder, Connection};

// 危険: 入力をそのまま SQL の文字列に混ぜて、会員を探す
async fn find_unsafe(conn: &Connection, email: &str) -> Result<Vec<String>, libsql::Error> {
    let sql = format!("SELECT email FROM users WHERE email = '{email}'");
    println!("   実行した SQL: {sql}");
    let mut rows = conn.query(&sql, ()).await?;
    let mut found = Vec::new();
    while let Some(row) = rows.next().await? {
        found.push(row.get::<String>(0)?);
    }
    Ok(found)
}

// 安全: 入力は ?1 に「値」として渡す（SQL の文そのものは変わらない）
async fn find_safe(conn: &Connection, email: &str) -> Result<Vec<String>, libsql::Error> {
    let mut rows = conn
        .query("SELECT email FROM users WHERE email = ?1", [email])
        .await?;
    let mut found = Vec::new();
    while let Some(row) = rows.next().await? {
        found.push(row.get::<String>(0)?);
    }
    Ok(found)
}

// 危険: URL から来た id の文字列を、そのまま DELETE 文に混ぜる
async fn delete_unsafe(conn: &Connection, owner: i64, id_text: &str) -> Result<u64, libsql::Error> {
    let sql = format!("DELETE FROM notes WHERE id = {id_text} AND user_id = {owner}");
    println!("   実行した SQL: {sql}");
    conn.execute(&sql, ()).await
}

// 安全: まず数に変換し（できなければ 404 にする）、値として渡す
async fn delete_safe(conn: &Connection, owner: i64, id_text: &str) -> Result<u64, String> {
    let id: i64 = id_text
        .parse()
        .map_err(|_| format!("id が数ではありません: {id_text:?}（404 にする）"))?;
    conn.execute(
        "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
        libsql::params![id, owner],
    )
    .await
    .map_err(|e| e.to_string())
}

async fn count_notes(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 攻撃者が入力欄に打ち込む文字列
    let evil_email = "x' OR '1'='1";
    println!("① 文字列を混ぜる版で {evil_email:?} を探す");
    println!("   見つかった: {:?}", find_unsafe(&conn, evil_email).await?);
    println!("② bind する版で同じ文字列を探す");
    println!("   見つかった: {:?}", find_safe(&conn, evil_email).await?);

    // Alice（1）が URL の id を書き換えて送ってくる
    let evil_id = "0 OR 1=1 --";
    println!(
        "③ 文字列を混ぜる版で id = {evil_id:?} を削除（ノートは {} 件）",
        count_notes(&conn).await?
    );
    let deleted = delete_unsafe(&conn, 1, evil_id).await?;
    println!(
        "   消えた行: {deleted}（残り {} 件）",
        count_notes(&conn).await?
    );

    // 消えてしまったので、会員ごと消してからデータを入れ直す
    conn.execute("DELETE FROM users", ()).await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    println!(
        "④ bind する版で同じ id を削除（ノートは {} 件）",
        count_notes(&conn).await?
    );
    println!("   結果: {:?}", delete_safe(&conn, 1, evil_id).await);
    println!(
        "   Bob のノート 3 を指定: {:?}",
        delete_safe(&conn, 1, "3").await
    );
    println!("   残り {} 件", count_notes(&conn).await?);
    Ok(())
}
```

```powershell
cargo run -q --bin u07_injection
```

**▼ 実行結果**

```text
① 文字列を混ぜる版で "x' OR '1'='1" を探す
   実行した SQL: SELECT email FROM users WHERE email = 'x' OR '1'='1'
   見つかった: ["alice@example.com", "bob@example.com", "carol@example.com"]
② bind する版で同じ文字列を探す
   見つかった: []
③ 文字列を混ぜる版で id = "0 OR 1=1 --" を削除（ノートは 8 件）
   実行した SQL: DELETE FROM notes WHERE id = 0 OR 1=1 -- AND user_id = 1
   消えた行: 8（残り 0 件）
④ bind する版で同じ id を削除（ノートは 8 件）
   結果: Err("id が数ではありません: \"0 OR 1=1 --\"（404 にする）")
   Bob のノート 3 を指定: Ok(0)
   残り 8 件
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-072" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="20" font-size="12" fill="#0f172a" text-anchor="middle">入力: x' OR '1'='1</text>
<rect x="10" y="34" width="340" height="200" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="180" y="58" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">文字列連結（format!）</text>
<rect x="24" y="72" width="312" height="44" rx="6" fill="#ffffff" stroke="#fca5a5"/>
<text x="180" y="99" font-size="11" fill="#0f172a" text-anchor="middle">WHERE email = 'x' OR '1'='1'</text>
<text x="180" y="138" font-size="11" fill="#475569" text-anchor="middle">入力の ' が文字列を閉じてしまい、</text>
<text x="180" y="156" font-size="11" fill="#475569" text-anchor="middle">OR '1'='1' が SQL の文法として読まれる</text>
<line x1="180" y1="166" x2="180" y2="186" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-072)"/>
<text x="180" y="210" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">条件がいつも真 → 全員分が返る</text>
<rect x="370" y="34" width="340" height="200" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="58" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">bind（?1 に値として渡す）</text>
<rect x="384" y="72" width="150" height="44" rx="6" fill="#ffffff" stroke="#6ee7b7"/>
<text x="459" y="91" font-size="11" fill="#475569" text-anchor="middle">SQL の文</text>
<text x="459" y="107" font-size="11" fill="#0f172a" text-anchor="middle">WHERE email = ?1</text>
<rect x="546" y="72" width="150" height="44" rx="6" fill="#ffffff" stroke="#6ee7b7"/>
<text x="621" y="91" font-size="11" fill="#475569" text-anchor="middle">値（別に届く）</text>
<text x="621" y="107" font-size="11" fill="#0f172a" text-anchor="middle">x' OR '1'='1</text>
<text x="540" y="138" font-size="11" fill="#475569" text-anchor="middle">文の形は先に決まっている。値は最後まで</text>
<text x="540" y="156" font-size="11" fill="#475569" text-anchor="middle">「ただの文字列」で、文法として読まれない</text>
<line x1="540" y1="166" x2="540" y2="186" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-072)"/>
<text x="540" y="210" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">そういうメールの人はいない → []</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 文字列連結では、入力の中の記号が SQL の文法になってしまう。bind では、SQL の文と値が別々に DB へ届くので、値は文法として読まれない</div>
</div>

##### 解説7: 何が起きたのか

- ①では、入力 `x' OR '1'='1` の最初の `'` が、SQL の文字列 `'x` を**閉じて**しまいました。残りの `OR '1'='1'` は SQL の文法として読まれ、条件が「email が x、**または** 1 が 1 に等しい」になります。後者はいつも真なので、**全員のメールアドレス**が返りました。ログイン画面でこれが起きれば、パスワードを知らなくても入れてしまうかもしれません。
- ③はもっと危険です。`0 OR 1=1 --` を混ぜた結果、`--` 以降（`AND user_id = 1`）が**コメントとして無視**され、条件が「id が 0 または 1=1」＝全部の行になりました。Alice の操作で、Bob の分も含めて**8件全部のノートが消えました**。
- ②と④の bind する版では、何も起きていません。`?1` に渡した値は、最後まで「1つの文字列」や「1つの数」として扱われ、SQL の文法の一部にはならないからです。さらに④は、URL から来た id を `parse()` で数に変換し、できなければ 404 にしています。**型に変換できた値だけを使う**のは、第4巻で学んだ「境界で検証する」考え方そのものです。

> **ヒント:** 「`'` を `''` に置き換えればよい」のように、自分で危ない記号を取り除く方法もありますが、漏れが出やすく、おすすめしません。値は**いつも bind** する、と決めておくのが一番確実です。表の名前や列の名前のように bind できない部分を組み立てるときは、プログラムの中で決めた**定数だけ**を使い、入力を混ぜないでください。

### よくあるエラーと読み方

#### エラー1: E0277 — 受け取る型が決まらない

```rust
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let mut rows = conn.query("SELECT 42", ()).await?;
    if let Some(row) = rows.next().await? {
        let n = row.get(0)?;
        println!("{n}");
    }
    Ok(())
}
```

```text
error[E0277]: the trait bound `!: libsql::rows::FromValue` is not satisfied
   --> src\bin\u07_ce_infer.rs:9:21
    |
  9 |         let n = row.get(0)?;
    |                     ^^^ the trait `libsql::rows::FromValue` is not implemented for `!`
    |
    = help: the following other types implement trait `libsql::rows::FromValue`:
              Option<T>
              String
              Value
              [u8; N]
              bool
              f64
              i32
              i64
            and 3 others
note: required by a bound in `Row::get`
   --> C:\Users\...\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\libsql-0.10.0\src\rows.rs:116:12
    |
114 |     pub fn get<T>(&self, idx: i32) -> Result<T>
    |            --- required by a bound in this associated function
115 |     where
116 |         T: FromValue,
    |            ^^^^^^^^^ required by this bound in `Row::get`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `sql-lab` (bin "u07_ce_infer") due to 1 previous error
```

- **何が起きたか:** `let n = row.get(0)?;` には型が書いてありません。`println!` で表示するだけでは型が1つに決まらないので、コンパイラは「どの型で受け取るか」を決められず、`!` という特別な型（ここでは「決まらなかった」印と思ってください）で試して失敗しました。
- **読み方:** `help:` の一覧が「`FromValue` を実装している型」、つまり **`get` で受け取れる型の一覧**です。
- **直し方:** `let n: i64 = row.get(0)?;` か `row.get::<i64>(0)?` と、型を書きます。

#### エラー2: E0308 — 配列に違う型の値を並べた

```rust
let user_text = String::from("牛乳を買う");
conn.execute("INSERT INTO memos (id, text) VALUES (?1, ?2)", [1, user_text])
    .await?;
```

```text
error[E0308]: mismatched types
  --> src\bin\u07_ce_params.rs:10:70
   |
10 |     conn.execute("INSERT INTO memos (id, text) VALUES (?1, ?2)", [1, user_text])
   |                                                                      ^^^^^^^^^ expected integer, found `String`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `sql-lab` (bin "u07_ce_params") due to 1 previous error
```

- **何が起きたか:** Rust の配列は、全部の要素が同じ型でなければなりません。最初の `1` で「整数の配列」と決まったので、`String` は入れられません。
- **直し方:** `libsql::params![1, user_text]` か、タプル `(1, user_text)` にします。

#### エラー3: 実行時の Err — NULL・列の番号・型・execute の使い間違い

> **▼ このコードがやること（先に日本語で）:** ノート 1 の行を `SELECT id, title, NULL, 2` で読み、`NULL` を `String` で、無い列（4 番）を、`2` を `bool` で受け取ってみます。さらに、行を返す `SELECT` を `execute` で実行し、最後に「行が無いとき」の `next()` の結果を確かめます。

```rust
let mut rows = conn
    .query("SELECT id, title, NULL, 2 FROM notes WHERE id = 1", ())
    .await?;
let row = rows.next().await?.expect("ノート 1 はある");

// ① NULL を String で受け取る
println!("① NULL → String: {:?}", row.get::<String>(2));
// ② 列の番号の数え間違い（列は 0〜3 の 4 つしかない）
println!("② 存在しない列 4: {:?}", row.get::<String>(4));
// ③ 0 と 1 以外の整数を bool で受け取る
println!("③ 2 → bool: {:?}", row.get::<bool>(3));
// ④ 行を返す SELECT を execute で実行する
let result = conn.execute("SELECT id FROM notes", ()).await;
println!("④ execute で SELECT: {result:?}");
// ⑤ 1行も無いのに next() の結果を unwrap せずに確かめる
let mut rows = conn
    .query("SELECT id FROM notes WHERE id = 999", ())
    .await?;
println!("⑤ 999 番の行: {:?}", rows.next().await?.is_some());
```

（`src/bin/u07_err_get.rs` の `main` の後半です。前半は例7-2 と同じく、メモリの DB に `schema.sql` と `seed.sql` を流しています。）

```text
① NULL → String: Err(NullValue)
② 存在しない列 4: Err(NullValue)
③ 2 → bool: Err(InvalidColumnType)
④ execute で SELECT: Err(ExecuteReturnedRows)
⑤ 999 番の行: false
```

| エラー | 何が起きたか | 直し方 |
|---|---|---|
| `NullValue` | `NULL` を `Option` でない型で受け取った | `Option<String>` で受け取る |
| `NullValue`（②） | 存在しない列の番号を指定した。**列の数え間違いでも同じエラー**になるので注意 | 列は 0 から数える。`SELECT` の列の順番と照らし合わせる |
| `InvalidColumnType` | 0 と 1 以外の整数を `bool` で受け取った | `i64` で受け取って自分で判断する |
| `ExecuteReturnedRows` | 行を返す文を `execute` で実行した | `query` を使う。`RETURNING` 付きの文も `query` |

⑤のように、1行も無いとき `next()` はエラーではなく `None` を返します。`expect` で決めつけず、`match` や `if let` で「無かった場合」を処理しましょう（RustNote では、無ければ 404 にします）。

#### エラー4: 型の違う値を get すると panic する

> **▼ このコードがやること（先に日本語で）:** 列の順番を取り違えて、`title`（文字列）を `i64` で受け取ってしまう場面を再現します。

```rust
// 列の順番を取り違えて、title（文字列）を i64 で取り出してしまう
let mut rows = conn
    .query("SELECT title, id FROM notes WHERE id = 1", ())
    .await?;
let row = rows.next().await?.expect("ノート 1 はある");
let id = row.get::<i64>(0)?;
println!("id = {id}");
```

```text

thread 'main' (26016) panicked at C:\Users\...\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\libsql-0.10.0\src\rows.rs:196:18:
internal error: entered unreachable code: invalid value type
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 文字列の値を `i64` で受け取ろうとしたところ、`Err` ではなく **panic**（プログラムの異常終了）になりました。`entered unreachable code`（来るはずのない場所に来た）は、libsql 0.10 の中で「この組み合わせは起きないはず」と考えられていた場所です。`Option<i64>` で受け取っても同じ panic になります（練習問題3）。
- **直し方:** `SELECT` の列の順番と、`get` の番号・型を必ずそろえます。RustNote では、列の並びを定数 `NOTE_COLUMNS` に1か所だけ書き、それを読む関数 `row_to_note` も1つにまとめて、食い違いが起きにくくしています（UNIT 09）。
- Web アプリの中でこれが起きると、そのリクエストには正しい返事を返せません。テストで必ず一度は全部の列を読むようにしておくと、こうした取り違えを公開前に見つけられます。

#### エラー5: bind する値の数が合わなくても、エラーにならない

```rust
// ① ?1 と ?2 があるのに、値を1つしか渡さない
let result = conn
    .execute(
        "UPDATE notes SET title = ?1 WHERE id = ?2",
        ["新しいタイトル"],
    )
    .await;
println!("① UPDATE（値が1つ足りない）: {result:?}");

// ② INSERT で値が足りない
let result = conn
    .execute("INSERT INTO notes (user_id, title) VALUES (?1, ?2)", [1])
    .await;
println!("② INSERT（値が1つ足りない）: {result:?}");

// ③ 値が1つ多い（3つ目の値はどこにも使われない）
let result = conn
    .execute(
        "UPDATE notes SET title = ?1 WHERE id = ?2",
        libsql::params!["新しいタイトル", 1, 2],
    )
    .await;
println!("③ UPDATE（値が1つ多い）: {result:?}");

// ④ 対策: prepare して、? の数と渡す値の数を確かめてから実行する
let stmt = conn
    .prepare("UPDATE notes SET title = ?1 WHERE id = ?2")
    .await?;
println!("④ この文の ? の数: {}", stmt.parameter_count());
```

（`src/bin/u07_err_bind.rs`。前半は例7-2 と同じ準備です。）

```text
① UPDATE（値が1つ足りない）: Ok(0)
② INSERT（値が1つ足りない）: Err(SqliteFailure(1299, "NOT NULL constraint failed: notes.title"))
③ UPDATE（値が1つ多い）: Ok(1)
④ この文の ? の数: 2
```

- ①は、足りない `?2` が **`NULL` として扱われ**、`WHERE id = NULL`（UNIT 02 で見たとおり、いつも NULL）になって 0 行変更で終わりました。エラーになりません。
- ②は、足りない `?2` が `NULL` になり、たまたま `NOT NULL` 制約のおかげでエラーになりました。
- ③は、3つ目の値が**黙って無視され**、`Ok(1)` で成功しました。
- どれも、プログラムの書き間違いなのに、気づきにくい結果です。値が多い SQL では、`named_params!` で名前を付けるか、テストで「変更した行数」まで確かめるようにします。`prepare` した文の `parameter_count()` で、`?` の数を確かめることもできます。

#### エラー6: エラー番号で、制約違反の種類を見分ける

`libsql::Error` を `{:?}` で表示すると、SQLite のエラー番号が見えます。

```rust
// わざと失敗する文を並べる
let bad_statements = [
    "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
    "INSERT INTO users (id, email, password_hash) VALUES (1, 'new@example.com', 'x')",
    "INSERT INTO notes (user_id, title) VALUES (1, NULL)",
    "INSERT INTO notes (user_id, title) VALUES (1, '')",
    "INSERT INTO notes (user_id, title) VALUES (99, '持ち主がいない')",
    "SELEC id FROM notes",
];
for sql in bad_statements {
    let error = conn.execute(sql, ()).await.unwrap_err();
    // {:?} は種類と番号、{} は人が読むための文
    println!("{error:?}");
    println!("  → {error}");
    // 種類と番号で見分ける
    let kind = match &error {
        libsql::Error::SqliteFailure(2067, _) => "UNIQUE 違反（409 にする候補）",
        libsql::Error::SqliteFailure(1555, _) => "主キーの重複",
        libsql::Error::SqliteFailure(1299, _) => "NOT NULL 違反",
        libsql::Error::SqliteFailure(275, _) => "CHECK 違反",
        libsql::Error::SqliteFailure(787, _) => "外部キー違反",
        libsql::Error::SqliteFailure(1, _) => "SQL の書き間違いなど",
        _ => "その他",
    };
    println!("  → 分類: {kind}");
}
```

（`src/bin/u07_err_codes.rs`。前半は例7-2 と同じ準備です。）

```text
SqliteFailure(2067, "UNIQUE constraint failed: users.email")
  → SQLite failure: `UNIQUE constraint failed: users.email`
  → 分類: UNIQUE 違反（409 にする候補）
SqliteFailure(1555, "UNIQUE constraint failed: users.id")
  → SQLite failure: `UNIQUE constraint failed: users.id`
  → 分類: 主キーの重複
SqliteFailure(1299, "NOT NULL constraint failed: notes.title")
  → SQLite failure: `NOT NULL constraint failed: notes.title`
  → 分類: NOT NULL 違反
SqliteFailure(275, "CHECK constraint failed: length(title) BETWEEN 1 AND 80")
  → SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`
  → 分類: CHECK 違反
SqliteFailure(787, "FOREIGN KEY constraint failed")
  → SQLite failure: `FOREIGN KEY constraint failed`
  → 分類: 外部キー違反
SqliteFailure(1, "near \"SELEC\": syntax error")
  → SQLite failure: `near "SELEC": syntax error`
  → 分類: SQL の書き間違いなど
```

- `{error:?}`（Debug）は `SqliteFailure(番号, 文)` と種類と番号を、`{error}`（Display）は `SQLite failure: …` と人が読む文を表示します。道具 `sql` が表示していたのは、Display のほうでした。
- 番号は SQLite が決めた「拡張エラーコード」です。同じ `UNIQUE constraint failed` でも、主キーの重複（1555）と、それ以外の `UNIQUE`（2067）で番号が違います。
- ただし、この番号で確実に見分けられるのは**ローカルの DB のとき**です。Turso のリモートにつないだときは、エラーが `Hrana(…)` という別の種類で返ってくるので（UNIT 08）、`SqliteFailure(番号, …)` の `match` には当てはまりません。そのため RustNote は、番号ではなく、どちらでも同じ文言 `UNIQUE constraint failed` が含まれているかで見分けています。

### RustNote ではここで使う

RustNote の SQL は、すべて bind で値を渡しています（`Rust/rustnote/src/repository/notes.rs`）。たとえばノートの追加は、こうです。

```rust
let conn = self.db.connect().await?;
let sql = format!(
    "INSERT INTO notes (user_id, title, body) VALUES (?1, ?2, ?3) RETURNING {NOTE_COLUMNS}"
);
let mut rows = conn
    .query(
        &sql,
        libsql::params![owner.get(), note.title.as_str(), note.body.as_str()],
    )
    .await?;
```

- `format!` を使っていますが、混ぜているのは `NOTE_COLUMNS`（`"id, user_id, title, body, done, version, created_at, updated_at"` という**プログラムの中の定数**）だけです。利用者の入力（タイトルと本文）は、`params!` で `?2`・`?3` に bind しています。これが「表や列の名前は定数だけ、値は bind」の実例です。
- `RETURNING` 付きなので、`execute` ではなく `query` で実行し、返ってきた1行を `row_to_note` で `Note` に変換します。`done` は `row.get::<i64>(4)? != 0` と、整数で受け取ってから `bool` にしています。
- 接続（`Connection`）は、`self.db.connect()` で**操作のたびに作って**います。1つの接続を、同時に来た複数のリクエストで取り合わないようにするためです。

エラーの変換（`src/error.rs`）は、エラー6 の最後で見たとおり、文言で `UNIQUE` 違反だけを見分けています。

```rust
// libsql のエラーを RepoError に変換する。
// 「UNIQUE constraint failed」はローカル（SQLite）でも Turso でも同じ文言なので、文言で見分ける。
impl From<libsql::Error> for RepoError {
    fn from(error: libsql::Error) -> Self {
        let message = error.to_string();
        if message.contains("UNIQUE constraint failed") {
            RepoError::UniqueViolation(message)
        } else {
            RepoError::Database(error)
        }
    }
}
```

`UniqueViolation` は、会員登録で「そのメールアドレスは登録済みです」（409）を出すのに使います。それ以外の DB のエラーは `Database` にまとめ、利用者には 500 として返し、詳しい原因はログにだけ書きます。

### 練習問題

**問題1:** 会員の番号 `owner` を受け取り、その会員の**完了済み**のノートの数を返す関数 `async fn count_done(conn: &Connection, owner: i64) -> Result<i64, libsql::Error>` を書いてください。会員 1・2・3 で呼んで結果を表示します。

**問題2:** 次の関数は、ノートのタイトルを部分一致で検索しますが、SQL インジェクションの穴があります。bind を使う形に直し、`"買い物"` と `"' OR 1=1 --"` で検索して結果を確かめてください。

```rust
async fn search_titles(conn: &Connection, owner: i64, word: &str) -> Result<Vec<String>, libsql::Error> {
    let sql = format!("SELECT title FROM notes WHERE user_id = {owner} AND title LIKE '%{word}%' ORDER BY id");
    // （以下、rows を読んで Vec<String> にする処理）
    todo!()
}
```

**問題3:** `SELECT 'メモ', NULL, 1, 'abc'` の1行を読み、次の (a)〜(e) で `get` したとき、結果がどうなるかを予想してください。

- (a) 0 列目を `String` で
- (b) 1 列目を `String` で
- (c) 1 列目を `Option<String>` で
- (d) 2 列目を `bool` で
- (e) 3 列目を `Option<i64>` で

### 解答と解説

**解答1:**

```rust
use libsql::{Builder, Connection};

// 指定した会員の、完了したノートの数を数える
async fn count_done(conn: &Connection, owner: i64) -> Result<i64, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = ?1 AND done = 1",
            [owner],
        )
        .await?;
    // COUNT(*) は、該当が 0 件でも必ず1行を返す
    match rows.next().await? {
        Some(row) => row.get::<i64>(0),
        None => Ok(0),
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    for owner in [1, 2, 3] {
        println!("会員 {owner}: 完了 {} 件", count_done(&conn, owner).await?);
    }
    Ok(())
}
```

```powershell
cargo run -q --bin u07_q1
```

```text
会員 1: 完了 1 件
会員 2: 完了 1 件
会員 3: 完了 0 件
```

`COUNT(*)` は、当てはまる行が 0 件でも「0」という1行を返します。それでも `rows.next()` の結果は `Option` なので、`expect` で決めつけず `match` で受けています。

**解答2:**

```rust
// 直した版: 検索の言葉は ?2 に値として渡す。% は Rust 側で付ける
async fn search_titles(
    conn: &Connection,
    owner: i64,
    word: &str,
) -> Result<Vec<String>, libsql::Error> {
    let pattern = format!("%{word}%");
    let mut rows = conn
        .query(
            "SELECT title FROM notes WHERE user_id = ?1 AND title LIKE ?2 ORDER BY id",
            libsql::params![owner, pattern],
        )
        .await?;
    let mut titles = Vec::new();
    while let Some(row) = rows.next().await? {
        titles.push(row.get::<String>(0)?);
    }
    Ok(titles)
}
```

`main` で `for word in ["買い物", "' OR 1=1 --"]` と回して、会員 1 で検索します（`src/bin/u07_q2.rs`）。

```text
"買い物" → ["買い物リスト"]
"' OR 1=1 --" → []
```

`%` は SQL の中ではなく、Rust 側で値に付けてから bind するのがポイントです。`'%?2%'` のように SQL の文字列の中に `?2` を書くと、それはただの文字「?2」になってしまい、bind されません。さらに、UNIT 02 の RustNote の `escape_like` のように `%` と `_` を `\` で無効にしておけば、検索語に `%` が入っていても正しく探せます。

**解答3:**

```text
(a) Ok("メモ")
(b) Err(NullValue)
(c) Ok(None)
(d) Ok(true)

thread 'main' (25592) panicked at C:\Users\...\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\libsql-0.10.0\src\rows.rs:196:18:
internal error: entered unreachable code: invalid value type
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

（`src/bin/u07_q3.rs` で、(a)〜(e) を順に `println!("{:?}", row.get::<型>(番号))` した結果です。）

- (a)・(d) はそのまま受け取れます。(b) は `NULL` を `Option` でない型で受け取ったので `NullValue`、(c) は `Option` なので `None` です。
- (e) は、`Option` で受け取っても panic しました。`Option` が守ってくれるのは「`NULL` のとき」だけで、「文字列を整数として読む」型の取り違えは防げません。

### まとめ

- `Builder` → `Database` → `Connection` の順に開き、行を返さない文は `execute`（変更した行数）、行を返す文は `query`（`Rows` → `next()` → `Row` → `get`）で実行します。`NULL` は `Option` で受け取ります。
- 値は必ず `?1`・`params!` などで bind し、SQL の文字列に混ぜません。混ぜてよいのは、プログラムの中の定数だけです。
- `NullValue`・`InvalidColumnType`・`ExecuteReturnedRows` は `Err` で返りますが、型の取り違えは panic し、bind の数の食い違いはエラーにすらなりません。列の並びは1か所にまとめ、テストで確かめます。
