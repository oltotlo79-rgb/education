## UNIT 02: 表を作る SQL と DB の入口

### この単元のゴール

- `migrations/` の3つの SQL ファイルが、どの表・索引・制約を作るかを説明できる
- `src/db.rs` の `migrate` が、まだ適用していない版だけを、1つずつトランザクションの中で適用し、`schema_migrations` に記録する流れを説明できる
- `Db::open_local` と `Db::connect` が入れている設定（WAL・外部キー・待ち時間）を、実際の値で確かめられる
- マイグレーションが途中で失敗したときに何が残るか、適用済みの SQL ファイルを書き換えたら何が起きるかを、実験で説明できる

### たとえ話でつかむ

ある建物に、何年もかけて部屋を増やしていく場面を考えます。

- 工事には**番号**が付いています。「1番: 1階を建てる」「2番: 2階を足す」「3番: 倉庫を足す」。工事は、必ず番号の順に、1回ずつ行います。
- 建物には**工事の記録簿**があり、終わった工事の番号が書いてあります。新しい担当者が来ても、記録簿を見れば「3番まで終わっている。次は4番から」と分かります。
- 工事の途中で問題が起きたら、その工事で作りかけた物は**全部取り壊して**、工事の前の状態に戻します。半分だけできた部屋を残すと、次の工事ができなくなるからです。
- 終わった工事の**図面を書き換えても、建物は変わりません**。2番の図面に「窓を1つ増やす」と書き足しても、すでに建った2階の窓は増えません。窓を増やしたいなら、新しく「4番: 2階に窓を足す」という工事を足します。

RustNote の DB も、同じやり方で作ります。

| 建物の工事 | RustNote |
|---|---|
| 番号の付いた工事 | **マイグレーション**（`migrations/0001_users.sql` など、番号順の SQL ファイル） |
| 工事の記録簿 | `schema_migrations` 表（適用済みの版の名前と時刻） |
| 番号の順に、まだの工事だけをする | `db::migrate` が、未適用の版だけを古い順に適用する |
| 途中で失敗したら、作りかけを取り壊す | 1つの版を**トランザクション**の中で適用し、失敗したら `rollback` |
| 終わった工事の図面を書き換えても変わらない | 適用済みの SQL ファイルを書き換えても、今ある DB には反映されない（エラー2） |

> **用語:** マイグレーション（migration：DB の表の作り（スキーマ）を変える手順を、番号順のファイルに分けて書いたもの。第7巻 UNIT 06）

<!-- -->

> **用語:** トランザクション（transaction：いくつかの変更を「全部やる」か「全部やめる」かの1まとまりにする仕組み。第7巻 UNIT 05）

### 図で理解する

3つのマイグレーションと、それが作る物の対応です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="210" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="50" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">0001_users.sql</text>
<rect x="20" y="90" width="210" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="120" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">0002_notes.sql</text>
<rect x="20" y="160" width="210" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="190" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">0003_sessions.sql</text>
<rect x="20" y="230" width="210" height="50" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="5 4"/>
<text x="125" y="252" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">db::migrate（src/db.rs）</text>
<text x="125" y="270" font-size="11" fill="#475569" text-anchor="middle">CREATE TABLE IF NOT EXISTS</text>
<rect x="300" y="20" width="400" height="50" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="316" y="42" font-size="12" font-weight="700" fill="#047857">users</text>
<text x="316" y="60" font-size="11" fill="#0f172a">email UNIQUE・password_hash（Argon2id の文字列）</text>
<rect x="300" y="90" width="400" height="50" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="316" y="112" font-size="12" font-weight="700" fill="#047857">notes ＋ 索引 idx_notes_user_updated</text>
<text x="316" y="130" font-size="11" fill="#0f172a">title の CHECK（1〜80 文字）・done の CHECK・user_id の外部キー</text>
<rect x="300" y="160" width="400" height="50" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="316" y="182" font-size="12" font-weight="700" fill="#047857">sessions ＋ 索引 idx_sessions_user・idx_sessions_expires</text>
<text x="316" y="200" font-size="11" fill="#0f172a">token_hash（BLOB の主キー）・expires_at（UNIX 秒）</text>
<rect x="300" y="230" width="400" height="50" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="316" y="252" font-size="12" font-weight="700" fill="#475569">schema_migrations</text>
<text x="316" y="270" font-size="11" fill="#0f172a">version（主キー）・applied_at（適用した時刻）</text>
<line x1="230" y1="45" x2="296" y2="45" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0201)"/>
<line x1="230" y1="115" x2="296" y2="115" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0201)"/>
<line x1="230" y1="185" x2="296" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0201)"/>
<line x1="230" y1="255" x2="296" y2="255" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0201)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: マイグレーションのファイルと、それが作る表と索引。記録簿の schema_migrations だけは、ファイルではなく db::migrate のコードが作る</div>
</div>

`db::migrate` は、次の順番で動きます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">① 記録簿を用意</text>
<text x="120" y="64" font-size="11" fill="#475569" text-anchor="middle">無ければ CREATE TABLE</text>
<rect x="260" y="20" width="200" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">② 適用済みの版を読む</text>
<text x="360" y="64" font-size="11" fill="#475569" text-anchor="middle">SELECT version …</text>
<rect x="500" y="20" width="200" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="600" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">③ MIGRATIONS を古い順に</text>
<text x="600" y="64" font-size="11" fill="#475569" text-anchor="middle">適用済みなら飛ばす</text>
<rect x="260" y="120" width="440" height="80" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="480" y="144" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">④ 1つの版をトランザクションの中で</text>
<text x="280" y="168" font-size="11" fill="#0f172a">tx.execute_batch(その版の SQL)</text>
<text x="280" y="188" font-size="11" fill="#0f172a">tx.execute("INSERT INTO schema_migrations (version) VALUES (?1)")</text>
<rect x="260" y="230" width="200" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="254" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">成功 → commit</text>
<text x="360" y="274" font-size="11" fill="#475569" text-anchor="middle">次の版へ（③に戻る）</text>
<rect x="500" y="230" width="200" height="56" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="600" y="254" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">失敗 → rollback</text>
<text x="600" y="274" font-size="11" fill="#475569" text-anchor="middle">その版の変更を全部取り消して止まる</text>
<line x1="220" y1="48" x2="256" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0202)"/>
<line x1="460" y1="48" x2="496" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0202)"/>
<line x1="600" y1="76" x2="600" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0202)"/>
<line x1="360" y1="200" x2="360" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0202)"/>
<line x1="600" y1="200" x2="600" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0202)"/>
<path d="M260,258 H230 V100 H600 V116" fill="none" stroke="#10b981" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-13-0202)"/>
<text x="120" y="160" font-size="11" fill="#047857" text-anchor="middle">全部終わったら</text>
<text x="120" y="178" font-size="11" fill="#047857" text-anchor="middle">今回適用した版の</text>
<text x="120" y="196" font-size="11" fill="#047857" text-anchor="middle">一覧を返す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: db::migrate の流れ。適用済みの版は飛ばし、まだの版だけを1つずつトランザクションの中で適用する。SQL の実行と記録簿への書き込みは、同じトランザクションに入れる</div>
</div>

- 大事なのは、④の中で、**SQL の実行と記録簿への書き込みを、同じトランザクションに入れている**ことです。もし SQL だけが成功して、記録が残らなかったら、次の起動で同じ SQL がもう一度流れ、「表がもうあります」のエラーになります。反対に、記録だけが残って SQL が失敗していたら、表が無いのに「適用済み」になります。同じトランザクションなら、両方が成功するか、両方とも無かったことになるかの、どちらかです。

### まず動かす最小コード

#### 例2-1: マイグレーションの SQL とコードの全文

> **▼ このファイルがやること（先に日本語で）:** `0001_users.sql` はユーザーの表、`0002_notes.sql` はノートの表と一覧を速くする索引、`0003_sessions.sql` はログイン中のセッションの表と2つの索引を作ります。どれも、決まり（`UNIQUE`・`CHECK`・外部キー）を表に書き込んでいます（第9巻 UNIT 03）。

```sql
@@FILE:rustnote/migrations/0001_users.sql@@
```

```sql
@@FILE:rustnote/migrations/0002_notes.sql@@
```

```sql
@@FILE:rustnote/migrations/0003_sessions.sql@@
```

> **▼ このファイルがやること（先に日本語で）:** `src/db.rs` は、DB への入口の `Db` 型と、マイグレーションの `migrate` 関数を持ちます。`Db` は、設定に合わせてローカルのファイルか Turso を開き、接続のたびに必要な設定を入れます。`migrate` は、SQL ファイルを `include_str!` で実行ファイルに埋め込み、まだ適用していない版だけを適用します。

```rust
@@FILE:rustnote/src/db.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: SQL のファイルは、実行ファイルの中に入る

- `MIGRATIONS` の `include_str!("../migrations/0001_users.sql")` は、コンパイルのときに、SQL ファイルの中身を文字列として実行ファイルに埋め込みます。公開した実行ファイルの隣に `migrations/` フォルダが無くても、マイグレーションを流せます（第9巻 UNIT 03 のエラー2）。
- `MIGRATIONS` の並び順が、適用の順番です。ファイル名の番号（0001・0002・0003）は、人が見て分かるように付けているもので、順番を決めているのはこの配列です。

##### 解説2: Db は、Arc で包んだ入口

- `Db` は、libsql の `Database` を `Arc` で包んでいます。`clone()` しても、中の DB は1つのままで、参照の数が増えるだけです（第3巻）。RustNote の `AppState` は、この `Db` を、3つのリポジトリに1つずつ `clone()` して渡しています。
- 接続（`Connection`）は、処理のたびに `connect()` で作ります。先頭のコメントのとおり、ローカルのファイルなら開くだけ、Turso なら HTTP の準備だけなので軽い、という判断です。

##### 解説3: 失敗の理由を2種類に分ける

- `MigrateError` は、「記録簿を準備できなかった」（`Prepare`）と、「この版の適用に失敗した」（`Apply`、失敗した版の名前つき）の2種類です。`#[source]` を付けたので、もとの libsql のエラーも、原因としてつながったまま残ります（第4巻 UNIT 05）。エラー1で、実際の表示を見ます。

#### 例2-2: 同じ DB に、migrate を2回かける

> **▼ このコードがやること（先に日本語で）:** 引数で渡した場所に DB ファイルを開き（無ければ作られる）、`db::migrate` を2回続けて呼んで、それぞれが返した「今回適用した版」を表示します。そのあと、記録簿の中身、できた表と索引、`notes` の列の名前を表示します。

```rust
@@FILE:rustnote/examples/u02_migrate.rs@@
```

```powershell
cargo run -q --example u02_migrate -- try.db
```

**▼ 実行結果**

```text
@@OUT:u02_migrate@@
```

- 1回目は3つの版がすべて適用され、2回目は `[]`（何も適用しない）でした。RustNote は起動のたびに `migrate` を呼びますが、2回目からは何もしません。何回呼んでも同じ結果になる（**冪等**な）作りです。
- 表には、マイグレーションの4つのほかに `sqlite_sequence` があります。`AUTOINCREMENT` を付けた列の「次に使う番号」を、SQLite が自動で記録する表です。自分で作る必要も、触る必要もありません。

> **用語:** 冪等（べきとう：同じ操作を何回くり返しても、1回したときと同じ結果になること）

### もう一歩: 実務に近い形

#### 例2-3: 接続ごとの設定を、libsql の素の接続と比べる

> **▼ このコードがやること（先に日本語で）:** 一時フォルダの DB を `Db::open_local` で開き、`Db::connect` の接続で、3つの設定（`journal_mode`・`foreign_keys`・`busy_timeout`）の値を表示します。同じファイルを、RustNote の `Db` を通さずに、libsql の素の接続でも開いて比べます。最後に、フォルダの中のファイルを表示します。

```rust
@@FILE:rustnote/examples/u02_pragmas.rs@@
```

```powershell
cargo run -q --example u02_pragmas
```

**▼ 実行結果**

```text
@@OUT:u02_pragmas@@
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="330" height="180" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="185" y="46" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ファイルに記録される設定</text>
<text x="40" y="80" font-size="12" font-weight="700" fill="#0f172a">journal_mode = wal</text>
<text x="40" y="102" font-size="11" fill="#475569">Db::open_local で1回だけ入れる</text>
<text x="40" y="124" font-size="11" fill="#475569">→ 素の接続でも wal のまま</text>
<text x="40" y="156" font-size="11" fill="#0f172a">読み込みと書き込みが重なっても待ちにくい</text>
<text x="40" y="176" font-size="11" fill="#0f172a">DB の横に -wal・-shm のファイルができる</text>
<rect x="370" y="20" width="330" height="180" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="535" y="46" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">接続ごとの設定</text>
<text x="390" y="80" font-size="12" font-weight="700" fill="#0f172a">busy_timeout = 5000</text>
<text x="390" y="102" font-size="11" fill="#475569">Db::connect のたびに入れる</text>
<text x="390" y="124" font-size="11" fill="#475569">→ 素の接続では 0（すぐ失敗する）</text>
<text x="390" y="156" font-size="12" font-weight="700" fill="#0f172a">foreign_keys = 1</text>
<text x="390" y="176" font-size="11" fill="#475569">libsql 0.10 は素の接続でも 1 だった</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: DB の設定の2種類。WAL はファイルに記録されるので1回でよく、待ち時間と外部キーは接続ごとに入れる。RustNote は、外部キーも念のため毎回入れている</div>
</div>

- **WAL（journal_mode = wal）:** ファイルに記録される設定です。`Db::open_local` で1回入れたので、素の接続でも `wal` のままでした。WAL にすると、DB のファイル（`u02.db`）の横に、`u02.db-wal`（まだ本体に書き込んでいない変更）と `u02.db-shm`（その目次）ができます。RustNote の `.gitignore` が `*.db-wal`・`*.db-shm` を外しているのは、このためです（UNIT 01）。
- **busy_timeout:** 接続ごとの設定です。`Db::connect` の接続は 5000（ミリ秒）、素の接続は 0 でした。0 のままだと、ほかの接続が書き込み中のとき、待たずにすぐ「DB がふさがっている」エラーになります。RustNote は、最大 5 秒は待つようにしています。
- **foreign_keys:** SQLite では、外部キーの確かめは「接続ごとに ON にしないと働かない」決まりです。ところが、この libsql 0.10 では、素の接続でも最初から `1`（ON）でした。RustNote の `Db::connect` は、それでも毎回 `PRAGMA foreign_keys = ON` を入れています。libsql の既定が変わっても外部キーが働くように、そして、コードを読んだ人が「外部キーを守らせている」と分かるように書いてある、と読めます。

### よくあるエラーと読み方

この節の実験では、練習用のコピーの `migrations/` のファイルを書き換えます。確かめ終わったら、教材のフォルダから、そのファイルをコピーし直して元に戻してください。

#### エラー1: マイグレーションが途中で失敗した

`0003_sessions.sql` の最後の行の列の名前を、`expires_at` から `expire_at`（`s` が無い）に打ち間違えて、新しい DB に `migrate` をかけた場合です。

```sql
CREATE INDEX idx_sessions_expires ON sessions (expire_at);
```

```powershell
cargo run -q --example u02_migrate -- broken.db
```

```text
@@OUT:u02_err_migration@@
```

- **何が起きたか:** `Error:` の行は、`MigrateError::Apply` の文で、どの版で失敗したか（`0003_sessions`）を教えています。`Caused by:` の下が、もとの libsql のエラーで、「`expire_at` という列がありません」です。
- **何が残ったか:** 失敗したあとの DB の中を、`migrate` をかけずに表示するプログラム（`examples/u02_state.rs`）で調べます。

```rust
@@FILE:rustnote/examples/u02_state.rs@@
```

```powershell
cargo run -q --example u02_state -- broken.db
```

```text
@@OUT:u02_after_failure@@
```

- 0001 と 0002 は記録されていますが、0003 は記録されていません。そして、`sessions` の表も、その前の行で作ったはずの索引 `idx_sessions_user` も、**ありません**。0003 の SQL は、`sessions` の表と1つ目の索引を作ったところで失敗しましたが、トランザクションの `rollback` で、作りかけの物が全部取り消されたのです。
- **直し方:** SQL ファイルの打ち間違いを直してから、同じ DB にもう一度 `migrate` をかけます。0001 と 0002 は記録簿にあるので飛ばされ、0003 だけが適用されました。

```text
@@OUT:u02_recover@@
```

#### エラー2（エラーが出ない間違い）: 適用済みの SQL ファイルを書き換えた

ノートに「ピン留め」の列 `pinned` を足したくなって、**適用済み**の `0002_notes.sql` の `CREATE TABLE notes` に、1行を書き足した場合です。

```sql
    body TEXT NOT NULL DEFAULT '',
    pinned INTEGER NOT NULL DEFAULT 0,
```

例2-2で作った、3つの版が適用済みの `try.db` に、`migrate` をかけます。

```powershell
cargo run -q --example u02_migrate -- try.db
```

```text
@@OUT:u02_edit_applied@@
```

- **何が起きたか:** 1回目も2回目も `[]` で、`notes` の列に `pinned` はありません。エラーも出ません。`migrate` は、版の**名前**（`0002_notes`）が記録簿にあるかどうかだけを見るので、中身が変わっても、もう一度は流さないのです。
- **なぜ困るのか:** 新しく作った DB（たとえば、別の人の PC や、テストの DB）では、書き換えた 0002 が流れるので、`pinned` があります。

```powershell
cargo run -q --example u02_migrate -- fresh.db
```

```text
@@OUTGREP:u02_edit_fresh:^(1回目|notes の列)@@
```

- 同じ RustNote なのに、**古い DB には `pinned` が無く、新しい DB にはある**という食い違いが、黙って生まれました。テストは新しい DB で動くので通り、本番の古い DB で初めて「列がありません」のエラーになります。
- **直し方:** 適用済みのファイルは、書き換えません。`0002_notes.sql` を元に戻し、変更は**新しい版**（`0004_…`）として足します（問1）。

### RustNote ではここで使う

起動の入口の `src/main.rs` は、DB を開いて、マイグレーションをかけてから、待ち受けを始めます（抜き出しています）。

```rust
    let db = Db::open(&config)
        .await
        .with_context(|| format!("DB（{}）を開けませんでした", config.database_url))?;
    let applied = db::migrate(&db)
        .await
        .context("マイグレーションに失敗しました")?;
    for version in &applied {
        eprintln!("マイグレーションを適用しました: {version}");
    }
```

- `migrate` が返した「今回適用した版」を、1つずつ表示しています。第12巻 UNIT 01 で、新しい DB で起動したときに `マイグレーションを適用しました: 0001_users` などが3行出たのは、ここです。2回目からは、何も表示されません。
- マイグレーションに失敗したら、`?` で起動をやめます。表がそろっていない DB のまま、リクエストを受け付けることはありません。

テストの道具 `tests/common/mod.rs` も、同じ関数を使っています（抜き出しています）。

```rust
/// マイグレーション済みの空の DB を作る
pub async fn test_db() -> TestDb {
    let dir = tempfile::tempdir().expect("一時フォルダを作れません");
    let db = Db::open_local(dir.path().join("test.db"))
        .await
        .expect("DB を開けません");
    db::migrate(&db).await.expect("マイグレーションに失敗");
    TestDb { db, _dir: dir }
}
```

- テストごとに、一時フォルダに新しい DB を作り、本番と**同じ `migrate`** をかけます。テスト用に別の `CREATE TABLE` を書かないので、「テストの表と本番の表が違う」ことが起きません。

### 練習問題

**問1:** エラー2の「ピン留め」の列を、正しいやり方で足してください。`0002_notes.sql` は元に戻したうえで、新しい版 `migrations/0004_note_pinned.sql` を作り、`src/db.rs` の `MIGRATIONS` に足します（練習用のコピーの `src/` を書き換える練習です。確かめたら元に戻します）。3つの版が適用済みの `try.db` に `migrate` をかけたら、何が適用されるでしょうか。

**問2:** `0002_notes.sql` の索引 `idx_notes_user_updated` は、「自分のノートを新しい順に」を速くするためのものです。RustNote の一覧の SQL（`src/repository/notes.rs`）が、本当にこの索引を使っているかを、`EXPLAIN QUERY PLAN` で確かめてください。持ち主の条件が無い SQL とも比べます。

**問3:** `.env` の `DATABASE_URL` に、まだ無いフォルダの中のファイル（たとえば `data/rustnote.db`）を書いたら、RustNote はどうなるでしょうか。`Db::open_local` で確かめてください。

### 解答と解説

**問1の解答:**

新しいファイル `migrations/0004_note_pinned.sql` です。

```sql
@@FILE:snippets/0004_note_pinned.sql@@
```

`src/db.rs` の `MIGRATIONS` の最後に、1つ足します。

```rust
    Migration {
        version: "0004_note_pinned",
        sql: include_str!("../migrations/0004_note_pinned.sql"),
    },
```

```powershell
cargo run -q --example u02_migrate -- try.db
```

```text
@@OUT:u02_q1@@
```

- 1回目の `migrate` で、`0004_note_pinned` だけが適用されました。0001〜0003 は記録簿にあるので飛ばされます。`notes` の列の最後に `pinned` が加わりました。`ALTER TABLE ... ADD COLUMN` は、列を最後に足すからです。
- 今あるノートには、`DEFAULT 0` の値が入ります。`NOT NULL` の列を足すときは、`DEFAULT` が無いと、今ある行に入れる値が無いので失敗します。
- この状態で `cargo test` を実行すると、41 件のテストがすべて通りました。新しい DB では 0001〜0004 がすべて流れ、古い DB では 0004 だけが流れて、どちらも同じ形になります。確かめたら、`src/db.rs` を元に戻し、`0004_note_pinned.sql` を消します。

**問2の解答:**

`examples/u02_q2.rs` です。

```rust
@@FILE:rustnote/examples/u02_q2.rs@@
```

```powershell
cargo run -q --example u02_q2
```

```text
@@OUT:u02_q2@@
```

- RustNote の一覧の SQL は `SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)` でした。索引を使って、持ち主（`user_id`）のノートだけを探しています。しかも、並べ替えの手順（`USE TEMP B-TREE FOR ORDER BY`）がありません。索引が、はじめから `updated_at DESC, id DESC` の順に並んでいるので、並べ替えが要らないのです。
- 持ち主の条件が無い SQL は、`SCAN notes`（全部の行を読む）と `USE TEMP B-TREE FOR ORDER BY`（読んだあとで並べ替える）でした。ノートが増えるほど遅くなります。索引の列の順番（`user_id` が先）は、「持ち主で絞ってから、新しい順に並べる」という使い方に合わせて決めてあるのです（第7巻 UNIT 04）。
- `r"..."` は、`\` をそのまま書ける Rust の文字列の書き方です。SQL の `ESCAPE '\'` を、`\\` と書かずに済みます。

**問3の解答:**

`examples/u02_q3.rs` です。

```rust
@@FILE:rustnote/examples/u02_q3.rs@@
```

```powershell
cargo run -q --example u02_q3
```

```text
@@OUT:u02_q3@@
```

- フォルダ（`data`）が無いと、`Unable to open connection to local database ...\rustnote.db: 14` で開けませんでした。14 は SQLite のエラーの番号で、「ファイルを開けない」（`SQLITE_CANTOPEN`）という意味です。SQLite は、ファイルは自動で作りますが、**フォルダは作りません**。
- フォルダを作ってから開くと、ファイルが自動で作られました。
- RustNote では、このエラーは `main.rs` の `with_context` で「DB（data/rustnote.db）を開けませんでした」と包まれて表示され、起動が止まります。公開するときの `Dockerfile` が、`mkdir -p /data` でフォルダを先に作っているのは、このためです（UNIT 10（準備中））。

### まとめ

- RustNote の表は、`migrations/` の番号順の SQL ファイルが作る。`db::migrate` は、未適用の版だけを古い順に、1つずつトランザクションの中で適用し、SQL の実行と記録簿への書き込みを同じトランザクションに入れる。失敗した版の作りかけは、`rollback` で残らない。
- `Db::open_local` は WAL（ファイルに記録される）を1回入れ、`Db::connect` は外部キーと待ち時間（接続ごと）を毎回入れる。libsql 0.10 は素の接続でも外部キーが ON だったが、RustNote は明示している。
- 適用済みの SQL ファイルは書き換えない。古い DB には反映されず、新しい DB とだけ形が変わってしまう。変更は、新しい版として足す。
