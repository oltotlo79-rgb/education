## UNIT 05: transaction・ACID・競合

### この単元のゴール

- transaction（トランザクション）で、複数の SQL を「**全部成功か、全部取り消しか**」にまとめられる（`BEGIN`・`COMMIT`・`ROLLBACK`）
- transaction の中で1つの文が失敗したとき、何が残るかを説明できる
- Rust の libsql で transaction を使い、`commit` しないまま片付けられると自動で取り消されることを確かめられる
- `database is locked` の意味と、`busy_timeout` で待つ方法を知る
- `version` 列を使った**楽観的ロック**で、同時編集の上書き事故を見つけ、409 を返せる

### たとえ話でつかむ

スーパーで買い物をするときのことを考えます。

1. 買い物かごを持つ（**BEGIN**）
2. 棚から商品をかごに入れていく（**INSERT・UPDATE など**）
3. レジで支払うと、全部が自分のものになる（**COMMIT**）
4. 途中で「やっぱりやめた」と思ったら、かごの中身を全部棚に戻す（**ROLLBACK**）

大事なのは、**かごに入れただけでは、まだ自分のものではない**ことです。支払いが済むまでは、いつでも全部を元に戻せます。また、レジで「卵は買うけれど牛乳はやめて、パンの半分だけ」というように、中途半端な状態で店を出ることもありません。

DB の transaction も同じです。「ノートを2冊まとめて保存する」「会員を消して、その人のデータも片付ける」のように、**途中で止まると困る複数の変更**を1つのかごに入れ、最後に全部まとめて確定するか、全部取り消すかを選びます。

> **用語:** transaction（トランザクション：複数の SQL を1つのまとまりとして扱い、全部確定（COMMIT）するか、全部取り消す（ROLLBACK）かのどちらかにする仕組み）

<!-- -->

> **用語:** ACID（アシッド：transaction が守る4つの性質の頭文字。Atomicity＝全部か無しか、Consistency＝制約を守った状態から制約を守った状態へ、Isolation＝ほかの transaction の途中の状態が見えない、Durability＝確定したら電源が落ちても消えない）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-051" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="90" width="110" height="60" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="65" y="115" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ノート 8 件</text>
<text x="65" y="135" font-size="11" fill="#475569" text-anchor="middle">確定した状態</text>
<rect x="160" y="70" width="240" height="100" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2" stroke-dasharray="6 4"/>
<text x="280" y="92" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">BEGIN 〜 （買い物かご）</text>
<text x="280" y="116" font-size="12" fill="#0f172a" text-anchor="middle">INSERT …（1 件追加）</text>
<text x="280" y="138" font-size="12" fill="#0f172a" text-anchor="middle">この中では 9 件に見える</text>
<text x="280" y="158" font-size="11" fill="#475569" text-anchor="middle">ほかの接続からは 8 件のまま</text>
<line x1="120" y1="120" x2="156" y2="120" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-051)"/>
<rect x="470" y="20" width="240" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="46" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">COMMIT（支払う）</text>
<text x="590" y="70" font-size="12" fill="#0f172a" text-anchor="middle">ノート 9 件で確定</text>
<text x="590" y="88" font-size="11" fill="#475569" text-anchor="middle">全員から 9 件に見える</text>
<rect x="470" y="140" width="240" height="80" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="590" y="166" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">ROLLBACK（棚に戻す）</text>
<text x="590" y="190" font-size="12" fill="#0f172a" text-anchor="middle">ノート 8 件に戻る</text>
<text x="590" y="208" font-size="11" fill="#475569" text-anchor="middle">かごの中身は全部なかったことに</text>
<line x1="400" y1="100" x2="466" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-051)"/>
<line x1="400" y1="140" x2="466" y2="178" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-051)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: transaction の中の変更は、COMMIT で確定するか、ROLLBACK で全部取り消すかのどちらか。確定するまでは、ほかの接続からは見えない</div>
</div>

### まず動かす最小コード

#### 例5-1: ROLLBACK と COMMIT

> **▼ このコードがやること（先に日本語で）:** ノートの件数を数えてから transaction を始め、1件追加して数え、`ROLLBACK` で取り消して数えます。次にもう一度 transaction を始め、1件追加して `COMMIT` で確定し、数えます。

```sql
-- ① BEGIN → INSERT → ROLLBACK: 取り消すと、何も残らない
SELECT COUNT(*) AS before FROM notes;
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '取り消されるノート');
SELECT COUNT(*) AS during FROM notes;
ROLLBACK;
SELECT COUNT(*) AS after_rollback FROM notes;
-- ② BEGIN → INSERT → COMMIT: 確定すると残る
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '確定するノート');
COMMIT;
SELECT COUNT(*) AS after_commit FROM notes;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_tx.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> SELECT COUNT(*) AS before FROM notes
+--------+
| before |
+--------+
| 8      |
+--------+
(1 行)

sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, '取り消されるノート')
OK（変更: 1 行）

sql> SELECT COUNT(*) AS during FROM notes
+--------+
| during |
+--------+
| 9      |
+--------+
(1 行)

sql> ROLLBACK
OK

sql> SELECT COUNT(*) AS after_rollback FROM notes
+----------------+
| after_rollback |
+----------------+
| 8              |
+----------------+
(1 行)

sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, '確定するノート')
OK（変更: 1 行）

sql> COMMIT
OK

sql> SELECT COUNT(*) AS after_commit FROM notes
+--------------+
| after_commit |
+--------------+
| 9            |
+--------------+
(1 行)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: BEGIN から COMMIT / ROLLBACK まで

- `BEGIN` で transaction が始まります。ここから `COMMIT` か `ROLLBACK` までの変更が、1つのかごに入ります。
- transaction の中で数えると 9 件でした。**自分のかごの中身は、自分には見えます**。
- `ROLLBACK` の後は 8 件に戻りました。追加した「取り消されるノート」は、最初から無かったことになります。
- `COMMIT` すると、変更が確定して 9 件になります。確定した変更は、`ROLLBACK` では取り消せません。

##### 解説2: transaction を書かないときは「1文ずつ自動で確定」

UNIT 01〜04 では `BEGIN` を書いていませんでした。そのときは、**1つの文ごとに、自動で小さな transaction が作られて確定**しています。これを autocommit（オートコミット）と呼びます。1つの `INSERT` や `UPDATE` は、それだけで「全部成功か、全部失敗か」が守られているので、UNIT 01 のエラーの例では、失敗した文は1行も追加していませんでした。transaction を自分で書くのは、**2つ以上の文**をまとめたいときです。

### もう一歩: 実務に近い形

#### 例5-2: 途中の文が失敗しても、transaction は続いている

> **▼ このコードがやること（先に日本語で）:** 2件のノートをまとめて保存するつもりで transaction を始めます。2件目が CHECK 違反で失敗したのに、うっかり `COMMIT` してしまうと何が残るかを確かめます。

```sql
-- 2件まとめて保存したい。2件目が CHECK 違反になると……
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '1件目');
INSERT INTO notes (user_id, title) VALUES (1, '');
-- うっかり COMMIT すると、1件目だけが確定してしまう
COMMIT;
SELECT id, title FROM notes WHERE id > 8;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_partial.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, '1件目')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (1, '')
エラー: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`

sql> COMMIT
OK

sql> SELECT id, title FROM notes WHERE id > 8
+----+-------+
| id | title |
+----+-------+
| 9  | 1件目 |
+----+-------+
(1 行)
```

##### 解説3: 失敗に気づいて ROLLBACK するのは、自分の仕事

意外かもしれませんが、**transaction の中で1つの文が失敗しても、transaction 全体は取り消されません**。失敗した文（2件目）だけが無かったことになり、transaction は続いています。そのまま `COMMIT` すると、成功した1件目だけが確定し、「2件まとめて」という約束が破れてしまいました。

正しくは、文の失敗に気づいたら `COMMIT` ではなく `ROLLBACK` します（練習問題2 で確かめます）。手で SQL を打つときは、エラーの表示を見落とさないことが大切です。Rust のプログラムでは、次の例5-3 のように `?` と drop を組み合わせると、この判断を自動にできます。

#### 例5-3: Rust の transaction は、commit しないと自動で取り消される

> **▼ このコードがやること（先に日本語で）:** 同じ DB ファイルに接続を2本（`a` と `b`）作ります。`a` で transaction を始めて1件追加し、`a` と `b` から件数を見比べます。`commit` も `rollback` もしないまま transaction の変数を片付けると、変更が取り消されることを確かめます。最後に、2件を「両方保存か、どちらも保存しない」にする関数 `save_two` を、失敗する場合と成功する場合で呼びます。

```rust
use libsql::{Builder, Connection};

// ノートの件数を数える
async fn count(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

// 2件のノートを「両方とも保存」か「どちらも保存しない」かにする
async fn save_two(conn: &Connection, first: &str, second: &str) -> Result<(), libsql::Error> {
    let tx = conn.transaction().await?;
    tx.execute("INSERT INTO notes (user_id, title) VALUES (1, ?1)", [first])
        .await?;
    // ここで失敗すると ? で関数を抜け、tx は commit されないまま片付けられる
    tx.execute(
        "INSERT INTO notes (user_id, title) VALUES (1, ?1)",
        [second],
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 前回の実行で作ったファイルを消してから始める
    let path = "u05_drop.db";
    let _ = std::fs::remove_file(path);

    let db = Builder::new_local(path).build().await?;
    // 同じ DB への接続を2本作る（a が書く人、b が別の画面の人）
    let a = db.connect()?;
    let b = db.connect()?;
    a.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    a.execute_batch(include_str!("../../sql/seed.sql")).await?;

    println!(
        "① 始める前: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );

    {
        let tx = a.transaction().await?;
        tx.execute(
            "INSERT INTO notes (user_id, title) VALUES (1, 'drop されるノート')",
            (),
        )
        .await?;
        println!(
            "② tx の中: tx = {}, b = {}",
            count(&tx).await?,
            count(&b).await?
        );
        println!("   a.is_autocommit() = {}", a.is_autocommit());
        // commit も rollback もしないまま、ここで tx が片付けられる（drop）
    }
    println!(
        "③ drop の後: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );
    println!("   a.is_autocommit() = {}", a.is_autocommit());

    let result = save_two(&a, "1件目", "").await;
    println!("④ save_two(\"1件目\", \"\") = {result:?}");
    println!("   その後: a = {}", count(&a).await?);

    save_two(&a, "1件目", "2件目").await?;
    println!(
        "⑤ save_two(\"1件目\", \"2件目\") の後: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );
    Ok(())
}
```

`src/bin/u05_drop.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_drop
```

**▼ 実行結果**

```text
① 始める前: a = 8, b = 8
② tx の中: tx = 9, b = 8
   a.is_autocommit() = false
③ drop の後: a = 8, b = 8
   a.is_autocommit() = true
④ save_two("1件目", "") = Err(SqliteFailure(275, "CHECK constraint failed: length(title) BETWEEN 1 AND 80"))
   その後: a = 8
⑤ save_two("1件目", "2件目") の後: a = 10, b = 10
```

##### 解説4: 2本の接続で「途中の状態が見えない」ことを確かめる

- `db.connect()` を2回呼ぶと、同じ DB ファイルへの接続（DB と話す窓口）が2本できます。Web アプリでは、同時に来たリクエストがそれぞれ別の接続を使うので、`a` と `b` を「別々の利用者」と考えてください。
- ②で、transaction の中（`tx`）からは 9 件、別の接続 `b` からは 8 件に見えました。**確定していない変更は、ほかの接続からは見えません**。これが ACID の I（Isolation、分離）です。
- `a.transaction()` は `BEGIN` を、`tx.commit()` は `COMMIT` を実行するメソッドです。`a.is_autocommit()` は「今、transaction の外（autocommit）か」を返し、transaction の中では `false` になります。

##### 解説5: drop されると、自動で ROLLBACK

- ③の直前で、`{ … }` のブロックが終わり、変数 `tx` が片付けられました（第3巻で学んだ drop です）。libsql の `Transaction` は、**commit されないまま drop されると、自動で取り消す**作りになっています。その結果、`a` も `b` も 8 件に戻り、`is_autocommit()` も `true` に戻りました。
- ④の `save_two` は、2件目の `INSERT` が CHECK 違反で失敗し、`?` で関数を抜けました。`tx.commit()` まで届かずに `tx` が drop されたので、**1件目も取り消され**、件数は 8 のままです。例5-2 の「うっかり COMMIT」が、構造的に起こらないわけです。
- ⑤は2件とも成功したので、`commit` されて 10 件になりました。

> **ヒント:** 「`?` で途中で抜けたら、自動で全部取り消し」になるのは、`commit` を**最後に1回だけ**呼ぶ書き方をしているからです。transaction の途中で `commit` を呼んだり、エラーを `?` ではなく握りつぶしたり（`let _ = tx.execute(…)`）すると、この安全さは失われます。

#### 例5-4: database is locked と busy_timeout

SQLite（libSQL）では、**同じ DB に同時に書き込めるのは1つの接続だけ**です。ある接続が書き込み用の transaction を始めている間に、ほかの接続が書き込もうとすると、どうなるでしょうか。

> **▼ このコードがやること（先に日本語で）:** 接続 `a` が書き込み用の鍵を先に取ります（`BEGIN IMMEDIATE`）。その間に、待たない設定の `b` が書き込もうとして失敗することを確かめます。次に `b` に「最大 5 秒待つ」と設定して書き込ませ、`a` が 300 ミリ秒後に確定したら `b` の書き込みが通ることを確かめます。

```rust
use std::time::{Duration, Instant};

use libsql::{Builder, TransactionBehavior};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let path = "u05_locked.db";
    let _ = std::fs::remove_file(path);
    let db = Builder::new_local(path).build().await?;
    let a = db.connect()?;
    let b = db.connect()?;
    a.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    a.execute_batch(include_str!("../../sql/seed.sql")).await?;

    // a が「書き込み用の鍵」を先に取る（BEGIN IMMEDIATE）
    let tx = a
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .await?;
    tx.execute("UPDATE notes SET done = 1 WHERE id = 1", ())
        .await?;

    // ① 待たない設定のまま、b が書き込もうとする
    let result = b
        .execute("UPDATE notes SET done = 1 WHERE id = 3", ())
        .await;
    println!("① 待たない b: {result:?}");

    // ② b に「最大 5 秒待つ」と設定し、別のタスクで書き込ませる
    b.busy_timeout(Duration::from_secs(5))?;
    let start = Instant::now();
    let waiter = tokio::spawn(async move {
        let result = b
            .execute("UPDATE notes SET done = 1 WHERE id = 3", ())
            .await;
        (result, start.elapsed())
    });

    // a は 300 ミリ秒後に確定して鍵を手放す
    tokio::time::sleep(Duration::from_millis(300)).await;
    tx.commit().await?;
    println!("② a が COMMIT（{} ミリ秒後）", start.elapsed().as_millis());

    let (result, waited) = waiter.await.expect("タスクが panic しない");
    println!(
        "② 待つ b: {result:?}（{} ミリ秒待った）",
        waited.as_millis()
    );
    Ok(())
}
```

```powershell
cargo run -q --bin u05_locked
```

**▼ 実行結果**（ミリ秒の数字は実行のたびに変わります）

```text
① 待たない b: Err(SqliteFailure(5, "database is locked"))
② a が COMMIT（325 ミリ秒後）
② 待つ b: Ok(1)（340 ミリ秒待った）
```

##### 解説6: 鍵の取り合いと、待つ設定

- `TransactionBehavior::Immediate` は `BEGIN IMMEDIATE` のことで、「transaction を始めた時点で、書き込み用の鍵を取る」指定です。ふつうの `BEGIN` は、最初に書き込む文を実行した時点で鍵を取ります。
- ①の `b` は、鍵が空くのを待たずに、すぐ `database is locked`（DB に鍵がかかっている）で失敗しました。`SqliteFailure(5, …)` の 5 は、SQLite の「BUSY」というエラー番号です。
- ②で `b.busy_timeout(5 秒)` を設定すると、`b` は鍵が空くまで最大 5 秒待ちます。`a` が約 300 ミリ秒後に `commit` して鍵を手放すと、`b` の書き込みが通り、`Ok(1)`（1行変更）になりました。
- `tokio::spawn` は、`b` の書き込みを「別の仕事」として同時に進めるための仕組みです（第5巻で学びます）。`tokio::time::sleep` は、Cargo.toml の tokio に `time` の機能を書いていなくても、libsql が同じ tokio の `time` を使っているので動きます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-052" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<line x1="90" y1="190" x2="700" y2="190" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-052)"/>
<text x="690" y="210" font-size="11" fill="#475569" text-anchor="end">時間</text>
<text x="40" y="64" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">接続 a</text>
<text x="40" y="124" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">接続 b</text>
<rect x="100" y="44" width="330" height="32" rx="6" fill="#dbeafe" stroke="#3b82f6"/>
<text x="265" y="65" font-size="12" fill="#1e40af" text-anchor="middle">BEGIN IMMEDIATE（書き込みの鍵を持つ）→ UPDATE</text>
<rect x="430" y="44" width="90" height="32" rx="6" fill="#bbf7d0" stroke="#10b981"/>
<text x="475" y="65" font-size="12" fill="#047857" text-anchor="middle">COMMIT</text>
<rect x="120" y="104" width="110" height="32" rx="6" fill="#fecaca" stroke="#ef4444"/>
<text x="175" y="125" font-size="11" fill="#b91c1c" text-anchor="middle">① すぐ失敗</text>
<text x="175" y="154" font-size="10" fill="#b91c1c" text-anchor="middle">database is locked</text>
<rect x="240" y="104" width="280" height="32" rx="6" fill="#fef9c3" stroke="#eab308" stroke-dasharray="5 4"/>
<text x="380" y="125" font-size="11" fill="#854d0e" text-anchor="middle">② busy_timeout で鍵が空くのを待つ</text>
<rect x="520" y="104" width="100" height="32" rx="6" fill="#bbf7d0" stroke="#10b981"/>
<text x="570" y="125" font-size="11" fill="#047857" text-anchor="middle">書き込み Ok(1)</text>
<line x1="520" y1="78" x2="520" y2="100" stroke="#10b981" stroke-width="2" marker-end="url(#ar-07-052)"/>
<text x="530" y="94" font-size="10" fill="#047857">鍵が空く</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 同時に書き込めるのは1つの接続だけ。待たない設定ならすぐ database is locked、busy_timeout を設定すれば鍵が空くまで待ってから書き込む</div>
</div>

#### 例5-5: 楽観的ロック — 同時編集の上書きを見つける

transaction は「1つのリクエストの中の、複数の変更」を守る仕組みです。しかし、Web アプリには、もっと長い時間の「同時」があります。

Alice が、ノート 1 の編集画面をタブ A とタブ B の2つで開いたとします。タブ A でタイトルを直して保存し、その後、古い内容のまま開いていたタブ B で保存すると、**タブ A の変更が黙って上書きされて消えます**。画面を開いてから保存するまでの数分間を、1つの transaction で守ることはできません（その間ずっと鍵を持つと、ほかの人が誰も書き込めなくなります）。

そこで、ノートに `version`（版の番号）の列を持たせます。

1. 編集画面を開いたとき、そのときの `version` をフォームに隠して埋め込む
2. 保存するとき、`WHERE … AND version = 開いたときの version` を付けて `UPDATE` し、同時に `version` を 1 増やす
3. 変更が 0 行なら、「開いた後に、ほかの画面で先に保存された」と分かる → 409（Conflict、競合）を返す

> **▼ このコードがやること（先に日本語で）:** タブ A とタブ B が、同じノート 1（`version` 1）を開いた場面を再現します。タブ A の保存は成功して `version` が 2 になり、古い `version` 1 のままのタブ B の保存は `VersionMismatch` になって 409 に変換されることを確かめます。比べるために、他人のノートを保存しようとした場合が 404 になることも確かめます。

```rust
use libsql::{Builder, Connection};

// 保存先（DB）の層の失敗
#[derive(Debug)]
enum RepoError {
    // version が食い違った（ほかの画面が先に保存した）
    VersionMismatch,
    // ノートが無い（または他人のノート）
    NotFound,
    Database(libsql::Error),
}

impl From<libsql::Error> for RepoError {
    fn from(error: libsql::Error) -> Self {
        RepoError::Database(error)
    }
}

// 利用者への返事を決める層の失敗（第4巻 UNIT 05 の形）
#[derive(Debug)]
enum AppError {
    NotFound,
    Conflict,
    Internal,
}

impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            RepoError::Database(error) => {
                // 原因はログに残し、利用者には詳しく見せない
                eprintln!("DB エラー: {error}");
                AppError::Internal
            }
        }
    }
}

impl AppError {
    fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Conflict => 409,
            AppError::Internal => 500,
        }
    }
}

// 画面を開いたときに読んだ version（expected）と一致したときだけ更新する
async fn update_title(
    conn: &Connection,
    owner: i64,
    id: i64,
    title: &str,
    expected: i64,
) -> Result<i64, RepoError> {
    let changed = conn
        .execute(
            "UPDATE notes SET title = ?1, version = version + 1
              WHERE id = ?2 AND user_id = ?3 AND version = ?4",
            libsql::params![title, id, owner, expected],
        )
        .await?;
    if changed == 1 {
        return Ok(expected + 1);
    }
    // 0 行だった: ノートが無いのか、version が違うのかを見分ける
    let mut rows = conn
        .query(
            "SELECT 1 FROM notes WHERE id = ?1 AND user_id = ?2",
            libsql::params![id, owner],
        )
        .await?;
    match rows.next().await? {
        Some(_) => Err(RepoError::VersionMismatch),
        None => Err(RepoError::NotFound),
    }
}

// ノート 1 の今の状態を表示する
async fn show(conn: &Connection) -> Result<(), libsql::Error> {
    let mut rows = conn
        .query("SELECT title, version FROM notes WHERE id = 1", ())
        .await?;
    if let Some(row) = rows.next().await? {
        let title = row.get::<String>(0)?;
        let version = row.get::<i64>(1)?;
        println!("   DB: title = {title}, version = {version}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // Alice（1）がタブ A とタブ B で、同じノート 1（version 1）の編集画面を開いた
    let opened_version = 1;
    show(&conn).await?;

    println!("タブ A が保存（version {opened_version} のつもり）");
    let result = update_title(&conn, 1, 1, "買い物リスト（A）", opened_version).await;
    println!("   結果: {result:?}");
    show(&conn).await?;

    println!("タブ B が保存（version {opened_version} のつもり）");
    let result = update_title(&conn, 1, 1, "買い物リスト（B）", opened_version).await;
    println!("   結果: {result:?}");
    if let Err(error) = result {
        let app_error = AppError::from(error);
        println!("   → {app_error:?} → HTTP {}", app_error.status_code());
    }
    show(&conn).await?;

    println!("Bob のノート 3 を Alice が保存しようとする");
    let result = update_title(&conn, 1, 3, "乗っ取り", 1).await;
    if let Err(error) = result {
        let app_error = AppError::from(error);
        println!("   → {app_error:?} → HTTP {}", app_error.status_code());
    }
    Ok(())
}
```

```powershell
cargo run -q --bin u05_conflict
```

**▼ 実行結果**

```text
   DB: title = 買い物リスト, version = 1
タブ A が保存（version 1 のつもり）
   結果: Ok(2)
   DB: title = 買い物リスト（A）, version = 2
タブ B が保存（version 1 のつもり）
   結果: Err(VersionMismatch)
   → Conflict → HTTP 409
   DB: title = 買い物リスト（A）, version = 2
Bob のノート 3 を Alice が保存しようとする
   → NotFound → HTTP 404
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-053" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="60" y="54" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">タブ A</text>
<text x="60" y="124" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">DB</text>
<text x="60" y="194" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">タブ B</text>
<rect x="110" y="34" width="130" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="175" y="56" font-size="11" fill="#0f172a" text-anchor="middle">開く（version 1）</text>
<rect x="110" y="174" width="130" height="34" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="175" y="196" font-size="11" fill="#0f172a" text-anchor="middle">開く（version 1）</text>
<rect x="110" y="104" width="130" height="34" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="175" y="126" font-size="11" fill="#0f172a" text-anchor="middle">version 1</text>
<rect x="290" y="34" width="150" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="365" y="56" font-size="11" fill="#0f172a" text-anchor="middle">保存（version = 1）</text>
<line x1="365" y1="68" x2="365" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-053)"/>
<rect x="290" y="104" width="150" height="34" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="365" y="126" font-size="11" fill="#047857" text-anchor="middle">変更 1 行 → version 2</text>
<rect x="490" y="174" width="210" height="34" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="595" y="196" font-size="11" fill="#0f172a" text-anchor="middle">保存（version = 1 のまま）</text>
<line x1="595" y1="174" x2="595" y2="142" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-053)"/>
<rect x="490" y="104" width="210" height="34" rx="6" fill="#fef2f2" stroke="#ef4444"/>
<text x="595" y="126" font-size="11" fill="#b91c1c" text-anchor="middle">変更 0 行 → 409（上書きしない）</text>
<text x="595" y="88" font-size="11" fill="#475569" text-anchor="middle">B は最新を読み直してから、もう一度保存する</text>
<line x1="240" y1="121" x2="286" y2="121" stroke="#475569" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-053)"/>
<line x1="440" y1="121" x2="486" y2="121" stroke="#475569" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-053)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 楽観的ロック。保存するときに「開いたときの version」を条件に付けるので、後から保存した古い画面は変更 0 行になり、上書き事故に気づける</div>
</div>

##### 解説7: なぜ「楽観的」なのか

- 「同時に編集されることは**めったに無い**だろう」と楽観的に考えて、**鍵はかけずに**編集させ、保存するときにだけ「食い違っていないか」を確かめる方法なので、楽観的ロック（optimistic locking）と呼びます。編集画面を開いている間ずっと鍵を持つ方法（悲観的ロック）と違い、ほかの人を待たせません。
- `update_title` は、変更が 0 行だったとき、「ノートが無い（他人のノート）」のか「`version` が違う」のかを、もう一度 `SELECT` して見分けています。前者は 404、後者は 409 です。409 のときは、利用者に「ほかの画面で先に保存されました」と知らせ、最新の内容を見せてから、もう一度保存してもらいます。
- `RepoError` を `AppError` に変える `From` の実装は、第4巻 UNIT 05 で学んだ形です。DB のエラー（`Database`）の中身はログにだけ出し、利用者には 500 と伝えます。

### よくあるエラーと読み方

```sql
-- ① 始めていないのに COMMIT
COMMIT;
-- ② BEGIN の中でもう一度 BEGIN
BEGIN;
BEGIN;
ROLLBACK;
-- ③ ROLLBACK の後にもう一度 ROLLBACK
ROLLBACK;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_errors.sql
```

```text
sql> COMMIT
エラー: SQLite failure: `cannot commit - no transaction is active`

sql> BEGIN
OK

sql> BEGIN
エラー: SQLite failure: `cannot start a transaction within a transaction`

sql> ROLLBACK
OK

sql> ROLLBACK
エラー: SQLite failure: `cannot rollback - no transaction is active`
```

| メッセージ | 何が起きたか | 直し方 |
|---|---|---|
| `cannot commit - no transaction is active` | transaction が始まっていないのに `COMMIT` した | `BEGIN` を忘れていないか確かめる |
| `cannot start a transaction within a transaction` | transaction の中で、もう一度 `BEGIN` した（入れ子にはできない） | 先に `COMMIT` か `ROLLBACK` で終わらせる |
| `cannot rollback - no transaction is active` | 取り消す transaction が無い | 二重に `ROLLBACK` していないか、途中で `COMMIT` されていないか確かめる |
| `database is locked`（例5-4） | ほかの接続が書き込みの鍵を持っている | `busy_timeout` で待つ。transaction を短くし、鍵を長く持たない |

`database is locked` が頻繁に出るときは、どこかの transaction が長すぎることがほとんどです。transaction の中で、外部の API を呼んだり、利用者の入力を待ったりしてはいけません。**transaction の中では DB の操作だけを、できるだけ短く**行います。

### RustNote ではここで使う

**楽観的ロック:** RustNote のノートの更新（`Rust/rustnote/src/repository/notes.rs` の `update`）は、例5-5 と同じ形です。

```rust
// version が一致したときだけ更新し、version を 1 増やす
let sql = format!(
    "UPDATE notes
        SET title = ?1, body = ?2, version = version + 1, updated_at = datetime('now')
      WHERE id = ?3 AND user_id = ?4 AND version = ?5
      RETURNING {NOTE_COLUMNS}"
);
```

編集画面（`src/routes/notes/id/edit.rs`）は、フォームに `version` を隠して埋め込み、保存で `RepoError::VersionMismatch` が返ると、最新の内容を読み直して、**入力した内容を残したまま 409 で再表示**します。もう一度「保存する」を押すと、最新の `version` で上書きできます。

**接続の設定:** `src/db.rs` は、DB を開くときに `PRAGMA journal_mode = WAL;` を実行し、接続のたびに `PRAGMA busy_timeout = 5000;`（最大 5 秒待つ）を設定しています。WAL（Write-Ahead Logging）は、書き込み中でも読み込みを待たせにくくする SQLite の記録方式です。

**migration:** 表の形を変える `migrate` 関数は、版ごとに `conn.transaction()` を始め、全部成功したら `commit`、途中で失敗したら `rollback` しています。次の UNIT 06 で詳しく見ます。

### 練習問題

**問題1:** 次の SQL を実行した後、ノートは何件になるでしょうか（練習用データは 8 件から始まります）。

```sql
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, 'A');
COMMIT;
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, 'B');
INSERT INTO notes (user_id, title) VALUES (1, 'C');
ROLLBACK;
SELECT COUNT(*) AS notes FROM notes;
```

**問題2:** 例5-2 を、「2件目の失敗に気づいたので、COMMIT ではなく ROLLBACK する」形に直してください。1件目が残らないことを確かめます。

**問題3:** 例5-5 と同じ場面を、SQL だけで再現してください。タブ A が `version = 1` で保存し、タブ B も `version = 1` で保存し（失敗する）、タブ B が画面を読み直して `version = 2` で保存し直します。最後にノート 1 のタイトルと `version` を確かめます。

### 解答と解説

**解答1:** 9 件です。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_q1.sql
```

```text
sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, 'A')
OK（変更: 1 行）

sql> COMMIT
OK

sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, 'B')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (1, 'C')
OK（変更: 1 行）

sql> ROLLBACK
OK

sql> SELECT COUNT(*) AS notes FROM notes
+-------+
| notes |
+-------+
| 9     |
+-------+
(1 行)
```

A は `COMMIT` で確定し、B と C は同じ transaction の中で `ROLLBACK` されたので、両方とも消えます。transaction の境目ごとに「何が確定したか」を数えるのがコツです。

**解答2:**

```sql
BEGIN;
INSERT INTO notes (user_id, title) VALUES (1, '1件目');
INSERT INTO notes (user_id, title) VALUES (1, '');
-- 失敗に気づいたので、COMMIT ではなく ROLLBACK する
ROLLBACK;
SELECT id, title FROM notes WHERE id > 8;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_q2.sql
```

```text
sql> BEGIN
OK

sql> INSERT INTO notes (user_id, title) VALUES (1, '1件目')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (1, '')
エラー: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`

sql> ROLLBACK
OK

sql> SELECT id, title FROM notes WHERE id > 8
+----+-------+
| id | title |
+----+-------+
(0 行)
```

`ROLLBACK` で1件目も取り消され、追加されたノートは 0 件です。「2件まとめて」の約束が守られました。

**解答3:**

```sql
-- タブ A: 開いたときの version 1 で保存
UPDATE notes SET title = '買い物リスト（A）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1;
-- タブ B: 古い version 1 のまま保存
UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1;
-- タブ B: 画面を読み直して、最新の version 2 で保存し直す
UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 2;
SELECT id, title, version FROM notes WHERE id = 1;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u05_q3.sql
```

```text
sql> UPDATE notes SET title = '買い物リスト（A）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1
OK（変更: 1 行）

sql> UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 1
OK（変更: 0 行）

sql> UPDATE notes SET title = '買い物リスト（B）', version = version + 1 WHERE id = 1 AND user_id = 1 AND version = 2
OK（変更: 1 行）

sql> SELECT id, title, version FROM notes WHERE id = 1
+----+-------------------+---------+
| id | title             | version |
+----+-------------------+---------+
| 1  | 買い物リスト（B） | 3       |
+----+-------------------+---------+
(1 行)
```

2回目の `UPDATE` が「変更: 0 行」になったところが、アプリが 409 を返す場面です。タブ B の人は、タブ A の変更（「買い物リスト（A）」）を画面で見たうえで、自分の内容で上書きすると決めたので、3回目で `version` 3 として保存されました。黙って上書きされるのではなく、**人が気づいて選べる**ことが大切です。

### まとめ

- transaction は複数の変更を「全部確定（COMMIT）か、全部取り消し（ROLLBACK）か」にまとめます。途中の文が失敗しても自動では取り消されないので、気づいたら ROLLBACK します。
- libsql の `Transaction` は、`commit` されないまま drop されると自動で取り消されます。`commit` を最後に1回だけ呼び、途中は `?` で抜ける書き方が安全です。
- 同時に書き込めるのは1つの接続だけです（`database is locked`）。画面をまたぐ同時編集は、`version` 列の楽観的ロックで見つけて 409 を返します。
