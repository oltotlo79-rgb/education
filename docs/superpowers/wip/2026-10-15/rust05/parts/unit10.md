## UNIT 10: バックグラウンドジョブと運用

### この単元のゴール

- 画面の応答と切り離したほうがよい仕事（メールの送信・画像の縮小・集計など）を見分け、**バックグラウンドジョブ**にする理由を説明できる
- メモリの中の列（UNIT 04 の mpsc）では、サーバーを止めると仕事が消えることを確かめ、仕事を DB の表に保存して、再起動の後に続きから処理できる
- 1つの `UPDATE … RETURNING` 文で仕事を取り出し、worker（ワーカー：仕事をする係）が何人いても、同じ仕事を2人が同時に持たないようにできる
- 回数の上限つきの再試行と、あきらめた仕事の置き場（**dead-letter**：デッドレター）を作れる
- **lease**（リース：持ち時間）で、落ちた worker の仕事をほかの worker が取り直せることと、そのために仕事の中身を冪等（UNIT 08）にしておく理由を説明できる
- worker の数を設定で決め、止めるときは仕事の切れ目で止まれる

### たとえ話でつかむ

クリーニング店にたとえます。

- お客さんは、受付で服を預けて**預かり票**を書いてもらい、すぐに帰ります。洗い終わるまで、店の前で待ったりはしません。これが、画面の応答と仕事を切り離す**バックグラウンドジョブ**です。
- 預かり票は、**台帳（DB）に書きます**。机の上の付箋（メモリ）に書いていたら、夜に店を閉めて掃除したときに捨てられてしまいます。
- 職人（worker）は、台帳から「まだ手をつけていない」票を1枚選び、**自分の名前と「17時までに仕上げます」を書き込んでから**作業を始めます（lease）。「選ぶ」と「名前を書く」を一度にやるので、2人の職人が同じ票を取ることはありません。
- 職人が途中で倒れたら、17時を過ぎた票は、ほかの職人が取り直します。そのとき、同じ服を2回洗うことになるかもしれません。だから、**2回洗っても困らないように**しておきます（冪等）。
- 何回洗ってもシミが落ちない服は、何度も洗い続けずに、**「お手上げ」の棚**（dead-letter）に移して、店長が見て判断します。

> **用語:** バックグラウンドジョブ（background job：利用者への応答とは別に、裏で行う仕事。メールの送信・画像の縮小・集計など）

<!-- -->

> **用語:** worker（ワーカー：仕事の列から仕事を取り出して、1つずつ処理する係。タスクやプロセスとして動かす）

<!-- -->

> **用語:** lease（リース：worker が仕事を取り出したときに持つ「持ち時間」。持ち時間を過ぎても終わっていない仕事は、ほかの worker が取り直せる）

<!-- -->

> **用語:** dead-letter（デッドレター：回数の上限までやり直しても成功しなかった仕事を、自動のやり直しから外して置いておく場所。人が理由を確かめて、直してから入れ直す）

### 図で理解する

まず、リクエストの処理とバックグラウンドジョブの関係です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="120" height="70" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="70" y="58" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">利用者</text>
<text x="70" y="78" font-size="10" fill="#475569" text-anchor="middle">「登録」を押す</text>
<rect x="190" y="30" width="200" height="70" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="290" y="54" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">リクエストの処理</text>
<text x="290" y="74" font-size="10" fill="#0f172a" text-anchor="middle">① ユーザーを保存</text>
<text x="290" y="90" font-size="10" fill="#0f172a" text-anchor="middle">② 「メールを送る」仕事を jobs 表へ</text>
<line x1="130" y1="58" x2="186" y2="58" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-101)"/>
<path d="M190,86 H160 V86 H134" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-05-101)"/>
<text x="160" y="118" font-size="10" fill="#047857" text-anchor="middle">③ すぐに答える</text>
<text x="160" y="132" font-size="10" fill="#047857" text-anchor="middle">（メールを待たない）</text>
<rect x="450" y="30" width="260" height="70" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="580" y="54" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">jobs 表（DB）</text>
<text x="580" y="74" font-size="10" fill="#0f172a" text-anchor="middle">#7 send_email hanako@… queued</text>
<text x="580" y="90" font-size="10" fill="#475569" text-anchor="middle">サーバーを止めても消えない</text>
<line x1="390" y1="65" x2="446" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-101)"/>
<rect x="450" y="160" width="260" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="580" y="184" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">worker（裏で動く係）</text>
<text x="580" y="204" font-size="10" fill="#0f172a" text-anchor="middle">④ 仕事を1つ取り出す → メールを送る</text>
<text x="580" y="220" font-size="10" fill="#0f172a" text-anchor="middle">⑤ 成功なら done、失敗なら後でやり直す</text>
<line x1="580" y1="100" x2="580" y2="156" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-101)"/>
<text x="20" y="190" font-size="11" fill="#0f172a">利用者の待ち時間 = ①②③ だけ</text>
<text x="20" y="210" font-size="11" fill="#0f172a">メールの相手が遅くても、画面は速い</text>
<text x="20" y="230" font-size="11" fill="#0f172a">メールの送信に失敗しても、やり直せる</text>
<text x="360" y="260" font-size="12" fill="#475569" text-anchor="middle">「今すぐ答えが要るもの」はリクエストの中で、「後でもよいもの」はジョブで行う</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: リクエストの処理とバックグラウンドジョブ。リクエストの中では仕事を jobs 表に入れるだけですぐに答え、送信などの時間のかかる仕事は worker が後で行う</div>
</div>

次に、1つの仕事がたどる状態です。この単元で作る `jobs` 表の `status` 列は、この4つの値のどれかになります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="90" width="150" height="70" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="95" y="118" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">queued</text>
<text x="95" y="140" font-size="10" fill="#475569" text-anchor="middle">待っている</text>
<rect x="285" y="90" width="150" height="70" rx="12" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="360" y="118" font-size="13" font-weight="700" fill="#854d0e" text-anchor="middle">running</text>
<text x="360" y="140" font-size="10" fill="#475569" text-anchor="middle">だれかが処理中</text>
<rect x="550" y="30" width="150" height="60" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="625" y="56" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">done</text>
<text x="625" y="76" font-size="10" fill="#475569" text-anchor="middle">終わった</text>
<rect x="550" y="160" width="150" height="60" rx="12" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="625" y="186" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">dead</text>
<text x="625" y="206" font-size="10" fill="#475569" text-anchor="middle">あきらめた（人が見る）</text>
<line x1="170" y1="115" x2="281" y2="115" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-102)"/>
<text x="226" y="106" font-size="10" fill="#0f172a" text-anchor="middle">claim（取り出す）</text>
<line x1="435" y1="110" x2="546" y2="66" stroke="#10b981" stroke-width="2" marker-end="url(#ar-05-102)"/>
<text x="500" y="76" font-size="10" fill="#047857" text-anchor="middle">complete</text>
<line x1="435" y1="140" x2="546" y2="184" stroke="#ef4444" stroke-width="2" marker-end="url(#ar-05-102)"/>
<text x="470" y="186" font-size="10" fill="#b91c1c" text-anchor="middle">fail（上限に達した）</text>
<path d="M360,160 V240 H95 V164" fill="none" stroke="#3b82f6" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-05-102)"/>
<text x="228" y="232" font-size="10" fill="#1e40af" text-anchor="middle">fail（まだやり直せる）: run_at を先にして列に戻す</text>
<path d="M330,90 C330,50 390,50 390,86" fill="none" stroke="#854d0e" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-05-102)"/>
<text x="360" y="40" font-size="10" fill="#854d0e" text-anchor="middle">lease が切れたら、ほかの worker が取り直す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 仕事の状態。待っている仕事を取り出すと running になり、成功で done、失敗でやり直すか dead になる。持ち時間が切れた running は、ほかの worker が取り直せる</div>
</div>

### まず動かす最小コード

#### 例10-1: メモリの中の列は、止めると仕事が消える

> **▼ このコードがやること（先に日本語で）:** UNIT 04 の mpsc の channel を「仕事の列」にして、メールを送る仕事を5件入れます。worker のタスクは1件に 0.3 秒かけて送ります。0.7 秒たったところで、サーバーの入れ替え（デプロイ）で止められたことにして、worker を止めます。送れた仕事と、消えた仕事の数を表示します。

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // メモリの中の仕事の列（UNIT 04 の mpsc）
    let (tx, mut rx) = mpsc::channel::<String>(100);
    for n in 1..=5 {
        tx.send(format!("メール{n}")).await.unwrap();
    }
    println!("5 件の仕事を列に入れました");

    let done = Arc::new(AtomicU32::new(0));
    let worker_done = Arc::clone(&done);
    let worker = tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            // 1件に 300ms かかる（メールの送信の代わり）
            tokio::time::sleep(Duration::from_millis(300)).await;
            worker_done.fetch_add(1, Ordering::Relaxed);
            println!(
                "{:.1} 秒: {job} を送りました",
                start.elapsed().as_secs_f64()
            );
        }
    });

    // 0.7 秒後に、サーバーの入れ替え（デプロイ）で止められたとする
    tokio::time::sleep(Duration::from_millis(700)).await;
    println!(
        "{:.1} 秒: サーバーを止めます",
        start.elapsed().as_secs_f64()
    );
    worker.abort();

    let sent = done.load(Ordering::Relaxed);
    println!(
        "送れた仕事: {sent} 件 / メモリと一緒に消えた仕事: {} 件",
        5 - sent
    );
}
```

`src/bin/u10_memory_queue.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_memory_queue
```

**▼ 実行結果**

```text
5 件の仕事を列に入れました
0.3 秒: メール1 を送りました
0.6 秒: メール2 を送りました
0.7 秒: サーバーを止めます
送れた仕事: 2 件 / メモリと一緒に消えた仕事: 3 件
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 列の中身は、プログラムのメモリの中にしかない

- mpsc の channel の箱（UNIT 04）は、プログラムのメモリの中にあります。プログラムが止まると、箱の中に残っていた3件の仕事も一緒に消えます。エラーも出ず、どこにも記録は残りません。
- サーバーは、新しい版を公開する（デプロイ）たびに止まって入れ替わります。落ちることもあります。「止まったら消えてもよい仕事」でなければ、メモリの列には置けません。

##### 解説2: 止めたのは、UNIT 06 の abort と同じ

- `worker.abort()` で、worker のタスクを取り消しました（UNIT 02 の例2-3）。本物のサーバーでは、強制終了や、graceful shutdown の猶予（UNIT 06）が切れたときに、同じことが起きます。
- graceful shutdown で「全部送り終わるまで待つ」こともできますが、仕事が何千件も貯まっていたら、待ちきれません。そして、落ちたとき（電源が切れた・メモリが足りなくなった）には、待つ機会すらありません。

> **ヒント:** メモリの列が向いているのは、「失っても困らない・すぐ終わる」仕事です。たとえば、ログを書き出す仕事や、キャッシュを温め直す仕事などです。利用者との約束（「登録しました。確認のメールを送ります」）が絡む仕事は、次の例10-2のように DB に保存します。

#### 準備: libsql と、仕事の列の部品を用意する

仕事を DB に保存するために、第7巻で使った libsql を足します（RustNote と同じ 0.10.0 です）。`async_lab` フォルダで次を実行します。

```powershell
cargo add libsql@=0.10.0
```

実際の出力の始めの部分です（`Locking 114 packages` の後ろの `Adding …` の行は省いています）。

```text
    Updating crates.io index
      Adding libsql =0.10.0 to dependencies
             Features:
             + core
             + hrana
             + libsql-sys
             + parser
             + remote
             + replication
             + serde
             + stream
             + sync
             + tls
             - cloudflare
             - encryption
             - wasm
    Updating crates.io index
     Locking 114 packages to highest Rust 1.98 compatible versions
```

この PC では、libsql を足した後の最初のビルドに約1分かかりました。ARM64 版の Windows では、第7巻の巻頭と同じく clang が必要です。

この単元の例で使う「仕事の列」の部品を、`src/job_queue.rs` に作ります。UNIT 08 の `mini_server` と同じように、`src/lib.rs` に1行足して使えるようにします。

`src/lib.rs`:

```rust
//! 練習用の部品をまとめたライブラリ（UNIT 08・UNIT 10 で使う）。

pub mod job_queue;
pub mod mini_server;
```

`src/job_queue.rs`:

```rust
//! UNIT 10 の練習用の「仕事の列」（ジョブキュー）。
//!
//! 仕事（ジョブ）を DB の jobs 表に保存するので、プログラムを止めても消えない。
//! 取り出すときは1つの UPDATE 文で「選ぶ」と「自分のものにする」を同時に行うので、
//! 何人の worker が同時に取りに来ても、同じ仕事を2人が同時に持つことはない。

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libsql::{Builder, Connection, Database, params};

/// jobs 表を作る SQL（列の意味は本文の表を参照）
const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY,
    kind TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'queued'
        CHECK (status IN ('queued', 'running', 'done', 'dead')),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    run_at INTEGER NOT NULL,
    locked_until INTEGER,
    last_error TEXT
);
";

/// 取り出した仕事
#[derive(Debug)]
pub struct Job {
    pub id: i64,
    pub kind: String,
    pub payload: String,
    /// これまでに取り出された回数（今回を含む）
    pub attempts: i64,
}

/// 今の時刻（1970年1月1日からのミリ秒）
pub fn now_ms() -> i64 {
    let since = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("時計が 1970 年より前になっています");
    since.as_millis() as i64
}

/// DB ファイルを開き（無ければ作られる）、jobs 表を用意する
pub async fn open(path: &str) -> Result<Database, libsql::Error> {
    let db = Builder::new_local(path).build().await?;
    let conn = db.connect()?;
    conn.execute_batch(SCHEMA).await?;
    Ok(db)
}

/// 新しい接続を作る（worker ごとに1本ずつ使う）
pub async fn connect(db: &Database) -> Result<Connection, libsql::Error> {
    let conn = db.connect()?;
    // ほかの接続が書き込み中なら、最大 5 秒待つ（第7巻 UNIT 05）
    conn.execute_batch("PRAGMA busy_timeout = 5000;").await?;
    Ok(conn)
}

/// 仕事を1つ列に入れ、その id を返す
pub async fn enqueue(
    conn: &Connection,
    kind: &str,
    payload: &str,
    max_attempts: i64,
) -> Result<i64, libsql::Error> {
    let mut rows = conn
        .query(
            "INSERT INTO jobs (kind, payload, max_attempts, run_at)
             VALUES (?1, ?2, ?3, ?4)
             RETURNING id",
            params![kind, payload, max_attempts, now_ms()],
        )
        .await?;
    let row = rows.next().await?.expect("INSERT ... RETURNING は1行返す");
    row.get::<i64>(0)
}

/// 実行してよい仕事を1つ取り出し、lease（持ち時間）の間だけ自分のものにする。無ければ None。
/// 対象は「待っていて、実行の時刻が来た仕事」と「持ち時間が切れたまま running の仕事」
pub async fn claim(conn: &Connection, lease: Duration) -> Result<Option<Job>, libsql::Error> {
    let now = now_ms();
    let mut rows = conn
        .query(
            "UPDATE jobs
             SET status = 'running', attempts = attempts + 1, locked_until = ?1
             WHERE id = (
                 SELECT id FROM jobs
                 WHERE (status = 'queued' AND run_at <= ?2)
                    OR (status = 'running' AND locked_until < ?2)
                 ORDER BY id
                 LIMIT 1
             )
             RETURNING id, kind, payload, attempts",
            params![now + lease.as_millis() as i64, now],
        )
        .await?;
    match rows.next().await? {
        Some(row) => Ok(Some(Job {
            id: row.get(0)?,
            kind: row.get(1)?,
            payload: row.get(2)?,
            attempts: row.get(3)?,
        })),
        None => Ok(None),
    }
}

/// 仕事が成功したことを記録する
pub async fn complete(conn: &Connection, id: i64) -> Result<(), libsql::Error> {
    conn.execute(
        "UPDATE jobs SET status = 'done', locked_until = NULL WHERE id = ?1",
        [id],
    )
    .await?;
    Ok(())
}

/// 失敗を記録する。まだやり直せるなら retry_after だけ後に実行するよう列に戻し、
/// 回数を使い切ったら dead（もうやり直さない）にする。新しい状態を返す
pub async fn fail(
    conn: &Connection,
    id: i64,
    error: &str,
    retry_after: Duration,
) -> Result<String, libsql::Error> {
    let mut rows = conn
        .query(
            "UPDATE jobs
             SET status = CASE WHEN attempts >= max_attempts THEN 'dead' ELSE 'queued' END,
                 run_at = ?2, locked_until = NULL, last_error = ?3
             WHERE id = ?1
             RETURNING status",
            params![id, now_ms() + retry_after.as_millis() as i64, error],
        )
        .await?;
    let row = rows.next().await?.expect("その id の仕事があるはず");
    row.get::<String>(0)
}

/// 状態ごとの件数を、状態の名前の順に返す
pub async fn counts(conn: &Connection) -> Result<Vec<(String, i64)>, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT status, COUNT(*) FROM jobs GROUP BY status ORDER BY status",
            (),
        )
        .await?;
    let mut result = Vec::new();
    while let Some(row) = rows.next().await? {
        result.push((row.get(0)?, row.get(1)?));
    }
    Ok(result)
}
```

`jobs` 表の列の意味は次のとおりです。

| 列 | 意味 |
|---|---|
| `id` | 仕事の番号 |
| `kind` | 仕事の種類（`send_email`・`make_thumbnail` など） |
| `payload` | 仕事に必要な値（宛先のメールアドレスなど） |
| `status` | 状態。`queued`（待っている）・`running`（だれかが処理中）・`done`（終わった）・`dead`（あきらめた）。`CHECK` で、ほかの値を入れられないようにしている |
| `attempts` | これまでに取り出された回数 |
| `max_attempts` | 何回まで取り出してよいか（やり直しの上限） |
| `run_at` | この時刻（ミリ秒）を過ぎたら実行してよい。やり直すときは、先の時刻にする |
| `locked_until` | 処理中の worker の持ち時間の期限（lease）。この時刻を過ぎても `running` のままなら、ほかの worker が取り直せる |
| `last_error` | 最後に失敗したときの理由 |

- 一番大事なのは `claim` です。`UPDATE jobs SET status = 'running' … WHERE id = (SELECT id … LIMIT 1) RETURNING …` は、**1つの文**で「実行してよい仕事を1つ選ぶ」と「`running` にして自分のものにする」を同時に行い、取り出した仕事を返します。SQLite（libSQL）では、1つの文の書き込みは途中で割り込まれないので、2人の worker が同時に `claim` しても、同じ仕事を2人が取ることはありません（例10-4で確かめ、「よくあるエラー1」で、文を分けたときの失敗を見ます）。
- `RETURNING` と `CHECK` と `busy_timeout` は、第7巻で学んだものです。時刻を `INTEGER`（ミリ秒）で持っているのは、大小の比べ方（`run_at <= 今`）を簡単にするためです。

#### 例10-2: 仕事を DB に保存すれば、再起動しても続きから処理できる

> **▼ このコードがやること（先に日本語で）:** 1回目の起動で、メールを送る仕事を5件、`jobs` 表に入れ、2件処理したところで止まります（サーバーが止められたことにします）。2回目の起動で、同じ DB ファイルを開き直し、残っていた仕事を続きから処理します。それぞれの時点で、状態ごとの件数を表示します。

```rust
use std::time::Duration;

use async_lab::job_queue;
use libsql::Connection;

// worker: 仕事を取り出して処理する。max_jobs 件やったら止まる（サーバーが止められた想定）
async fn run_worker(conn: &Connection, max_jobs: usize) -> Result<usize, libsql::Error> {
    let mut done = 0;
    while done < max_jobs {
        let Some(job) = job_queue::claim(conn, Duration::from_secs(30)).await? else {
            break;
        };
        // メールの送信の代わり
        tokio::time::sleep(Duration::from_millis(100)).await;
        job_queue::complete(conn, job.id).await?;
        println!("  #{} {} を処理しました", job.id, job.payload);
        done += 1;
    }
    Ok(done)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 何回動かしても同じ結果になるよう、前回の DB を消して作り直す
    let _ = std::fs::remove_dir_all("u10jobs");
    std::fs::create_dir_all("u10jobs").expect("u10jobs フォルダを作れません");

    // ① 1回目の起動: 5 件の仕事を入れ、2 件処理したところで止まる
    {
        let db = job_queue::open("u10jobs/jobs.db").await?;
        let conn = job_queue::connect(&db).await?;
        for n in 1..=5 {
            job_queue::enqueue(&conn, "send_email", &format!("メール{n}"), 3).await?;
        }
        println!("① 1回目の起動");
        run_worker(&conn, 2).await?;
        println!("  止める前の件数: {:?}", job_queue::counts(&conn).await?);
    }
    // ここで db と conn が片付けられる（プログラムを止めたのと同じ）

    // ② 2回目の起動: 同じ DB ファイルを開き直すと、残りの仕事を続きから処理できる
    let db = job_queue::open("u10jobs/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    println!("② 2回目の起動");
    println!(
        "  起動したときの件数: {:?}",
        job_queue::counts(&conn).await?
    );
    run_worker(&conn, usize::MAX).await?;
    println!("  最後の件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
```

`src/bin/u10_db_queue.rs` として保存して実行します。`async_lab` フォルダの中に `u10jobs` フォルダができます。

```powershell
cargo run -q --bin u10_db_queue
```

**▼ 実行結果**

```text
① 1回目の起動
  #1 メール1 を処理しました
  #2 メール2 を処理しました
  止める前の件数: [("done", 2), ("queued", 3)]
② 2回目の起動
  起動したときの件数: [("done", 2), ("queued", 3)]
  #3 メール3 を処理しました
  #4 メール4 を処理しました
  #5 メール5 を処理しました
  最後の件数: [("done", 5)]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 仕事は、DB のファイルに残っている

- 1回目の起動の `{ }` を抜けたところで、`db` と `conn` が片付けられます。プログラムを止めたのと同じ状態です。
- 2回目の起動で同じファイルを開き直すと、`[("done", 2), ("queued", 3)]` と、止める前とまったく同じ件数が残っていました。メモリの列（例10-1）では消えてしまった3件が、DB には `queued` のまま残っています。
- worker は、それを続きから処理しました。最後は5件とも `done` です。

##### 解説2: worker は「取り出す → 処理する → 記録する」を繰り返すだけ

- `run_worker` は、`claim` で仕事を1つ取り出し、処理して、`complete` で `done` を記録します。`claim` が `None`（実行してよい仕事が無い）を返したら、ループを抜けます。
- `let Some(job) = … else { break; };` は、「`Some` なら中身を `job` に入れて続け、`None` ならループを抜ける」という書き方です（let-else：第2巻 UNIT 08）。
- 本物の worker は、`None` のときにループを抜けずに、少し待ってから（例10-3）、または新しい仕事が来るまで、もう一度 `claim` します。

##### 解説3: lease の 30 秒は、「1件の仕事にかかる時間より十分長く」

- `claim` に渡している `Duration::from_secs(30)` が持ち時間です。この間に `complete` か `fail` を記録しないと、ほかの worker が「この worker は落ちた」と見なして、取り直します（例10-5）。
- 持ち時間は、1件の仕事にふつうかかる時間より、十分長くします。短すぎると、まだ処理中なのに、ほかの worker にも取られてしまいます。

### もう一歩: 実務に近い形

#### 例10-3: 失敗したら間隔を空けてやり直し、上限で dead にする

> **▼ このコードがやること（先に日本語で）:** 3件のメールの仕事を入れます。`ok@` は1回で成功、`flaky@` は2回目まで失敗して3回目で成功、`broken@` は何回やっても失敗します。失敗したら、100 ミリ秒・200 ミリ秒…と間隔を空けて（UNIT 08 の指数バックオフ）やり直し、3回失敗したら `dead` にします。最後に、`dead` の仕事の理由を表示します。

```rust
use std::time::{Duration, Instant};

use async_lab::job_queue::{self, Job};

// 仕事の中身。broken は必ず失敗し、flaky は 2 回目まで失敗する
async fn perform(job: &Job) -> Result<(), String> {
    tokio::time::sleep(Duration::from_millis(20)).await;
    match job.payload.as_str() {
        "broken@example.com" => Err(String::from("宛先のサーバーが受け取りを拒否しました")),
        "flaky@example.com" if job.attempts < 3 => Err(String::from("時間切れ")),
        _ => Ok(()),
    }
}

// n 回目の失敗のあと、次にやり直すまでの時間: 100ms, 200ms, 400ms, ...
fn backoff(attempts: i64) -> Duration {
    Duration::from_millis(100 * 2u64.pow(attempts as u32 - 1))
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10retry");
    std::fs::create_dir_all("u10retry").expect("u10retry フォルダを作れません");
    let db = job_queue::open("u10retry/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    for to in ["ok@example.com", "flaky@example.com", "broken@example.com"] {
        job_queue::enqueue(&conn, "send_email", to, 3).await?;
    }

    let start = Instant::now();
    loop {
        match job_queue::claim(&conn, Duration::from_secs(30)).await? {
            Some(job) => match perform(&job).await {
                Ok(()) => {
                    job_queue::complete(&conn, job.id).await?;
                    println!(
                        "{:.2} 秒: #{} {} 成功（{} 回目）",
                        start.elapsed().as_secs_f64(),
                        job.id,
                        job.payload,
                        job.attempts
                    );
                }
                Err(error) => {
                    let status =
                        job_queue::fail(&conn, job.id, &error, backoff(job.attempts)).await?;
                    println!(
                        "{:.2} 秒: #{} {} 失敗（{} 回目）: {error} → {status}",
                        start.elapsed().as_secs_f64(),
                        job.id,
                        job.payload,
                        job.attempts
                    );
                }
            },
            None => {
                // 今すぐ実行できる仕事が無い。待っている仕事も無ければ終わり
                let counts = job_queue::counts(&conn).await?;
                if !counts.iter().any(|(status, _)| status == "queued") {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }
    }
    println!("最後の件数: {:?}", job_queue::counts(&conn).await?);

    // dead になった仕事は、理由と一緒に残っている（人が確かめて、直してから入れ直す）
    let mut rows = conn
        .query(
            "SELECT id, payload, attempts, last_error FROM jobs WHERE status = 'dead'",
            (),
        )
        .await?;
    while let Some(row) = rows.next().await? {
        println!(
            "dead: #{} {}（{} 回）最後のエラー: {}",
            row.get::<i64>(0)?,
            row.get::<String>(1)?,
            row.get::<i64>(2)?,
            row.get::<String>(3)?
        );
    }
    Ok(())
}
```

`src/bin/u10_retry_dead.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_retry_dead
```

**▼ 実行結果**

```text
0.04 秒: #1 ok@example.com 成功（1 回目）
0.08 秒: #2 flaky@example.com 失敗（1 回目）: 時間切れ → queued
0.12 秒: #3 broken@example.com 失敗（1 回目）: 宛先のサーバーが受け取りを拒否しました → queued
0.22 秒: #2 flaky@example.com 失敗（2 回目）: 時間切れ → queued
0.26 秒: #3 broken@example.com 失敗（2 回目）: 宛先のサーバーが受け取りを拒否しました → queued
0.48 秒: #2 flaky@example.com 成功（3 回目）
0.53 秒: #3 broken@example.com 失敗（3 回目）: 宛先のサーバーが受け取りを拒否しました → dead
最後の件数: [("dead", 1), ("done", 2)]
dead: #3 broken@example.com（3 回）最後のエラー: 宛先のサーバーが受け取りを拒否しました
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: やり直しは「run_at を先の時刻にして、列に戻す」

- `fail` は、`status` を `queued` に戻し、`run_at` を「今 + 待ち時間」にします。`claim` は `run_at <= 今` の仕事しか取り出さないので、待ち時間が過ぎるまで、その仕事は取り出されません。
- 実行結果では、`flaky@` の1回目の失敗（0.08 秒）の後、2回目は 0.22 秒でした。1回目の失敗の後の待ち時間 100 ミリ秒が過ぎるのを待ってから、取り出されています。2回目の後は 200 ミリ秒待って、0.48 秒に3回目で成功しました。
- worker を止めて再起動しても、`run_at` は DB に残っているので、「いつやり直すか」も失われません。

##### 解説2: 上限に達したら dead にして、理由を残す

- `fail` の SQL の `CASE WHEN attempts >= max_attempts THEN 'dead' ELSE 'queued' END` が、上限の判断です。`broken@` は3回目の失敗で `attempts`（3）が `max_attempts`（3）に達したので、`dead` になりました。
- `dead` の仕事は、もう自動では取り出されません（`claim` の対象は `queued` と、持ち時間の切れた `running` だけです）。何度やっても失敗する仕事を永遠にやり直し続けて、相手のサーバーや自分の worker の時間を無駄にすることがありません。
- `last_error` に最後の失敗の理由が残っているので、人が見て「宛先が間違っている」と分かれば、宛先を直して `status` を `queued` に戻し（入れ直し）、もう一度やり直させられます。

##### 解説3: 仕事が無いときは、少し待ってから見に行く

- `claim` が `None` を返しても、`queued` の仕事（待ち時間の途中のもの）が残っていれば、50 ミリ秒待ってから、もう一度 `claim` します。待たずにすぐ `claim` を繰り返すと、DB に問い合わせ続けて CPU を無駄に使います。
- このように、決まった間隔で「何か仕事は無いか」を見に行くやり方を**ポーリング**と呼びます。本物の worker では、間隔を数百ミリ秒〜数秒にし、設定で変えられるようにします。

> **ヒント:** `dead` の仕事が増えていないかは、UNIT 09 のメトリクスで見張ります。たとえば「`dead` の数」を数えて、1件でも増えたら気づけるようにしておきます。`dead` は「人の判断が要る」という合図なので、放っておくと、利用者に約束したメールが届かないままになります。

#### 例10-4: 2人の worker で、同じ仕事を二重に処理しない

> **▼ このコードがやること（先に日本語で）:** 画像を縮小する仕事を10件入れます。worker を、環境変数 `WORKERS` で決めた数（無ければ2）だけタスクとして動かし、それぞれ自分の接続で `claim` を繰り返します。だれがどの仕事を処理したかを記録し、最後に、重複が無いかを確かめます。

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10two");
    std::fs::create_dir_all("u10two").expect("u10two フォルダを作れません");
    let db = Arc::new(job_queue::open("u10two/jobs.db").await?);
    let conn = job_queue::connect(&db).await?;
    for n in 1..=10 {
        job_queue::enqueue(&conn, "make_thumbnail", &format!("画像{n}"), 3).await?;
    }

    // worker の数は設定（環境変数 WORKERS）で決める。無ければ 2
    let workers: usize = std::env::var("WORKERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2);
    // だれがどの仕事をしたかの記録
    let log = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for number in 1..=workers {
        let db = Arc::clone(&db);
        let log = Arc::clone(&log);
        handles.push(tokio::spawn(async move {
            // worker ごとに自分の接続を使う
            let conn = job_queue::connect(&db).await?;
            while let Some(job) = job_queue::claim(&conn, Duration::from_secs(30)).await? {
                tokio::time::sleep(Duration::from_millis(50)).await;
                job_queue::complete(&conn, job.id).await?;
                log.lock().unwrap().push((job.id, number));
            }
            Ok::<(), libsql::Error>(())
        }));
    }
    for handle in handles {
        handle.await.expect("worker が panic しました")?;
    }

    let mut log = log.lock().unwrap().clone();
    log.sort();
    for number in 1..=workers {
        let ids: Vec<i64> = log
            .iter()
            .filter(|(_, worker)| *worker == number)
            .map(|(id, _)| *id)
            .collect();
        println!("worker{number}: {} 件 {:?}", ids.len(), ids);
    }
    let mut ids: Vec<i64> = log.iter().map(|(id, _)| *id).collect();
    let total = ids.len();
    ids.dedup();
    println!(
        "処理した回数 {total} / 仕事の数 {} / 重複 {} 件",
        ids.len(),
        total - ids.len()
    );
    println!("最後の件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
```

`src/bin/u10_two_workers.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_two_workers
```

**▼ 実行結果**

```text
worker1: 5 件 [2, 4, 6, 8, 10]
worker2: 5 件 [1, 3, 5, 7, 9]
処理した回数 10 / 仕事の数 10 / 重複 0 件
最後の件数: [("done", 10)]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 2人が同時に取りに行っても、重複は 0 件

- 2人の worker は、同時に `claim` を繰り返しました。それでも、10件の仕事は1回ずつ、合わせて10回だけ処理され、重複は0件でした。どちらが何件処理するかは、実行するたびに変わることがあります。
- `claim` の1つの `UPDATE` 文の中で「選ぶ」と「`running` にする」が同時に行われるので、片方が選んだ仕事は、もう片方が選ぶ時点ではもう `queued` ではないからです。同時に書き込もうとしたときは、`busy_timeout`（5 秒）の範囲で、後から来たほうが待ってくれます。

##### 解説2: worker ごとに、自分の接続を使う

- 各 worker は、`job_queue::connect(&db)` で自分専用の接続を作っています。`Database`（`db`）は `Arc` で共有し、接続はタスクの中で作ります。
- `while let Some(job) = …claim(…).await? { … }` は、「仕事が取れる間は繰り返し、`None` になったら終わる」ループです。`?` で DB のエラーをタスクの戻り値（`Result`）として返し、`main` の側では `handle.await.expect(…)?` で、panic とエラーの両方を受け取っています（UNIT 02 の練習問題3の形）。

##### 解説3: worker の数は、設定で変えられるようにする

- `std::env::var("WORKERS")` で環境変数を読み、数に変えられなければ 2 にしています。PowerShell なら `$env:WORKERS = "4"` としてから実行すると、4人の worker で動きます（UNIT 09 の `RUST_LOG` と同じ設定のしかたです）。
- 本物のアプリでは、worker の数・持ち時間・ポーリングの間隔・やり直しの上限などを、第4巻 UNIT 09 の `Config` のような設定の型にまとめて、起動時に環境変数から読みます。公開してから、混み具合に合わせて調整できるようにしておくためです。

> **注意:** worker の数を増やすほど速くなるとは限りません。SQLite（libSQL のローカルファイル）は、書き込みを1つずつしか受け付けないので、`claim` と `complete` の書き込みが混み合います。また、仕事の中身が外の API なら、相手の上限（UNIT 05 の Semaphore・UNIT 08 の 429）にも気をつけます。数は、測りながら決めましょう。

#### 例10-5: 落ちた worker の仕事を、持ち時間の後に取り直す

> **▼ このコードがやること（先に日本語で）:** 持ち時間（lease）を 300 ミリ秒にします。worker A が仕事を取り出したところで落ちた（`complete` を記録しない）ことにします。すぐに worker B が取りに来ても取れませんが、持ち時間が切れた 0.4 秒後には、B が同じ仕事を取り出してやり直せることを確かめます。

```rust
use std::time::{Duration, Instant};

use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10lease");
    std::fs::create_dir_all("u10lease").expect("u10lease フォルダを作れません");
    let db = job_queue::open("u10lease/jobs.db").await?;
    let worker_a = job_queue::connect(&db).await?;
    let worker_b = job_queue::connect(&db).await?;
    job_queue::enqueue(&worker_a, "send_email", "hanako@example.com", 3).await?;

    let start = Instant::now();
    // 持ち時間（lease）は 300ms
    let lease = Duration::from_millis(300);

    // ① worker A が仕事を取り出したところで、落ちてしまった（complete を呼ばない）
    let job = job_queue::claim(&worker_a, lease)
        .await?
        .expect("仕事がある");
    println!(
        "{:.1} 秒: A が #{} を取り出した（{} 回目）。ここで A が落ちた",
        start.elapsed().as_secs_f64(),
        job.id,
        job.attempts
    );

    // ② すぐに worker B が取りに来ても、A の持ち時間の間は取れない
    let job = job_queue::claim(&worker_b, lease).await?;
    println!(
        "{:.1} 秒: B が取りに来た → {:?}",
        start.elapsed().as_secs_f64(),
        job.map(|job| job.id)
    );

    // ③ 持ち時間が切れた後なら、B が取り出してやり直せる
    tokio::time::sleep(Duration::from_millis(400)).await;
    let job = job_queue::claim(&worker_b, lease)
        .await?
        .expect("仕事がある");
    println!(
        "{:.1} 秒: B が #{} を取り出した（{} 回目）",
        start.elapsed().as_secs_f64(),
        job.id,
        job.attempts
    );
    job_queue::complete(&worker_b, job.id).await?;
    println!("最後の件数: {:?}", job_queue::counts(&worker_b).await?);
    Ok(())
}
```

`src/bin/u10_lease.rs` として保存して実行します。

```powershell
cargo run -q --bin u10_lease
```

**▼ 実行結果**

```text
0.0 秒: A が #1 を取り出した（1 回目）。ここで A が落ちた
0.0 秒: B が取りに来た → None
0.4 秒: B が #1 を取り出した（2 回目）
最後の件数: [("done", 1)]
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-103" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="48" font-size="12" font-weight="700" fill="#1e40af">worker A</text>
<rect x="110" y="30" width="270" height="28" rx="6" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="245" y="49" font-size="11" fill="#854d0e" text-anchor="middle">#1 の持ち時間（locked_until まで 300ms）</text>
<text x="118" y="76" font-size="16" font-weight="700" fill="#ef4444">✕</text>
<text x="138" y="76" font-size="10" fill="#b91c1c">取り出した直後に落ちた（complete しない）</text>
<text x="20" y="128" font-size="12" font-weight="700" fill="#047857">worker B</text>
<rect x="110" y="112" width="70" height="26" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="145" y="129" font-size="10" fill="#475569" text-anchor="middle">→ None</text>
<rect x="430" y="112" width="250" height="26" rx="6" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<text x="555" y="129" font-size="10" fill="#047857" text-anchor="middle">#1 を取り直して処理（2 回目）→ done</text>
<line x1="380" y1="22" x2="380" y2="150" stroke="#ef4444" stroke-width="2" stroke-dasharray="4 4"/>
<text x="386" y="96" font-size="10" fill="#b91c1c">持ち時間が切れた</text>
<line x1="110" y1="172" x2="690" y2="172" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-103)"/>
<line x1="110" y1="166" x2="110" y2="178" stroke="#475569" stroke-width="2"/>
<line x1="380" y1="166" x2="380" y2="178" stroke="#475569" stroke-width="2"/>
<line x1="430" y1="166" x2="430" y2="178" stroke="#475569" stroke-width="2"/>
<text x="110" y="194" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="380" y="194" font-size="11" fill="#475569" text-anchor="middle">0.3</text>
<text x="430" y="194" font-size="11" fill="#475569" text-anchor="middle">0.4</text>
<text x="360" y="226" font-size="12" fill="#475569" text-anchor="middle">A が本当は落ちていなくて、遅かっただけなら、#1 は A と B の2回処理される（少なくとも1回）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: lease で落ちた worker の仕事を取り直すようす。持ち時間の間はほかの worker は取れず、切れた後は取り直せる。そのため、同じ仕事が2回動くことがある</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 持ち時間の間は、ほかの worker は取れない

- A が取り出した仕事は、`status` が `running` で、`locked_until` が「今 + 300 ミリ秒」になっています。B がすぐに `claim` しても、`claim` の条件「`running` で、`locked_until < 今`」に当てはまらないので、`None` でした。
- A がふつうに動いていれば、持ち時間の間に `complete` を記録します。2人が同じ仕事を同時に処理することはありません。

##### 解説2: 持ち時間が切れたら、ほかの worker が取り直す

- 0.4 秒には、`locked_until` を過ぎていたので、B が同じ仕事を取り出せました。`attempts` が 2 になっています。A がどんな形で落ちても（panic・強制終了・マシンの故障）、仕事は持ち時間の後に必ず誰かが拾います。
- 例10-1のメモリの列では、落ちた worker が持っていた仕事は、ほかの仕事と一緒に消えていました。DB と lease の組み合わせなら、仕事は「必ず最後までやり遂げられる（あるいは dead になる）」ようになります。

##### 解説3: そのかわり、仕事は「少なくとも1回」動く

- もし A が落ちたのではなく、ただ遅かっただけ（持ち時間を過ぎてから `complete` した）なら、同じ仕事が A と B の2回処理されます。持ち時間のしくみでは、「**必ず1回だけ**」は約束できず、「**少なくとも1回**」（at-least-once：アットリースト・ワンス）になります。
- だから、仕事の中身は、2回動いても困らないように作ります。UNIT 08 の冪等キーと同じ考え方で、「この仕事でメールを送ったか」を DB に記録しておき、2回目は送らない、のようにします（練習問題3）。

> **用語:** 少なくとも1回（at-least-once：仕事が失われないかわりに、2回以上動くことがある、という約束。lease を使うジョブの列の多くはこれ。仕事の中身を冪等にして使う）

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 「選ぶ」と「自分のものにする」を別々の文にすると、2人が同じ仕事を取る

`claim` を、`SELECT`（選ぶ）と `UPDATE`（`running` にする）の2つの文に分けて書いた場合です。違いが見えやすいように、2つの文の間に 50 ミリ秒の待ちを入れています。

```rust
use std::sync::Arc;
use std::time::Duration;

use async_lab::job_queue;
use libsql::Connection;

// 悪い例: 「選ぶ（SELECT）」と「自分のものにする（UPDATE）」を、別々の文で行う
async fn claim_badly(conn: &Connection) -> Result<Option<i64>, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT id FROM jobs WHERE status = 'queued' ORDER BY id LIMIT 1",
            (),
        )
        .await?;
    let Some(row) = rows.next().await? else {
        return Ok(None);
    };
    let id = row.get::<i64>(0)?;
    drop(row);
    drop(rows);
    // 選んでから自分のものにするまでの間に、ほかの worker も同じ仕事を選べてしまう
    tokio::time::sleep(Duration::from_millis(50)).await;
    conn.execute("UPDATE jobs SET status = 'running' WHERE id = ?1", [id])
        .await?;
    Ok(Some(id))
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("e10race");
    std::fs::create_dir_all("e10race").expect("e10race フォルダを作れません");
    let db = Arc::new(job_queue::open("e10race/jobs.db").await?);
    let conn = job_queue::connect(&db).await?;
    job_queue::enqueue(&conn, "send_email", "hanako@example.com", 3).await?;

    let mut handles = Vec::new();
    for name in ["worker1", "worker2"] {
        let db = Arc::clone(&db);
        handles.push(tokio::spawn(async move {
            let conn = job_queue::connect(&db).await?;
            let claimed = claim_badly(&conn).await?;
            println!("{name} が取った仕事: {claimed:?}");
            Ok::<(), libsql::Error>(())
        }));
    }
    for handle in handles {
        handle.await.expect("worker が panic しました")?;
    }
    Ok(())
}
```

```text
worker2 が取った仕事: Some(1)
worker1 が取った仕事: Some(1)
```

（`src/bin/e10_double_claim.rs` として実行した結果です。`cargo run` の `Compiling ...` などの行は省いています。）

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-104" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="44" font-size="12" font-weight="700" fill="#0f172a">worker1</text>
<rect x="100" y="28" width="150" height="26" rx="6" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<text x="175" y="45" font-size="10" fill="#1e40af" text-anchor="middle">SELECT → #1 を選ぶ</text>
<rect x="400" y="28" width="170" height="26" rx="6" fill="#fee2e2" stroke="#ef4444" stroke-width="2"/>
<text x="485" y="45" font-size="10" fill="#b91c1c" text-anchor="middle">UPDATE #1 → running</text>
<text x="20" y="98" font-size="12" font-weight="700" fill="#0f172a">worker2</text>
<rect x="120" y="82" width="150" height="26" rx="6" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<text x="195" y="99" font-size="10" fill="#1e40af" text-anchor="middle">SELECT → #1 を選ぶ</text>
<rect x="420" y="82" width="170" height="26" rx="6" fill="#fee2e2" stroke="#ef4444" stroke-width="2"/>
<text x="505" y="99" font-size="10" fill="#b91c1c" text-anchor="middle">UPDATE #1 → running</text>
<rect x="250" y="20" width="150" height="96" rx="8" fill="none" stroke="#ef4444" stroke-dasharray="5 4"/>
<text x="325" y="134" font-size="11" fill="#b91c1c" text-anchor="middle">この間はどちらから見ても #1 は queued</text>
<rect x="20" y="152" width="680" height="56" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="360" y="176" font-size="11" fill="#047857" text-anchor="middle">直し方: UPDATE … WHERE id = (SELECT … LIMIT 1) RETURNING … の1つの文で、選ぶと取るを同時に行う</text>
<text x="360" y="196" font-size="11" fill="#047857" text-anchor="middle">（job_queue::claim の形。1つの文の書き込みは、途中で割り込まれない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 選ぶと取るを別々の文にしたときの取り合い。2人とも、相手が取る前の #1 を選んでしまう。1つの文にすれば起きない</div>
</div>

- **何が起きたか:** 1件しかない仕事を、2人の worker が**両方とも**取りました。エラーは出ていません。メールを送る仕事なら、同じメールが2通届きます。
- worker1 が `SELECT` で #1 を選んでから、`UPDATE` で `running` にするまでの間（図の点線の枠）、#1 はまだ `queued` です。その間に worker2 も `SELECT` すると、同じ #1 を選んでしまいます。待ちを入れなくても、たくさんの worker がたくさんの仕事を取り合えば、いつかは同じことが起きます。
- **直し方:** `job_queue::claim` のように、`UPDATE … WHERE id = (SELECT … LIMIT 1) RETURNING …` の**1つの文**で、選ぶと取るを同時に行います。別々の文で書きたいなら、2つの文を transaction で囲み、ほかの書き込みが割り込めないようにします（第7巻 UNIT 05）。
- `drop(row); drop(rows);` は、読みかけの結果を片付けてから待つためのものです。第7巻 UNIT 10 で見たとおり、`Row` は文（Statement）を持っているので、片付けないまま待つと、読み込みが続いていることになってしまいます。

#### 寄り道: 名前に update を含む実行ファイルが動かない（os error 740）

このプログラムは、最初は `e10_select_then_update.rs` という名前で作りました。すると Windows で、実行しようとしたところで、次のように止まりました。

```text
error: could not execute process `target\debug\e10_select_then_update.exe` (never executed)

Caused by:
  要求された操作には管理者特権が必要です。 (os error 740)
```

- Windows は、名前に `update` などの言葉を含む実行ファイルを「インストーラーかもしれない」と判断して、管理者の権限を求めることがあります。プログラムの中身は関係なく、名前だけで起きます。
- `src/bin/` のファイルの名前に `update`・`setup`・`install` などを使うと、同じことが起きるかもしれません。そのときは名前を変えてください（この例では `e10_double_claim.rs` にしました）。

#### エラー2: 決めていない状態の値を書き込む（CHECK constraint failed）

```rust
use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("e10status");
    std::fs::create_dir_all("e10status").expect("e10status フォルダを作れません");
    let db = job_queue::open("e10status/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    let id = job_queue::enqueue(&conn, "send_email", "hanako@example.com", 3).await?;

    // 'done' と書くつもりで 'finished' と書いてしまった
    conn.execute("UPDATE jobs SET status = 'finished' WHERE id = ?1", [id])
        .await?;
    println!("書き換えました");
    Ok(())
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
     Running `target\debug\e10_bad_status.exe`
Error: SqliteFailure(275, "CHECK constraint failed: status IN ('queued', 'running', 'done', 'dead')")
error: process didn't exit successfully: `target\debug\e10_bad_status.exe` (exit code: 1)
```

- **何が起きたか:** 「CHECK の約束が破られました: `status` は 'queued'・'running'・'done'・'dead' のどれか」というエラーです。`jobs` 表の `status` には `CHECK (status IN (…))` を付けてあるので、決めていない値 `'finished'` は、DB の入口で断られました（第7巻 UNIT 01 の制約）。275 は SQLite の「CHECK の約束違反」のエラー番号です。
- もし `CHECK` が無かったら、`'finished'` の仕事は、`claim` の対象（`queued`）にも、件数を数えるときの `done` にも入らず、**どこにも数えられない仕事**として静かに残り続けます。
- **直し方:** 値を `'done'` に直します。さらに、状態の文字列を SQL のあちこちに書かず、`job_queue` の関数（`complete`・`fail`）の中だけで書くようにすると、打ち間違いが起きる場所が減ります。Rust の側で `enum JobStatus { Queued, Running, Done, Dead }` を作って、文字列への変換を1か所にまとめるのも、よい方法です（第4巻 UNIT 01 の「不正な状態を作れない型」）。

#### エラー3（エラーが出ない間違い）: やり直しの上限と間隔を付けない

例10-3の `max_attempts` と `backoff` を付けずに、「失敗したら、すぐ `queued` に戻す」だけにすると、`broken@` のような仕事は、worker が動いている限り、永遠に、待ち時間なしでやり直され続けます。

- 相手のサーバーには、失敗するとわかっているリクエストが大量に届きます（UNIT 08 の「すぐに、何度でもやり直す」と同じ問題です）。
- worker の時間も、その仕事に取られ続けて、ほかの仕事が遅れます。ログには同じエラーが延々と並び、本当に見るべきエラーが埋もれます。
- **直し方:** 例10-3のとおり、回数の上限（`max_attempts`）と、倍々にのびる待ち時間（`run_at`）を必ず付け、上限に達したら `dead` にして人に知らせます。

### RustNote ではここで使う

RustNote には、バックグラウンドジョブはありません。メールを送る機能が無く、裏で行う仕事も、ほとんど無いからです。唯一の「裏の仕事」に近いものは、期限切れのセッションの掃除です。ログインの処理（`src/auth.rs`）の最後で、次のように行っています。

```rust
    // ついでに、期限切れのセッションを掃除しておく
    state.sessions.delete_expired(SystemTime::now()).await?;
    Ok(())
```

- ログインのリクエストの**中で、ついでに**掃除しています。掃除は短い `DELETE` 1回なので、利用者を待たせる時間はわずかです。このくらいの仕事なら、ジョブの仕組みを作るより、この形のほうが簡単で十分です。
- もし RustNote に「登録したら確認のメールを送る」機能を足すなら、この単元の形の出番です。登録のリクエストでは、ユーザーを保存するのと同じ transaction の中で `jobs` 表に「メールを送る」仕事を入れ、すぐに 303 で答えます（UNIT 08 の PRG）。メールは worker が送り、相手のサーバーが遅くても、失敗しても、登録の画面には影響しません。

もう1つ、この単元の「運用」の目で RustNote を見ると、起動のたびに `main` の中で migration（第7巻 UNIT 06）を適用していることにも気づきます（UNIT 01 の「RustNote ではここで使う」で見たコードです）。

- migration は「1回だけ、すべてのサーバーより先に」行いたい仕事です。Fly.io でマシンを2台以上同時に起動すると、2台が同時に migration を適用しようとして、ぶつかるおそれがあります（引き継ぎの改善候補に記録しています。実際にぶつかるかは、まだ確かめていません）。
- このような「アプリを動かす前に1回だけ行う仕事」は、Fly.io の `release_command`（新しい版を公開する前に1回だけ動かすコマンド）のような、運用の仕組みで行うのがふつうです。第12巻で扱います。

> **ヒント:** worker を、Web サーバーと同じプログラムの中で動かすか（`tokio::spawn` で裏のタスクにする）、別のプログラムとして動かすかも、運用の判断です。同じプログラムなら手軽ですが、Web サーバーを止めると worker も止まります。どちらにしても、止めるときは UNIT 06 の graceful shutdown で、仕事の切れ目まで待ってから止めます（練習問題2）。

### 練習問題

**問1:** 例10-3の `u10_retry_dead.rs` で、3件の仕事の `max_attempts` を `3` から `5` に変えると、`broken@` は何回目で `dead` になり、それは何秒ごろになるでしょうか。待ち時間が 100・200・400・800 ミリ秒とのびることから予想して、実行してください（`src/bin/u10_q1.rs`）。

**問2:** 報告書を作る仕事を10件入れ、1件に 200 ミリ秒かかる worker を1つ動かします。0.5 秒たったら `CancellationToken`（UNIT 06）で止める合図を出し、worker は**今の仕事を終えてから**止まるようにしてください。止まった後に、処理した件数と、残りの件数を表示します（`src/bin/u10_q2.rs`）。

**問3:** 例10-5のように、worker A がメールを送った後、`complete` を記録する前に落ち、持ち時間の後に worker B が同じ仕事をやり直します。メールが2通送られないように、「送った記録」を `sent_emails` 表に残し、同じ仕事では2回目を送らないようにしてください（`src/bin/u10_q3.rs`）。

### 解答と解説

**問1の解答:** `job_queue::enqueue(&conn, "send_email", to, 5)` に変えて実行した結果です。

```text
0.03 秒: #1 ok@example.com 成功（1 回目）
0.07 秒: #2 flaky@example.com 失敗（1 回目）: 時間切れ → queued
0.10 秒: #3 broken@example.com 失敗（1 回目）: 宛先のサーバーが受け取りを拒否しました → queued
0.27 秒: #2 flaky@example.com 失敗（2 回目）: 時間切れ → queued
0.30 秒: #3 broken@example.com 失敗（2 回目）: 宛先のサーバーが受け取りを拒否しました → queued
0.52 秒: #2 flaky@example.com 成功（3 回目）
0.55 秒: #3 broken@example.com 失敗（3 回目）: 宛先のサーバーが受け取りを拒否しました → queued
1.00 秒: #3 broken@example.com 失敗（4 回目）: 宛先のサーバーが受け取りを拒否しました → queued
1.87 秒: #3 broken@example.com 失敗（5 回目）: 宛先のサーバーが受け取りを拒否しました → dead
最後の件数: [("dead", 1), ("done", 2)]
dead: #3 broken@example.com（5 回）最後のエラー: 宛先のサーバーが受け取りを拒否しました
```

- `broken@` は、5回目の失敗で `dead` になりました。3回目の後は 400 ミリ秒、4回目の後は 800 ミリ秒待ったので、4回目は約 1.0 秒、5回目は約 1.9 秒です（ポーリングの 50 ミリ秒の待ちの分だけ、少し遅れます）。
- 上限を増やすと、一時的な失敗には強くなりますが、本当に壊れている仕事が `dead` になるまでの時間も長くなります。倍々にのびるので、上限を 10 にすると、最後の待ちは 100 × 2⁸ = 25.6 秒、待ちの合計は約 51 秒になります。仕事の種類ごとに、「どれくらい待てば直りそうか」で決めます。

**問2の解答:**

```rust
use std::time::{Duration, Instant};

use async_lab::job_queue;
use libsql::Connection;
use tokio_util::sync::CancellationToken;

// 止める合図が来たら、今の仕事を終えてから止まる worker
async fn worker(
    conn: Connection,
    token: CancellationToken,
    start: Instant,
) -> Result<u32, libsql::Error> {
    let mut done = 0;
    // 仕事と仕事の間でだけ、合図を確かめる（仕事の途中では止まらない）
    while !token.is_cancelled() {
        match job_queue::claim(&conn, Duration::from_secs(30)).await? {
            Some(job) => {
                // 1件に 200ms かかる
                tokio::time::sleep(Duration::from_millis(200)).await;
                job_queue::complete(&conn, job.id).await?;
                done += 1;
                println!(
                    "{:.1} 秒: #{} を処理しました",
                    start.elapsed().as_secs_f64(),
                    job.id
                );
            }
            // 仕事が無いときは、少し待つ。待っている間に合図が来たら、すぐに止まる
            None => tokio::select! {
                _ = token.cancelled() => break,
                _ = tokio::time::sleep(Duration::from_millis(50)) => {}
            },
        }
    }
    Ok(done)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10q2");
    std::fs::create_dir_all("u10q2").expect("u10q2 フォルダを作れません");
    let db = job_queue::open("u10q2/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    for n in 1..=10 {
        job_queue::enqueue(&conn, "make_report", &format!("報告書{n}"), 3).await?;
    }

    let start = Instant::now();
    let token = CancellationToken::new();
    let handle = tokio::spawn(worker(job_queue::connect(&db).await?, token.clone(), start));

    // 0.5 秒後に止める合図を出す（Ctrl+C を受け取ったつもり）
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!(
        "{:.1} 秒: 止める合図を出します",
        start.elapsed().as_secs_f64()
    );
    token.cancel();

    let done = handle.await.expect("worker が panic しました")?;
    println!(
        "{:.1} 秒: worker が止まりました（処理した仕事 {done} 件）",
        start.elapsed().as_secs_f64()
    );
    println!("残りの件数: {:?}", job_queue::counts(&conn).await?);
    Ok(())
}
```

```text
0.2 秒: #1 を処理しました
0.4 秒: #2 を処理しました
0.5 秒: 止める合図を出します
0.7 秒: #3 を処理しました
0.7 秒: worker が止まりました（処理した仕事 3 件）
残りの件数: [("done", 3), ("queued", 7)]
```

- 合図は 0.5 秒に出ましたが、そのとき処理中だった #3 は、0.7 秒に最後まで処理されてから止まりました。`while !token.is_cancelled()` で、**仕事と仕事の間でだけ**合図を確かめているからです。UNIT 06 の例6-3のように、途中で止めて中途半端な状態を残すことがありません。
- 残りの7件は `queued` のまま DB に残っているので、次に起動したときに続きから処理されます（例10-2）。
- 仕事が無くて待っているとき（`None` の枝）は、`select!` で合図と競争させているので、すぐに止まれます（UNIT 04 の練習問題3と同じ形）。
- 1件の仕事がとても長い（数分かかる）なら、仕事の途中にも区切りを作って合図を確かめるか、持ち時間を長めにして、止まった後に別の worker がやり直せるようにします。UNIT 06 のとおり、止めるのを待つ時間にも上限を付けておきましょう。

**問3の解答:**

```rust
use std::time::Duration;

use async_lab::job_queue::{self, Job};
use libsql::Connection;

// メールを送る仕事。送った記録を sent_emails 表に残し、同じ仕事では2回送らない
async fn send_email_once(conn: &Connection, job: &Job) -> Result<(), libsql::Error> {
    // job_id は PRIMARY KEY なので、同じ仕事の記録は1つしか入らない
    let inserted = conn
        .execute(
            "INSERT OR IGNORE INTO sent_emails (job_id, address) VALUES (?1, ?2)",
            libsql::params![job.id, job.payload.as_str()],
        )
        .await?;
    if inserted == 1 {
        println!(
            "  {} にメールを送りました（{} 回目）",
            job.payload, job.attempts
        );
    } else {
        println!(
            "  #{} はもう送ってあるので、送りません（{} 回目）",
            job.id, job.attempts
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("u10q3");
    std::fs::create_dir_all("u10q3").expect("u10q3 フォルダを作れません");
    let db = job_queue::open("u10q3/jobs.db").await?;
    let worker_a = job_queue::connect(&db).await?;
    let worker_b = job_queue::connect(&db).await?;
    worker_a
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS sent_emails (
                 job_id INTEGER PRIMARY KEY,
                 address TEXT NOT NULL
             );",
        )
        .await?;
    job_queue::enqueue(&worker_a, "send_email", "hanako@example.com", 3).await?;
    let lease = Duration::from_millis(300);

    // ① A はメールを送ったが、complete を記録する前に落ちた
    println!("① worker A");
    let job = job_queue::claim(&worker_a, lease)
        .await?
        .expect("仕事がある");
    send_email_once(&worker_a, &job).await?;

    // ② 持ち時間が切れたので、B が同じ仕事をやり直す
    tokio::time::sleep(Duration::from_millis(400)).await;
    println!("② worker B");
    let job = job_queue::claim(&worker_b, lease)
        .await?
        .expect("仕事がある");
    send_email_once(&worker_b, &job).await?;
    job_queue::complete(&worker_b, job.id).await?;

    let mut rows = worker_b
        .query("SELECT COUNT(*) FROM sent_emails", ())
        .await?;
    let row = rows.next().await?.expect("COUNT は1行返す");
    println!("送ったメールの数: {}", row.get::<i64>(0)?);
    Ok(())
}
```

```text
① worker A
  hanako@example.com にメールを送りました（1 回目）
② worker B
  #1 はもう送ってあるので、送りません（2 回目）
送ったメールの数: 1
```

- `sent_emails` 表の `job_id` を `PRIMARY KEY` にし、`INSERT OR IGNORE` で記録しています。同じ `job_id` の記録がすでにあれば、`INSERT` は何もせず、変えた行数 0 を返します（第7巻 UNIT 03 の影響行数）。行数が 1 のときだけ「送った」とし、0 なら送りません。
- B は、A が落ちた仕事をやり直しましたが、記録を見て2回目を送りませんでした。送ったメールは1通だけです。UNIT 08 の冪等キーを、仕事の番号（`job_id`）で作った形です。
- ただし、この例では「記録を入れる」と「メールを送る」の順番に注意が要ります。記録を入れた直後、実際に送る前に落ちると、記録だけが残って、メールは1通も送られません。本物のメールの送信では、メールのサービスが冪等キーを受け付けるなら、`job_id` をキーとして渡すのが、いちばん確実です（UNIT 08 の例8-5）。「必ず1回だけ」を作るのは、それほど難しいことなのです。

### まとめ

- 画面の応答と切り離せる仕事はバックグラウンドジョブにする。メモリの列は止めると消えるので、失ってはいけない仕事は DB の表に保存し、再起動の後に続きから処理する。
- 仕事は `UPDATE … WHERE id = (SELECT … LIMIT 1) RETURNING …` の1つの文で取り出し、何人の worker でも二重に取らないようにする。失敗は `run_at` を先にして、間隔を倍々にのばしてやり直し、上限で `dead` にして理由を残す。
- lease で落ちた worker の仕事を取り直せるかわりに、仕事は「少なくとも1回」動くので、中身を冪等にする。worker の数などは設定で決め、止めるときは仕事の切れ目で止まる。
