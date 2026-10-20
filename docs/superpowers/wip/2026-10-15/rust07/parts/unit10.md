## UNIT 10: backup・token・監視・障害対応

### この単元のゴール

- `VACUUM INTO` で、動いている DB の写し（**backup**：バックアップ）を作り、`PRAGMA integrity_check` で壊れていないかを確かめられる
- うっかり消してしまった行を、写しを `ATTACH` して取り戻せる。写しを作った後に書いたデータは戻らないことを、実際に確かめて説明できる
- Turso Cloud の backup（point-in-time recovery）と、トークンの期限・読み込み専用・無効化の使い方を説明できる。トークンが漏れたときの手順を、正しい順番で言える
- `SELECT 1` だけのヘルスチェックの弱点を説明し、表まで確かめる確認と、遅い SQL の記録を書ける
- 障害が起きたときに「気づく → 止める → 調べる → 戻す → 確かめる → 振り返る」の順で動ける

### たとえ話でつかむ

家の暮らしにたとえます。

- **backup は、写真のアルバム**です。写真には、撮った瞬間の部屋しか写りません。撮った後に買った家具は、アルバムには入っていません。だから、決まった間隔で撮り続けます。
- **トークンは、家の合鍵**です。なくしたら、鍵そのもの（錠前）を交換します。交換すると古い鍵は使えなくなりますが、家族全員の鍵も使えなくなるので、新しい鍵を配り直します。
- **監視は、火災報知器**です。「玄関のドアが開くか」（`SELECT 1`）だけでなく、「部屋の中が無事か」（表が読めるか）を見て、煙（エラーや遅い SQL のログ）に早く気づきます。
- **障害対応は、避難訓練**です。火事が起きてから手順を考えても、慌てて間違えます。手順を先に決めて、練習しておきます。

> **用語:** backup（バックアップ：DB のある瞬間の写し。壊れたり消したりしたときに、写しから戻す）

<!-- -->

> **用語:** 監視（monitoring：アプリと DB が正しく動いているかを、ヘルスチェックやログで見続けること）

<!-- -->

> **用語:** 障害（インシデント：アプリが使えない・データが消えた・秘密が漏れた、のように、利用者に影響が出る出来事）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 310" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<line x1="30" y1="52" x2="690" y2="52" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-101)"/>
<text x="696" y="44" font-size="11" fill="#475569" text-anchor="end">時間</text>
<circle cx="120" cy="52" r="7" fill="#3b82f6"/>
<text x="120" y="28" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">夜: 写しを作る</text>
<text x="120" y="76" font-size="10" fill="#475569" text-anchor="middle">VACUUM INTO</text>
<circle cx="350" cy="52" r="7" fill="#10b981"/>
<text x="350" y="28" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">朝: #9 を書く</text>
<text x="350" y="76" font-size="10" fill="#475569" text-anchor="middle">写しには入っていない</text>
<circle cx="580" cy="52" r="7" fill="#ef4444"/>
<text x="580" y="28" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">事故: 全部消えた</text>
<text x="580" y="76" font-size="10" fill="#475569" text-anchor="middle">WHERE を忘れた DELETE</text>
<rect x="30" y="100" width="190" height="64" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="125" y="126" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">app.db（本番）</text>
<text x="125" y="148" font-size="11" fill="#475569" text-anchor="middle">開いたまま写せる</text>
<line x1="220" y1="132" x2="262" y2="132" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-101)"/>
<rect x="266" y="100" width="200" height="64" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="366" y="126" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">backup-2026-10-21.db</text>
<text x="366" y="148" font-size="11" fill="#475569" text-anchor="middle">その瞬間の写真（#1〜#8）</text>
<line x1="466" y1="132" x2="508" y2="132" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-101)"/>
<rect x="512" y="100" width="180" height="64" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="602" y="124" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">写しから戻す</text>
<text x="602" y="142" font-size="10" fill="#475569" text-anchor="middle">ATTACH して、消えた行だけ</text>
<text x="602" y="156" font-size="10" fill="#475569" text-anchor="middle">INSERT … SELECT</text>
<rect x="30" y="184" width="662" height="40" rx="8" fill="#fef2f2" stroke="#ef4444"/>
<text x="361" y="209" font-size="12" fill="#b91c1c" text-anchor="middle">戻らないもの: 写しの後に書いた #9。写しの間隔が、失うかもしれないデータの時間になる</text>
<rect x="30" y="238" width="662" height="62" rx="10" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="46" y="262" font-size="12" font-weight="700" fill="#854d0e">Turso Cloud の場合: COMMIT のたびに、自動で backup が残る（point-in-time recovery）</text>
<text x="46" y="284" font-size="11" fill="#475569">turso db create 新しい名前 --from-db 元の名前 --timestamp 時刻 → 新しい DB ができる（URL もトークンも新しくなる）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: backup の取り方と戻し方。写しは「その瞬間の写真」なので、写しの後に書いたデータは戻らない。Turso Cloud では、過去の時刻を指定して新しい DB を作る</div>
</div>

もう1つ、障害が起きたときに動く順番です。この単元の後半で、それぞれの段階で使う道具（写し・ヘルスチェック・ログ）を作ります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="200" height="84" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="120" y="40" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">① 気づく</text>
<text x="120" y="62" font-size="10" fill="#475569" text-anchor="middle">/health が 503・[error] のログ</text>
<text x="120" y="80" font-size="10" fill="#475569" text-anchor="middle">「ノートが消えた」という声</text>
<line x1="220" y1="58" x2="256" y2="58" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-102)"/>
<rect x="260" y="16" width="200" height="84" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="40" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">② 止める</text>
<text x="360" y="62" font-size="10" fill="#475569" text-anchor="middle">書き込みを止めて</text>
<text x="360" y="80" font-size="10" fill="#475569" text-anchor="middle">被害を広げない</text>
<line x1="460" y1="58" x2="496" y2="58" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-102)"/>
<rect x="500" y="16" width="200" height="84" rx="10" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="600" y="40" font-size="13" font-weight="700" fill="#854d0e" text-anchor="middle">③ 調べる</text>
<text x="600" y="62" font-size="10" fill="#475569" text-anchor="middle">いつから・どの表・何件</text>
<text x="600" y="80" font-size="10" fill="#475569" text-anchor="middle">今の DB も写しておく</text>
<path d="M600,100 L600,120 L120,120 L120,136" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-102)"/>
<rect x="20" y="140" width="200" height="84" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="164" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">④ 戻す</text>
<text x="120" y="186" font-size="10" fill="#475569" text-anchor="middle">写しから一部だけ・全部置き換え</text>
<text x="120" y="204" font-size="10" fill="#475569" text-anchor="middle">Turso なら過去の時刻から</text>
<line x1="220" y1="182" x2="256" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-102)"/>
<rect x="260" y="140" width="200" height="84" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="164" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">⑤ 確かめる</text>
<text x="360" y="186" font-size="10" fill="#475569" text-anchor="middle">件数・integrity_check</text>
<text x="360" y="204" font-size="10" fill="#475569" text-anchor="middle">画面を自分の目で見る</text>
<line x1="460" y1="182" x2="496" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-102)"/>
<rect x="500" y="140" width="200" height="84" rx="10" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="600" y="164" font-size="13" font-weight="700" fill="#6d28d9" text-anchor="middle">⑥ 振り返る</text>
<text x="600" y="186" font-size="10" fill="#475569" text-anchor="middle">原因と、二度と起こさない</text>
<text x="600" y="204" font-size="10" fill="#475569" text-anchor="middle">工夫を書き残す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 障害が起きたときに動く順番。戻す前に「止める」と「調べる」をはさみ、戻した後は必ず確かめて、振り返りを書き残す</div>
</div>

- **② 止める**を飛ばすと、調べている間にも新しい書き込みが入り、戻す範囲が分からなくなります。
- **③ 調べる**で「今の DB も写しておく」のは、戻す作業を失敗したときに、事故の直後の状態へもう一度やり直せるようにするためです。原因を調べる材料にもなります。

### まず動かす最小コード

#### 例10-1: VACUUM INTO で写しを作り、確かめる

> **▼ このコードがやること（先に日本語で）:** 本番の代わりの DB ファイル `u10/app.db` を作り、練習用のデータを入れます。DB を開いたまま `VACUUM INTO` で写し `u10/backup-2026-10-21.db` を作ります。写しの後にノートを1件書き足してから、写しを別の DB として開き、件数と `PRAGMA integrity_check` の結果を確かめます。

```rust
use libsql::{Builder, Connection};

// users と notes の行数を、1回の SQL でまとめて数える
async fn counts(conn: &Connection) -> Result<(i64, i64), libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT (SELECT COUNT(*) FROM users), (SELECT COUNT(*) FROM notes)",
            (),
        )
        .await?;
    let row = rows.next().await?.expect("この SELECT は必ず1行返す");
    Ok((row.get::<i64>(0)?, row.get::<i64>(1)?))
}

// ファイルが壊れていないかを SQLite に調べてもらう（問題がなければ "ok"）
async fn integrity(conn: &Connection) -> Result<String, libsql::Error> {
    let mut rows = conn.query("PRAGMA integrity_check", ()).await?;
    let row = rows
        .next()
        .await?
        .expect("integrity_check は必ず1行以上返す");
    row.get::<String>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 何回動かしても同じ結果になるよう、前回のフォルダを消して作り直す
    let _ = std::fs::remove_dir_all("u10");
    std::fs::create_dir_all("u10").expect("u10 フォルダを作れません");

    // ① 本番の代わりの DB を作り、練習用のデータを入れる
    let db = Builder::new_local("u10/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    let (users, notes) = counts(&conn).await?;
    println!("app.db: users {users} 件、notes {notes} 件");

    // ② VACUUM INTO で、DB を開いたまま別のファイルへ写しを書き出す
    let backup = "u10/backup-2026-10-21.db";
    conn.execute("VACUUM INTO ?1", [backup]).await?;
    let size = std::fs::metadata(backup).expect("写しがありません").len();
    println!("写しを作りました: {backup}（{size} バイト）");

    // ③ 写しを作った後に、ノートを1件書き足す
    conn.execute(
        "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
        libsql::params![1, "写しの後に書いたノート"],
    )
    .await?;
    let (_, notes) = counts(&conn).await?;
    println!("app.db: 書き足した後の notes {notes} 件");

    // ④ 写しを別の DB として開き、中身と壊れていないかを確かめる
    let copy = Builder::new_local(backup).build().await?;
    let copy_conn = copy.connect()?;
    let (users, notes) = counts(&copy_conn).await?;
    println!("写し: users {users} 件、notes {notes} 件");
    println!("写しの integrity_check: {}", integrity(&copy_conn).await?);
    Ok(())
}
```

`src/bin/u10_backup.rs` として保存して実行します。`sql-lab` フォルダの中に `u10` フォルダができます。

```powershell
cargo run -q --bin u10_backup
```

**▼ 実行結果**

```text
app.db: users 3 件、notes 8 件
写しを作りました: u10/backup-2026-10-21.db（20480 バイト）
app.db: 書き足した後の notes 9 件
写し: users 3 件、notes 8 件
写しの integrity_check: ok
```

本番の `app.db` は 9 件に増えましたが、写しは 8 件のままです。写しは、作った瞬間の「写真」だからです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: VACUUM INTO は、DB を開いたまま一貫した写しを作る

- `VACUUM INTO 'ファイル名'` は、今の DB の中身を、別のファイルに丸ごと書き出す SQL です。アプリが DB を開いたままでも使えます。
- 書き出した写しは、ある1つの瞬間の、つじつまの合った状態です。書きかけの transaction が半分だけ入る、ということはありません。
- 使われていない場所は詰めて書き出されるので、写しは元のファイルと同じか、それより小さくなります。

> **注意:** アプリが動いている間に、エクスプローラーや `Copy-Item` で `app.db` を直接コピーするのはやめましょう。書き込みの途中を写してしまい、開けない写しになることがあります（SQLite の公式ドキュメントも注意しています）。ファイルをそのままコピーしてよいのは、アプリを止めて、DB を誰も開いていないときだけです。

##### 解説2: 写しのファイル名も、?1 で bind する

- `conn.execute("VACUUM INTO ?1", [backup])` のように、ファイル名もプレースホルダーで渡せます。UNIT 07 で学んだとおり、`format!` で SQL に文字列をつなげる書き方は使いません。
- 名前に日付（`backup-2026-10-21.db`）を入れておくと、いつの写しかが一目で分かります。同じ名前のファイルがすでにあると失敗するので、日付や時刻で名前を変えます（「よくあるエラー」で確かめます）。

##### 解説3: 写しは「その瞬間の写真」。後から書いたものは入らない

- ③ で書き足したノートは、`app.db` には入りましたが、写しには入っていません。
- もし ③ の直後に `app.db` が壊れて、写しから戻したら、③ のノートは失われます。**写しを作る間隔が、失うかもしれないデータの時間**です。毎晩写すなら最大で1日分、1時間ごとなら最大で1時間分です。

##### 解説4: 写しは、開いて確かめるまでが仕事

- `PRAGMA integrity_check` は、DB のファイルが壊れていないかを SQLite に調べてもらう命令です。問題がなければ `ok` の1行だけを返し、問題があれば、見つかった問題を1行ずつ返します。
- 写しを作ったら、必ず開いて、件数と `integrity_check` を確かめます。開けない写しは、無いのと同じです。いざ戻そうとしたときに初めて壊れていると分かっても、手遅れです。

> **ヒント:** 写しは、本番と**別の場所**（別のディスク・別のマシン・クラウドの保存場所）にも置きます。本番と同じディスクにしか写しが無いと、ディスクが壊れたときに本番と写しが一緒に消えます。

### もう一歩: 実務に近い形

#### 例10-2: WHERE を忘れた DELETE を、写しから取り戻す

> **▼ このコードがやること（先に日本語で）:** 夜のうちに写しを取っておき、朝に Alice が新しいノート（#9）を書きます。その後、「#3 だけ消すつもりで WHERE を書き忘れた `DELETE`」で、ノートを全部消してしまいます。写しを `bk` という名前で横に付け（`ATTACH`）、今の DB に無い行だけを写しから戻します。

```rust
use libsql::{Builder, Connection};

// notes の id を小さい順に全部読む
async fn note_ids(conn: &Connection) -> Result<Vec<i64>, libsql::Error> {
    let mut rows = conn.query("SELECT id FROM notes ORDER BY id", ()).await?;
    let mut ids = Vec::new();
    while let Some(row) = rows.next().await? {
        ids.push(row.get::<i64>(0)?);
    }
    Ok(ids)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10r");
    std::fs::create_dir_all("u10r").expect("u10r フォルダを作れません");

    // ① 本番の代わりの DB を作り、夜のうちに写しを取っておく
    let db = Builder::new_local("u10r/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    conn.execute("VACUUM INTO ?1", ["u10r/backup.db"]).await?;

    // ② 次の朝、Alice が新しいノートを書く（写しには入っていない）
    conn.execute(
        "INSERT INTO notes (user_id, title) VALUES (?1, ?2)",
        libsql::params![1, "朝に書いたノート"],
    )
    .await?;
    println!("事故の前: {:?}", note_ids(&conn).await?);

    // ③ 事故: #3 だけ消すつもりが、WHERE を書き忘れた
    let deleted = conn.execute("DELETE FROM notes", ()).await?;
    println!("DELETE で消えた行: {deleted} 件（1件のつもりだった）");
    println!("事故の後: {:?}", note_ids(&conn).await?);

    // ④ 写しを「bk」という名前で横に付け、消えた行だけを写しから戻す
    conn.execute("ATTACH DATABASE ?1 AS bk", ["u10r/backup.db"])
        .await?;
    let tx = conn.transaction().await?;
    let restored = tx
        .execute(
            "INSERT INTO main.notes
                 (id, user_id, title, body, done, version, created_at, updated_at)
             SELECT id, user_id, title, body, done, version, created_at, updated_at
             FROM bk.notes
             WHERE id NOT IN (SELECT id FROM main.notes)",
            (),
        )
        .await?;
    tx.commit().await?;
    conn.execute("DETACH DATABASE bk", ()).await?;
    println!("写しから戻した行: {restored} 件");
    println!("戻した後: {:?}", note_ids(&conn).await?);
    Ok(())
}
```

`src/bin/u10_restore.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_restore
```

**▼ 実行結果**

```text
事故の前: [1, 2, 3, 4, 5, 6, 7, 8, 9]
DELETE で消えた行: 9 件（1件のつもりだった）
事故の後: []
写しから戻した行: 8 件
戻した後: [1, 2, 3, 4, 5, 6, 7, 8]
```

消えた 9 件のうち、写しにあった 8 件は戻りました。写しの後に書いた #9 は戻りません。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 消えた行の数が、事故にいちばん早く気づける合図

- `execute` は、変更した行の数を返します（UNIT 03・07）。1件のつもりの `DELETE` が `9` を返したら、その時点で何かがおかしいと分かります。
- この数を見ずに捨てると、利用者から「ノートが消えた」と言われるまで気づけません。`DELETE` と `UPDATE` の行数は、必ず受け取って確かめる習慣をつけましょう（練習問題2で、行数が多すぎたら取り消す関数を作ります）。

##### 解説2: ATTACH で、2つの DB を1つの接続から使う

- `ATTACH DATABASE 'ファイル' AS bk` は、別の DB ファイルを `bk` という名前で、今の接続に横付けします。
- 横付けした後は、`main.notes`（もともと開いていた DB の表）と `bk.notes`（写しの表）を、1つの SQL の中で呼び分けられます。
- 使い終わったら `DETACH DATABASE bk` で外します。

##### 解説3: 今ある行は上書きせず、列は名前で並べる

- `WHERE id NOT IN (SELECT id FROM main.notes)` で、「今の DB に無い id の行」だけを写しから選びます。事故の後に残っていた行や、戻す前に新しく書かれた行を、古い写しで上書きしないためです。
- `INSERT` と `SELECT` の列は、`*` を使わずに名前で書き並べます。UNIT 09 の「列の順番の取り違え」と同じで、2つの表の列の順番が違っていても、名前で書けば正しい列に入ります。
- 戻す SQL は transaction の中で流します。途中で失敗したら、半分だけ戻った状態にならず、全部取り消されます（UNIT 05）。

##### 解説4: 戻らないものと、戻りすぎるものがある

- **戻らないもの:** 写しの後に書いた #9 です。写しに無いデータは、どうやっても写しからは戻せません。Alice には、#9 を書き直してもらうよう伝える必要があります。
- **戻りすぎるもの:** 本当に消したかった #3 も、写しから戻ってしまいました。戻した後に、改めて `DELETE FROM notes WHERE id = 3` を流します。
- どちらも、**戻した後に「何が戻って、何が戻らなかったか」を確かめる**ことで初めて分かります。障害対応の順番で「⑤ 確かめる」を飛ばさないのは、このためです。

##### 解説5: 全部を戻すときは、アプリを止めてファイルを置き換える

一部の行ではなく、DB 全体を写しの時点に戻すこともあります（表を `DROP TABLE` で消してしまった、など）。その場合は、次の順番で行います。

1. アプリを止めて、DB を誰も開いていない状態にする
2. 今の `app.db` を、別の名前（例: `app-broken-2026-10-21.db`）に変えて残す（調べるため・やり直すため）
3. 写しを `app.db` の名前でコピーする
4. 写しを開いて、件数と `integrity_check` を確かめる
5. アプリを起動し、画面で確かめる

#### Turso Cloud の backup と、DB の用意（コマンドの形）

Turso Cloud の DB では、データは Turso のサーバーにあり、手元にファイルがありません。代わりに、次のしくみがあります（2026年10月に公式ドキュメントで確かめた内容です。プランの中身は変わることがあるので、使う前に確かめてください）。

- **point-in-time recovery（PITR）:** `COMMIT` のたびに、自動で backup が残ります。戻せる範囲は、Free プランで直近24時間、有料のプランで10日・30日・90日です。
- 戻すときは、元の DB を書き換えるのではなく、**過去の時刻の状態から新しい DB を作ります**。新しい DB には新しい URL があり、トークンも作り直します。アプリの `DATABASE_URL` と `DATABASE_AUTH_TOKEN` を新しい DB のものに替え、確かめ終わってから古い DB を消します。
- 有料のプランでは、消してしまった DB そのものも、5日以内なら戻せます。

> **注意:** Turso Cloud は、COMMIT の成功を返す前に、データを AWS の保存サービス（S3）に書きます。サーバーのディスクが壊れてもデータは消えにくい、ということです。ただし、**WHERE を忘れた `DELETE` も、同じように確実に保存されます**。「消えにくい」ことと「間違えたときに戻せる」ことは別の話で、後者のために backup（PITR）があります。

よく使う Turso CLI のコマンドを、表にまとめます。この本では Turso のアカウントを使わないので、コマンドは実行していません（出力も載せていません）。形と順番を覚えておき、使うときは `turso db tokens create --help` のように、`--help` で今の版の使い方を確かめてください。Turso CLI は Windows 版が無いので、WSL の中で使います（UNIT 08）。

| やりたいこと | コマンドの形 |
|---|---|
| ログインする（WSL ではブラウザーを開けないので `--headless`） | `turso auth login --headless` |
| DB を作る | `turso db create rustnote` |
| DB の URL を見る | `turso db show rustnote --url` |
| トークンを作る（30日で期限切れ） | `turso db tokens create rustnote --expiration 30d` |
| 読み込み専用のトークンを作る | `turso db tokens create rustnote --read-only --expiration 7d` |
| その DB のトークンを全部無効にする | `turso db tokens invalidate rustnote` |
| 手元に SQLite のファイルとして写しを取る | `turso db export rustnote --output-file rustnote-2026-10-21.db` |
| 過去の時刻の状態から、新しい DB を作る | `turso db create rustnote-restored --from-db rustnote --timestamp 2026-10-21T09:00:00+09:00` |
| 手元の SQLite のファイルから、DB を作る | `turso db create rustnote-copy --from-file ./backup-2026-10-21.db` |

- `--timestamp` の時刻は、`2026-10-21T09:00:00+09:00` のように、日付・時刻・時差（日本は `+09:00`）をつなげて書きます。
- `turso db export` で取った写しには最新の変更が入っていないことがある、と公式ドキュメントに書かれています。直近の状態が必要なときは PITR を使います。
- `--from-file` を使うと、例10-1 の `VACUUM INTO` で作った写しから、Turso Cloud の DB を作れます（ファイルの大きさは 2GB まで）。

#### トークンを管理する: 期限・読み込み専用・無効化

UNIT 08 では、トークンを環境変数から読み、`Secret` 型に包んでログに出さないようにしました。ここでは、トークンそのものの作り方と、漏れたときの手順を決めます。

- **期限を付ける:** `--expiration 30d` のように期限を付けると、もし漏れても、期限が来れば使えなくなります。期限が切れる前に新しいトークンに入れ替える日を、カレンダーに書いておきます。
- **必要な権限だけ渡す:** 集計や写しを取るだけのツールには、`--read-only`（読むだけで、書けない）のトークンを渡します。そのトークンが漏れても、データを書き換えられることはありません。
- **置き場所:** トークンは、本番の環境変数（Fly.io なら `fly secrets set` で設定する秘密の値）にだけ置きます。コード・Git・チャット・スクリーンショットには残しません。
- **1つだけ止めることはできない:** `turso db tokens invalidate` は、署名の鍵を作り直して、**その DB の、その時点までのトークンを全部**使えなくします。公式ドキュメントによると、同じグループ（DB の集まり）のトークンも一緒に無効になります。

トークンが漏れたと気づいたら、次の順番で動きます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-103" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="124" height="96" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="72" y="44" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">① 全部無効に</text>
<text x="72" y="68" font-size="10" fill="#475569" text-anchor="middle">tokens invalidate</text>
<text x="72" y="90" font-size="10" fill="#475569" text-anchor="middle">漏れた鍵を止める</text>
<line x1="134" y1="68" x2="148" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-103)"/>
<rect x="152" y="20" width="124" height="96" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="214" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② 新しく作る</text>
<text x="214" y="68" font-size="10" fill="#475569" text-anchor="middle">tokens create</text>
<text x="214" y="90" font-size="10" fill="#475569" text-anchor="middle">--expiration 30d</text>
<line x1="276" y1="68" x2="290" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-103)"/>
<rect x="294" y="20" width="124" height="96" rx="10" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="356" y="44" font-size="12" font-weight="700" fill="#854d0e" text-anchor="middle">③ 値を替える</text>
<text x="356" y="68" font-size="10" fill="#475569" text-anchor="middle">本番の秘密の値</text>
<text x="356" y="90" font-size="10" fill="#475569" text-anchor="middle">Git には書かない</text>
<line x1="418" y1="68" x2="432" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-103)"/>
<rect x="436" y="20" width="124" height="96" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="498" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">④ 動き直す</text>
<text x="498" y="68" font-size="10" fill="#475569" text-anchor="middle">アプリを再起動</text>
<text x="498" y="90" font-size="10" fill="#475569" text-anchor="middle">新しい値を読む</text>
<line x1="560" y1="68" x2="574" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-103)"/>
<rect x="578" y="20" width="132" height="96" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="644" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">⑤ 確かめる</text>
<text x="644" y="68" font-size="10" fill="#475569" text-anchor="middle">/health が 200</text>
<text x="644" y="90" font-size="10" fill="#475569" text-anchor="middle">[error] が出ていない</text>
<rect x="10" y="132" width="550" height="26" rx="6" fill="#e2e8f0"/>
<text x="285" y="150" font-size="11" fill="#334155" text-anchor="middle">この間、アプリは DB につながらない（手順を先に決めて、短くする）</text>
<rect x="10" y="172" width="700" height="64" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="24" y="196" font-size="11" fill="#c2410c">① と ② を逆にすると、先に作った新しいトークンも ① で無効になってしまう</text>
<text x="24" y="220" font-size="11" fill="#c2410c">漏れた場所（Git・チャット）から消すより先に ① をする。消しても、すでに誰かが写したかもしれない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: トークンが漏れたときの手順。先に全部を無効にしてから新しいトークンを作り、本番の値を替えて動き直し、確かめる</div>
</div>

- ① から ④ が終わるまで、アプリは DB につながりません。この時間を短くするため、手順とコマンドを先に書き出しておき、練習しておきます。
- 漏れていた間に、知らない人がデータを書き換えていないかも調べます。PITR で「漏れる前の時刻」の DB を新しく作り、今の DB と見比べる、という使い方ができます。

#### 例10-3: 表まで確かめるヘルスチェックと、遅い SQL の記録

> **▼ このコードがやること（先に日本語で）:** 正しい DB `u10h/app.db`（表と migration の記録あり）と、1文字打ち間違えたパス `u10h/ap.db` の2つを開き、「`SELECT 1` が通るか」と「`schema_migrations` が読めて、migration が入っているか」の2種類の確認をします。続けて、SQL の時間を測り、100 ミリ秒以上かかった SQL に `[warn]` の印を付けて記録します。

```rust
use std::time::{Duration, Instant};

use libsql::{Builder, Connection};

// これより時間がかかった SQL を「遅い」として記録する
const SLOW: Duration = Duration::from_millis(100);

// 浅い確認: SQL を受け付けるかだけを見る
async fn ping(conn: &Connection) -> Result<(), libsql::Error> {
    let mut rows = conn.query("SELECT 1", ()).await?;
    rows.next().await?;
    Ok(())
}

// 深い確認: アプリが使う表が読めて、migration が入っているかまで見る
async fn check(conn: &Connection) -> Result<i64, String> {
    let mut rows = conn
        .query("SELECT COUNT(*) FROM schema_migrations", ())
        .await
        .map_err(|e| e.to_string())?;
    let row = rows
        .next()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("COUNT(*) が行を返しません")?;
    let applied = row.get::<i64>(0).map_err(|e| e.to_string())?;
    if applied == 0 {
        return Err("migration が1つも入っていません".to_string());
    }
    Ok(applied)
}

// SQL を1件送り、かかった時間を測る。遅ければ [warn] を出す
async fn timed_count(conn: &Connection, label: &str, sql: &str) -> Result<i64, libsql::Error> {
    let started = Instant::now();
    let mut rows = conn.query(sql, ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    let value = row.get::<i64>(0)?;
    let elapsed = started.elapsed();
    if elapsed >= SLOW {
        eprintln!("[warn] 遅い SQL（{} ms）: {label}", elapsed.as_millis());
    }
    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10h");
    std::fs::create_dir_all("u10h").expect("u10h フォルダを作れません");

    // 正しい DB を用意する（表と、migration の記録）
    {
        let db = Builder::new_local("u10h/app.db").build().await?;
        let conn = db.connect()?;
        conn.execute_batch(include_str!("../../sql/schema.sql"))
            .await?;
        conn.execute_batch(include_str!("../../sql/seed.sql"))
            .await?;
        conn.execute_batch(
            "CREATE TABLE schema_migrations (
                 version TEXT PRIMARY KEY,
                 applied_at TEXT NOT NULL DEFAULT (datetime('now'))
             );
             INSERT INTO schema_migrations (version) VALUES ('0001_users'), ('0002_notes');",
        )
        .await?;
    }

    // ① 正しいパスと、1文字打ち間違えたパスで、2種類の確認をする
    for path in ["u10h/app.db", "u10h/ap.db"] {
        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        let shallow = match ping(&conn).await {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("失敗: {e}"),
        };
        let deep = match check(&conn).await {
            Ok(n) => format!("ok（migration {n} 件）"),
            Err(e) => format!("失敗: {e}"),
        };
        println!("{path}: SELECT 1 → {shallow} / 表の確認 → {deep}");
    }

    // ② 時間のかかる SQL を送り、遅いものに印を付ける
    let db = Builder::new_local("u10h/app.db").build().await?;
    let conn = db.connect()?;
    let n = timed_count(&conn, "notes の件数", "SELECT COUNT(*) FROM notes").await?;
    println!("notes の件数: {n}");
    let n = timed_count(
        &conn,
        "1 から 300 万まで数える",
        "WITH RECURSIVE c(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM c WHERE n < 3000000)
         SELECT COUNT(*) FROM c",
    )
    .await?;
    println!("数えた数: {n}");
    Ok(())
}
```

`src/bin/u10_health.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_health
```

**▼ 実行結果**（`[warn]` の行のミリ秒は、PC や実行のたびに変わります）

```text
u10h/app.db: SELECT 1 → ok / 表の確認 → ok（migration 2 件）
u10h/ap.db: SELECT 1 → ok / 表の確認 → 失敗: SQLite failure: `no such table: schema_migrations`
notes の件数: 8
[warn] 遅い SQL（1449 ms）: 1 から 300 万まで数える
数えた数: 3000000
```

打ち間違えたパス `ap.db` でも、`SELECT 1` は `ok` でした。表まで確かめる確認だけが、失敗に気づけています。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 打ち間違えたパスでも、空の DB が開けてしまう

- `Builder::new_local` は、指定したファイルが無ければ、**新しい空の DB ファイルを作って**開きます。実行の後に `u10h` フォルダを見ると、`ap.db` ができています。
- 空の DB でも `SELECT 1` は答えを返すので、浅い確認は `ok` になります。「DB につながっている」ことは分かっても、「正しい DB で、表がそろっている」ことは分かりません。

##### 解説2: アプリが頼っている表を、軽く読んで確かめる

- `check` は `schema_migrations` の行数を数えます。表が無ければ `no such table`、行が 0 件なら「migration が1つも入っていません」で失敗にします。
- Web アプリのヘルスチェック（`/health`）では、失敗したら **503 Service Unavailable** を返します。アプリを動かすサービス（Fly.io など）は、ヘルスチェックの答えを見て、そのマシンが元気かどうかを判断します。
- ヘルスチェックは数十秒ごとに呼ばれるので、重い SQL は入れません。行数の少ない `schema_migrations` を数える程度にし、何万行もある `notes` を数えるような確認は避けます。

##### 解説3: SQL の時間を測り、決まった印でログに残す

- `Instant::now()` で測り始め、`started.elapsed()` でかかった時間を受け取ります。`SLOW`（100 ミリ秒）以上なら、`[warn]` で始まる行を標準エラー出力に書きます。
- 印（`[warn]`・`[error]`）と書き方を決めておくと、たくさんのログの中から、文字で検索して探せます。「`[warn] 遅い SQL` が昨日から急に増えた」と分かれば、索引の付け忘れ（UNIT 04）や N+1（UNIT 09）を疑えます。
- 例の 300 万まで数える SQL は、遅い SQL の代わりです。ふだんのアプリの SQL は、`notes の件数` のように、印が付かない速さで終わります。

##### 解説4: ログに出してはいけないものもある

- 遅い SQL の記録には、`label`（何の SQL か）だけを出し、`?1` に渡した値は出していません。値には、メールアドレスやノートの本文が入ることがあるからです。
- トークンも、ログに出しません（UNIT 08 の `Secret` 型）。ログは、障害のときにたくさんの人が見るものだからです。

### よくあるエラーと読み方

`VACUUM INTO` で起きやすい3つの失敗を、1つのプログラムで確かめます。

```rust
use libsql::Builder;

// execute の結果を「成功」か、エラーの文で表す
fn show(result: Result<u64, libsql::Error>) -> String {
    match result {
        Ok(_) => "成功".to_string(),
        Err(e) => format!("エラー: {e}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10e");
    std::fs::create_dir_all("u10e").expect("u10e フォルダを作れません");
    let db = Builder::new_local("u10e/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // ① 読みかけの rows と row が残ったまま、写しを作ろうとする
    let mut rows = conn.query("SELECT title FROM notes", ()).await?;
    let first = rows.next().await?.expect("ノートは8件ある");
    println!("1件目: {}", first.get::<String>(0)?);
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("① rows と row が残ったまま: {}", show(r));
    drop(rows);
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("① rows だけ捨てた後: {}", show(r));
    drop(first);

    // ② 同じ名前で2回、写しを作ろうとする
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("② 1回目: {}", show(r));
    let r = conn.execute("VACUUM INTO ?1", ["u10e/backup.db"]).await;
    println!("② 2回目: {}", show(r));

    // ③ transaction の途中で、写しを作ろうとする
    let tx = conn.transaction().await?;
    let r = tx.execute("VACUUM INTO ?1", ["u10e/backup2.db"]).await;
    println!("③ transaction の中: {}", show(r));
    tx.rollback().await?;
    Ok(())
}
```

`src/bin/u10_err_vacuum.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_err_vacuum
```

**▼ 実行結果**

```text
1件目: 買い物リスト
① rows と row が残ったまま: エラー: SQLite failure: `cannot VACUUM - SQL statements in progress`
① rows だけ捨てた後: エラー: SQLite failure: `cannot VACUUM - SQL statements in progress`
② 1回目: 成功
② 2回目: エラー: SQLite failure: `output file already exists`
③ transaction の中: エラー: SQLite failure: `cannot VACUUM from within a transaction`
```

#### エラー1: cannot VACUUM - SQL statements in progress — 読みかけの SQL が残っている

```text
エラー: SQLite failure: `cannot VACUUM - SQL statements in progress`
```

- **読み方:** 「SQL の文がまだ実行中なので、VACUUM できません」という意味です。`query` で読み始めた `rows` が、まだ最後まで読まれずに残っています。
- **気をつけるところ:** `drop(rows)` で `rows` だけを捨てても、まだ失敗しました。libsql 0.10 のローカルの DB では、`rows.next()` で受け取った `Row`（ここでは `first`）も、読みかけの文を持っているからです（libsql のソース `local/rows.rs` で、`Row` が `Statement` を持っています）。`first` も捨てた後の ② の1回目は、成功しています。
- **直し方:** 読むところを `{}` で囲むか、例10-1 の `counts` のように関数に分けて、写しを作る前に `rows` と `Row` を両方終わらせます。

```rust
// 読むところを {} で囲むと、rows と first はここで捨てられる
{
    let mut rows = conn.query("SELECT title FROM notes", ()).await?;
    let first = rows.next().await?.expect("ノートは8件ある");
    println!("1件目: {}", first.get::<String>(0)?);
}
conn.execute("VACUUM INTO ?1", ["u10f/backup.db"]).await?;
println!("写しを作りました");
```

```text
1件目: 買い物リスト
写しを作りました
```

#### エラー2: output file already exists — 同じ名前の写しがすでにある

```text
エラー: SQLite failure: `output file already exists`
```

- **読み方:** 「書き出す先のファイルが、すでにあります」という意味です。`VACUUM INTO` は、すでにあるファイルを上書きしません。古い写しをうっかり消さないための、安全のしくみです。
- **直し方:** 写しの名前に日付や時刻を入れて、毎回違う名前にします（`backup-2026-10-21.db`、1日に何度も取るなら `backup-2026-10-21-0900.db`）。古い写しを整理するときは、残す数を決めて消します（練習問題1）。

#### エラー3: cannot VACUUM from within a transaction — transaction の途中

```text
エラー: SQLite failure: `cannot VACUUM from within a transaction`
```

- **読み方:** 「transaction の中では VACUUM できません」という意味です。`BEGIN`（`conn.transaction()`）の後、`COMMIT` か `ROLLBACK` の前に `VACUUM INTO` を流しました。
- **直し方:** transaction を `commit` か `rollback` で終わらせてから、写しを作ります。「migration を流す前に写しを取る」（UNIT 06）ときも、migration の transaction を始める**前**に取ります。

#### エラー4: no such table: bk.notes — ATTACH するファイル名を打ち間違えた

```rust
use std::path::Path;

use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10a");
    std::fs::create_dir_all("u10a").expect("u10a フォルダを作れません");
    let db = Builder::new_local("u10a/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    conn.execute("VACUUM INTO ?1", ["u10a/backup.db"]).await?;

    // 写しの名前を1文字打ち間違えた（backup.db のつもり）
    let path = "u10a/backpu.db";
    println!("ATTACH の前に {path} はある?: {}", Path::new(path).exists());
    conn.execute("ATTACH DATABASE ?1 AS bk", [path]).await?;
    println!("ATTACH の後に {path} はある?: {}", Path::new(path).exists());
    let mut rows = conn.query("SELECT COUNT(*) FROM bk.notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    println!("写しの notes: {} 件", row.get::<i64>(0)?);
    Ok(())
}
```

`src/bin/u10_err_attach.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_err_attach
```

**▼ 実行結果**

```text
ATTACH の前に u10a/backpu.db はある?: false
ATTACH の後に u10a/backpu.db はある?: true
Error: SqliteFailure(1, "no such table: bk.notes")
```

- **読み方:** `ATTACH` そのものは成功し、`SELECT` で「`bk.notes` という表がありません」と失敗しました。`ATTACH` も `new_local` と同じで、ファイルが無ければ**空の DB を新しく作って**横付けするからです。実行の前には無かった `backpu.db` が、実行の後にはできています。
- **直し方:** `ATTACH` の前に `Path::new(path).exists()` で、写しのファイルがあるかを確かめます。無ければ、その場で止めて、名前を確かめ直します。間違ってできてしまった空のファイルは消しておきます（後で「これが写しかな」と迷わないように）。

### RustNote ではここで使う

RustNote のヘルスチェックは、`Rust/rustnote/src/routes/health.rs` の `GET /health` です。DB への確認は、`src/db.rs` の `Db::ping` が行います。

```rust
/// DB に問い合わせができるか確かめる（GET /health で使う）
pub async fn ping(&self) -> Result<(), libsql::Error> {
    let conn = self.connect().await?;
    let mut rows = conn.query("SELECT 1", ()).await?;
    rows.next().await?;
    Ok(())
}
```

```rust
#[route(GET)]
pub async fn health(cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
    match app_state(cx).db.ping().await {
        Ok(()) => Ok((StatusCode::OK, Json(Health { status: "ok" }))),
        Err(error) => {
            eprintln!("[error] ヘルスチェックで DB に接続できません: {error}");
            Ok((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(Health {
                    status: "unavailable",
                }),
            ))
        }
    }
}
```

- `ping` は、例10-3 の浅い確認と同じ `SELECT 1` です。Turso につながらない、トークンが無効になった、といった「DB に問い合わせできない」失敗には気づけて、503 と `[error]` のログを返します。
- ただし、例10-3 で見たとおり、`SELECT 1` は「正しい DB か」までは確かめません。RustNote は起動時に migration を流す（`src/main.rs`）ので、ローカルのファイルを指す `DATABASE_URL` を打ち間違えると、空の新しい DB に表が作られ、**ノートが1件も無いアプリとして元気に動きます**。ヘルスチェックも 200 のままです。

この打ち間違いに気づく手がかりは、起動時のログにあります。RustNote は、今回新しく適用した migration だけを `マイグレーションを適用しました: …` と書きます。次は、RustNote を手元で初めて起動して、ひととおり操作したときのログの一部です。

```text
マイグレーションを適用しました: 0001_users
マイグレーションを適用しました: 0002_notes
マイグレーションを適用しました: 0003_sessions
RustNote を起動しました: http://127.0.0.1:3300
GET / -> 200 OK (782.6µs)
GET /notes -> 200 OK (3.2532ms)
POST /notes/1/edit -> 409 Conflict (7.9545ms)
```

- 初めての起動なので、3つの migration が適用されています。**本番を2回目以降に起動したときに `0001_users` が適用されていたら、空の新しい DB を開いた合図**です。すぐに `DATABASE_URL` を確かめます。
- `GET /notes -> 200 OK (3.2532ms)` は、`src/routes.rs` の `access_log` が書く行です。リクエストごとに、方法・パス・返事の種類・かかった時間が残ります。`/health` は数十秒ごとに来るので、この行を書かないようにしてあります。例10-3 の `[warn]` と同じように、時間の長い行や `500` の行を検索して、遅いページや失敗に気づけます。
- 500 になったときの原因は、UNIT 09 で見た `log_if_internal`（`src/views.rs`）が、`[error] 外側の説明 <- 内側の原因` の形でつないで書きます。

backup とトークンについては、次のようになっています。

- RustNote には、写しを取る機能は入っていません。ローカルのファイルで動かすなら、例10-1 の `VACUUM INTO` を決まった時刻に流すしくみが別に要ります。Turso Cloud で動かすなら、PITR が自動で残ります。
- トークンは、`src/config.rs` が起動時に1回だけ `DATABASE_AUTH_TOKEN` から読み、すぐ `Secret` に包みます（UNIT 08）。だから、トークンを入れ替えたら、**アプリを動かし直すまでは古いトークンのまま**です。トークンが漏れたときの手順で「④ 動き直す」が要るのは、このためです。

### 練習問題

**問題1:** 写しを毎日作ると、ファイルがどんどん増えます。フォルダの中の `backup-*.db` を、新しいほうから3つだけ残して、残りを消す関数 `fn prune_backups(dir: &Path, keep: usize) -> std::io::Result<Vec<String>>` を書いてください。戻り値は、消したファイルの名前の一覧です。`backup-` で始まらないファイル（`app.db` や、手で置いた `memo.txt`）は消してはいけません。

**問題2:** 例10-2 の事故を防ぐために、`DELETE` を transaction の中で試し、消える行が上限より多ければ取り消す関数を書いてください。

```rust
async fn delete_at_most(
    conn: &Connection,
    sql: &str,
    params: impl IntoParams,
    max: u64,
) -> Result<u64, GuardError>
```

上限 1 件で、「WHERE の書き忘れ」「`id` のつもりが `user_id`」「正しい1件」の3つを試し、`notes` の件数がどう変わるかを表示してください。

**問題3:** 開発中に、本番のトークンを書いた `.env` ファイルを、うっかり公開の Git リポジトリにプッシュしてしまいました。気づいてすぐ、「そのコミットを消して、強制プッシュすれば大丈夫」と考えた人がいます。何が足りないでしょうか。やることを、順番に挙げてください。

### 解答と解説

**解答1:** `read_dir` でファイル名を集め、`backup-` で始まって `.db` で終わる名前だけを選び、並べ替えて古いほうから消します。

```rust
// dir の中の backup-*.db を新しいほうから keep 個だけ残し、残りを消す。消した名前を返す
fn prune_backups(dir: &Path, keep: usize) -> std::io::Result<Vec<String>> {
    let mut names: Vec<String> = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("backup-") && name.ends_with(".db"))
        .collect();
    // 名前の日付は「年-月-日」の順で、桁もそろえてあるので、文字の順に並べると古い順になる
    names.sort();
    let remove_count = names.len().saturating_sub(keep);
    let removed: Vec<String> = names.drain(..remove_count).collect();
    for name in &removed {
        std::fs::remove_file(dir.join(name))?;
    }
    Ok(removed)
}
```

`main` では、`VACUUM INTO` で `backup-2026-10-17.db`〜`backup-2026-10-21.db` の5日分の写しを作り、`memo.txt` も置いてから、`prune_backups(dir, 3)` を呼びます（`src/bin/u10_q1.rs`）。

```text
整理の前: ["app.db", "backup-2026-10-17.db", "backup-2026-10-18.db", "backup-2026-10-19.db", "backup-2026-10-20.db", "backup-2026-10-21.db", "memo.txt"]
消した写し: ["backup-2026-10-17.db", "backup-2026-10-18.db"]
整理の後: ["app.db", "backup-2026-10-19.db", "backup-2026-10-20.db", "backup-2026-10-21.db", "memo.txt"]
```

- 名前の並べ替えで古い順になるのは、日付を「年-月-日」の順に、桁をそろえて（`2026-10-05` のように0を付けて）書いているからです。`2026-10-5` のように桁がそろっていないと、文字の順では `2026-10-17` の後に `2026-10-5` が来てしまいます。
- `saturating_sub` は、引き算の答えが 0 より小さくなるときに 0 にします。写しが3つ以下なら `remove_count` は 0 になり、何も消しません。
- 消す前に、残す写しが本当に開けるか（例10-1 の `integrity_check`）を確かめてから消すと、さらに安全です。

**解答2:** `conn.transaction()` で始め、`execute` が返した行数が `max` より多ければ `rollback`、そうでなければ `commit` します。

```rust
#[derive(Debug, Error)]
enum GuardError {
    #[error("{found} 件消えるところでした（上限は {max} 件）。取り消しました")]
    TooMany { found: u64, max: u64 },
    #[error(transparent)]
    Db(#[from] libsql::Error),
}

// transaction の中で DELETE を試し、消える行が max より多ければ取り消す
async fn delete_at_most(
    conn: &Connection,
    sql: &str,
    params: impl IntoParams,
    max: u64,
) -> Result<u64, GuardError> {
    let tx = conn.transaction().await?;
    let found = tx.execute(sql, params).await?;
    if found > max {
        tx.rollback().await?;
        return Err(GuardError::TooMany { found, max });
    }
    tx.commit().await?;
    Ok(found)
}
```

`IntoParams` は `use libsql::{Builder, Connection, params::IntoParams};` で読み込みます。`main` では、メモリの DB に練習用のデータを入れ、3つの `DELETE` を上限 1 件で試します（`src/bin/u10_q2.rs`）。

```rust
let cases: [(&str, &str, Vec<i64>); 3] = [
    ("WHERE の書き忘れ", "DELETE FROM notes", vec![]),
    (
        "id のつもりが user_id",
        "DELETE FROM notes WHERE user_id = ?1",
        vec![1],
    ),
    ("正しい1件", "DELETE FROM notes WHERE id = ?1", vec![3]),
];
for (label, sql, values) in cases {
    match delete_at_most(&conn, sql, values, 1).await {
        Ok(n) => println!("{label}: {n} 件消しました"),
        Err(e) => println!("{label}: {e}"),
    }
    println!("  → notes は今 {} 件", count_notes(&conn).await?);
}
```

```text
WHERE の書き忘れ: 8 件消えるところでした（上限は 1 件）。取り消しました
  → notes は今 8 件
id のつもりが user_id: 5 件消えるところでした（上限は 1 件）。取り消しました
  → notes は今 8 件
正しい1件: 1 件消しました
  → notes は今 7 件
```

- 間違った2つの `DELETE` は、行を消した**後**に取り消されたので、`notes` は 8 件のままです。transaction の中なら、「消してみて、数を見てから決める」ことができます。
- `?` で途中から抜けた場合も、`tx` が drop されて自動で取り消されます（UNIT 05）。
- 値は `Vec<i64>` のまま渡しています。`vec![]`（空）なら、値を当てはめない SQL として動きます。

**解答3:** コミットを消すだけでは足りません。公開されたトークンは、すでに誰かに写されたかもしれないからです。公開のリポジトリは、自動で秘密の値を探すプログラムにすぐ読まれることがあり、ほかの人の手元に取られた写し（clone や fork）からは消せません。トークンそのものを使えなくすることが、いちばん先です。

1. `turso db tokens invalidate rustnote` で、トークンを全部無効にする（ここでアプリも DB につながらなくなる）
2. `turso db tokens create rustnote --expiration 30d` で、新しいトークンを作る（① より前に作ると、① で一緒に無効になる）
3. 本番の秘密の値（Fly.io なら `fly secrets set`）を、新しいトークンに替える
4. アプリを動かし直す（RustNote は起動時にだけトークンを読む）
5. `/health` が 200 になり、`[error]` が出ていないことを確かめる
6. 漏れていた間に、知らない書き込みが無かったかを調べる（PITR で漏れる前の時刻の DB を作って比べる、など）
7. リポジトリから `.env` を消し、`.gitignore` に `.env` を足して、二度とコミットされないようにする
8. 何が起きて、どう直したかを書き残す（障害対応の「⑥ 振り返る」）

- 7 番の「コミットから消す」は、やってかまいませんが、1 番の代わりにはなりません。
- 最初から `--expiration` を付けておけば、もし気づくのが遅れても、トークンが使える時間は期限までに区切られます。

### まとめ

- `VACUUM INTO ?1` で、DB を開いたまま、つじつまの合った写しを作れます。写しは開いて、件数と `PRAGMA integrity_check` で確かめるまでが仕事です。動いている DB のファイルを直接コピーしてはいけません。
- 写しは「その瞬間の写真」です。写しの後に書いたデータは戻らず、写しの間隔が失うかもしれない時間になります。一部だけ戻すときは `ATTACH` して `INSERT … SELECT … WHERE id NOT IN (…)` を transaction の中で流し、戻した後に「何が戻って、何が戻らなかったか」を確かめます。
- Turso Cloud では PITR が自動で残り、過去の時刻から**新しい DB** を作って戻します。トークンには期限を付け、必要な権限（`--read-only`）だけを渡します。漏れたら、先に全部を無効にしてから新しく作り、本番の値を替えて動き直します。
- `SELECT 1` だけのヘルスチェックは、空の DB でも `ok` になります。アプリが頼る表まで軽く確かめ、遅い SQL や失敗には `[warn]`・`[error]` の決まった印を付けてログに残します。
- 障害が起きたら、「気づく → 止める → 調べる → 戻す → 確かめる → 振り返る」の順で動きます。戻す前に今の DB も写しておき、振り返りを書き残して、次の事故を防ぎます。
