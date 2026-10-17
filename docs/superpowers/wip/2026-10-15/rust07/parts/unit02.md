## UNIT 02: SELECT・WHERE・ORDER BY・LIMIT

### この単元のゴール

- `SELECT` で必要な列だけを読み、`WHERE` で行を絞り込める（`=`・`AND`・`OR`・`LIKE`・`IS NULL`）
- `ORDER BY` で並び順を決め、同じ値の行があっても**並びが1通りに決まる**ように書ける
- `LIMIT` と `OFFSET` でページ送りができ、`COUNT(*)` で件数を数えられる
- `JOIN` と `LEFT JOIN` で2つの表をつないで読める
- `no such column`・`ambiguous column name` などのエラーを読んで直せる

### たとえ話でつかむ

図書館の司書さんに、本を探してもらう場面を考えます。あなたは注文書に次のように書いて渡します。

- **どの棚から:** 「小説の棚から」
- **どんな本を:** 「作者が○○さんで、まだ借りられていない本」
- **どの順番で:** 「新しく入った順に」
- **何冊:** 「最初の 2 冊だけ」
- **何を写して:** 「題名と出版年だけ書き写して」

`SELECT` 文は、この注文書そのものです。

| 注文書の欄 | SQL | 例 |
|---|---|---|
| 何を写して | `SELECT 列, 列` | `SELECT id, title` |
| どの棚から | `FROM 表` | `FROM notes` |
| どんな本を | `WHERE 条件` | `WHERE user_id = 1 AND done = 0` |
| どの順番で | `ORDER BY 列 DESC` | `ORDER BY updated_at DESC` |
| 何冊 | `LIMIT 件数` | `LIMIT 2` |

司書さんは、注文書に書いていないことはしてくれません。「順番」を書かなければ、**棚から取り出した順にバラバラに**持ってきます。SQL も同じで、`ORDER BY` を書かない結果の順番は、DB の都合で決まります。今たまたま id の順に並んでいても、データが増えたり索引ができたりすると変わることがあります。

> **用語:** クエリ（query：DB への問い合わせ。特に `SELECT` 文のこと）

<!-- -->

> **用語:** 条件式（WHERE の後ろに書く、行ごとに「真（1）・偽（0）・不明（NULL）」が決まる式）

### 図で理解する

`SELECT` 文は「書く順番」と「DB が処理する順番」が違います。下の図は、Alice（user_id 1）のノートを新しい順に 2 件読む文で、行がどう減っていくかを表しています。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-021" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" font-size="13" fill="#0f172a" text-anchor="middle">SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC LIMIT 2</text>
<rect x="10" y="44" width="125" height="104" rx="10" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="72" y="66" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">① FROM notes</text>
<text x="72" y="92" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle">8 行</text>
<text x="72" y="116" font-size="11" fill="#475569" text-anchor="middle">表の全部の行</text>
<text x="72" y="133" font-size="11" fill="#475569" text-anchor="middle">から始める</text>
<rect x="155" y="44" width="125" height="104" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="217" y="66" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② WHERE</text>
<text x="217" y="92" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle">5 行</text>
<text x="217" y="116" font-size="11" fill="#475569" text-anchor="middle">user_id = 1 の</text>
<text x="217" y="133" font-size="11" fill="#475569" text-anchor="middle">行だけ残す</text>
<rect x="300" y="44" width="125" height="104" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="362" y="66" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">③ SELECT</text>
<text x="362" y="92" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle">5 行</text>
<text x="362" y="116" font-size="11" fill="#475569" text-anchor="middle">id と title の</text>
<text x="362" y="133" font-size="11" fill="#475569" text-anchor="middle">2 列を取り出す</text>
<rect x="445" y="44" width="125" height="104" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="507" y="66" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">④ ORDER BY</text>
<text x="507" y="92" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle">5 行</text>
<text x="507" y="116" font-size="11" fill="#475569" text-anchor="middle">更新日時の</text>
<text x="507" y="133" font-size="11" fill="#475569" text-anchor="middle">新しい順に並べる</text>
<rect x="590" y="44" width="120" height="104" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="650" y="66" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">⑤ LIMIT 2</text>
<text x="650" y="92" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle">2 行</text>
<text x="650" y="116" font-size="11" fill="#475569" text-anchor="middle">先頭の 2 行だけ</text>
<text x="650" y="133" font-size="11" fill="#475569" text-anchor="middle">返す</text>
<line x1="135" y1="96" x2="152" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-021)"/>
<line x1="280" y1="96" x2="297" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-021)"/>
<line x1="425" y1="96" x2="442" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-021)"/>
<line x1="570" y1="96" x2="587" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-021)"/>
<rect x="10" y="172" width="700" height="80" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="30" y="198" font-size="12" font-weight="700" fill="#0f172a">書く順番:</text>
<text x="110" y="198" font-size="12" fill="#475569">SELECT → FROM → WHERE → ORDER BY → LIMIT</text>
<text x="30" y="226" font-size="12" font-weight="700" fill="#0f172a">処理の順番:</text>
<text x="110" y="226" font-size="12" fill="#475569">FROM → WHERE → SELECT → ORDER BY → LIMIT（上の ①〜⑤）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: SELECT 文は、表の全部の行から始めて、条件で絞り、列を選び、並べて、先頭だけを返す</div>
</div>

処理の順番を知っておくと、「`LIMIT 2` は並べ替えた**後**の先頭 2 件」「`WHERE` で落とした行は、並べ替えの対象にもならない」ということが自然に分かります。

### まず動かす最小コード

この単元から UNIT 05 までは、RustNote と同じ形の表と、決まった練習用データを使います。次の2つのファイルを `sql/` に保存してください。

#### 準備: sql/schema.sql と sql/seed.sql

> **▼ このコードがやること（先に日本語で）:** `schema.sql` は RustNote の `users` 表と `notes` 表を作ります（UNIT 01 の例1-3 と同じ定義。索引は UNIT 04 で足します）。`seed.sql` は、会員3人とノート8冊の練習用データを入れます。日時は、結果が毎回同じになるよう決まった値にしています。

```sql
-- RustNote の users と notes（migrations/0001・0002 と同じ定義。索引は UNIT 04 で足す）
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    body TEXT NOT NULL DEFAULT '',
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)),
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

```sql
-- 練習用のデータ（日時は結果が毎回同じになるよう、決まった値を入れる）
INSERT INTO users (id, email, password_hash, created_at) VALUES
    (1, 'alice@example.com', 'dummy-a', '2026-09-30 10:00:00'),
    (2, 'bob@example.com', 'dummy-b', '2026-09-30 11:00:00'),
    (3, 'carol@example.com', 'dummy-c', '2026-09-30 12:00:00');

INSERT INTO notes (id, user_id, title, body, done, version, created_at, updated_at) VALUES
    (1, 1, '買い物リスト', '牛乳、卵、パン', 0, 1, '2026-10-01 09:00:00', '2026-10-01 09:00:00'),
    (2, 1, 'Rust の勉強', '所有権を復習する', 1, 3, '2026-10-02 20:15:00', '2026-10-05 21:00:00'),
    (3, 2, '旅行の計画', '京都で紅葉を見る', 0, 1, '2026-10-03 12:30:00', '2026-10-03 12:30:00'),
    (4, 1, '読書メモ', '第7巻を読む', 0, 2, '2026-10-04 07:45:00', '2026-10-06 08:00:00'),
    (5, 2, '買い物', '電池', 1, 2, '2026-10-05 18:00:00', '2026-10-07 19:30:00'),
    (6, 1, 'SQL の練習', 'WHERE と ORDER BY', 0, 1, '2026-10-06 22:10:00', '2026-10-06 22:10:00'),
    (7, 1, '100% 達成', '割引の % 記号に注意', 0, 1, '2026-10-07 10:00:00', '2026-10-07 10:00:00'),
    (8, 2, 'ジムの予定', '火曜と金曜', 0, 1, '2026-10-08 06:30:00', '2026-10-07 19:30:00');
```

- 1つの `INSERT` で、`VALUES (…), (…), (…)` と複数の行をまとめて入れられます。
- データの特徴: **Alice（1）が 5 冊、Bob（2）が 3 冊、Carol（3）は 0 冊**。Bob のノート 5 と 8 は `updated_at` が同じ `2026-10-07 19:30:00` です。ノート 7 のタイトルには `%` 記号が入っています。この3つは、後でわざと使います。

#### 例2-1: SELECT・WHERE・ORDER BY・LIMIT の基本

> **▼ このコードがやること（先に日本語で）:** ノートの3つの列だけを読み、次に Alice のノートだけに絞り、さらに「まだ終わっていないもの」に絞ります。続けて、更新日時の新しい順に並べ、先頭 2 件だけにし、最後に件数を数えます。

```sql
-- ① 必要な列だけを読む
SELECT id, user_id, title FROM notes;
-- ② WHERE で行を絞る（Alice = user_id 1 のノートだけ）
SELECT id, title, done FROM notes WHERE user_id = 1;
-- ③ 条件を AND でつなぐ（Alice の、まだ終わっていないノート）
SELECT id, title FROM notes WHERE user_id = 1 AND done = 0;
-- ④ ORDER BY で並べ替え（更新日時の新しい順）
SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC;
-- ⑤ LIMIT で件数を制限（新しい順に 2 件だけ）
SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC LIMIT 2;
-- ⑥ 数える
SELECT COUNT(*) AS alice_notes FROM notes WHERE user_id = 1;
```

`sql/u02_select.sql` として保存し、`--setup` で表とデータを先に用意してから実行します。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_select.sql
```

**▼ 実行結果**（①と②は省略。③から）

```text
（準備: sql/schema.sql の 2 文を実行しました）
（準備: sql/seed.sql の 2 文を実行しました）
sql> SELECT id, title FROM notes WHERE user_id = 1 AND done = 0
+----+--------------+
| id | title        |
+----+--------------+
| 1  | 買い物リスト |
| 4  | 読書メモ     |
| 6  | SQL の練習   |
| 7  | 100% 達成    |
+----+--------------+
(4 行)

sql> SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC
+----+--------------+---------------------+
| id | title        | updated_at          |
+----+--------------+---------------------+
| 7  | 100% 達成    | 2026-10-07 10:00:00 |
| 6  | SQL の練習   | 2026-10-06 22:10:00 |
| 4  | 読書メモ     | 2026-10-06 08:00:00 |
| 2  | Rust の勉強  | 2026-10-05 21:00:00 |
| 1  | 買い物リスト | 2026-10-01 09:00:00 |
+----+--------------+---------------------+
(5 行)

sql> SELECT id, title, updated_at FROM notes WHERE user_id = 1 ORDER BY updated_at DESC LIMIT 2
+----+------------+---------------------+
| id | title      | updated_at          |
+----+------------+---------------------+
| 7  | 100% 達成  | 2026-10-07 10:00:00 |
| 6  | SQL の練習 | 2026-10-06 22:10:00 |
+----+------------+---------------------+
(2 行)

sql> SELECT COUNT(*) AS alice_notes FROM notes WHERE user_id = 1
+-------------+
| alice_notes |
+-------------+
| 5           |
+-------------+
(1 行)
```

①は 8 行全部、②は Alice の 5 行が表示されます（自分で実行して確かめてください）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: SELECT は「列を選ぶ」

- `SELECT id, user_id, title FROM notes` は「`notes` 表から、`id`・`user_id`・`title` の3列を読む」です。
- UNIT 01 で使った `SELECT *` は「全部の列」でした。練習には便利ですが、アプリでは**必要な列だけを書く**のが基本です。表に列が増えたとき（UNIT 06 の migration）に、読む列が勝手に増えたり順番が変わったりしないからです。

##### 解説2: WHERE は「行を選ぶ」

- `WHERE user_id = 1` は「`user_id` が 1 の行だけ」。SQL の「等しい」は `=` が1つです（Rust の `==` ではありません）。「等しくない」は `<>` か `!=` です。
- `AND` は「両方とも満たす」、`OR` は「どちらかを満たす」です。両方を混ぜるときは、`user_id = 1 AND (done = 0 OR title = 'メモ')` のように**カッコで意味をはっきりさせます**。`AND` は `OR` より先に計算されるので、カッコが無いと意図と違う組み合わせになります。

##### 解説3: ORDER BY は「並べ方を決める」

- `ORDER BY updated_at DESC` は「`updated_at` の大きい順（新しい順）」。`DESC`（descending、降順）を省くと `ASC`（ascending、昇順＝小さい順）になります。
- `updated_at` は `'2026-10-07 10:00:00'` の形の**文字列**です。文字列は辞書の順に比べられますが、「年-月-日 時:分:秒」の形で桁数がそろっていれば、辞書順と時間の順が一致します。RustNote が日時をこの形でそろえているのは、そのためです。

##### 解説4: LIMIT と COUNT(*)

- `LIMIT 2` は「並べた結果の先頭 2 行だけ返す」。上の図のとおり、`ORDER BY` の後に効きます。
- `COUNT(*)` は「条件に合う行の数」を1つの値で返します。`AS alice_notes` は結果の列に名前を付ける書き方（別名）です。名前を付けないと、列名は `COUNT(*)` になります。
- `COUNT` のように、たくさんの行をまとめて1つの値にする関数を**集計関数**と呼びます。ほかに `SUM`（合計）・`MAX`（最大）・`MIN`（最小）・`AVG`（平均）があります。

### もう一歩: 実務に近い形

#### 例2-2: 並びを1通りに決める・ページ送り・部分一致検索

> **▼ このコードがやること（先に日本語で）:** 更新日時が同じ Bob の2冊を、新しい順に並べてみます。次に「同じ日時なら id の大きい順」と決めて、並びを1通りにします。その並べ方で Alice の 2 ページ目を読み、最後に `LIKE` で部分一致検索をします。`%` 記号そのものを探す方法も確かめます。

```sql
-- ① Bob のノートを新しい順に（5 と 8 は更新日時が同じ）
SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC;
-- ② 同じ日時なら id の大きい順、と決めておく（並びが必ず1通りに決まる）
SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC, id DESC;
-- ③ ページ送り: 1ページ 2 件で、Alice の 2 ページ目（3〜4 件目）
SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 2;
-- ④ LIKE で部分一致検索（タイトルか本文に「買い物」を含む）
SELECT id, user_id, title FROM notes WHERE title LIKE '%買い物%' OR body LIKE '%買い物%';
-- ⑤ 「%」を探したいのに、% は「何でもよい文字の並び」の意味になってしまう
SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%%%';
-- ⑥ ESCAPE で「\ の次の文字は、ただの文字」と決める
SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%\%%' ESCAPE '\';
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_more.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC
+----+------------+---------------------+
| id | title      | updated_at          |
+----+------------+---------------------+
| 5  | 買い物     | 2026-10-07 19:30:00 |
| 8  | ジムの予定 | 2026-10-07 19:30:00 |
| 3  | 旅行の計画 | 2026-10-03 12:30:00 |
+----+------------+---------------------+
(3 行)

sql> SELECT id, title, updated_at FROM notes WHERE user_id = 2 ORDER BY updated_at DESC, id DESC
+----+------------+---------------------+
| id | title      | updated_at          |
+----+------------+---------------------+
| 8  | ジムの予定 | 2026-10-07 19:30:00 |
| 5  | 買い物     | 2026-10-07 19:30:00 |
| 3  | 旅行の計画 | 2026-10-03 12:30:00 |
+----+------------+---------------------+
(3 行)

sql> SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 2
+----+-------------+
| id | title       |
+----+-------------+
| 4  | 読書メモ    |
| 2  | Rust の勉強 |
+----+-------------+
(2 行)

sql> SELECT id, user_id, title FROM notes WHERE title LIKE '%買い物%' OR body LIKE '%買い物%'
+----+---------+--------------+
| id | user_id | title        |
+----+---------+--------------+
| 1  | 1       | 買い物リスト |
| 5  | 2       | 買い物       |
+----+---------+--------------+
(2 行)

sql> SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%%%'
+----+--------------+
| id | title        |
+----+--------------+
| 1  | 買い物リスト |
| 2  | Rust の勉強  |
| 4  | 読書メモ     |
| 6  | SQL の練習   |
| 7  | 100% 達成    |
+----+--------------+
(5 行)

sql> SELECT id, title FROM notes WHERE user_id = 1 AND title LIKE '%\%%' ESCAPE '\'
+----+-----------+
| id | title     |
+----+-----------+
| 7  | 100% 達成 |
+----+-----------+
(1 行)
```

##### 解説5: 同点の行の順番は決まっていない

①では、更新日時が同じ 5 と 8 が「5 → 8」の順に出ました。しかし、これは**たまたま**です。`ORDER BY updated_at DESC` は「日時の新しい順」とは言っていますが、「日時が同じときはどうするか」は何も言っていません。同じ日時の行の順番は DB の都合で決まり、索引を作ったりデータが増えたりすると入れ替わることがあります。

②のように `ORDER BY updated_at DESC, id DESC` と、**最後に主キーを足しておく**と、同じ日時でも id で順番が決まり、並びが必ず1通りになります。この「同点を決めるための最後の列」を tie-breaker（タイブレーカー）と呼びます。

##### 解説6: LIMIT と OFFSET でページ送り

- `OFFSET 2` は「先頭から 2 行を飛ばす」。`LIMIT 2 OFFSET 2` で「3 件目と 4 件目」になります。
- 1ページの件数を `n`、ページ番号を `p`（1 から数える）とすると、`LIMIT n OFFSET (p - 1) * n` です。
- 並びが1通りに決まっていないと、ページ送りで**同じノートが2回出たり、1回も出なかったり**します。1ページ目と2ページ目は別々の SQL なので、その間に同点の行の順番が入れ替わると、境目の行がずれるからです。ページ送りには tie-breaker が必須です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-022" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="26" font-size="13" font-weight="700" fill="#0f172a">Alice のノートを ORDER BY updated_at DESC, id DESC で並べた結果</text>
<rect x="20" y="40" width="120" height="44" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="80" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">#7 100% 達成</text>
<text x="80" y="76" font-size="11" fill="#475569" text-anchor="middle">10-07 10:00</text>
<rect x="150" y="40" width="120" height="44" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="210" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">#6 SQL の練習</text>
<text x="210" y="76" font-size="11" fill="#475569" text-anchor="middle">10-06 22:10</text>
<rect x="290" y="40" width="120" height="44" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="350" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">#4 読書メモ</text>
<text x="350" y="76" font-size="11" fill="#475569" text-anchor="middle">10-06 08:00</text>
<rect x="420" y="40" width="120" height="44" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="480" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">#2 Rust の勉強</text>
<text x="480" y="76" font-size="11" fill="#475569" text-anchor="middle">10-05 21:00</text>
<rect x="560" y="40" width="140" height="44" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="630" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">#1 買い物リスト</text>
<text x="630" y="76" font-size="11" fill="#475569" text-anchor="middle">10-01 09:00</text>
<path d="M20,96 V104 H270 V96" fill="none" stroke="#3b82f6" stroke-width="2"/>
<path d="M290,96 V104 H540 V96" fill="none" stroke="#f97316" stroke-width="2"/>
<path d="M560,96 V104 H700 V96" fill="none" stroke="#10b981" stroke-width="2"/>
<text x="145" y="124" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">1 ページ目</text>
<text x="145" y="142" font-size="11" fill="#475569" text-anchor="middle">LIMIT 2 OFFSET 0</text>
<text x="415" y="124" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">2 ページ目</text>
<text x="415" y="142" font-size="11" fill="#475569" text-anchor="middle">LIMIT 2 OFFSET 2</text>
<text x="630" y="124" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">3 ページ目</text>
<text x="630" y="142" font-size="11" fill="#475569" text-anchor="middle">LIMIT 2 OFFSET 4</text>
<rect x="20" y="162" width="680" height="54" rx="8" fill="#fef2f2" stroke="#fca5a5"/>
<text x="360" y="184" font-size="12" fill="#b91c1c" text-anchor="middle">tie-breaker（最後の id DESC）が無いと、同じ日時の行の順番がページごとに入れ替わり、</text>
<text x="360" y="203" font-size="12" fill="#b91c1c" text-anchor="middle">境目のノートが 2 回出たり、どのページにも出なかったりする</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 1ページ 2 件のページ送り。OFFSET は「飛ばす件数」。並びが1通りに決まっていることが前提</div>
</div>

##### 解説7: LIKE の `%` と `_`、そして ESCAPE

- `LIKE` は「形が似ているか」で比べます。`%` は「何文字でもよい（0 文字も可）」、`_` は「ちょうど1文字」を表す**ワイルドカード**です。`'%買い物%'` は「どこかに『買い物』を含む」です。
- ⑤の `'%%%'` は「`%` を含む」のつもりでも、3つとも「何でもよい」の意味になり、**全部の行**に一致してしまいました。
- ⑥の `ESCAPE '\'` は「`\` の次の文字は、ワイルドカードではなく、ただの文字」という宣言です。`'%\%%'` は「何か・`%` という文字・何か」になり、ノート 7 だけが見つかります。
- 検索欄に利用者が `%` や `_` を入力することはよくあります。RustNote では、入力の `\`・`%`・`_` の前に `\` を付けてから `LIKE` に渡しています（後の「RustNote ではここで使う」で見ます）。

#### 例2-3: JOIN で2つの表をつなぐ

> **▼ このコードがやること（先に日本語で）:** ノートの横に、持ち主のメールアドレスを並べます。次に、会員ごとのノートの数を数えます。ノートが1冊も無い Carol も「0 冊」として出します。最後に、ノートが1冊も無い会員だけを探します。

```sql
-- ① JOIN: ノートと、その持ち主のメールアドレスを横に並べる
SELECT notes.id, users.email, notes.title
FROM notes
JOIN users ON users.id = notes.user_id
ORDER BY notes.id;
-- ② LEFT JOIN + GROUP BY: 会員ごとのノートの数（0 件の Carol も出す）
SELECT users.email, COUNT(notes.id) AS note_count
FROM users
LEFT JOIN notes ON notes.user_id = users.id
GROUP BY users.id
ORDER BY users.id;
```

```sql
-- ③ LEFT JOIN で、ノートが1件もない会員を探す（相手がいない行は NULL で埋まる）
SELECT users.id, users.email, notes.id AS note_id
FROM users
LEFT JOIN notes ON notes.user_id = users.id
WHERE notes.id IS NULL;
```

前の2文を `sql/u02_join.sql`、③を `sql/u02_null.sql` の先頭に保存して実行します（`u02_null.sql` の残りは「よくあるエラー」で使います）。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_join.sql
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_null.sql
```

**▼ 実行結果**（①は 8 行のうち先頭 3 行だけ載せます）

```text
sql> SELECT notes.id, users.email, notes.title …
+----+-------------------+--------------+
| id | email             | title        |
+----+-------------------+--------------+
| 1  | alice@example.com | 買い物リスト |
| 2  | alice@example.com | Rust の勉強  |
| 3  | bob@example.com   | 旅行の計画   |
（4〜8 行目は省略）
(8 行)

sql> SELECT users.email, COUNT(notes.id) AS note_count …
+-------------------+------------+
| email             | note_count |
+-------------------+------------+
| alice@example.com | 5          |
| bob@example.com   | 3          |
| carol@example.com | 0          |
+-------------------+------------+
(3 行)

sql> SELECT users.id, users.email, notes.id AS note_id …
+----+-------------------+---------+
| id | email             | note_id |
+----+-------------------+---------+
| 3  | carol@example.com | NULL    |
+----+-------------------+---------+
(1 行)
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-023" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="24" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">JOIN（内部結合）</text>
<text x="540" y="24" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">LEFT JOIN（左外部結合）</text>
<rect x="20" y="40" width="320" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="62" font-size="12" fill="#0f172a" text-anchor="middle">alice ─ ノート 1, 2, 4, 6, 7（5 行）</text>
<rect x="20" y="82" width="320" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="104" font-size="12" fill="#0f172a" text-anchor="middle">bob ─ ノート 3, 5, 8（3 行）</text>
<rect x="20" y="124" width="320" height="34" rx="6" fill="#f1f5f9" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="180" y="146" font-size="12" fill="#94a3b8" text-anchor="middle">carol ─ 相手がいないので消える</text>
<rect x="380" y="40" width="320" height="34" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="540" y="62" font-size="12" fill="#0f172a" text-anchor="middle">alice ─ ノート 1, 2, 4, 6, 7（5 行）</text>
<rect x="380" y="82" width="320" height="34" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="540" y="104" font-size="12" fill="#0f172a" text-anchor="middle">bob ─ ノート 3, 5, 8（3 行）</text>
<rect x="380" y="124" width="320" height="34" rx="6" fill="#fef9c3" stroke="#eab308"/>
<text x="540" y="146" font-size="12" fill="#0f172a" text-anchor="middle">carol ─ notes の列は全部 NULL（1 行）</text>
<text x="180" y="190" font-size="12" fill="#475569" text-anchor="middle">両方の表に相手がいる行だけ</text>
<text x="540" y="190" font-size="12" fill="#475569" text-anchor="middle">左の表（users）の行は必ず残る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: JOIN と LEFT JOIN の違い。ノートが無い Carol は、LEFT JOIN にしたときだけ結果に残る</div>
</div>

##### 解説8: JOIN は「関係をたどって横に並べる」

- `JOIN users ON users.id = notes.user_id` は「ノートの行ごとに、`user_id` と同じ `id` を持つ会員の行を探して、横につなぐ」です。UNIT 01 で作った外部キーの関係を、読むときにたどっているわけです。
- 2つの表に同じ名前の列（`id`）があるので、`notes.id`・`users.email` のように **表の名前. 列の名前** で書きます。
- `JOIN`（正式には `INNER JOIN`）は、**両方の表に相手がいる行だけ**を残します。ノートが無い Carol は、①の結果に出てきません。
- `LEFT JOIN` は、**左（`FROM` に書いた表）の行を必ず残し**、相手がいなければ右の表の列を `NULL` で埋めます。②で Carol が `0` と出たのは、`COUNT(notes.id)` が「`NULL` でない `notes.id` の数」を数えるからです。`COUNT(*)` にすると、`NULL` で埋まった行も 1 と数えてしまい、Carol が 1 冊になります。
- `GROUP BY users.id` は「会員ごとにグループにまとめて、グループごとに集計する」という指定です。

### よくあるエラーと読み方

```sql
-- ① 列名のつづり間違い
SELECT id, titel FROM notes;
-- ② 表名のつづり間違い
SELECT * FROM note;
-- ③ JOIN した2つの表に同じ名前の列（id）がある
SELECT id, email, title FROM notes JOIN users ON users.id = notes.user_id;
-- ④ NULL は = では比べられない（エラーにはならず、何も返らない）
SELECT NULL = NULL AS eq, NULL IS NULL AS is_null, 1 = NULL AS one_eq_null;
-- ⑤ 文字列を " で囲んでしまう（" は本来「列名」の印）
SELECT id, title FROM notes WHERE title = "title";
SELECT id, title FROM notes WHERE title = "買い物";
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_errors.sql
```

```text
sql> SELECT id, titel FROM notes
エラー: SQLite failure: `no such column: titel`

sql> SELECT * FROM note
エラー: SQLite failure: `no such table: note`

sql> SELECT id, email, title FROM notes JOIN users ON users.id = notes.user_id
エラー: SQLite failure: `ambiguous column name: id`

sql> SELECT NULL = NULL AS eq, NULL IS NULL AS is_null, 1 = NULL AS one_eq_null
+------+---------+-------------+
| eq   | is_null | one_eq_null |
+------+---------+-------------+
| NULL | 1       | NULL        |
+------+---------+-------------+
(1 行)

sql> SELECT id, title FROM notes WHERE title = "title"
+----+--------------+
| id | title        |
+----+--------------+
| 1  | 買い物リスト |
| 2  | Rust の勉強  |
（3〜8 行目は省略。全部の行が返る）
(8 行)

sql> SELECT id, title FROM notes WHERE title = "買い物"
+----+--------+
| id | title  |
+----+--------+
| 5  | 買い物 |
+----+--------+
(1 行)
```

| 番号 | メッセージ・結果 | 何が起きたか | 直し方 |
|---|---|---|---|
| ① | `no such column: titel` | そういう名前の列が無い | つづりを `title` に直す |
| ② | `no such table: note` | そういう名前の表が無い | `notes` に直す。表を作る前に実行していないかも確かめる |
| ③ | `ambiguous column name: id` | `id` が `notes` と `users` の両方にあり、どちらか決められない（ambiguous＝あいまい） | `notes.id` のように表の名前を付ける |
| ④ | エラーにならず `NULL` | `NULL` は「分からない値」なので、`=` で比べた結果も「分からない（NULL）」 | `IS NULL` / `IS NOT NULL` で比べる |
| ⑤ | エラーにならず 8 行 | `"title"` は**列名**として読まれ、`title = title`（いつも真）になった | 文字列は `'…'` で囲む |

- ④は、**エラーにならないのに結果がおかしくなる**こわい例です。`WHERE body = NULL` と書くと、本文が NULL の行があっても1行も返りません。`WHERE` は条件が「真（1）」の行だけを残し、「NULL」の行は落とすからです。
- ⑤もエラーになりません。SQL では `'…'` が文字列、`"…"` が列名や表名です。SQLite は、`"買い物"` という列が無いときだけ「昔の書き方との互換のため」文字列として扱うので、たまたま動いてしまいます。動いたり動かなかったりする書き方なので、**文字列は必ず `'…'`** と覚えてください。

#### つまずき: LIKE は英字の大文字・小文字を区別しない

```sql
-- ② LIKE は英字の大文字と小文字を区別しない
SELECT 'Rust' LIKE 'rust' AS ascii_like, 'Rust' = 'rust' AS ascii_eq;
-- ③ 小文字の「rust」で検索しても、タイトルの「Rust」が見つかる
SELECT id, title FROM notes WHERE title LIKE '%rust%';
```

`sql/u02_null.sql` の続きです（先ほどの実行結果の後半）。

```text
sql> SELECT 'Rust' LIKE 'rust' AS ascii_like, 'Rust' = 'rust' AS ascii_eq
+------------+----------+
| ascii_like | ascii_eq |
+------------+----------+
| 1          | 0        |
+------------+----------+
(1 行)

sql> SELECT id, title FROM notes WHERE title LIKE '%rust%'
+----+-------------+
| id | title       |
+----+-------------+
| 2  | Rust の勉強 |
+----+-------------+
(1 行)
```

- SQL では、真は `1`、偽は `0` で表示されます。`LIKE` は英字の大文字と小文字を区別せずに一致（1）しましたが、`=` は区別して不一致（0）でした。
- 検索欄では、区別しないほうが便利なことが多いので、RustNote はこの性質をそのまま使っています。区別しないのは **ASCII の英字（A〜Z）だけ**で、全角の英字やひらがな・カタカナの違いは区別されます。

### RustNote ではここで使う

RustNote のノート一覧（`GET /notes?q=キーワード&page=2`）は、この単元の SQL の組み合わせです。`Rust/rustnote/src/repository/notes.rs` の `search` から、SQL の部分を抜き出します。

```rust
// LIKE の中の \ を「次の文字をそのまま扱う」という意味にする（ESCAPE '\'）
let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";

// 1) 条件に合う件数（ページ数の計算に使う）
let count_sql = format!("SELECT COUNT(*) FROM notes WHERE {condition}");

// 2) このページに表示する分（新しく更新した順）
let list_sql = format!(
    "SELECT {NOTE_COLUMNS} FROM notes WHERE {condition}
     ORDER BY updated_at DESC, id DESC
     LIMIT ?3 OFFSET ?4"
);
```

- `user_id = ?1` が**持ち主の条件**です。ログイン中の人のノートだけを読みます。
- `?1`・`?2` は、後から値を当てはめる**場所取り**（プレースホルダー）です。キーワードを SQL の文字列に直接つなげないことで、SQL インジェクションを防ぎます（UNIT 07 で詳しく学びます）。
- `(title LIKE ?2 … OR body LIKE ?2 …)` のカッコに注目してください。カッコが無いと `user_id = ?1 AND title LIKE ?2 OR body LIKE ?2` となり、`AND` が先に計算されて「**他人のノートでも、本文が一致すれば出てしまう**」という事故になります。
- `ORDER BY updated_at DESC, id DESC` は例2-2 の tie-breaker つきの並べ方、`LIMIT ?3 OFFSET ?4` はページ送りです。1ページの件数は `PER_PAGE = 20` で決めています。
- キーワードは、次の関数で `\`・`%`・`_` の前に `\` を付けてから `%…%` で囲んでいます。

```rust
fn escape_like(keyword: &str) -> String {
    let mut escaped = String::with_capacity(keyword.len());
    for c in keyword.chars() {
        if matches!(c, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}
```

### 練習問題

**問題1:** Alice（user_id 1）のノートのうち、完了済み（`done = 1`）のものが何冊あるかを、1つの `SELECT` で求めてください。

**問題2:** Bob（user_id 2）のノートを、**作った日時の古い順**に並べてください。作った日時が同じでも並びが1通りに決まるようにします。

**問題3:** 1ページ 2 件で、Alice のノートの **3 ページ目**を読む SQL を書いてください（並べ方は例2-2 の③と同じ）。何件返るかも予想してください。

### 解答と解説

3つの解答を `sql/u02_q.sql` にまとめて実行します。

```sql
-- 解答1: Alice（1）の完了済み（done = 1）のノートの数
SELECT COUNT(*) AS done_notes FROM notes WHERE user_id = 1 AND done = 1;
-- 解答2: Bob（2）のノートを、作った順（古い順）に
SELECT id, title, created_at FROM notes WHERE user_id = 2 ORDER BY created_at ASC, id ASC;
-- 解答3: Alice の 3 ページ目（1 ページ 2 件）
SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 4;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_q.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> SELECT COUNT(*) AS done_notes FROM notes WHERE user_id = 1 AND done = 1
+------------+
| done_notes |
+------------+
| 1          |
+------------+
(1 行)

sql> SELECT id, title, created_at FROM notes WHERE user_id = 2 ORDER BY created_at ASC, id ASC
+----+------------+---------------------+
| id | title      | created_at          |
+----+------------+---------------------+
| 3  | 旅行の計画 | 2026-10-03 12:30:00 |
| 5  | 買い物     | 2026-10-05 18:00:00 |
| 8  | ジムの予定 | 2026-10-08 06:30:00 |
+----+------------+---------------------+
(3 行)

sql> SELECT id, title FROM notes WHERE user_id = 1 ORDER BY updated_at DESC, id DESC LIMIT 2 OFFSET 4
+----+--------------+
| id | title        |
+----+--------------+
| 1  | 買い物リスト |
+----+--------------+
(1 行)
```

- **解答1:** 条件は `user_id = 1 AND done = 1` の2つ。`COUNT(*)` で数えると 1 冊（ノート 2「Rust の勉強」）です。
- **解答2:** 古い順は `ASC` です。この練習データでは作った日時が重ならないので `id ASC` が無くても同じ結果になりますが、データが増えれば重なることがあります。**並べるときは、いつも最後に主キーを足す**習慣をつけましょう。
- **解答3:** 3 ページ目は `OFFSET (3 - 1) * 2 = 4`。Alice のノートは 5 冊なので、5 件目の1冊だけが返ります。範囲の外（たとえば `OFFSET 10`）を指定してもエラーにはならず、0 行が返ります。

### まとめ

- `SELECT 列 FROM 表 WHERE 条件 ORDER BY 列 LIMIT 件数` で、必要な行を、決まった順番で、少しずつ読み出せます。処理は FROM → WHERE → SELECT → ORDER BY → LIMIT の順です。
- 並べるときは最後に主キー（tie-breaker）を足し、並びを1通りに決めます。ページ送りの前提です。`LIKE` の `%`・`_` を文字として探すときは `ESCAPE` を使います。
- `NULL` は `IS NULL` で比べ、文字列は `'…'` で囲みます。どちらもエラーにならずに結果だけがおかしくなるので、注意が必要です。
