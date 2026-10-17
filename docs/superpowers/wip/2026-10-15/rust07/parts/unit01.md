## UNIT 01: 関係モデルと CREATE TABLE

### この単元のゴール

- 表（テーブル）・行・列・主キー・外部キーが何を表しているかを、RustNote の `users` と `notes` を例に説明できる
- `CREATE TABLE` で、列の型と制約（`NOT NULL`・`UNIQUE`・`CHECK`・`DEFAULT`・`REFERENCES`）を書ける
- 制約に違反したときのエラーメッセージを読んで、「どの約束を破ったのか」を言える
- SQLite（libSQL）の型が「ゆるい」ことと `STRICT`、外部キーの確認を切り替える `PRAGMA foreign_keys` の注意点を知る

### たとえ話でつかむ

スポーツクラブの受付を思い浮かべてください。受付には2冊の帳面があります。

- **会員名簿:** 1行に1人ずつ、「会員番号・メールアドレス・入会日」を書きます。会員番号は1人に1つだけで、ほかの人と重なりません。
- **ノート台帳:** 1行に1冊ずつ、「ノート番号・持ち主の会員番号・タイトル・完了したか」を書きます。

ノート台帳の「持ち主の会員番号」の欄には、**会員名簿に実在する番号しか書けない**決まりにします。そうしておけば、「会員番号 99 番のノート」のような、持ち主のいないノートは生まれません。また、受付係は記入のたびに「タイトルが空欄になっていないか」「同じメールアドレスの人がもう名簿にいないか」をチェックします。

データベース（DB）の表は、この帳面とまったく同じ考え方でできています。

| 帳面の言葉 | DB の言葉 | RustNote での例 |
|---|---|---|
| 帳面1冊 | 表（テーブル） | `users`、`notes` |
| 1人分・1冊分の記入欄 | 行（ロー、レコード） | Alice の1行、「買い物リスト」の1行 |
| 記入欄の項目 | 列（カラム） | `email`、`title`、`done` |
| 会員番号 | 主キー（PRIMARY KEY） | `users.id`、`notes.id` |
| 持ち主の会員番号 | 外部キー（FOREIGN KEY） | `notes.user_id` → `users.id` |
| 受付係のチェック | 制約（CONSTRAINT） | `NOT NULL`、`UNIQUE`、`CHECK` |

> **用語:** SQL（エスキューエル：DB に「表を作れ」「この条件の行を読め」と頼むための言葉。Rust のようなプログラミング言語とは別の、DB 専用の言葉）

<!-- -->

> **用語:** 主キー（しゅキー、PRIMARY KEY：表の中で行を1つに決めるための列。同じ値の行は2つ存在できない）

<!-- -->

> **用語:** 外部キー（がいぶキー、FOREIGN KEY：ほかの表の主キーを指す列。指している先が実在しない値は入れられない）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-011" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="165" y="28" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">users（会員の表）</text>
<text x="550" y="28" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">notes（ノートの表）</text>
<text x="165" y="58" font-size="12" fill="#475569" text-anchor="middle">↓ 列（カラム）＝記入欄の項目</text>
<text x="550" y="58" font-size="12" fill="#475569" text-anchor="middle">↓ 列（カラム）</text>
<rect x="20" y="70" width="40" height="30" fill="#fde68a" stroke="#64748b"/>
<rect x="60" y="70" width="150" height="30" fill="#e2e8f0" stroke="#64748b"/>
<rect x="210" y="70" width="100" height="30" fill="#e2e8f0" stroke="#64748b"/>
<text x="40" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">id</text>
<text x="135" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">email</text>
<text x="260" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">password_hash</text>
<rect x="20" y="100" width="40" height="30" fill="#fef9c3" stroke="#64748b"/>
<rect x="60" y="100" width="150" height="30" fill="#ffffff" stroke="#64748b"/>
<rect x="210" y="100" width="100" height="30" fill="#ffffff" stroke="#64748b"/>
<text x="40" y="120" font-size="12" fill="#0f172a" text-anchor="middle">1</text>
<text x="135" y="120" font-size="12" fill="#0f172a" text-anchor="middle">alice@example.com</text>
<text x="260" y="120" font-size="12" fill="#0f172a" text-anchor="middle">dummy-a</text>
<rect x="20" y="130" width="40" height="30" fill="#fef9c3" stroke="#64748b"/>
<rect x="60" y="130" width="150" height="30" fill="#ffffff" stroke="#64748b"/>
<rect x="210" y="130" width="100" height="30" fill="#ffffff" stroke="#64748b"/>
<text x="40" y="150" font-size="12" fill="#0f172a" text-anchor="middle">2</text>
<text x="135" y="150" font-size="12" fill="#0f172a" text-anchor="middle">bob@example.com</text>
<text x="260" y="150" font-size="12" fill="#0f172a" text-anchor="middle">dummy-b</text>
<text x="358" y="120" font-size="12" fill="#475569" text-anchor="middle">← 1行＝</text>
<text x="358" y="138" font-size="12" fill="#475569" text-anchor="middle">1人分</text>
<rect x="410" y="70" width="40" height="30" fill="#fde68a" stroke="#64748b"/>
<rect x="450" y="70" width="60" height="30" fill="#fed7aa" stroke="#64748b"/>
<rect x="510" y="70" width="130" height="30" fill="#e2e8f0" stroke="#64748b"/>
<rect x="640" y="70" width="50" height="30" fill="#e2e8f0" stroke="#64748b"/>
<text x="430" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">id</text>
<text x="480" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">user_id</text>
<text x="575" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">title</text>
<text x="665" y="90" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">done</text>
<rect x="410" y="100" width="40" height="30" fill="#fef9c3" stroke="#64748b"/>
<rect x="450" y="100" width="60" height="30" fill="#ffedd5" stroke="#64748b"/>
<rect x="510" y="100" width="130" height="30" fill="#ffffff" stroke="#64748b"/>
<rect x="640" y="100" width="50" height="30" fill="#ffffff" stroke="#64748b"/>
<text x="430" y="120" font-size="12" fill="#0f172a" text-anchor="middle">1</text>
<text x="480" y="120" font-size="12" fill="#0f172a" text-anchor="middle">1</text>
<text x="575" y="120" font-size="12" fill="#0f172a" text-anchor="middle">買い物リスト</text>
<text x="665" y="120" font-size="12" fill="#0f172a" text-anchor="middle">0</text>
<rect x="410" y="130" width="40" height="30" fill="#fef9c3" stroke="#64748b"/>
<rect x="450" y="130" width="60" height="30" fill="#ffedd5" stroke="#64748b"/>
<rect x="510" y="130" width="130" height="30" fill="#ffffff" stroke="#64748b"/>
<rect x="640" y="130" width="50" height="30" fill="#ffffff" stroke="#64748b"/>
<text x="430" y="150" font-size="12" fill="#0f172a" text-anchor="middle">2</text>
<text x="480" y="150" font-size="12" fill="#0f172a" text-anchor="middle">2</text>
<text x="575" y="150" font-size="12" fill="#0f172a" text-anchor="middle">旅行の計画</text>
<text x="665" y="150" font-size="12" fill="#0f172a" text-anchor="middle">0</text>
<path d="M480,162 V200 H40 V164" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-07-011)"/>
<text x="260" y="194" font-size="12" fill="#c2410c" text-anchor="middle">外部キー: user_id には users.id に実在する番号だけが入る</text>
<rect x="20" y="226" width="22" height="16" fill="#fef9c3" stroke="#64748b"/>
<text x="50" y="239" font-size="12" fill="#0f172a">主キー（id）: 行を1つに決める番号。表の中で重ならない</text>
<rect x="20" y="254" width="22" height="16" fill="#ffedd5" stroke="#64748b"/>
<text x="50" y="267" font-size="12" fill="#0f172a">外部キー（user_id）: ほかの表の主キーを指す列。「このノートの持ち主は誰か」を表す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 2つの表と、それをつなぐ外部キー。ノートの行は user_id で持ち主の行を指す</div>
</div>

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-012" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="180" height="44" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="57" font-size="13" fill="#0f172a" text-anchor="middle">フォームからの入力</text>
<rect x="20" y="103" width="180" height="44" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="130" font-size="13" fill="#0f172a" text-anchor="middle">手作業で打った SQL</text>
<rect x="20" y="176" width="180" height="44" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="203" font-size="13" fill="#0f172a" text-anchor="middle">バグのある別のコード</text>
<rect x="255" y="22" width="190" height="60" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="350" y="47" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">アプリの門番</text>
<text x="350" y="68" font-size="12" fill="#1e40af" text-anchor="middle">NoteTitle::parse など</text>
<line x1="200" y1="52" x2="252" y2="52" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-012)"/>
<path d="M445,52 H600 V82" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-012)"/>
<text x="520" y="44" font-size="12" fill="#047857" text-anchor="middle">検査済みの値</text>
<line x1="200" y1="125" x2="497" y2="125" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-07-012)"/>
<path d="M200,198 H600 V188" fill="none" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-07-012)"/>
<text x="350" y="117" font-size="12" fill="#c2410c" text-anchor="middle">門番を通らない道もある</text>
<rect x="500" y="85" width="200" height="100" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="112" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">DB の制約（最後の守り）</text>
<text x="600" y="136" font-size="12" fill="#0f172a" text-anchor="middle">NOT NULL・UNIQUE</text>
<text x="600" y="156" font-size="12" fill="#0f172a" text-anchor="middle">CHECK・外部キー</text>
<text x="600" y="232" font-size="12" fill="#c2410c" text-anchor="middle">違反した行は1行も保存されない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: アプリの検証が最初の守り、DB の制約が最後の守り。どの道から来ても、制約は必ず確認される</div>
</div>

上の図のポイントは、**制約は DB の中にある**ということです。第4巻で作った `NoteTitle::parse` はアプリの門番ですが、門番を通らずに DB へ届く道（管理者が手で打った SQL、別のプログラム、門番を呼び忘れたバグ）は必ずあります。表に `CHECK` や `UNIQUE` を書いておけば、どの道から来た値でも DB が最後に確認してくれます。アプリの検証と DB の制約は「どちらか一方」ではなく、**両方置く**のが基本です。

### まず動かす最小コード

この巻の SQL は、巻頭で用意した練習用プロジェクト `sql-lab` の道具 `sql` で実行します。SQL ファイルを1文ずつ実行して、結果を表の形で表示してくれる道具です。`--db` を付けないときは「メモリの中だけの DB」を使うので、**実行のたびにまっさらな DB から始まります**。何度実行しても同じ結果になるので、安心して試してください。

#### 例1-1: users と notes の表を作り、行を入れる

> **▼ このコードがやること（先に日本語で）:** 会員の表 `users` とノートの表 `notes` を作ります。`notes` の `user_id` は `users` の `id` を指す外部キーにします。会員を2人、ノートを2冊入れてから、2つの表の中身を表示します。

```sql
-- 会員（ユーザー）の表
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ノートの表。user_id で「どの会員のノートか」を指す
CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    body TEXT NOT NULL DEFAULT '',
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1))
);

-- 行を入れてみる（id は書かなくても 1, 2, … と自動で決まる）
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a');
INSERT INTO users (email, password_hash) VALUES ('bob@example.com', 'dummy-b');
INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト');
INSERT INTO notes (user_id, title, body) VALUES (2, '旅行の計画', '京都で紅葉を見る');

-- 中身を確かめる
SELECT id, email, password_hash FROM users;
SELECT * FROM notes;
```

`sql/u01_create.sql` として保存し、`sql-lab` フォルダで次のコマンドを実行します。

```powershell
cargo run -q --bin sql -- sql/u01_create.sql
```

**▼ 実行結果**

```text
sql> CREATE TABLE users ( …
OK

sql> CREATE TABLE notes ( …
OK

sql> INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-a')
OK（変更: 1 行）

sql> INSERT INTO users (email, password_hash) VALUES ('bob@example.com', 'dummy-b')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title, body) VALUES (2, '旅行の計画', '京都で紅葉を見る')
OK（変更: 1 行）

sql> SELECT id, email, password_hash FROM users
+----+-------------------+---------------+
| id | email             | password_hash |
+----+-------------------+---------------+
| 1  | alice@example.com | dummy-a       |
| 2  | bob@example.com   | dummy-b       |
+----+-------------------+---------------+
(2 行)

sql> SELECT * FROM notes
+----+---------+--------------+------------------+------+
| id | user_id | title        | body             | done |
+----+---------+--------------+------------------+------+
| 1  | 1       | 買い物リスト |                  | 0    |
| 2  | 2       | 旅行の計画   | 京都で紅葉を見る | 0    |
+----+---------+--------------+------------------+------+
(2 行)
```

`sql>` の後ろが実行した文（2行以上ある文は1行目だけを表示し、`…` を付けています）、その下が結果です。`-q` は「cargo 自身の `Compiling …` などの表示を出さない」という指定です。

#### ▼ コードを1つずつ分解して解説

##### 解説1: CREATE TABLE の形

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    ...
);
```

- `CREATE TABLE 表の名前 ( … );` で表を作ります。カッコの中に、列を**カンマ区切り**で並べます。最後の列の後ろにはカンマを付けません。
- 1つの列は「**列名 型 制約…**」の順に書きます。`email TEXT NOT NULL UNIQUE` なら「列名は email、型は文字列（TEXT）、空っぽ禁止（NOT NULL）、重複禁止（UNIQUE）」です。
- SQL のキーワード（`CREATE`、`TABLE` など）は大文字でも小文字でも同じ意味です。本書では読みやすさのため、キーワードを大文字、表や列の名前を小文字で書きます。
- 文の終わりには `;`（セミコロン）を付けます。

##### 解説2: 主キー `INTEGER PRIMARY KEY`

- `id INTEGER PRIMARY KEY` は「`id` 列が主キー」という意味です。主キーの値は表の中で重なりません。
- SQLite（libSQL）では、`INTEGER PRIMARY KEY` の列に値を書かずに `INSERT` すると、**今ある最大の値 + 1** が自動で入ります。だから `INSERT INTO users (email, password_hash) …` で id を書かなくても、1, 2 と番号が付きました。
- 主キーには、メールアドレスのような「変わるかもしれない値」ではなく、意味を持たない番号を使うのが定番です。メールアドレスは引っ越しのように変わることがありますが、番号は一生変わりません。

##### 解説3: `NOT NULL`・`UNIQUE`・`DEFAULT`

- `NOT NULL` は「**値が無い状態（NULL）を許さない**」制約です。NULL（ヌル）は SQL で「値が無い」を表す特別な印で、Rust の `None` に近いものです。
- `UNIQUE` は「**同じ値の行を2つ作らせない**」制約です。`email` に付けたので、同じメールアドレスで2回登録できません。
- `DEFAULT (datetime('now'))` は「`INSERT` で値を書かなかったら、この値を入れる」という指定です。`datetime('now')` は今の日時を `'2026-10-17 21:35:21'` のような文字列で返す SQLite の関数で、時刻は **UTC**（世界標準時。日本時間より9時間遅い）です。
- `body TEXT NOT NULL DEFAULT ''` は「本文は空っぽ禁止。ただし書かなければ空の文字列 `''` が入る」という組み合わせです。結果の表で `買い物リスト` の body が空欄に見えるのは、`''` が入っているからです。

> **用語:** NULL（ヌル：SQL で「値が無い」ことを表す特別な印。0 や空文字 `''` とは別物）

##### 解説4: `CHECK` で値の範囲を決める

```sql
title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1))
```

- `CHECK (条件)` は「**この条件を満たす値しか入れさせない**」制約です。
- `length(title)` はタイトルの**文字数**です。`BETWEEN 1 AND 80` は「1 以上 80 以下」。第4巻の `NoteTitle::parse`（1〜80文字）と同じ約束を、DB 側にも書いています。
- `done IN (0, 1)` は「0 か 1 のどちらか」。SQLite には真偽値（`bool`）専用の型が無いので、**0 = 未完了、1 = 完了**という整数で表します。

##### 解説5: `REFERENCES` で外部キーを作る

```sql
user_id INTEGER NOT NULL REFERENCES users(id),
```

- `REFERENCES users(id)` は「この列の値は、`users` 表の `id` 列に実在する値でなければならない」という意味です。これが外部キーです。
- 「ノート 2 の持ち主は会員 2」のように、表と表の**関係**を表します。DB を「関係データベース（リレーショナルデータベース）」と呼ぶのは、この関係が中心にあるからです。

##### 解説6: INSERT と SELECT

```sql
INSERT INTO notes (user_id, title) VALUES (1, '買い物リスト');
SELECT * FROM notes;
```

- `INSERT INTO 表 (列, 列…) VALUES (値, 値…)` で行を1つ追加します。列と値は**同じ順番・同じ数**で書きます。書かなかった列（`id`・`body`・`done`）には自動の番号や `DEFAULT` の値が入ります。
- 文字列の値は `'…'`（シングルクォート）で囲みます。`"…"` ではありません（理由は UNIT 02 のエラーで見ます）。
- `SELECT * FROM notes` は「`notes` の全部の列を読む」。`*` は「全部の列」の意味です。SELECT は UNIT 02 で詳しく学びます。

### もう一歩: 実務に近い形

#### 例1-2: SQLite の型は「ゆるい」— typeof と STRICT

> **▼ このコードがやること（先に日本語で）:** いろいろな値の種類を `typeof()` で調べ、文字数とバイト数の違いを確かめます。次に、普通の表と `STRICT` を付けた表に、整数の列なのに文字列を入れてみて、結果を比べます。

```sql
-- 値の種類（ストレージクラス）を typeof() で調べる
SELECT typeof(42) AS a, typeof(3.14) AS b, typeof('こんにちは') AS c, typeof(x'CAFE') AS d, typeof(NULL) AS e;
-- 文字数とバイト数
SELECT length('あいう') AS chars, length(CAST('あいう' AS BLOB)) AS bytes;
-- 普通の表は、列の型と違う値も（変換できなければ）そのまま入ってしまう
CREATE TABLE loose (n INTEGER);
INSERT INTO loose VALUES ('12'), ('abc');
SELECT n, typeof(n) FROM loose;
-- STRICT を付けた表は、型の違う値を断る
CREATE TABLE strict_t (n INTEGER) STRICT;
INSERT INTO strict_t VALUES ('12');
INSERT INTO strict_t VALUES ('abc');
SELECT n, typeof(n) FROM strict_t;
```

```powershell
cargo run -q --bin sql -- sql/u01_types.sql
```

**▼ 実行結果**

```text
sql> SELECT typeof(42) AS a, typeof(3.14) AS b, typeof('こんにちは') AS c, typeof(x'CAFE') AS d, typeof(NULL) AS e
+---------+------+------+------+------+
| a       | b    | c    | d    | e    |
+---------+------+------+------+------+
| integer | real | text | blob | null |
+---------+------+------+------+------+
(1 行)

sql> SELECT length('あいう') AS chars, length(CAST('あいう' AS BLOB)) AS bytes
+-------+-------+
| chars | bytes |
+-------+-------+
| 3     | 9     |
+-------+-------+
(1 行)

sql> CREATE TABLE loose (n INTEGER)
OK

sql> INSERT INTO loose VALUES ('12'), ('abc')
OK（変更: 2 行）

sql> SELECT n, typeof(n) FROM loose
+-----+-----------+
| n   | typeof(n) |
+-----+-----------+
| 12  | integer   |
| abc | text      |
+-----+-----------+
(2 行)

sql> CREATE TABLE strict_t (n INTEGER) STRICT
OK

sql> INSERT INTO strict_t VALUES ('12')
OK（変更: 1 行）

sql> INSERT INTO strict_t VALUES ('abc')
エラー: SQLite failure: `cannot store TEXT value in INTEGER column strict_t.n`

sql> SELECT n, typeof(n) FROM strict_t
+----+-----------+
| n  | typeof(n) |
+----+-----------+
| 12 | integer   |
+----+-----------+
(1 行)
```

##### 解説7: 値の種類は5つだけ

SQLite（libSQL）が値をしまうときの種類（**ストレージクラス**）は、次の5つだけです。Rust の型との対応も覚えておくと、UNIT 07 で Rust から読み書きするときに迷いません。

| ストレージクラス | 中身 | 例 | Rust で受け取る型 |
|---|---|---|---|
| `integer` | 整数 | `42` | `i64` |
| `real` | 小数 | `3.14` | `f64` |
| `text` | 文字列（UTF-8） | `'こんにちは'` | `String` |
| `blob` | バイトの並び | `x'CAFE'` | `Vec<u8>` |
| `null` | 値が無い | `NULL` | `Option<…>` の `None` |

- 日付や真偽値の専用の型はありません。RustNote では、日時は `'2026-10-17 21:35:21'` の形の **text**、完了フラグは 0/1 の **integer** で持ちます。
- `length('あいう')` は 3（文字数）ですが、`BLOB` に変換してから数えると 9（バイト数）です。日本語は UTF-8 で1文字3バイトだからです。`CHECK (length(title) BETWEEN 1 AND 80)` は**文字数**で数えるので、Rust 側の `chars().count()` と数え方がそろっています。

##### 解説8: 普通の表は「ゆるい」、STRICT は厳しい

- 普通の表では、列の型は「できればこの型にしてね」という**希望**にすぎません。`'12'` は整数に変換できたので `integer` で入りましたが、`'abc'` は変換できないので、**整数の列に文字列のまま入ってしまいました**。これを型の親和性（アフィニティ）と呼びます。
- 表の最後に `STRICT` を付けると、型の違う値は `cannot store TEXT value in INTEGER column strict_t.n`（INTEGER の列 strict_t.n に TEXT の値はしまえません）というエラーで断られます。
- RustNote の表には `STRICT` を付けていません。値は必ず Rust の型（`i64` や `String`）を通ってから入るので、型の取り違えは Rust のコンパイラが先に防いでくれるからです。ただし、手作業の SQL も含めて型を守りたい表を新しく作るなら、`STRICT` は検討する価値があります。

#### 例1-3: RustNote の本物の表を見る

例1-1 は練習用に削った形でした。RustNote の本物の `notes` 表（`Rust/rustnote/migrations/0002_notes.sql`）は次のとおりです。

```sql
-- 0002: ノートのテーブル
-- title は 1〜80 文字（length() は文字数を数える。Rust 側の chars().count() と同じ）。
-- done は 0（未完了）か 1（完了）。
-- version は楽観的ロック用の番号。更新のたびに 1 増える。
-- 日時は UTC の 'YYYY-MM-DD HH:MM:SS' 形式の文字列（datetime('now') の形）。
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

-- 「自分のノートを新しい順に」を速くするための索引
CREATE INDEX idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC);
```

例1-1 との違いは次の5つです。それぞれ、この巻の後の単元で学びます。

| 違い | 意味 | 学ぶ単元 |
|---|---|---|
| `AUTOINCREMENT` | 消したノートの番号を、二度と使い回さない | この単元（例1-4） |
| `ON DELETE CASCADE` | 会員を消したら、その人のノートも一緒に消す | UNIT 03 |
| `version` | 同時に編集されたことに気づくための番号 | UNIT 05 |
| `created_at` / `updated_at` | 作った日時と、最後に変えた日時 | UNIT 02・03 |
| `CREATE INDEX` | 検索を速くする索引 | UNIT 04 |

RustNote の3つの表（`0001_users.sql`・`0002_notes.sql`・`0003_sessions.sql`）を作ったあと、DB の中に何があるかを一覧にしてみます。`--setup` で指定したファイルは、結果を表示せずに先に実行されます。

```sql
-- DB の中にある表と索引の一覧（sqlite_schema は SQLite が自動で持っている表）
SELECT type, name, tbl_name FROM sqlite_schema ORDER BY tbl_name, type DESC, name;
```

```powershell
cargo run -q --bin sql -- --setup migrations/0001_users.sql --setup migrations/0002_notes.sql --setup migrations/0003_sessions.sql sql/u01_schema_list.sql
```

**▼ 実行結果**

```text
（準備: migrations/0001_users.sql の 1 文を実行しました）
（準備: migrations/0002_notes.sql の 2 文を実行しました）
（準備: migrations/0003_sessions.sql の 3 文を実行しました）
sql> SELECT type, name, tbl_name FROM sqlite_schema ORDER BY tbl_name, type DESC, name
+-------+-----------------------------+-----------------+
| type  | name                        | tbl_name        |
+-------+-----------------------------+-----------------+
| table | notes                       | notes           |
| index | idx_notes_user_updated      | notes           |
| table | sessions                    | sessions        |
| index | idx_sessions_expires        | sessions        |
| index | idx_sessions_user           | sessions        |
| index | sqlite_autoindex_sessions_1 | sessions        |
| table | sqlite_sequence             | sqlite_sequence |
| table | users                       | users           |
| index | sqlite_autoindex_users_1    | users           |
+-------+-----------------------------+-----------------+
(9 行)
```

##### 解説9: 自分で作っていない名前が混ざっている

- `sqlite_schema` は、DB が「どんな表や索引があるか」を自分で記録している特別な表です。DB の中身を調べたいときの入口になります。
- `sqlite_autoindex_users_1` は、`email` の `UNIQUE` を確かめるために SQLite が**自動で作った索引**です。重複を素早く見つけるには索引が必要だからです。`sessions` の `token_hash BLOB PRIMARY KEY`（整数でない主キー）にも同じものが作られています。
- `sqlite_sequence` は、`AUTOINCREMENT` を付けた表の「これまでに使った最大の番号」を覚えておくための表です。次の例1-4 で中身を見ます。

#### 例1-4: AUTOINCREMENT は番号を使い回さない

> **▼ このコードがやること（先に日本語で）:** `AUTOINCREMENT` なしの表と、ありの表に3行ずつ入れ、いちばん大きい番号（3）の行を消します。そのあと1行ずつ足して、新しい行に何番が付くかを比べます。

```sql
-- AUTOINCREMENT なしと、ありの表を作る
CREATE TABLE plain (id INTEGER PRIMARY KEY, v TEXT NOT NULL);
CREATE TABLE auto (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT NOT NULL);
-- どちらにも3行入れてから、いちばん大きい id（3）の行を消す
INSERT INTO plain (v) VALUES ('a'), ('b'), ('c');
INSERT INTO auto (v) VALUES ('a'), ('b'), ('c');
DELETE FROM plain WHERE id = 3;
DELETE FROM auto WHERE id = 3;
-- もう1行ずつ入れると、新しい id はいくつになるか
INSERT INTO plain (v) VALUES ('d');
INSERT INTO auto (v) VALUES ('d');
SELECT 'plain' AS tbl, id, v FROM plain WHERE v = 'd'
UNION ALL
SELECT 'auto', id, v FROM auto WHERE v = 'd';
-- AUTOINCREMENT の表は「これまでの最大の id」を sqlite_sequence に覚えている
SELECT name, seq FROM sqlite_sequence;
```

```powershell
cargo run -q --bin sql -- sql/u01_autoinc.sql
```

**▼ 実行結果**（前半の `OK` の行は省略）

```text
sql> SELECT 'plain' AS tbl, id, v FROM plain WHERE v = 'd' …
+-------+----+---+
| tbl   | id | v |
+-------+----+---+
| plain | 3  | d |
| auto  | 4  | d |
+-------+----+---+
(2 行)

sql> SELECT name, seq FROM sqlite_sequence
+------+-----+
| name | seq |
+------+-----+
| auto | 4   |
+------+-----+
(1 行)
```

##### 解説10: なぜ RustNote は AUTOINCREMENT を付けるのか

- `AUTOINCREMENT` なしの `plain` では、消した番号 3 が**新しい行に使い回されました**（「今ある最大の値 + 1」だからです）。`AUTOINCREMENT` ありの `auto` は、`sqlite_sequence` に覚えた最大値 3 の次の 4 を使いました。
- ノートの番号は URL（`/notes/3`）に出ます。もし番号を使い回すと、「ブックマークしていた `/notes/3` を開いたら、消したはずのノートではなく、新しく作った別のノートが出た」ということが起こりえます。RustNote は、それを避けるために `AUTOINCREMENT` を付けています。
- `UNION ALL` は「2つの SELECT の結果を縦につなぐ」書き方です。ここでは比較を見やすくするためだけに使っています。

### よくあるエラーと読み方

例1-1 の表に、わざと約束を破る行を入れてみます。`--setup sql/u01_create.sql` で例1-1 の表と4行を先に用意してから、6つの文を順に実行します。

```sql
-- ① UNIQUE 違反: 同じメールアドレスで2人目を登録する
INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-x');
-- ② NOT NULL 違反: password_hash を書き忘れる
INSERT INTO users (email) VALUES ('carol@example.com');
-- ③ CHECK 違反: 空のタイトル
INSERT INTO notes (user_id, title) VALUES (1, '');
-- ④ CHECK 違反: done に 0 と 1 以外
INSERT INTO notes (user_id, title, done) VALUES (1, 'テスト', 2);
-- ⑤ FOREIGN KEY 違反: 存在しない会員（id = 99）のノート
INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート');
-- ⑥ 構文エラー: 列名の間のカンマを忘れた
INSERT INTO notes (user_id title) VALUES (1, 'カンマ忘れ');
-- 失敗した文は、1行も追加していない
SELECT (SELECT COUNT(*) FROM users) AS users_count, (SELECT COUNT(*) FROM notes) AS notes_count;
```

```powershell
cargo run -q --bin sql -- --setup sql/u01_create.sql sql/u01_errors.sql
```

```text
（準備: sql/u01_create.sql の 8 文を実行しました）
sql> INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'dummy-x')
エラー: SQLite failure: `UNIQUE constraint failed: users.email`

sql> INSERT INTO users (email) VALUES ('carol@example.com')
エラー: SQLite failure: `NOT NULL constraint failed: users.password_hash`

sql> INSERT INTO notes (user_id, title) VALUES (1, '')
エラー: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`

sql> INSERT INTO notes (user_id, title, done) VALUES (1, 'テスト', 2)
エラー: SQLite failure: `CHECK constraint failed: done IN (0, 1)`

sql> INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート')
エラー: SQLite failure: `FOREIGN KEY constraint failed`

sql> INSERT INTO notes (user_id title) VALUES (1, 'カンマ忘れ')
エラー: SQLite failure: `near "title": syntax error`

sql> SELECT (SELECT COUNT(*) FROM users) AS users_count, (SELECT COUNT(*) FROM notes) AS notes_count
+-------------+-------------+
| users_count | notes_count |
+-------------+-------------+
| 2           | 2           |
+-------------+-------------+
(1 行)
```

まず、メッセージの形を読みましょう。`SQLite failure:` は Rust の libsql クレートが付ける前置きで、**バッククォート（`` ` ``）の中が DB 本体のメッセージ**です。読むのは中身だけで十分です。最後の `SELECT` で、失敗した文は1行も追加していない（会員もノートも2のまま）ことも確かめられます。

| 番号 | メッセージ | 何が起きたか | 直し方 |
|---|---|---|---|
| ① | `UNIQUE constraint failed: users.email` | `users.email` に同じ値がもうある | 別のメールにする。アプリでは「登録済みです」と知らせる |
| ② | `NOT NULL constraint failed: users.password_hash` | 値を書かなかった列に `DEFAULT` が無く、NULL になった | `INSERT` の列と値に `password_hash` を足す |
| ③ | `CHECK constraint failed: length(title) BETWEEN 1 AND 80` | 空のタイトルが文字数の条件に合わない | 1〜80 文字のタイトルにする |
| ④ | `CHECK constraint failed: done IN (0, 1)` | done に 0・1 以外の値 | 0 か 1 にする |
| ⑤ | `FOREIGN KEY constraint failed` | `users` に id 99 の会員がいない | 実在する会員の id を使う |
| ⑥ | `near "title": syntax error` | `title` の手前で SQL の文法が崩れた | `(user_id, title)` とカンマを足す |

- `CHECK` のエラーには、破った条件の式がそのまま出ます。どの列の、どの約束かがすぐ分かります。
- ⑤ の外部キーのエラーだけは、**どの列が原因かを教えてくれません**。外部キーが複数ある表では、`INSERT` の値を1つずつ確かめる必要があります。
- ⑥ の `near "title"` は「`title` の**あたり**でおかしくなった」という意味です。本当の原因は `title` の直前（カンマ忘れ）にあることが多いので、示された場所の**少し前**も見てください。

#### エラー7: table … already exists（同じ表を2回作った）

```sql
-- 同じ名前の表を2回作ろうとする
CREATE TABLE memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL);
CREATE TABLE memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL);
-- IF NOT EXISTS を付けると「無ければ作る。あれば何もしない」になる
CREATE TABLE IF NOT EXISTS memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL);
```

```powershell
cargo run -q --bin sql -- sql/u01_ifnot.sql
```

```text
sql> CREATE TABLE memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)
OK

sql> CREATE TABLE memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)
エラー: SQLite failure: `table memos already exists`

sql> CREATE TABLE IF NOT EXISTS memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)
OK
```

- **何が起きたか:** ファイルに保存する DB（`--db ファイル名` を付けたとき）で同じ SQL を2回実行すると、2回目で必ずこのエラーになります。
- **直し方:** 練習なら `IF NOT EXISTS` を付けます。ただし、`IF NOT EXISTS` は「**同じ名前の表があれば、中身が違っていても何もしない**」点に注意してください。列を足した新しい定義に書き換えても、古い表がそのまま残ります。本番の表の作り直しや列の追加は、UNIT 06 の migration（マイグレーション）で、順番と記録を付けて行います。

#### つまずき: 外部キーの確認を切ると、壊れたデータが入る

外部キーの確認は、`PRAGMA foreign_keys` という設定で ON / OFF できます。わざと OFF にすると、どうなるでしょうか。

```sql
-- 外部キーの確認を切ってしまうと……
PRAGMA foreign_keys = OFF;
INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート');
SELECT id, user_id, title FROM notes;
-- 「指している先が無い行」を探す
PRAGMA foreign_key_check;
-- 元に戻す（この接続では、ここから先また確認される）
PRAGMA foreign_keys = ON;
PRAGMA foreign_keys;
```

```powershell
cargo run -q --bin sql -- --setup sql/u01_create.sql sql/u01_fk_off.sql
```

```text
（準備: sql/u01_create.sql の 8 文を実行しました）
sql> PRAGMA foreign_keys = OFF
OK

sql> INSERT INTO notes (user_id, title) VALUES (99, '持ち主のいないノート')
OK（変更: 1 行）

sql> SELECT id, user_id, title FROM notes
+----+---------+----------------------+
| id | user_id | title                |
+----+---------+----------------------+
| 1  | 1       | 買い物リスト         |
| 2  | 2       | 旅行の計画           |
| 3  | 99      | 持ち主のいないノート |
+----+---------+----------------------+
(3 行)

sql> PRAGMA foreign_key_check
+-------+-------+--------+------+
| table | rowid | parent | fkid |
+-------+-------+--------+------+
| notes | 3     | users  | 0    |
+-------+-------+--------+------+
(1 行)

sql> PRAGMA foreign_keys = ON
OK

sql> PRAGMA foreign_keys
+--------------+
| foreign_keys |
+--------------+
| 1            |
+--------------+
(1 行)
```

- OFF の間は、エラー⑤ で断られた「会員 99 のノート」が**すんなり入ってしまいました**。一度入った壊れた行は、ON に戻しても自動では消えません。
- `PRAGMA foreign_key_check` は、「指している先が無い行」を探して一覧にする命令です。`notes` 表の rowid（行番号）3 が、`users` に実在しない会員を指している、と分かります。
- **ここが落とし穴です。** 本書で使う libsql クレート 0.10 のローカル DB では、何も設定しなくても最初から ON でした（`PRAGMA foreign_keys` だけを実行すると `1` が返ります）。しかし、SQLite 本来の既定は **OFF** で、接続ごとに `PRAGMA foreign_keys = ON` を実行しないと外部キーは確認されません。DB を開く道具や設定によって ON / OFF が変わるので、「たぶん ON だろう」と思い込まないことが大切です。

### RustNote ではここで使う

RustNote は、DB に接続するたびに外部キーの確認を明示的に ON にしています（`Rust/rustnote/src/db.rs` の一部）。上の落とし穴のとおり、既定値に頼らないためです。

```rust
// SQLite は接続ごとに設定が必要:
// foreign_keys … REFERENCES（外部キー）を守らせる
// busy_timeout … ほかの接続が書き込み中なら最大 5 秒待つ
conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;")
    .await?;
```

また、DB の制約とアプリ側の型は、次のように**同じ約束を二重に**守っています。

| 列 | DB の制約 | アプリ側（Rust）の守り |
|---|---|---|
| `users.email` | `NOT NULL UNIQUE` | `Email::parse` で形式を確かめ、小文字にそろえる。`UNIQUE` 違反は「登録済み」（409）として画面に出す |
| `notes.user_id` | `NOT NULL REFERENCES users(id)` | `UserId` 型。ログイン中の人の番号しか渡さない |
| `notes.title` | `CHECK (length(title) BETWEEN 1 AND 80)` | `NoteTitle::parse`（前後の空白を除いて 1〜`MAX_TITLE_CHARS`（80）文字） |
| `notes.done` | `CHECK (done IN (0, 1))` | Rust では `bool`。読むときに `0` 以外を `true` にする |

RustNote のリポジトリ（`src/error.rs`）は、DB のエラーメッセージに `UNIQUE constraint failed` が含まれているかを調べて、「登録済みのメールアドレス」というエラーに分類しています。この文言はローカルの SQLite でも Turso でも同じなので、エラー①で見たメッセージが、そのまま分類の手がかりになっているわけです。

### 練習問題

**問題1:** ノートに付ける「タグ」の表 `tags` を `CREATE TABLE` で作ってください。列は `id`（主キー）、`user_id`（`users` を指す。会員が消えたらタグも消える）、`name`（1〜20 文字）、`created_at`（作った日時。自動で入る）です。さらに「**同じ人が同じ名前のタグを2つ作れない。ただし、別の人なら同じ名前で作れる**」ようにしてください。作ったら、例1-1 の表を `--setup` で用意して、Alice と Bob がそれぞれ「仕事」タグを作れること、Alice がもう1つ「仕事」タグを作れないことを確かめます。

**問題2:** 例1-1 の表に、次の4つの `INSERT` を実行します。それぞれ成功するか、失敗するならどの制約に引っかかるかを、**実行する前に**予想してください。

```sql
-- (a) 本文に NULL をはっきり入れる
INSERT INTO notes (user_id, title, body) VALUES (2, 'メモ', NULL);
-- (b) タイトルが空白3つだけ
INSERT INTO notes (user_id, title) VALUES (1, '   ');
-- (c) 会員 3 のノート
INSERT INTO notes (user_id, title) VALUES (3, '宿題');
-- (d) done に 1
INSERT INTO notes (user_id, title, done) VALUES (2, '洗濯', 1);
```

**問題3:** 同僚が「DB に `CHECK (length(title) BETWEEN 1 AND 80)` があるのだから、Rust の `NoteTitle::parse` は要らないのでは？」と言っています。問題2 の結果も参考に、両方が必要な理由を説明してください。

### 解答と解説

**解答1:**

```sql
-- 解答1: タグの表
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 20),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (user_id, name)
);
-- 確かめる
INSERT INTO tags (user_id, name) VALUES (1, '仕事');
INSERT INTO tags (user_id, name) VALUES (2, '仕事');
INSERT INTO tags (user_id, name) VALUES (1, '仕事');
INSERT INTO tags (user_id, name) VALUES (1, '');
SELECT id, user_id, name FROM tags;
```

```powershell
cargo run -q --bin sql -- --setup sql/u01_create.sql sql/u01_q1.sql
```

**▼ 実行結果**

```text
（準備: sql/u01_create.sql の 8 文を実行しました）
sql> CREATE TABLE tags ( …
OK

sql> INSERT INTO tags (user_id, name) VALUES (1, '仕事')
OK（変更: 1 行）

sql> INSERT INTO tags (user_id, name) VALUES (2, '仕事')
OK（変更: 1 行）

sql> INSERT INTO tags (user_id, name) VALUES (1, '仕事')
エラー: SQLite failure: `UNIQUE constraint failed: tags.user_id, tags.name`

sql> INSERT INTO tags (user_id, name) VALUES (1, '')
エラー: SQLite failure: `CHECK constraint failed: length(name) BETWEEN 1 AND 20`

sql> SELECT id, user_id, name FROM tags
+----+---------+------+
| id | user_id | name |
+----+---------+------+
| 1  | 1       | 仕事 |
| 2  | 2       | 仕事 |
+----+---------+------+
(2 行)
```

- ポイントは最後の `UNIQUE (user_id, name)` です。列の後ろに `UNIQUE` を書くと「その列だけで重複禁止」ですが、表の終わりに `UNIQUE (列, 列)` と書くと「**列の組み合わせ**で重複禁止」になります。`name UNIQUE` にしてしまうと、Bob が「仕事」タグを作れなくなります。
- エラーメッセージにも `tags.user_id, tags.name` と、組み合わせの列が両方出ています。
- 「会員が消えたらタグも消える」は `ON DELETE CASCADE` です（UNIT 03 で動きを確かめます）。

**解答2:** 予想を書いてから、次のコマンドで確かめます。

```powershell
cargo run -q --bin sql -- --setup sql/u01_create.sql sql/u01_q2.sql
```

**▼ 実行結果**（確認用に、最後に `'[' || title || ']'` でタイトルを `[ ]` で囲んで表示する SELECT を足しています。`||` は文字列をつなぐ演算子です）

```text
（準備: sql/u01_create.sql の 8 文を実行しました）
sql> INSERT INTO notes (user_id, title, body) VALUES (2, 'メモ', NULL)
エラー: SQLite failure: `NOT NULL constraint failed: notes.body`

sql> INSERT INTO notes (user_id, title) VALUES (1, '   ')
OK（変更: 1 行）

sql> INSERT INTO notes (user_id, title) VALUES (3, '宿題')
エラー: SQLite failure: `FOREIGN KEY constraint failed`

sql> INSERT INTO notes (user_id, title, done) VALUES (2, '洗濯', 1)
OK（変更: 1 行）

sql> SELECT id, user_id, '[' || title || ']' AS title, done FROM notes
+----+---------+----------------+------+
| id | user_id | title          | done |
+----+---------+----------------+------+
| 1  | 1       | [買い物リスト] | 0    |
| 2  | 2       | [旅行の計画]   | 0    |
| 3  | 1       | [   ]          | 0    |
| 4  | 2       | [洗濯]         | 1    |
+----+---------+----------------+------+
(4 行)
```

- (a) は失敗します。`DEFAULT ''` が使われるのは「**列を書かなかったとき**」だけです。`NULL` とはっきり書くと「値が無い」を入れようとしたことになり、`NOT NULL` に断られます。
- (b) は**成功してしまいます**。空白も1文字に数えるので、3文字のタイトルとして `CHECK` を通ります。
- (c) は失敗します。例1-1 の会員は id 1 と 2 だけなので、外部キーの確認に引っかかります。
- (d) は成功します。

**解答3:** 両方が必要な理由は、守る役割が違うからです。

- **DB の制約は「最後の守り」:** アプリを通らない道（手作業の SQL、別のプログラム、門番を呼び忘れたバグ）から来た値も止めます。ただし、問題2 (b) のように「空白だけ」は止められませんし、エラーメッセージも利用者向けではありません。
- **`NoteTitle::parse` は「最初の守り」:** 前後の空白を取り除いてから数えるので、空白だけのタイトルを「空です」と正しく断れます。また、「タイトルを入力してください」のような分かりやすい理由をフォームに表示できます。DB まで行かずに断れるので、無駄な通信も減ります。

### まとめ

- 表は「同じ形の行の集まり」です。主キーで行を1つに決め、外部キーで表と表の関係を表します。
- `NOT NULL`・`UNIQUE`・`CHECK`・`REFERENCES` は DB の中の「最後の守り」です。アプリの検証（`NoteTitle::parse` など）と**両方**置きます。
- SQLite の型はゆるく、外部キーの確認は設定で ON / OFF が変わります。思い込まずに、`typeof()` や `PRAGMA foreign_keys` で確かめましょう。
