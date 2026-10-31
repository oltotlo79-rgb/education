## UNIT 10: rollback・backup・障害からの復旧・更新

### この単元のゴール

- 動いている DB のファイルを、そのままコピーすると危ない場面を示し、`VACUUM INTO` で正しくバックアップを取れる
- 突然止まったあとの DB と、バックアップから戻す手順（`-wal`・`-shm` も一緒によける）を試し、間違った戻し方で何が起きるかを読める
- 壊れた版を公開してしまったときに、前の版のイメージに戻す（rollback）手順を、手元の Docker と Fly.io の公式の手順で説明できる。戻らない物（DB・secrets・`fly.toml`）を言える
- 依存の更新を、「別のフォルダで更新 → テスト → 公開 → 確かめる」の順で、安全に進められる

### たとえ話でつかむ

**手帳と付箋**を考えます。

- 大事な手帳（DB）に、新しいことを書くときは、まず**付箋**（`-wal` のファイル）に書いて貼っておき、あとで手帳に清書します。
- 手帳を見る人が、だれもいなくなると、付箋の中身は清書されて、付箋ははがされます。
- だれかが手帳を開いたままだと、付箋は貼られたままです。このとき、**手帳だけをコピー**すると、付箋に書いた分が抜けます。
- **清書係**（`VACUUM INTO`）に頼むと、付箋の分も含めて、清書した写しを1冊作ってくれます。
- 写しから手帳を戻すとき、**古い付箋を貼ったまま**にすると、写しと付箋の話が合わず、手帳がおかしくなります。
- 書き方の説明書（アプリの版）を前の版に戻しても、手帳の中身は戻りません。

| 手帳と付箋 | RustNote |
|---|---|
| 手帳 | `data/rustnote.db` |
| 付箋 | `rustnote.db-wal`（と、目次の `rustnote.db-shm`） |
| 手帳を開いている人 | DB への接続（RustNote のリクエスト・ほかの道具） |
| 清書係の写し | `VACUUM INTO` のバックアップ（例10-1） |
| 古い付箋を貼ったまま戻す | `-wal` を残したまま、`rustnote.db` だけを上書き（エラー） |
| 説明書を前の版に戻す | 前の版のイメージで動かす（rollback。例10-3） |

> **用語:** WAL（ワル。Write-Ahead Logging：書き込みを、まず `-wal` のファイルに書き、あとで本体にまとめて書き戻す、SQLite のしくみ。RustNote は、ローカルの DB を WAL で開く。第7巻 UNIT 05）

<!-- -->

> **用語:** rollback（ロールバック：新しい版に問題があったとき、前の版に戻すこと。この単元では、アプリの版（イメージ）を戻すことを言う。DB の中身は戻らない）

### 図で理解する

ほかの接続が DB を開いている間に、バックアップを取った結果です（例10-1の場面B）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-10a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
<marker id="ar-12-10b" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#16a34a"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">場面B: ほかの接続が DB を開いている間に、バックアップを取る</text>
<rect x="20" y="36" width="240" height="198" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="140" y="56" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">data フォルダ</text>
<rect x="36" y="66" width="208" height="46" rx="6" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.2"/>
<text x="140" y="85" font-size="12" fill="#0f172a" text-anchor="middle">rustnote.db（49,152 バイト）</text>
<text x="140" y="104" font-size="11" fill="#0369a1" text-anchor="middle">ノート 5 件まで</text>
<rect x="36" y="120" width="208" height="46" rx="6" fill="#fef3c7" stroke="#d97706" stroke-width="1.2"/>
<text x="140" y="139" font-size="12" fill="#0f172a" text-anchor="middle">rustnote.db-wal（37,112 バイト）</text>
<text x="140" y="158" font-size="11" fill="#b45309" text-anchor="middle">あとから足した 3 件</text>
<rect x="36" y="174" width="208" height="46" rx="6" fill="#f1f5f9" stroke="#94a3b8" stroke-width="1.2"/>
<text x="140" y="193" font-size="12" fill="#0f172a" text-anchor="middle">rustnote.db-shm（32,768 バイト）</text>
<text x="140" y="212" font-size="11" fill="#475569" text-anchor="middle">-wal の目次</text>
<line x1="248" y1="89" x2="464" y2="85" stroke="#dc2626" stroke-width="1.8" marker-end="url(#ar-12-10a)"/>
<text x="356" y="74" font-size="11" fill="#b91c1c" text-anchor="middle">rustnote.db だけをコピー</text>
<line x1="264" y1="150" x2="464" y2="183" stroke="#16a34a" stroke-width="1.8" marker-end="url(#ar-12-10b)"/>
<text x="372" y="150" font-size="11" fill="#15803d" text-anchor="middle">VACUUM INTO（-wal も読む）</text>
<rect x="470" y="50" width="230" height="70" rx="8" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
<text x="585" y="77" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">copyB.db</text>
<text x="585" y="101" font-size="12" fill="#b91c1c" text-anchor="middle">notes: 5 行（3 件が抜けた）</text>
<rect x="470" y="150" width="230" height="70" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="585" y="177" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">backup.db</text>
<text x="585" y="201" font-size="12" fill="#15803d" text-anchor="middle">notes: 8 行（全部）</text>
<rect x="20" y="246" width="680" height="46" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="265" font-size="12" fill="#1d4ed8">ほかの接続が開いている間は、-wal が消えない。ファイルのコピーでは、-wal の分が抜ける</text>
<text x="34" y="284" font-size="12" fill="#1d4ed8">VACUUM INTO は SQLite を通して読むので、ある瞬間の DB 全体を、1つのファイルに写す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: ファイルのコピーと VACUUM INTO。足した3件は、まだ -wal の中にあった</div>
</div>

壊れた版を出してから、前の版に戻すまでです（例10-3）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-10c" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">v1 → v2 → v1（DB は、同じボリュームのまま）</text>
<rect x="20" y="40" width="180" height="96" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="110" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">rustnote:v1</text>
<text x="110" y="92" font-size="12" fill="#15803d" text-anchor="middle">スモークテスト</text>
<text x="110" y="112" font-size="12" fill="#15803d" text-anchor="middle">全部 OK</text>
<rect x="270" y="40" width="180" height="96" rx="8" fill="#fee2e2" stroke="#dc2626" stroke-width="1.5"/>
<text x="360" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">rustnote:v2</text>
<text x="360" y="92" font-size="12" fill="#0f172a" text-anchor="middle">/health は 200</text>
<text x="360" y="112" font-size="12" fill="#b91c1c" text-anchor="middle">一覧だけ 500（NG 1つ）</text>
<rect x="520" y="40" width="180" height="96" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="610" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">rustnote:v1 に戻す</text>
<text x="610" y="92" font-size="12" fill="#15803d" text-anchor="middle">スモークテスト</text>
<text x="610" y="112" font-size="12" fill="#15803d" text-anchor="middle">全部 OK</text>
<line x1="202" y1="88" x2="266" y2="88" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-12-10c)"/>
<text x="235" y="80" font-size="11" fill="#475569" text-anchor="middle">入れ替え</text>
<line x1="452" y1="88" x2="516" y2="88" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-12-10c)"/>
<text x="485" y="80" font-size="11" fill="#475569" text-anchor="middle">戻す</text>
<line x1="110" y1="136" x2="110" y2="164" stroke="#64748b" stroke-width="1.2" stroke-dasharray="4 3"/>
<line x1="360" y1="136" x2="360" y2="164" stroke="#64748b" stroke-width="1.2" stroke-dasharray="4 3"/>
<line x1="610" y1="136" x2="610" y2="164" stroke="#64748b" stroke-width="1.2" stroke-dasharray="4 3"/>
<rect x="20" y="164" width="680" height="40" rx="6" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="360" y="189" font-size="12" fill="#0369a1" text-anchor="middle">ボリューム rn-u10data（/data/rustnote.db）: 入れ替えても、戻しても、同じ DB のまま</text>
<rect x="20" y="222" width="680" height="66" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="244" font-size="12" fill="#1d4ed8">ヘルスチェック（/health）は v2 でも通る → 公開は「成功」してしまう。スモークテストで気づく</text>
<text x="34" y="264" font-size="12" fill="#1d4ed8">Fly.io では: fly releases --image で前のイメージを探し、fly deploy --image で戻す</text>
<text x="34" y="282" font-size="11" fill="#475569">戻るのはイメージだけ。DB（マイグレーション）・secrets・fly.toml は戻らない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: 前の版のイメージに戻す。アプリの版は戻るが、DB の中身はそのまま</div>
</div>

### まず動かす最小コード

#### 例10-1: 動いている DB の、バックアップを取る

release の RustNote を、ローカルのファイルの DB（`data/rustnote.db`）で動かし、登録してノートを5件作りました。そのあと、2つの場面で、「`rustnote.db` だけのコピー」と「`VACUUM INTO` の写し」を比べました。

- **場面A:** RustNote だけが動いていて、いまはだれもリクエストを送っていない
- **場面B:** ほかの道具が、DB に接続を開いたままにしている（この例では `u10_hold` で作る）。その間にノートを3件足す

使った小さなプログラムは、次の3つです。どれも RustNote の `Db::open_local`（第13巻 UNIT 02）で DB を開きます。

`examples/u10_backup.rs`（`VACUUM INTO` で写す）:

```rust
//! 第12巻 UNIT 10: 動いている RustNote の DB を、VACUUM INTO で、1つのファイルに写し取る（バックアップ）。
//! 実行: cargo run -q --example u10_backup -- 元の.db 写し.db

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(from), Some(to)) = (args.next(), args.next()) else {
        anyhow::bail!("使い方: u10_backup 元の.db 写し.db");
    };
    let db = Db::open_local(&from).await?;
    let conn = db.connect().await?;
    // VACUUM INTO は、ほかの接続が書いている途中でも、ある瞬間の DB 全体を1つのファイルに書き出す
    conn.execute("VACUUM INTO ?1", [to.as_str()]).await?;
    println!("{from} を {to} に写しました");
    Ok(())
}
```

`examples/u10_hold.rs`（接続を開いたままにする）:

```rust
//! 第12巻 UNIT 10: DB のファイルに、接続を1つ開いたままにする（ほかの道具が DB を開いている状態を作る）。
//! 実行: cargo run -q --example u10_hold -- 調べる.db 秒数

use std::time::Duration;

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "rustnote.db".to_string());
    let seconds: u64 = args.next().unwrap_or_else(|| "20".to_string()).parse()?;
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let count = match rows.next().await? {
        Some(row) => row.get::<i64>(0)?,
        None => 0,
    };
    drop(rows);
    println!("notes: {count} 行。接続を開いたまま {seconds} 秒待ちます");
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    drop(conn);
    Ok(())
}
```

`examples/u10_count.rs`（数えて、壊れていないかも確かめる）:

```rust
//! 第12巻 UNIT 10: DB のファイルの中の、ユーザーとノートの数を数え、壊れていないかも確かめる（バックアップを確かめる）。
//! 実行: cargo run -q --example u10_count -- 調べる.db

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| "rustnote.db".to_string());
    let db = Db::open_local(&path).await?;
    let conn = db.connect().await?;
    for table in ["users", "notes"] {
        let mut rows = conn
            .query(&format!("SELECT COUNT(*) FROM {table}"), ())
            .await?;
        let count = match rows.next().await? {
            Some(row) => row.get::<i64>(0)?,
            None => 0,
        };
        println!("{table}: {count} 行");
    }
    // 第7巻 UNIT 10: 壊れていなければ「ok」の1行。壊れていれば、見つかった問題が1行ずつ返る
    let mut rows = conn.query("PRAGMA integrity_check", ()).await?;
    let mut problems = Vec::new();
    while let Some(row) = rows.next().await? {
        problems.push(row.get::<String>(0)?);
    }
    match problems.as_slice() {
        [only] if only == "ok" => println!("integrity_check: ok"),
        _ => {
            println!("integrity_check: 問題が {} 件", problems.len());
            for problem in problems.iter().take(3) {
                println!("  {problem}");
            }
        }
    }
    Ok(())
}
```

```powershell
# 1つ目の PowerShell（RustNote のフォルダ）: release の RustNote を、ローカルのファイルの DB で動かす
$env:DATABASE_URL = "data/rustnote.db"
.\target\release\rustnote.exe
```

```powershell
# 2つ目の PowerShell（同じフォルダ。場面B）: 接続を 60 秒開いたままにする（その間に、ブラウザでノートを3件足す）
cargo run -q --example u10_hold -- data\rustnote.db 60
# rustnote.db だけをコピーする
Copy-Item data\rustnote.db copyB.db
# VACUUM INTO で写す
cargo run -q --example u10_backup -- data\rustnote.db backup.db
# 数える
cargo run -q --example u10_count -- copyB.db
cargo run -q --example u10_count -- backup.db
```

```text
--- 場面A: data フォルダ（ノートを5件作ったあと）
  rustnote.db: 49152 バイト
--- rustnote.db だけをコピーしたもの（copyA.db）
  users: 1 行
  notes: 5 行
  integrity_check: ok
```

```text
--- 場面B: ほかの接続（u10_hold）
  notes: 5 行。接続を開いたまま 60 秒待ちます
--- 場面B: data フォルダ（ノートをもう3件、全部で8件作ったあと）
  rustnote.db: 49152 バイト
  rustnote.db-shm: 32768 バイト
  rustnote.db-wal: 37112 バイト
--- rustnote.db だけをコピーしたもの（copyB.db）
  users: 1 行
  notes: 5 行
  integrity_check: ok
--- VACUUM INTO で写したもの（backup.db）
  data/rustnote.db を backup.db に写しました
  users: 1 行
  notes: 8 行
  integrity_check: ok
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 場面A では、コピーでも足りた

- RustNote は、リクエストのたびに DB の接続を作り、終わると閉じます（`Db::connect`。第13巻 UNIT 02）。最後の接続が閉じると、SQLite は `-wal` の中身を本体に書き戻し、`-wal` を消します。
- だから場面A では、`data` フォルダに `rustnote.db` しか無く、コピーにも5件全部が入っていました。

##### 解説2: 場面B では、コピーから3件が抜けた

- ほかの接続（`u10_hold`）が開いている間は、`-wal` が消えません。足した3件は、まだ `rustnote.db-wal`（37,112 バイト）の中にあり、`rustnote.db` の大きさは 49,152 バイトのままでした。
- `rustnote.db` だけをコピーすると、`notes` は **5 行**で、3件が抜けました。エラーも出ないので、戻すときまで気づきません。
- `VACUUM INTO` は、SQLite を通して読むので、`-wal` の中身も含めた、ある瞬間の DB 全体を、1つのファイルに書き出します。写しには **8 行**ありました。
- 本番では、ほかの道具（DB を見る道具・バックアップの道具・2つ目のプロセス）が DB を開いていることがあります。ファイルのコピーで済ませず、`VACUUM INTO` を使います（第7巻 UNIT 10 でも、写しを `PRAGMA integrity_check` で確かめました）。

##### 解説3: Turso のときのバックアップ

- 本番の RustNote の DB は Turso にあり、マシンの中にファイルはありません。Turso の公式のドキュメントで確かめた方法は、次の2つです（この巻では実行しません）。
- **過去の時刻に戻した、新しい DB を作る（point-in-time recovery）:** `turso db create 新しい名前 --from-db 元の名前 --timestamp 2026-10-31T00:00:00Z`。戻せるのは、Free プランで直近の 24 時間、Developer プランで 10 日です。新しい DB ができるので、URL とトークンを入れ替えます（UNIT 05）。
- **SQL に書き出す:** `turso db shell 名前 .dump > dump.sql` で、DB を作り直せる SQL を書き出します。読み込むときは `turso db shell 新しい名前 < dump.sql` です。

### もう一歩: 実務に近い形

#### 例10-2: 突然止まったあとの DB と、バックアップからの戻し方

例10-1の場面Bのあとに、ノートをもう2件（全部で10件）作り、RustNote と `u10_hold` を、強制終了しました（マシンが急に落ちた代わりです）。

```text
--- 止まったあとの data フォルダ
  rustnote.db: 49152 バイト
  rustnote.db-shm: 32768 バイト
  rustnote.db-wal: 61832 バイト
--- 止まったあとの DB（-wal も一緒にあるもの）を数える
  users: 1 行
  notes: 10 行
  integrity_check: ok
```

- 強制終了なので、`-wal` が残りました（61,832 バイト）。それでも、3つのファイルがそろったまま開くと、SQLite が `-wal` を読んで、**10 行**全部が見えました。`-wal` は DB の一部です。落ちたあとに、「ゴミのファイル」だと思って消してはいけません。

次に、バックアップ（8件のとき）に戻します。古い `rustnote.db`・`-wal`・`-shm` の3つを、全部 `old` によけてから、`backup.db` を `rustnote.db` として置きました。

```text
--- 正しい戻し方: 古い rustnote.db・-wal・-shm を old/ によけて、backup.db を rustnote.db として置く
  rustnote.db: 49152 バイト
--- そのフォルダで RustNote を起動して、ログインと一覧
  ログイン: 303・一覧: 200 全8件
```

- ログインでき、一覧は **全8件**でした。バックアップのあとに作った2件（memo9・memo10）は戻りません。**バックアップは、取った時刻までしか戻せません**。どれだけ失ってよいか（1日分か、1時間分か）から、バックアップを取る間かくを決めます。

#### 例10-3: 壊れた版を公開してしまい、前の版のイメージに戻す

UNIT 02 で作ったイメージを `rustnote:v1` とし、一覧の SQL の列の名前を1文字まちがえた `rustnote:v2` を作りました。

```diff
--- a/src/repository/notes.rs
+++ b/src/repository/notes.rs
@@ -109,7 +109,7 @@
         // 2) このページに表示する分（新しく更新した順）
         let list_sql = format!(
             "SELECT {NOTE_COLUMNS} FROM notes WHERE {condition}
-             ORDER BY updated_at DESC, id DESC
+             ORDER BY update_at DESC, id DESC
              LIMIT ?3 OFFSET ?4"
         );
         let offset = i64::try_from(query.offset()).unwrap_or(i64::MAX);
```

v2 は、UNIT 02 と同じ `Dockerfile` で `docker build` して作りました（82 秒）。2つのイメージは、別々の ID で残っています。

```text
rustnote:v2  3402c7502d10  170MB
rustnote:v1  b8305d2bc79f  170MB
```

DB は、`rn-u10data` というボリューム（`/data`）に置き、v1 → v2 → v1 の順に、同じボリュームで動かして、UNIT 08 のスモークテストをかけました。

```powershell
docker tag rustnote:u02 rustnote:v1
docker build -t rustnote:v2 .
docker run -d --name rn-u10 -p 18080:8080 -v rn-u10data:/data rustnote:v1
powershell -File smoke.ps1 -Base http://127.0.0.1:18080
# v2 に入れ替える（v1 のコンテナを消して、同じボリュームで v2 を動かす）
docker rm -f rn-u10
docker run -d --name rn-u10 -p 18080:8080 -v rn-u10data:/data rustnote:v2
powershell -File smoke.ps1 -Base http://127.0.0.1:18080
# 前の版に戻す
docker rm -f rn-u10
docker run -d --name rn-u10 -p 18080:8080 -v rn-u10data:/data rustnote:v1
powershell -File smoke.ps1 -Base http://127.0.0.1:18080
```

v1 のとき:

```text
OK   GET /health（200）
OK   GET /（トップ）（200）
OK   POST /signup（登録）（303）
OK   GET /notes（一覧）（200）
OK   POST /notes（作成）（303）
OK   GET 作ったノート（200）
OK   POST /logout（303）
全部 OK
（終了コード: 0）
```

v2 に入れ替えたとき:

```text
GET /health: 200
OK   GET /health（200）
OK   GET /（トップ）（200）
OK   POST /signup（登録）（303）
NG   GET /notes（一覧）（期待 200・実際 500）
OK   POST /notes（作成）（303）
OK   GET 作ったノート（200）
OK   POST /logout（303）
だめだった数: 1
（終了コード: 1）
```

v2 のログ:

```text
[error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: `no such column: update_at`
GET /notes -> 500 Internal Server Error (1.479014ms)
```

v1 に戻したとき:

```text
OK   GET /health（200）
OK   GET /（トップ）（200）
OK   POST /signup（登録）（303）
OK   GET /notes（一覧）（200）
OK   POST /notes（作成）（303）
OK   GET 作ったノート（200）
OK   POST /logout（303）
全部 OK
（終了コード: 0）
```

##### 解説: ヘルスチェックは通ったのに、一覧が壊れていた

- v2 でも `/health` は 200 でした。`/health` は `SELECT 1` しか確かめないので（UNIT 06）、Fly.io なら、公開は「成功」して終わります。気づけたのは、スモークテストの「一覧」が 500 だったからです。ログには `no such column: update_at`（そんな列は無い）が出ていました。
- 前の版のイメージ `rustnote:v1` を、同じボリュームで動かし直すと、全部 OK に戻りました。作り直し（ビルド）は要らないので、すぐに戻せます。
- このまちがいは、`cargo test` で見つかりました。v2 のフォルダでテストを動かすと、4つのテストが失敗しました。公開する前に、テスト（第11巻の CI）を通していれば、出さずに済みました。

```text
     Running unittests src\lib.rs 
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\main.rs 
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\http.rs 
test empty_title_is_400 ... FAILED
test list_searches_and_paginates ... FAILED
test login_and_logout ... FAILED
test result: FAILED. 11 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.07s
error: test failed, to rerun pass `--test http`
     Running tests\repository.rs 
test search_filters_by_keyword_and_paginates ... FAILED
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s
error: test failed, to rerun pass `--test repository`
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
error: 2 targets failed:
```

#### Fly.io で、前の版に戻す（公式のドキュメントで確かめた手順。この巻では実行しません）

```powershell
# 1. これまでの公開と、そのときのイメージを並べる
fly releases --image
# 2. 問題が起きる前のイメージを、もう一度公開する（作り直しはしない）
fly deploy --image registry.fly.io/rustnote-example:deployment-XXXXXXXX
```

- `fly releases` の `--image` は、それぞれの公開のイメージの名前を表示します。`fly deploy --image` は、ビルドをせず、指定したイメージでマシンを入れ替えます。
- 公式のドキュメント（rollback の手引き）によると、戻るのは**イメージだけ**です。DB（マイグレーション）・secrets・`fly.toml` の設定は戻りません。古いイメージは、今の `fly.toml` と secrets で動きます。
- Fly.io は、古いイメージをいつまでも取っておくとは約束していません。確実に戻れるようにするには、`fly deploy --image-label v1.2.3` のように名前を付けて公開するか、自分のレジストリ（GHCR など）にもイメージを置いておきます。
- 戻したあとも、`fly logs` とスモークテスト（UNIT 08）で、本当に直ったかを確かめます。

#### DB の形を変えた版は、イメージを戻しても直らない

- UNIT 07 のエラーでは、列の名前を変えるマイグレーション（`RENAME COLUMN`）のあとで古い版を動かすと、一覧が 500 になりました。イメージを前の版に戻しても、DB の列の名前は戻らないので、**戻した先の古い版が壊れます**。
- だから、マイグレーションは「足すだけ」で出し、古い版がそのまま動ける形にしておきます（UNIT 07 の「広げてから、縮める」）。そうしておけば、いつでもイメージを戻せます。

#### 例10-4: 依存の更新を、別のフォルダで試す

まず、何が新しくなるかを、`Cargo.lock` を書き換えずに見ます。

```powershell
cargo update --dry-run
```

```text
    Updating crates.io index
     Locking 1 package to highest Rust 1.99 compatible version
    Updating log v0.4.34 -> v0.4.35
note: pass `--verbose` to see 1 unchanged dependencies behind latest
warning: not updating lockfile due to dry run
```

新しくなるのは `log` の1つ（0.4.34 → 0.4.35）だけでした。本物のフォルダの `Cargo.lock` は変えずに、コピーしたフォルダで `cargo update` をかけて、テストを通しました。

```powershell
# コピーしたフォルダで
cargo update
cargo test
```

```text
    Updating crates.io index
     Locking 1 package to highest Rust 1.99 compatible version
    Updating log v0.4.34 -> v0.4.35
note: pass `--verbose` to see 1 unchanged dependencies behind latest
```

`Cargo.lock` の、変わった行です。

```diff
-version = "0.4.34"
+version = "0.4.35"
-checksum = "f9f8bd3e56ce4dfc153cf470fffbfa98c7620958b312ca5c3a4b8d5181fd13c6"
+checksum = "5d15f86d12da44cd8d4ec3c73984537eb7f4cadcb9776a575a0a8a05ca11005c"
```

```text
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.08s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
（cargo test にかかった時間: 39 秒）
```

- テストは全部通りました（19・14・8 件）。このあと、本物のフォルダで同じ更新をしてコミットし、CI → 公開 → スモークテストの順に進めます。公開してから問題が見つかっても、例10-3のとおり、前のイメージに戻せます。
- 弱点の知らせ（`cargo audit`）も見ました。結果は、第11巻 UNIT 10 と同じ、8件の弱点の知らせでした（`h2`・`rustls`・`rustls-webpki`。どれも libsql 0.10.0 の中で使われている部品です）。

```text
    Scanning Cargo.lock for vulnerabilities (367 crate dependencies)
error: 8 vulnerabilities found!
warning: 2 allowed warnings found
```

```text
libsql = "0.10.0"    # The libSQL database library
```

- crates.io の libsql は、今も 0.10.0 が最新でした（`cargo search libsql`）。`cargo update` では直せないので、第11巻 UNIT 10 のとおり、理由と見直す日を書いて受け入れ、`cargo audit` を毎日動かして、libsql の新しい版を見張ります。

### よくあるエラーと読み方

#### エラー: `-wal` を残したまま、`rustnote.db` だけを上書きして戻した

例10-2と同じ、強制終了のあとのフォルダで、`rustnote.db` だけを `backup.db` で上書きしました。`-wal` と `-shm` は残したままです。

```text
--- 間違った戻し方: rustnote.db だけを backup.db で上書き（-wal・-shm は残したまま）
  rustnote.db: 49152 バイト
  rustnote.db-shm: 32768 バイト
  rustnote.db-wal: 61832 バイト
--- 数える
  users: 1 行
  notes: 8 行
  Error: SQLite failure: `database disk image is malformed`
--- そのフォルダで RustNote を起動して、ログインと一覧
  ログイン: 500・一覧: 303 
--- RustNote のログ
  [error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: `database disk image is malformed`
  POST /login -> 500 Internal Server Error (29.2681ms)
  GET /notes -> 303 See Other (2.9792ms)
  GET /notes -> 303 See Other (1.9772ms)
```

- **何が起きたか:** 数えると、users 1 行・notes 8 行で、一見だいじょうぶに見えました。ところが `PRAGMA integrity_check` で `database disk image is malformed`（DB のファイルの形が壊れている）になりました。RustNote でも、ログインが 500 になりました。
- **なぜ:** SQLite は、DB を開くとき、隣にある `rustnote.db-wal` を「この DB の、まだ書き戻していない分」として読みます。残っていたのは古い DB の `-wal` なので、写しの上に古い DB の書き込みが重なり、形が合わなくなりました。SQLite の公式のドキュメント（How To Corrupt An SQLite Database File）にも、DB のファイルを別のファイルで上書きするときに、元の DB の書きかけの記録を消さないと壊れやすい、とあります。
- **直し方:** 戻すときは、`rustnote.db`・`-wal`・`-shm` の3つを全部よけてから、写しを置きます（例10-2）。よけた3つは、すぐに消さずに取っておきます（あとで、そこから取り出したい行があるかもしれません）。戻したら、件数だけでなく `integrity_check` も確かめます。

### 復旧の手順書

障害が起きたときに、慌てずに動けるよう、先に表にしておきます（第7巻 UNIT 10 の「気づく → 止める → 調べる → 戻す → 確かめる → 振り返る」）。

| 起きたこと | まずすること | 戻す方法 | 確かめ |
|---|---|---|---|
| 新しい版で、一部のページが 500 | ログ（`fly logs`）でエラーを読む | 前の版のイメージ（`fly deploy --image`） | スモークテスト（UNIT 08） |
| 新しい版が起動しない・ヘルスチェックが通らない | Fly.io は、新しいマシンに振り分けない（UNIT 06）。ログを読む | 前の版のイメージ | `/health` とスモークテスト |
| Turso の DB のデータを消してしまった | 書き込みを止める（マシンを止める） | point-in-time recovery で、新しい DB を作り、URL とトークンを入れ替える（UNIT 05） | 件数と、スモークテスト |
| ローカルのファイルの DB が壊れた | アプリを止める。`-wal`・`-shm` も含めて、3つのファイルをよけて取っておく | `VACUUM INTO` の写しを置く（例10-2） | `integrity_check` と、ログインと一覧 |
| 依存に弱点の知らせ（`cargo audit`） | 弱点の中身と、直った版があるかを読む（第11巻 UNIT 10） | 別のフォルダで更新 → テスト → 公開（例10-4） | スモークテスト |
| トークンが漏れた | Turso のトークンを無効にする（UNIT 05 の順番） | 新しいトークンを secrets に入れる | ログインと一覧 |

### RustNote ではここで使う

- **バックアップ:** 本番の DB は Turso なので、point-in-time recovery（Free は 24 時間）と、`turso db shell … .dump` を使います。ローカルの DB は、`VACUUM INTO` で写します。
- **rollback:** 公開ごとのイメージは `fly releases --image` で見られます。RustNote のマイグレーションは、これまで全部「足すだけ」（`CREATE TABLE`）なので、前の版のイメージに戻しても動きます。
- **更新:** Rust の版は `rust-toolchain.toml` と `Dockerfile` の2か所、Topcoat の版は `Cargo.toml` と `Dockerfile`（topcoat CLI）の2か所に書いてあります。更新するときは、組で直します。

```text
rust-toolchain.toml: channel = "1.99.1"
Dockerfile:          FROM rust:1.99.1-trixie AS builder
Dockerfile:          RUN cargo install topcoat-cli --version 0.11.0 --locked
Cargo.toml:          topcoat = "=0.11.0"
Cargo.toml:          rust-version = "1.99"
```

### 練習問題

**問1:** 例10-1の場面A では、`rustnote.db` だけのコピーでも、5件全部が入っていました。それなら、RustNote しか DB を開いていない間は、ファイルのコピーでバックアップしてよいでしょうか。

**問2:** 例10-3で、v2 を公開していた間に、利用者が新しいノートを作りました。v1 に戻したあと、そのノートはどうなるでしょうか。

**問3:** Turso の DB で、まちがえて、ある人のノートを全部消してしまいました。消したのは 30 分前です。戻す手順と、気をつけることを書いてください。

### 解答と解説

**問1の解答:**

- よくありません。コピーしている、ちょうどその時にリクエストが来て書き込みが起きると、その間だけ `-wal` ができ、本体のファイルも書き換わっています。コピーが、書き込みの途中の形になるかもしれません。
- SQLite の公式のドキュメント（How To Corrupt An SQLite Database File）でも、ファイルのコピーで安全なのは、書き込みの途中でないときだけとされています。動いている DB でも安全な方法として、`VACUUM INTO` が挙げられています。
- `VACUUM INTO` は、SQLite を通して、ある瞬間の DB 全体を写すので、いつ取っても形がそろいます。取った写しは、`integrity_check` と件数で確かめます（`u10_count`）。

**問2の解答:**

- 残ります。イメージを戻しても、DB（ボリュームや Turso）の中身は変わらないからです。例10-3でも、v2 の間にスモークテストが作ったユーザーとノートは、同じボリュームの DB に書かれています。
- ただし v2 が、v1 の知らない列や表に書いていた場合は、その分を v1 は使いません。だから新しい列は「足すだけ」にして、v1 がそのまま動けるようにしておきます（UNIT 07）。

**問3の解答（例）:**

1. 消した時刻より前（たとえば 35 分前）を、UTC で決めます。
2. `turso db create rustnote-restore --from-db rustnote --timestamp 2026-10-31T01:25:00Z` のように、その時刻の、新しい DB を作ります。元の DB は、そのまま残ります。
3. 新しい DB のトークンを作り、`fly secrets set` で、`DATABASE_URL` と `DATABASE_AUTH_TOKEN` を入れ替えます（UNIT 05。マシンが動かし直されます）。
4. スモークテストと、その人のノートの数で確かめます。

- **気をつけること:** 戻した時刻のあとに、ほかの人が書いたノートは、新しい DB にはありません。元の DB から、その分だけを取り出して足すかを決めます（第7巻 UNIT 10 の `ATTACH` の考え方）。Free プランでは、戻せるのは 24 時間以内です。古い DB は、確かめ終わってから消します。

### まとめ

- 動いている DB の `rustnote.db` だけをコピーすると、ほかの接続が開いている間は、`-wal` にある分（例では3件）が抜けた。`VACUUM INTO` の写しには、全部（8件）入っていた。
- 突然止まったあとは `-wal` が残るが、3つそろったまま開けば全部（10件）見えた。戻すときは、`rustnote.db`・`-wal`・`-shm` を全部よけてから写しを置く。`-wal` を残したまま上書きすると、DB が壊れた（`database disk image is malformed`）。
- 壊れた版は、前の版のイメージで動かせば戻る。DB・secrets・`fly.toml` は戻らないので、マイグレーションは「足すだけ」にしておく。Fly.io では `fly releases --image` と `fly deploy --image`。
- 依存の更新は、`--dry-run` で見て、別のフォルダで更新し、テストを通してから公開する。版を書く場所が2か所ある物は、組で直す。
