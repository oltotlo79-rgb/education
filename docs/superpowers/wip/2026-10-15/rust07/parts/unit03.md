## UNIT 03: INSERT・UPDATE・DELETE と持ち主の条件

### この単元のゴール

- `INSERT`・`UPDATE`・`DELETE` で行を追加・変更・削除でき、`RETURNING` で変わった後の行を受け取れる
- 「変更した行数」を見て、成功したのか、対象が無かったのかを判断できる
- `UPDATE` と `DELETE` の `WHERE` に**持ち主の条件**（`AND user_id = ?`）を入れて、他人のノートを変更できないようにできる
- `ON DELETE CASCADE` の動きと、`WHERE` を書き忘れたときの怖さを知る

### たとえ話でつかむ

銀行の貸金庫を思い浮かべてください。金庫には番号が付いていますが、**番号を言っただけでは開けてもらえません**。窓口の人は「その金庫の持ち主が、あなた本人か」を必ず確かめます。番号と本人確認の**両方**がそろって、初めて金庫が開きます。

Web アプリのノートも同じです。RustNote の編集画面の URL は `/notes/3/edit` のように、ノートの番号が丸見えです。いたずら好きの人が URL の 3 を 4 や 5 に書き換えて送ってくることは、**必ず起こる**と考えてください。そのとき SQL が「番号 3 のノートを書き換える」だけだと、他人のノートが書き換わってしまいます。「番号 3 で、**しかも持ち主がログイン中の人**のノートを書き換える」と書けば、他人のノートは対象になりません。

もう1つ大事なのが、窓口でもらう**受付票**です。「何件の手続きをしたか」が書いてあります。SQL を実行すると、DB は「変更した行数」を返してくれます。これを見れば、「本当に書き換わったのか」「対象が見つからなかったのか」が分かります。

> **用語:** 認可（にんか、authorization：ログインした人が「その操作をしてよいか」を確かめること。「誰か」を確かめる認証（authentication）とは別のもの。第8巻で詳しく学ぶ）

<!-- -->

> **用語:** 影響行数（えいきょうぎょうすう：`INSERT`・`UPDATE`・`DELETE` が実際に追加・変更・削除した行の数。道具 `sql` では `OK（変更: 1 行）` と表示される）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-031" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="100" width="150" height="80" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="126" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Alice（user_id 1）</text>
<text x="85" y="148" font-size="11" fill="#475569" text-anchor="middle">URL を書き換えて送る</text>
<text x="85" y="166" font-size="11" fill="#475569" text-anchor="middle">POST /notes/3/edit</text>
<rect x="215" y="20" width="270" height="110" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="350" y="44" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">NG: 番号だけで探す</text>
<text x="350" y="68" font-size="12" fill="#0f172a" text-anchor="middle">UPDATE notes SET title = '乗っ取り'</text>
<text x="350" y="86" font-size="12" fill="#0f172a" text-anchor="middle">WHERE id = 3</text>
<text x="350" y="114" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">変更: 1 行（Bob のノートが変わる）</text>
<rect x="215" y="155" width="270" height="120" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="350" y="179" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">OK: 持ち主の条件を足す</text>
<text x="350" y="203" font-size="12" fill="#0f172a" text-anchor="middle">UPDATE notes SET title = '乗っ取り'</text>
<text x="350" y="221" font-size="12" fill="#0f172a" text-anchor="middle">WHERE id = 3 AND user_id = 1</text>
<text x="350" y="249" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">変更: 0 行（何も変わらない）</text>
<line x1="160" y1="125" x2="211" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-031)"/>
<line x1="160" y1="155" x2="211" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-031)"/>
<rect x="530" y="20" width="180" height="110" rx="10" fill="#ffffff" stroke="#cbd5e1"/>
<text x="620" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">notes の id 3</text>
<text x="620" y="68" font-size="12" fill="#0f172a" text-anchor="middle">持ち主: Bob（2）</text>
<text x="620" y="90" font-size="12" fill="#b91c1c" text-anchor="middle">タイトル: 乗っ取り</text>
<text x="620" y="112" font-size="11" fill="#475569" text-anchor="middle">他人に書き換えられた</text>
<rect x="530" y="155" width="180" height="120" rx="10" fill="#ffffff" stroke="#cbd5e1"/>
<text x="620" y="179" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリの判断</text>
<text x="620" y="203" font-size="12" fill="#0f172a" text-anchor="middle">変更 0 行</text>
<text x="620" y="223" font-size="12" fill="#0f172a" text-anchor="middle">＝ 対象が無い</text>
<text x="620" y="249" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">404 を返す</text>
<line x1="485" y1="75" x2="526" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-031)"/>
<line x1="485" y1="215" x2="526" y2="215" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-031)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 持ち主の条件が無いと、URL の番号を書き換えるだけで他人のノートが変わる。条件を足すと変更は 0 行になり、アプリは「見つからない」として扱える</div>
</div>

上の図の OK のほうでは、「Bob のノート 3 は存在するが、Alice のものではない」場合も、「ノート 3 がそもそも存在しない」場合も、同じ「変更 0 行」になります。アプリはどちらも **404（見つかりません）** として返します。「他人のノートです（403）」と答えると、「番号 3 のノートは存在する」という情報を相手に教えてしまうからです。

### まず動かす最小コード

#### 例3-1: 追加・変更・完了の切り替え・削除

> **▼ このコードがやること（先に日本語で）:** Alice のノートを1冊追加し、`RETURNING` で付いた番号を受け取ります。そのノートのタイトルを変え、完了と未完了を切り替え、最後に削除します。変更のたびに `version` を 1 ずつ増やし、各段階で中身を確かめます。

```sql
-- ① INSERT: Alice（1）のノートを追加し、RETURNING で入った行を返してもらう
INSERT INTO notes (user_id, title, body) VALUES (1, '牛乳を買う', '帰りにスーパーへ')
RETURNING id, title, done, version;
-- ② UPDATE: ノート 9 のタイトルを変え、version を 1 増やす
UPDATE notes SET title = '牛乳と卵を買う', version = version + 1
WHERE id = 9 AND user_id = 1;
SELECT id, title, done, version FROM notes WHERE id = 9;
-- ③ 完了の切り替え: done を 0 ⇔ 1 に反転する
UPDATE notes SET done = 1 - done, version = version + 1
WHERE id = 9 AND user_id = 1;
SELECT id, title, done, version FROM notes WHERE id = 9;
-- ④ DELETE: ノート 9 を消す
DELETE FROM notes WHERE id = 9 AND user_id = 1;
SELECT COUNT(*) AS found FROM notes WHERE id = 9;
```

`sql/u03_write.sql` として保存し、UNIT 02 と同じ表とデータを用意してから実行します。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_write.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> INSERT INTO notes (user_id, title, body) VALUES (1, '牛乳を買う', '帰りにスーパーへ') …
+----+------------+------+---------+
| id | title      | done | version |
+----+------------+------+---------+
| 9  | 牛乳を買う | 0    | 1       |
+----+------------+------+---------+
(1 行)

sql> UPDATE notes SET title = '牛乳と卵を買う', version = version + 1 …
OK（変更: 1 行）

sql> SELECT id, title, done, version FROM notes WHERE id = 9
+----+----------------+------+---------+
| id | title          | done | version |
+----+----------------+------+---------+
| 9  | 牛乳と卵を買う | 0    | 2       |
+----+----------------+------+---------+
(1 行)

sql> UPDATE notes SET done = 1 - done, version = version + 1 …
OK（変更: 1 行）

sql> SELECT id, title, done, version FROM notes WHERE id = 9
+----+----------------+------+---------+
| id | title          | done | version |
+----+----------------+------+---------+
| 9  | 牛乳と卵を買う | 1    | 3       |
+----+----------------+------+---------+
(1 行)

sql> DELETE FROM notes WHERE id = 9 AND user_id = 1
OK（変更: 1 行）

sql> SELECT COUNT(*) AS found FROM notes WHERE id = 9
+-------+
| found |
+-------+
| 0     |
+-------+
(1 行)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: INSERT と RETURNING

- `INSERT INTO 表 (列…) VALUES (値…)` は UNIT 01 で学んだ形です。書かなかった列には `DEFAULT` の値が入ります（`done` は 0、`version` は 1、日時は今の UTC）。
- `RETURNING 列…` を後ろに付けると、**追加された行を、そのまま結果として返してくれます**。自動で付いた `id`（9）や、`DEFAULT` で入った値を、もう一度 `SELECT` しなくても受け取れます。
- `RETURNING` は `UPDATE` と `DELETE` にも付けられます。この「1回の往復で、変更と結果の受け取りを済ませる」形を、RustNote はたくさん使っています。

##### 解説2: UPDATE の形

```sql
UPDATE notes SET title = '牛乳と卵を買う', version = version + 1
WHERE id = 9 AND user_id = 1;
```

- `UPDATE 表 SET 列 = 新しい値, 列 = 新しい値 WHERE 条件` です。`SET` の後ろに、変える列をカンマ区切りで並べます。
- `version = version + 1` の右側の `version` は「**変える前の値**」です。1 だったので 2 になりました。Rust の `version += 1` と同じ意味です。
- `WHERE` に当てはまった行**全部**が変わります。今回は id で1行に決まるので「変更: 1 行」でした。

##### 解説3: done = 1 - done で切り替える

- `done` は 0 か 1 です。`1 - 0 = 1`、`1 - 1 = 0` なので、`done = 1 - done` は「0 なら 1 に、1 なら 0 に」反転します。
- 「今の値を読んでから、反対の値を書く」と2回に分けると、その間にほかの人が切り替えたときに食い違います。**1つの `UPDATE` の中で計算する**と、DB が1回でまとめて処理してくれるので安全です。

##### 解説4: DELETE の形

- `DELETE FROM 表 WHERE 条件` で、条件に当てはまる行を全部消します。消した行は元に戻せません（戻す方法は UNIT 05 の transaction と UNIT 10 の backup で学びます）。
- 最後の `SELECT COUNT(*)` が 0 なので、ノート 9 が消えたことが分かります。

> **ヒント:** 例3-1 の4つの文は、どれも `WHERE` に `AND user_id = 1` が付いています。これが、この単元でいちばん大事な「持ち主の条件」です。次の例で、付けないとどうなるかを見ます。

### もう一歩: 実務に近い形

#### 例3-2: 持ち主の条件が無いと、他人のノートが変わる

> **▼ このコードがやること（先に日本語で）:** Alice が URL を書き換えて、Bob のノート 3 を狙ったときの SQL を再現します。番号だけで探すと Bob のノートが書き換わること、持ち主の条件を足すと「変更 0 行」で何も起きないことを比べます。

```sql
-- Alice（user_id = 1）が、URL の id を 3 に書き換えて Bob のノートを狙う
-- NG: id だけで探すと、他人のノートでも書き換えられてしまう
UPDATE notes SET title = '乗っ取り' WHERE id = 3;
SELECT id, user_id, title FROM notes WHERE id = 3;
-- （元に戻しておく）
UPDATE notes SET title = '旅行の計画' WHERE id = 3;
-- OK: 「id が 3」かつ「持ち主がログイン中の人（1）」の行だけを対象にする
UPDATE notes SET title = '乗っ取り' WHERE id = 3 AND user_id = 1;
DELETE FROM notes WHERE id = 3 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 3 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 3;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_owner.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> UPDATE notes SET title = '乗っ取り' WHERE id = 3
OK（変更: 1 行）

sql> SELECT id, user_id, title FROM notes WHERE id = 3
+----+---------+----------+
| id | user_id | title    |
+----+---------+----------+
| 3  | 2       | 乗っ取り |
+----+---------+----------+
(1 行)

sql> UPDATE notes SET title = '旅行の計画' WHERE id = 3
OK（変更: 1 行）

sql> UPDATE notes SET title = '乗っ取り' WHERE id = 3 AND user_id = 1
OK（変更: 0 行）

sql> DELETE FROM notes WHERE id = 3 AND user_id = 1
OK（変更: 0 行）

sql> SELECT id, user_id, title FROM notes WHERE id = 3 AND user_id = 1
+----+---------+-------+
| id | user_id | title |
+----+---------+-------+
(0 行)

sql> SELECT id, user_id, title FROM notes WHERE id = 3
+----+---------+------------+
| id | user_id | title      |
+----+---------+------------+
| 3  | 2       | 旅行の計画 |
+----+---------+------------+
(1 行)
```

##### 解説5: 「変更 0 行」はエラーではない

- 持ち主の条件つきの `UPDATE` と `DELETE` は、**エラーにならずに「変更: 0 行」で終わりました**。SQL としては正しい文で、「条件に合う行が1つも無かった」だけだからです。
- つまり、アプリが影響行数を見なければ、「保存しました」と表示してしまいます。**影響行数が 0 なら「対象が見つからない」として 404 を返す**、ここまでやって初めて正しい処理になります。
- 最後の `SELECT` で、Bob のノート 3 は無事に元のままだと確かめられます。
- 持ち主の条件は `UPDATE` と `DELETE` だけでなく、**1件を表示する `SELECT` にも**付けます（`WHERE id = ? AND user_id = ?`）。表示の段階で他人のノートを見せてしまえば、それだけで情報漏れです。

#### 例3-3: 会員を消すと、ノートはどうなるか（ON DELETE CASCADE）

> **▼ このコードがやること（先に日本語で）:** 会員ごとのノートの数を確かめてから、Bob を会員の表から削除します。`notes.user_id` に `ON DELETE CASCADE` が付いているので、Bob のノートも一緒に消えることを確かめます。比べるために、`ON DELETE` を付けていない UNIT 01 の表でも、会員を消してみます。

```sql
-- 会員ごとのノートの数
SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id;
-- Bob（2）が退会する
DELETE FROM users WHERE id = 2;
-- ON DELETE CASCADE なので、Bob のノートも一緒に消えている
SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id;
```

```sql
-- ON DELETE の指定が無い表で、ノートを持っている会員を消そうとする
DELETE FROM users WHERE id = 1;
```

前者を `sql/u03_cascade.sql`、後者を `sql/u03_fk_delete.sql` として保存します。後者は、UNIT 01 の例1-1 の表（`ON DELETE` の指定なし）で試します。

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_cascade.sql
cargo run -q --bin sql -- --setup sql/u01_create.sql sql/u03_fk_delete.sql
```

**▼ 実行結果**（準備の行は省略）

```text
sql> SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id
+---------+-------+
| user_id | notes |
+---------+-------+
| 1       | 5     |
| 2       | 3     |
+---------+-------+
(2 行)

sql> DELETE FROM users WHERE id = 2
OK（変更: 1 行）

sql> SELECT user_id, COUNT(*) AS notes FROM notes GROUP BY user_id
+---------+-------+
| user_id | notes |
+---------+-------+
| 1       | 5     |
+---------+-------+
(1 行)
```

```text
sql> DELETE FROM users WHERE id = 1
エラー: SQLite failure: `FOREIGN KEY constraint failed`
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-032" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ON DELETE CASCADE あり（RustNote）</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">ON DELETE の指定なし（UNIT 01 の練習の表）</text>
<rect x="20" y="40" width="320" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="180" y="65" font-size="12" fill="#0f172a" text-anchor="middle">DELETE FROM users WHERE id = 2</text>
<line x1="180" y1="80" x2="180" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-032)"/>
<rect x="20" y="106" width="320" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="126" font-size="12" fill="#0f172a" text-anchor="middle">Bob の行を消す</text>
<text x="180" y="143" font-size="12" fill="#047857" text-anchor="middle">→ Bob のノート 3 冊も自動で消える</text>
<rect x="20" y="164" width="320" height="50" rx="8" fill="#ffffff" stroke="#cbd5e1"/>
<text x="180" y="186" font-size="11" fill="#475569" text-anchor="middle">「持ち主のいないノート」は残らない</text>
<text x="180" y="204" font-size="11" fill="#475569" text-anchor="middle">（消したデータは戻らないので注意）</text>
<rect x="380" y="40" width="320" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="540" y="65" font-size="12" fill="#0f172a" text-anchor="middle">DELETE FROM users WHERE id = 1</text>
<line x1="540" y1="80" x2="540" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-032)"/>
<rect x="380" y="106" width="320" height="44" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="540" y="126" font-size="12" fill="#0f172a" text-anchor="middle">ノートが指している会員は消せない</text>
<text x="540" y="143" font-size="12" fill="#b91c1c" text-anchor="middle">→ FOREIGN KEY constraint failed</text>
<rect x="380" y="164" width="320" height="50" rx="8" fill="#ffffff" stroke="#cbd5e1"/>
<text x="540" y="186" font-size="11" fill="#475569" text-anchor="middle">先にノートを消してから、会員を消す</text>
<text x="540" y="204" font-size="11" fill="#475569" text-anchor="middle">（うっかり消すのを防げる）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 指している先の行を消したときの動き。CASCADE は一緒に消し、指定なしは削除そのものを断る</div>
</div>

##### 解説6: どちらを選ぶか

- `ON DELETE CASCADE`（カスケード＝滝のように連鎖する）は、**親の行（会員）を消すと、それを指している子の行（ノート）も自動で消す**指定です。RustNote では「退会したら、その人のノートとログイン中のセッションも全部消す」ので、`notes` と `sessions` の `user_id` に付けています。
- 指定が無いと、子の行が残っている親の行は消せません。外部キーの約束（指している先は実在する）を守るためです。「うっかり大事なデータを連鎖で消してしまう」のを防げるので、こちらが向いている表もあります。
- どちらにするかは、**「親が消えたとき、子は意味を持つか」**で決めます。持ち主のいないノートには意味が無いので、RustNote は CASCADE を選んでいます。

#### 例3-4: 消す前に、同じ条件で SELECT する

> **▼ このコードがやること（先に日本語で）:** Bob の完了済みのノートを消す前に、同じ `WHERE` で `SELECT` して、対象が何行かを確かめます。予想どおり1行だったので `DELETE` し、残りを確かめます。

```sql
-- 消す前に、同じ WHERE で SELECT して「何行が対象か」を確かめる
SELECT id, title FROM notes WHERE user_id = 2 AND done = 1;
DELETE FROM notes WHERE user_id = 2 AND done = 1;
SELECT id, title, done FROM notes WHERE user_id = 2;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_preview.sql
```

**▼ 実行結果**（準備の2行は省略）

```text
sql> SELECT id, title FROM notes WHERE user_id = 2 AND done = 1
+----+--------+
| id | title  |
+----+--------+
| 5  | 買い物 |
+----+--------+
(1 行)

sql> DELETE FROM notes WHERE user_id = 2 AND done = 1
OK（変更: 1 行）

sql> SELECT id, title, done FROM notes WHERE user_id = 2
+----+------------+------+
| id | title      | done |
+----+------------+------+
| 3  | 旅行の計画 | 0    |
| 8  | ジムの予定 | 0    |
+----+------------+------+
(2 行)
```

手作業で本番の DB を直すときは、この**「同じ WHERE で SELECT → 件数を確かめる → UPDATE / DELETE」**の順番を必ず守ってください。`SELECT` の件数と、`DELETE` の「変更: 1 行」が一致したことで、狙いどおりの行だけを消せたと確かめられます。

### よくあるエラーと読み方

```sql
-- ① UPDATE でも CHECK は守られる: タイトルを空にする
UPDATE notes SET title = '' WHERE id = 1 AND user_id = 1;
-- ② NOT NULL: 本文を NULL にする
UPDATE notes SET body = NULL WHERE id = 1 AND user_id = 1;
-- ③ 列の数と値の数が合わない
INSERT INTO notes (user_id, title, body) VALUES (1, 'タイトルだけ');
-- ④ 存在しない会員に付け替える
UPDATE notes SET user_id = 99 WHERE id = 1 AND user_id = 1;
-- ⑤ WHERE を書き忘れた UPDATE（エラーにならず、全部の行が変わってしまう）
UPDATE notes SET done = 1;
SELECT done, COUNT(*) AS notes FROM notes GROUP BY done;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_errors.sql
```

```text
sql> UPDATE notes SET title = '' WHERE id = 1 AND user_id = 1
エラー: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`

sql> UPDATE notes SET body = NULL WHERE id = 1 AND user_id = 1
エラー: SQLite failure: `NOT NULL constraint failed: notes.body`

sql> INSERT INTO notes (user_id, title, body) VALUES (1, 'タイトルだけ')
エラー: SQLite failure: `2 values for 3 columns`

sql> UPDATE notes SET user_id = 99 WHERE id = 1 AND user_id = 1
エラー: SQLite failure: `FOREIGN KEY constraint failed`

sql> UPDATE notes SET done = 1
OK（変更: 8 行）

sql> SELECT done, COUNT(*) AS notes FROM notes GROUP BY done
+------+-------+
| done | notes |
+------+-------+
| 1    | 8     |
+------+-------+
(1 行)
```

| 番号 | メッセージ・結果 | 何が起きたか | 直し方 |
|---|---|---|---|
| ① | `CHECK constraint failed: …` | `UPDATE` でも、表の制約は `INSERT` と同じように確かめられる | 1〜80 文字のタイトルにする |
| ② | `NOT NULL constraint failed: notes.body` | 本文を `NULL` にしようとした | 空にしたいなら `''` を入れる |
| ③ | `2 values for 3 columns` | 列は3つなのに、値が2つしかない | 列と値の数をそろえる |
| ④ | `FOREIGN KEY constraint failed` | 付け替え先の会員 99 がいない | 実在する会員の id にする |
| ⑤ | エラーにならず `変更: 8 行` | `WHERE` が無いので、**表の全部の行**が対象になった | `WHERE` を書く。実行前に同じ条件で `SELECT` する |

⑤ がこの単元でいちばん怖い失敗です。SQL としては正しいので、DB は何も言わずに全部の行を書き換えます。`DELETE FROM notes;` と書けば、全員のノートが消えます。アプリのコードでは `WHERE` を書き忘れないよう、**SQL を1か所（Repository）に集めてレビューしやすくする**のが定番の対策です（UNIT 09）。手作業の場合は、例3-4 の順番に加えて、UNIT 05 の transaction の中で実行し、件数がおかしければ取り消せるようにします。

### RustNote ではここで使う

RustNote の書き換えの SQL は、すべて持ち主の条件と影響行数の確認がセットです（`Rust/rustnote/src/repository/notes.rs`）。

完了の切り替えは、例3-1 の③に `updated_at` の更新と `RETURNING` を足した形です。

```rust
// done を 0 ⇔ 1 に反転する。中身が変わるので version も上げる
let sql = format!(
    "UPDATE notes
        SET done = 1 - done, version = version + 1, updated_at = datetime('now')
      WHERE id = ?1 AND user_id = ?2
      RETURNING {NOTE_COLUMNS}"
);
let mut rows = conn
    .query(&sql, libsql::params![id.get(), owner.get()])
    .await?;
match rows.next().await? {
    Some(row) => row_to_note(&row),
    None => Err(RepoError::NotFound),
}
```

- `RETURNING` が行を返さなかった（`None`）＝ 変更 0 行です。それを `RepoError::NotFound` に変え、画面では 404 になります。
- `?1`・`?2` に入るのは、URL から読んだノートの番号と、**ログイン中の人の番号**です。持ち主の番号をフォームから受け取ってはいけません。フォームの値は、URL と同じく書き換えられるからです。

削除は `RETURNING` を使わず、`execute` が返す影響行数を見ています。

```rust
let deleted = conn
    .execute(
        "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
        libsql::params![id.get(), owner.get()],
    )
    .await?;
if deleted == 0 {
    return Err(RepoError::NotFound);
}
Ok(())
```

タイトルと本文の更新（`update`）には、もう1つ `AND version = ?5` という条件が付いています。「編集画面を開いた後に、別のタブで先に保存されていないか」を見分けるためのもので、UNIT 05 で学びます。

### 練習問題

**問題1:** Bob（user_id 2）がノート 8 を「完了」にする `UPDATE` を書いてください。持ち主の条件を付け、`version` を 1 増やし、`updated_at` を今の日時にして、`RETURNING` で `id`・`title`・`done`・`version` を受け取ります。

**問題2:** Alice（user_id 1）が、Bob のノート 5 を消そうとして `DELETE FROM notes WHERE id = 5 AND user_id = 1;` を実行しました。何行消えるでしょうか。ノート 5 がまだ残っていることを確かめる `SELECT` も書いてください。

**問題3:** Alice がノート 1 の本文を「牛乳、卵、パン、バター」に変える `UPDATE` を書いてください。`version` を 1 増やし、`updated_at` を今の日時にして、`RETURNING` で `id`・`body`・`version`・`updated_at` を受け取ります。`updated_at` に表示される時刻が、今の日本時間とずれている理由も説明してください。

### 解答と解説

```sql
-- 解答1: Bob（2）がノート 8 を完了にする（持ち主の条件つき）
UPDATE notes SET done = 1, version = version + 1, updated_at = datetime('now')
WHERE id = 8 AND user_id = 2
RETURNING id, title, done, version;
-- 解答2: Alice（1）が Bob のノート 5 を消そうとする
DELETE FROM notes WHERE id = 5 AND user_id = 1;
SELECT id, user_id, title FROM notes WHERE id = 5;
-- 解答3: Alice が本文を書き換え、version と updated_at も更新して結果を受け取る
UPDATE notes SET body = '牛乳、卵、パン、バター', version = version + 1, updated_at = datetime('now')
WHERE id = 1 AND user_id = 1
RETURNING id, body, version, updated_at;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u03_q.sql
```

**▼ 実行結果**（準備の2行は省略。`updated_at` は実行した時刻によって変わります）

```text
sql> UPDATE notes SET done = 1, version = version + 1, updated_at = datetime('now') …
+----+------------+------+---------+
| id | title      | done | version |
+----+------------+------+---------+
| 8  | ジムの予定 | 1    | 2       |
+----+------------+------+---------+
(1 行)

sql> DELETE FROM notes WHERE id = 5 AND user_id = 1
OK（変更: 0 行）

sql> SELECT id, user_id, title FROM notes WHERE id = 5
+----+---------+--------+
| id | user_id | title  |
+----+---------+--------+
| 5  | 2       | 買い物 |
+----+---------+--------+
(1 行)

sql> UPDATE notes SET body = '牛乳、卵、パン、バター', version = version + 1, updated_at = datetime('now') …
+----+------------------------+---------+---------------------+
| id | body                   | version | updated_at          |
+----+------------------------+---------+---------------------+
| 1  | 牛乳、卵、パン、バター | 2       | 2026-10-17 22:39:59 |
+----+------------------------+---------+---------------------+
(1 行)
```

- **解答1:** `SET` の後ろに3つの列をカンマで並べます。`RETURNING` のおかげで、変わった後の `done`（1）と `version`（2）を1回で受け取れました。完了に「する」だけなら `done = 1`、切り替えるなら `done = 1 - done` です。
- **解答2:** 0 行です。ノート 5 の持ち主は Bob（2）なので、`user_id = 1` の条件に合いません。`SELECT` でノート 5 が残っていることを確かめられます。アプリなら、ここで 404 を返します。
- **解答3:** `datetime('now')` は **UTC（世界標準時）** の時刻を返すからです。この実行は日本時間の 2026-10-18 07:39:59 に行ったので、9 時間前の `2026-10-17 22:39:59` と記録されました。RustNote は DB の中では UTC で持ち、画面に出すときに利用者の時間帯に合わせる、という考え方です。

### まとめ

- `INSERT`・`UPDATE`・`DELETE` は、`RETURNING` を付けると変わった後の行を1回で受け取れます。`UPDATE` の右辺の列は「変える前の値」です。
- 書き換えの `WHERE` には必ず**持ち主の条件**を入れます。他人のノートは「変更 0 行」になるので、影響行数を見て 404 を返します。
- `WHERE` を忘れた `UPDATE` / `DELETE` はエラーにならずに全部の行を変えます。手作業では、同じ条件の `SELECT` で件数を確かめてから実行します。
