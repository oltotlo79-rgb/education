## UNIT 04: index と EXPLAIN QUERY PLAN

### この単元のゴール

- index（索引）が何をする仕組みかを、本の索引にたとえて説明できる
- `EXPLAIN QUERY PLAN` を読み、`SCAN`（全部を見る）と `SEARCH`（索引で探す）を見分けられる
- `WHERE` と `ORDER BY` に合わせて、**複合索引の列の順番**を決められる
- 索引には、書き込みが遅くなる・DB が大きくなるという費用があることを、実測した数字で説明できる

### たとえ話でつかむ

分厚い料理の本で「肉じゃが」の作り方を探すとき、2つの探し方があります。

- **1ページ目から全部めくる:** 確実に見つかりますが、500 ページあれば 500 ページ全部を見ることになります。
- **巻末の索引を引く:** 「に」の行に「肉じゃが …… 128」と書いてあるので、128 ページへ直接飛べます。

巻末の索引は、本文とは別に「料理名を五十音順に並べ替えて、ページ番号を書いた一覧」です。並んでいるので、目的の行をすぐ見つけられます。そのかわり、新しい料理を本に足すたびに、索引にも1行書き足す手間がかかり、本も索引の分だけ厚くなります。

DB の index（インデックス、索引）もまったく同じです。表とは別に「列の値を並べ替えて、行の場所を書いた一覧」を持ちます。探すのは速くなり、書き込みは少し遅くなり、ファイルは大きくなります。

> **用語:** index（インデックス、索引：表の列の値を並べ替えて、行の場所と一緒に持っておく別のデータ。検索を速くする）

<!-- -->

> **用語:** SCAN / SEARCH（スキャン／サーチ：`EXPLAIN QUERY PLAN` に出る言葉。SCAN は表や索引を最初から最後まで全部見ること、SEARCH は索引を使って目的の場所へ直接飛ぶこと）

### 図で理解する

10 万件のノートから「会員 42 のノートを新しい順に 20 件」探すとき、索引が無い場合と有る場合で、DB の仕事量は次のように違います。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-041" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">索引なし: SCAN</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">索引あり: SEARCH</text>
<rect x="20" y="36" width="320" height="140" rx="8" fill="#fef2f2" stroke="#fca5a5"/>
<rect x="34" y="48" width="292" height="14" fill="#fecaca"/>
<rect x="34" y="66" width="292" height="14" fill="#fecaca"/>
<rect x="34" y="84" width="292" height="14" fill="#fecaca"/>
<rect x="34" y="102" width="292" height="14" fill="#fecaca"/>
<rect x="34" y="120" width="292" height="14" fill="#fecaca"/>
<rect x="34" y="138" width="292" height="14" fill="#fecaca"/>
<text x="180" y="168" font-size="11" fill="#475569" text-anchor="middle">notes 表の 10 万行を、1行目から全部見る</text>
<line x1="180" y1="176" x2="180" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-041)"/>
<rect x="20" y="200" width="320" height="34" rx="8" fill="#ffffff" stroke="#fca5a5"/>
<text x="180" y="222" font-size="12" fill="#0f172a" text-anchor="middle">会員 42 の 100 行を集めて、別に並べ替える</text>
<text x="180" y="262" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">1 回 約 10.6 ミリ秒</text>
<rect x="380" y="36" width="320" height="140" rx="8" fill="#ecfdf5" stroke="#6ee7b7"/>
<text x="400" y="56" font-size="11" fill="#475569">索引（user_id, updated_at DESC, id DESC）</text>
<rect x="394" y="66" width="292" height="14" fill="#d1fae5"/>
<text x="400" y="77" font-size="10" fill="#475569">41 …</text>
<rect x="394" y="84" width="292" height="14" fill="#34d399"/>
<text x="400" y="95" font-size="10" fill="#064e3b">42  99041, 98041, 97041 …（新しい順）</text>
<rect x="394" y="102" width="292" height="14" fill="#34d399"/>
<text x="400" y="113" font-size="10" fill="#064e3b">42  … 1041, 41</text>
<rect x="394" y="120" width="292" height="14" fill="#d1fae5"/>
<text x="400" y="131" font-size="10" fill="#475569">43 …</text>
<text x="540" y="168" font-size="11" fill="#475569" text-anchor="middle">会員 42 の場所へ直接飛ぶ。もう新しい順に並んでいる</text>
<line x1="540" y1="176" x2="540" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-041)"/>
<rect x="380" y="200" width="320" height="34" rx="8" fill="#ffffff" stroke="#6ee7b7"/>
<text x="540" y="222" font-size="12" fill="#0f172a" text-anchor="middle">先頭から 20 件読んだら終わり</text>
<text x="540" y="262" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">1 回 約 0.046 ミリ秒（約 230 倍速い）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 索引が無いと表を全部めくって並べ替える。索引があると、会員 42 の場所へ直接飛び、並んだ順に 20 件読むだけで済む（時間はこの単元の例4-3 の実測値）</div>
</div>

### まず動かす最小コード

索引の効き目は、行が少ないと分かりません。そこで、会員 1,000 人・ノート 10 万件の練習用データを作る SQL を用意します。

```sql
-- 会員を 1,000 人作る（WITH RECURSIVE で 1〜1000 の数を作り、それを使って INSERT する）
WITH RECURSIVE seq(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM seq WHERE n < 1000)
INSERT INTO users (id, email, password_hash)
SELECT n, 'user' || n || '@example.com', 'dummy' FROM seq;

-- ノートを 100,000 件作る（1人あたり 100 件。更新日時は1分ずつずらす）
WITH RECURSIVE seq(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM seq WHERE n < 100000)
INSERT INTO notes (user_id, title, body, updated_at)
SELECT (n % 1000) + 1, 'ノート ' || n, '本文 ' || n, datetime('2026-01-01', '+' || n || ' minutes')
FROM seq;
```

これを `sql/u04_big.sql` として保存します。`WITH RECURSIVE` は「1, 2, 3, … と数を作る」ための書き方で、ここでは道具として使うだけなので、覚えなくてかまいません。`(n % 1000) + 1` で、ノート n の持ち主を「n を 1000 で割った余り + 1」にしています。会員 42 のノートは、id が 41・1041・2041・…・99041 の 100 件です。

#### 例4-1: 索引の前と後で、計画を比べる

> **▼ このコードがやること（先に日本語で）:** 「会員 42 のノートを新しい順に 20 件」読む文について、DB がどう探すつもりか（計画）を表示します。比べるために、主キーで1件探す文の計画も見ます。次に RustNote と同じ索引を作り、同じ文の計画をもう一度見ます。最後に、索引があっても結果は同じであることを確かめます。

```sql
-- ① 索引が無いとき: 会員 42 番のノートを新しい順に 20 件
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ② 主キー（id）で1件探すとき
EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE id = 42;
-- ③ 索引を作る（RustNote の migrations/0002 と同じ）
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- ④ ①と同じ問い合わせの計画を、もう一度見る
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ⑤ 結果は同じ（索引は速さだけを変え、答えは変えない）
SELECT id, title, updated_at FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 3;
```

`sql/u04_plan.sql` として保存し、表の準備（`schema.sql`）と 10 万件のデータ（`u04_big.sql`）を `--setup` で用意してから実行します。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/u04_big.sql sql/u04_plan.sql
```

**▼ 実行結果**

```text
（準備: sql/schema.sql の 2 文を実行しました）
（準備: sql/u04_big.sql の 2 文を実行しました）
sql> EXPLAIN QUERY PLAN …
QUERY PLAN
- SCAN notes
- USE TEMP B-TREE FOR ORDER BY

sql> EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE id = 42
QUERY PLAN
- SEARCH notes USING INTEGER PRIMARY KEY (rowid=?)

sql> CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)
OK

sql> EXPLAIN QUERY PLAN …
QUERY PLAN
- SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)

sql> SELECT id, title, updated_at FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 3
+-------+--------------+---------------------+
| id    | title        | updated_at          |
+-------+--------------+---------------------+
| 99041 | ノート 99041 | 2026-03-10 18:41:00 |
| 98041 | ノート 98041 | 2026-03-10 02:01:00 |
| 97041 | ノート 97041 | 2026-03-09 09:21:00 |
+-------+--------------+---------------------+
(3 行)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: EXPLAIN QUERY PLAN は「実行せずに、計画だけ見せて」

- 文の前に `EXPLAIN QUERY PLAN` を付けると、DB はその文を**実行せずに**、「どうやって探すつもりか」だけを返します。道具 `sql` は、それを `QUERY PLAN` の下に木の形で表示します。
- ①の `SCAN notes` は「notes 表を全部見る」、`USE TEMP B-TREE FOR ORDER BY` は「並べ替えのために、一時的な作業場所を作る」です。10 万行を見て、会員 42 の 100 行を集め、それを並べ替えてから 20 件を返す、という計画です。
- ②の `SEARCH notes USING INTEGER PRIMARY KEY (rowid=?)` は「主キーを使って直接探す」です。主キーは最初から索引と同じように並んでいるので、索引を作らなくても速く探せます（`rowid` は SQLite の行番号で、`INTEGER PRIMARY KEY` の列はその別名です）。

##### 解説2: CREATE INDEX の形

```sql
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
```

- `CREATE INDEX 索引の名前 ON 表 (列, 列…)` で索引を作ります。名前は自由ですが、`idx_表_列` のように中身が分かる名前にしておくと、後で計画を読むときに助かります。
- 列を複数並べた索引を**複合索引**と呼びます。この索引は「まず `user_id` の順、同じ `user_id` の中では `updated_at` の新しい順、それも同じなら `id` の大きい順」に並んでいます。
- ④の計画は `SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)` に変わり、`USE TEMP B-TREE` が消えました。索引の中で会員 42 の行は**すでに新しい順に並んでいる**ので、並べ替えが要らなくなったのです。

##### 解説3: 索引は答えを変えない

⑤の結果は、索引の有る無しで同じです。索引は「速さ」だけを変える仕組みで、結果が変わることはありません。だから、遅い問い合わせを見つけたら、まず索引を足せないかを考えます。

### もう一歩: 実務に近い形

#### 例4-2: 索引が効くとき・効かないとき

> **▼ このコードがやること（先に日本語で）:** 索引を作った後で、5つの文の計画を見ます。索引の先頭の列を条件に使わない場合、`LIKE '%…%'` の場合、`UNIQUE` が自動で作った索引の場合、列に関数をかぶせた場合、件数を数える場合です。

```sql
-- ① 索引の先頭の列（user_id）を条件に使わないと、索引で探せない
EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE updated_at >= '2026-03-01';
-- ② LIKE '%…%' は索引で探せないが、user_id で先に絞れば残りを調べるだけで済む
EXPLAIN QUERY PLAN SELECT id FROM notes WHERE user_id = 42 AND title LIKE '%買い物%';
-- ③ UNIQUE 制約が自動で作った索引（sqlite_autoindex_users_1）が使われる
EXPLAIN QUERY PLAN SELECT id FROM users WHERE email = 'user42@example.com';
-- ④ 列に関数をかぶせると、その索引は使えない
EXPLAIN QUERY PLAN SELECT id FROM users WHERE lower(email) = 'user42@example.com';
-- ⑤ 数える問い合わせも索引で速くなる
EXPLAIN QUERY PLAN SELECT COUNT(*) FROM notes WHERE user_id = 42;
```

索引を作る1行だけの SQL を `sql/u04_index.sql` に保存しておき（`CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);`）、それも `--setup` に加えて実行します。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/u04_big.sql --setup sql/u04_index.sql sql/u04_more.sql
```

**▼ 実行結果**（準備の3行は省略）

```text
sql> EXPLAIN QUERY PLAN SELECT id, title FROM notes WHERE updated_at >= '2026-03-01'
QUERY PLAN
- SCAN notes

sql> EXPLAIN QUERY PLAN SELECT id FROM notes WHERE user_id = 42 AND title LIKE '%買い物%'
QUERY PLAN
- SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)

sql> EXPLAIN QUERY PLAN SELECT id FROM users WHERE email = 'user42@example.com'
QUERY PLAN
- SEARCH users USING COVERING INDEX sqlite_autoindex_users_1 (email=?)

sql> EXPLAIN QUERY PLAN SELECT id FROM users WHERE lower(email) = 'user42@example.com'
QUERY PLAN
- SCAN users USING COVERING INDEX sqlite_autoindex_users_1

sql> EXPLAIN QUERY PLAN SELECT COUNT(*) FROM notes WHERE user_id = 42
QUERY PLAN
- SEARCH notes USING COVERING INDEX idx_notes_user_updated (user_id=?)
```

##### 解説4: 複合索引は「先頭の列から」しか使えない

- ①は `updated_at` だけで探そうとして、`SCAN notes` になりました。索引は `user_id` の順に並んでいるので、`updated_at` だけでは「どこから探せばよいか」が決まらないのです。
- 学校の名簿が「組 → 出席番号」の順に並んでいるとき、「3 組の人」はすぐ探せますが、「出席番号 5 番の人」だけでは、全部の組を見るしかありません。複合索引も同じで、**先頭の列から順に**条件に使えるときだけ役に立ちます。
- ②は `LIKE '%買い物%'` を含みますが、`user_id = 42` で索引を使って 100 行に絞り、その 100 行だけでタイトルを調べます。`%` で始まる `LIKE` は索引で探せませんが、ほかの条件で先に絞れば十分速くなります。

##### 解説5: 列に関数をかぶせると、索引は使えない

- ③は、UNIT 01 で見た `sqlite_autoindex_users_1`（`email` の `UNIQUE` のために自動で作られた索引）で直接探せています。`COVERING INDEX` は「**索引だけで答えが出せたので、表を見に行かなかった**」という意味で、さらに速い形です。
- ④は `lower(email)` と列に関数をかぶせたので、`SCAN` になりました。索引に並んでいるのは `email` そのものの値で、`lower(email)` の値ではないからです。
- RustNote は、登録とログインのときに `Email::parse` でメールアドレスを小文字にそろえてから保存・検索しています。だから SQL に `lower()` を書く必要がなく、索引がそのまま使えます。

#### 例4-3: 速さと費用を測る

計画だけでなく、実際の時間も測ってみましょう。次のプログラムは、メモリの中の DB に 10 万件のノートを用意し、一覧を読む時間と、1,000 件を追加する時間を、索引の前と後で比べます。

> **▼ このコードがやること（先に日本語で）:** メモリの DB に表と 10 万件のデータを作ります。「会員 42 のノートを新しい順に 20 件」を 50 回読んで1回あたりの平均時間を測り、1,000 件を追加する時間も測ります。索引を作ってから、同じ2つを測り直して比べます。

```rust
use std::time::{Duration, Instant};

use libsql::{Builder, Connection};

// 会員 42 番のノートを、新しい順に 20 件読む問い合わせ
const LIST_SQL: &str = "SELECT id, title FROM notes WHERE user_id = ?1
     ORDER BY updated_at DESC, id DESC LIMIT 20";

// 同じ問い合わせを times 回くり返し、1回あたりの平均時間を返す
async fn average_time(conn: &Connection, times: u32) -> Result<Duration, libsql::Error> {
    let start = Instant::now();
    for _ in 0..times {
        let mut rows = conn.query(LIST_SQL, [42]).await?;
        // 行を最後まで読み切る（読まないと、問い合わせが最後まで実行されない）
        while rows.next().await?.is_some() {}
    }
    Ok(start.elapsed() / times)
}

// 1,000 件のノートを追加するのにかかった時間を返す
async fn insert_time(conn: &Connection) -> Result<Duration, libsql::Error> {
    let start = Instant::now();
    let tx = conn.transaction().await?;
    for n in 0..1000 {
        tx.execute(
            "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
            libsql::params![(n % 1000) + 1, format!("追加 {n}")],
        )
        .await?;
    }
    tx.commit().await?;
    Ok(start.elapsed())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // メモリの中だけの DB に、表と 100,000 件のノートを用意する
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/u04_big.sql"))
        .await?;

    let without_index = average_time(&conn, 50).await?;
    let insert_without = insert_time(&conn).await?;

    conn.execute(
        "CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)",
        (),
    )
    .await?;

    let with_index = average_time(&conn, 50).await?;
    let insert_with = insert_time(&conn).await?;

    println!("一覧の読み出し（1回の平均）");
    println!("  索引なし: {without_index:?}");
    println!("  索引あり: {with_index:?}");
    println!(
        "  → 約 {} 倍速い",
        without_index.as_micros() / with_index.as_micros().max(1)
    );
    println!("1,000 件の追加（合計）");
    println!("  索引なし: {insert_without:?}");
    println!("  索引あり: {insert_with:?}");
    Ok(())
}
```

`src/bin/u04_timing.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_timing
```

**▼ 実行結果**（時間は実行のたび、パソコンごとに変わります）

```text
一覧の読み出し（1回の平均）
  索引なし: 10.587986ms
  索引あり: 46.158µs
  → 約 230 倍速い
1,000 件の追加（合計）
  索引なし: 13.6973ms
  索引あり: 18.7172ms
```

さらに、索引で DB がどれだけ大きくなるかも見ます。`PRAGMA page_count` は DB のページ数（1ページは `PRAGMA page_size` バイト）を返します。

```sql
-- 索引を作る前の DB の大きさ（ページ数。1ページは page_size バイト）
PRAGMA page_size;
PRAGMA page_count;
-- 索引を作る
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- 作った後の大きさ
PRAGMA page_count;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/u04_big.sql sql/u04_size.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> PRAGMA page_size
+-----------+
| page_size |
+-----------+
| 4096      |
+-----------+
(1 行)

sql> PRAGMA page_count
+------------+
| page_count |
+------------+
| 2055       |
+------------+
(1 行)

sql> CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)
OK

sql> PRAGMA page_count
+------------+
| page_count |
+------------+
| 2899       |
+------------+
(1 行)
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="120" y="24" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">一覧の読み出し（1回）</text>
<text x="360" y="24" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">1,000 件の追加</text>
<text x="600" y="24" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DB の大きさ</text>
<line x1="20" y1="200" x2="700" y2="200" stroke="#cbd5e1"/>
<rect x="50" y="50" width="50" height="150" fill="#fca5a5"/>
<rect x="140" y="199" width="50" height="1" fill="#34d399"/>
<text x="75" y="44" font-size="11" fill="#b91c1c" text-anchor="middle">10.6ms</text>
<text x="165" y="192" font-size="11" fill="#047857" text-anchor="middle">0.046ms</text>
<rect x="290" y="91" width="50" height="109" fill="#fca5a5"/>
<rect x="380" y="51" width="50" height="149" fill="#34d399"/>
<text x="315" y="85" font-size="11" fill="#b91c1c" text-anchor="middle">13.7ms</text>
<text x="405" y="45" font-size="11" fill="#047857" text-anchor="middle">18.7ms</text>
<rect x="530" y="94" width="50" height="106" fill="#fca5a5"/>
<rect x="620" y="50" width="50" height="150" fill="#34d399"/>
<text x="555" y="88" font-size="11" fill="#b91c1c" text-anchor="middle">8.4MB</text>
<text x="645" y="44" font-size="11" fill="#047857" text-anchor="middle">11.9MB</text>
<text x="75" y="218" font-size="11" fill="#475569" text-anchor="middle">索引なし</text>
<text x="165" y="218" font-size="11" fill="#475569" text-anchor="middle">索引あり</text>
<text x="315" y="218" font-size="11" fill="#475569" text-anchor="middle">索引なし</text>
<text x="405" y="218" font-size="11" fill="#475569" text-anchor="middle">索引あり</text>
<text x="555" y="218" font-size="11" fill="#475569" text-anchor="middle">索引なし</text>
<text x="645" y="218" font-size="11" fill="#475569" text-anchor="middle">索引あり</text>
<text x="120" y="240" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">約 230 倍速い</text>
<text x="360" y="240" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">約 1.4 倍遅い</text>
<text x="600" y="240" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">約 1.4 倍大きい</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 索引の効き目と費用（例4-3 の実測値。棒の高さは項目ごとに、大きいほうを同じ高さにそろえている）。読み出しは劇的に速くなるが、書き込みと容量は少し増える</div>
</div>

##### 解説6: 測った数字の読み方

- 一覧の読み出しは、約 10.6 ミリ秒から約 0.046 ミリ秒（46 マイクロ秒）へ、**約 230 倍**速くなりました。`µs` はマイクロ秒（100 万分の 1 秒）です。
- 1,000 件の追加は、約 13.7 ミリ秒から約 18.7 ミリ秒へ、**少し遅く**なりました。行を足すたびに、索引にも1行ずつ書き足すからです。
- DB の大きさは、2055 ページ（約 8.4MB）から 2899 ページ（約 11.9MB）へ、索引の分だけ**約 4 割大きく**なりました。
- このプログラムは「開発用（debug）」のビルドで測っています。`cargo run -q --release --bin u04_timing` で最適化したビルドにすると、この PC では索引なし 4.9 ミリ秒・索引あり 0.019 ミリ秒（約 259 倍）でした。絶対の数字は環境で変わりますが、「読み出しは桁違いに速く、書き込みは少し遅く」という傾向は変わりません。
- プログラムの中の `conn.transaction()` と `tx.commit()` は、1,000 件の追加を1つにまとめる仕組みで、次の UNIT 05 で学びます。libsql の使い方（`Builder`・`query`・`rows.next()`）は UNIT 07 で詳しく学びます。

> **ヒント:** だからといって「全部の列に索引を付ければ速くなる」わけではありません。使われない索引は、書き込みを遅くし、容量を増やすだけです。索引は、**実際に使われている `WHERE` と `ORDER BY` に合わせて**、必要な分だけ作ります。作ったら、`EXPLAIN QUERY PLAN` で本当に使われているかを確かめます。

### よくあるエラーと読み方

```sql
-- ① 同じ名前の索引をもう一度作る
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
-- ② 列名のつづり間違い
CREATE INDEX idx_notes_usr ON notes (usr_id);
-- ③ 列の順番を逆にした索引（updated_at が先）だけがあると……
DROP INDEX idx_notes_user_updated;
CREATE INDEX idx_notes_updated_user ON notes (updated_at DESC, user_id);
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 ORDER BY updated_at DESC, id DESC LIMIT 20;
-- ④ 存在しない索引を消す
DROP INDEX idx_nope;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/u04_big.sql --setup sql/u04_index.sql sql/u04_errors.sql
```

```text
sql> CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)
エラー: SQLite failure: `index idx_notes_user_updated already exists`

sql> CREATE INDEX idx_notes_usr ON notes (usr_id)
エラー: SQLite failure: `no such column: usr_id`

sql> DROP INDEX idx_notes_user_updated
OK

sql> CREATE INDEX idx_notes_updated_user ON notes (updated_at DESC, user_id)
OK

sql> EXPLAIN QUERY PLAN …
QUERY PLAN
- SCAN notes USING INDEX idx_notes_updated_user
- USE TEMP B-TREE FOR LAST TERM OF ORDER BY

sql> DROP INDEX idx_nope
エラー: SQLite failure: `no such index: idx_nope`
```

| 番号 | メッセージ・結果 | 何が起きたか | 直し方 |
|---|---|---|---|
| ① | `index idx_notes_user_updated already exists` | 同じ名前の索引がもうある | `CREATE INDEX IF NOT EXISTS` にする。ただし、中身が違っても何もしない点に注意（UNIT 01 のエラー7 と同じ） |
| ② | `no such column: usr_id` | 索引に指定した列が表に無い | 列名を直す |
| ③ | エラーにならず `SCAN … USING INDEX` | 索引はあるが、先頭の列が `updated_at` なので会員 42 の場所へ飛べない。索引の順に全部を見て、最後の `id DESC` だけ別に並べ替えている | 条件に使う列（`user_id`）を先頭にした索引にする |
| ④ | `no such index: idx_nope` | 消そうとした索引が無い | `DROP INDEX IF EXISTS` にする |

③は、エラーが出ないので気づきにくい失敗です。`SCAN … USING INDEX` は「索引を使っている」ように見えますが、`SCAN` なので**全部を見ています**。計画を読むときは、`USING INDEX` より先に、行の頭が `SEARCH` か `SCAN` かを見てください。複合索引の列の順番は、**`=` で絞る列 → 並べ替える列**の順にするのが基本です。

### RustNote ではここで使う

RustNote の migrations には、次の3つの索引があります。どれも、実際に使う SQL に合わせて作っています。

| 索引 | 列 | 速くする SQL（`Rust/rustnote/src/repository/`） |
|---|---|---|
| `idx_notes_user_updated` | `notes (user_id, updated_at DESC, id DESC)` | ノート一覧と検索（`notes.rs` の `search`）: `WHERE user_id = ?1 AND (…LIKE…) ORDER BY updated_at DESC, id DESC LIMIT ?3 OFFSET ?4` |
| `idx_sessions_expires` | `sessions (expires_at)` | 期限切れのセッションの削除（`sessions.rs`）: `DELETE FROM sessions WHERE expires_at <= ?1` |
| `idx_sessions_user` | `sessions (user_id)` | 会員を消したときの `ON DELETE CASCADE`（その会員のセッションを探して消す） |

このほか、`users.email` の `UNIQUE` と `sessions.token_hash` の主キーには、SQLite が自動で索引を作っています。ログイン（`WHERE email = ?1`）と、ログイン中かどうかの確認（`WHERE s.token_hash = ?1`）は、それらの索引で速く探せます。

> **ヒント:** `idx_sessions_user` のように、外部キーの列（子の表の `user_id`）に索引を作っておくのは定番です。親の行を消すとき、DB は子の表から「その親を指している行」を探します。索引が無いと、会員を1人消すたびに `sessions` 表を全部見ることになります。

### 練習問題

**問題1:** RustNote に「未完了のノートだけを表示する」機能を足すことになり、次の SQL を使います。今の索引 `idx_notes_user_updated` のままで、索引で探せるでしょうか。予想してから、`EXPLAIN QUERY PLAN` で確かめてください。

```sql
SELECT id, title FROM notes WHERE user_id = 42 AND done = 0 ORDER BY updated_at DESC, id DESC LIMIT 20;
```

**問題2:** 索引の無い `sessions` 表で、期限切れのセッションを消す `DELETE FROM sessions WHERE expires_at <= 1760000000;` の計画を見てください。次に `expires_at` の索引を作り、計画がどう変わるかを確かめてください（`EXPLAIN QUERY PLAN` は `DELETE` にも使えます）。

**問題3:** 同僚が「どの列で検索されるか分からないから、notes の全部の列に1つずつ索引を作っておこう」と提案しました。例4-3 の結果をもとに、意見を述べてください。

### 解答と解説

**解答1:** 次の SQL を `sql/u04_q1.sql` として保存し、索引を作った状態で実行します。

```sql
-- 未完了（done = 0）のノートだけを、新しい順に 20 件
EXPLAIN QUERY PLAN
SELECT id, title FROM notes WHERE user_id = 42 AND done = 0 ORDER BY updated_at DESC, id DESC LIMIT 20;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/u04_big.sql --setup sql/u04_index.sql sql/u04_q1.sql
```

```text
sql> EXPLAIN QUERY PLAN …
QUERY PLAN
- SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)
```

索引で探せます。`user_id = 42` で会員 42 の場所へ飛び、そこから新しい順に読みながら `done = 0` でない行を飛ばし、20 件そろったら終わりです。並べ替え（`TEMP B-TREE`）も出ていません。会員1人のノートはせいぜい数百件なので、`done` のための索引をさらに足す必要は、ふつうはありません。

**解答2:**

```sql
-- 索引の無い sessions 表を作る（練習用）
CREATE TABLE sessions (
    token_hash BLOB PRIMARY KEY,
    user_id INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);
-- 期限切れのセッションを消す文の計画（索引なし）
EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000;
-- expires_at の索引を作る
CREATE INDEX idx_sessions_expires ON sessions (expires_at);
-- もう一度、計画を見る
EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000;
```

```powershell
cargo run -q --bin sql -- sql/u04_q2.sql
```

```text
sql> CREATE TABLE sessions ( …
OK

sql> EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000
QUERY PLAN
- SCAN sessions

sql> CREATE INDEX idx_sessions_expires ON sessions (expires_at)
OK

sql> EXPLAIN QUERY PLAN DELETE FROM sessions WHERE expires_at <= 1760000000
QUERY PLAN
- SEARCH sessions USING INDEX idx_sessions_expires (expires_at<?)
```

`SCAN` が `SEARCH` に変わりました。索引は `=` だけでなく、`<`・`<=`・`>`・`BETWEEN` のような**範囲の条件**にも使えます（計画の表示は `<=` でも `(expires_at<?)` と出ます）。

**解答3:** 反対です。例4-3 のとおり、索引は1つ増えるごとに書き込みを遅くし、DB を大きくします。使われない索引は費用だけがかかります。また、問題1 で見たように、DB が1つの問い合わせで使う索引はふつう1つだけなので、1列ずつの索引をたくさん作っても、`user_id` で絞って `updated_at` で並べるような問い合わせには役に立ちません。**実際に使う SQL を先に決め、その `WHERE` と `ORDER BY` に合う複合索引を作り、`EXPLAIN QUERY PLAN` で確かめる**のが正しい順番です。

### まとめ

- 索引は「並べ替えた別の一覧」です。`EXPLAIN QUERY PLAN` で、`SCAN`（全部を見る）か `SEARCH`（索引で飛ぶ）かを確かめます。
- 複合索引は先頭の列から順にしか使えません。「`=` で絞る列 → 並べ替える列」の順に作ります。列に関数をかぶせると索引は使えません。
- 索引は読み出しを桁違いに速くしますが、書き込みと容量の費用がかかります。実際に使う SQL に合わせて、必要な分だけ作ります。
