## UNIT 08: local・remote・embedded replica

### この単元のゴール

- DB の置き場所の3つの形（**ローカル**・**リモート**・**組み込みレプリカ**）について、データがどこにあり、読み書きがどこへ行くかを図で説明できる
- アプリを動かす場所（自分の PC・サーバー1台・サーバー複数台）に合わせて、どの形を選ぶかを判断できる
- DB の URL とトークンをコードに書かず、**環境変数**から読んで形を選ぶプログラムを書ける。トークンを画面やログに出さない
- 接続の失敗（`os error 10061`・`Hrana`）、`sync` の使い間違い、features の不足（E0599）、debug ビルドのスタックあふれを、メッセージから見分けられる

### たとえ話でつかむ

お店の「お客さま台帳」で考えます。

- **ローカル（自分の手帳）:** 台帳を、自分の机の手帳に書きます。見るのも書くのも一瞬で、電話もいりません。ただし、ほかのお店（別のサーバー）からは見えません。
- **リモート（本社に電話）:** 台帳は本社に1冊だけあります。見るときも書くときも、毎回本社に電話します。どのお店からでも同じ台帳を使えますが、電話のたびに少し待ちます。
- **組み込みレプリカ（本社の台帳のコピーを手元に置く）:** 本社の台帳の**コピー**を手元に置きます。見るときは手元のコピーを見るので速く、書くときは本社に電話します。ほかのお店が書いた分は、ときどき本社から**最新のコピーを取り寄せる**（これが `sync`）まで、手元には届きません。

どれが一番よいかは、お店の数と、見る回数と書く回数のどちらが多いかで決まります。この単元では、3つの形を実際に開いて、違いと失敗のしかたを確かめます。

> **用語:** 組み込みレプリカ（embedded replica：Turso の DB のコピーを、アプリと同じ PC のファイルに置く形。読み込みは手元のコピーから、書き込みは Turso の本体へ送られる）

<!-- -->

> **用語:** primary（プライマリ：コピーの元になる本体の DB。組み込みレプリカの形では、Turso の側にある DB のこと）

<!-- -->

> **用語:** sync（シンク：同期。primary の変更を取り寄せて、手元のコピーを最新にすること）

<!-- -->

> **用語:** 環境変数（プログラムの外から渡す設定の値。`DATABASE_URL` のように名前と値の組で、PowerShell では `$env:名前 = "値"` で設定する）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-081" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="120" y="26" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">① ローカル</text>
<text x="360" y="26" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">② リモート</text>
<text x="600" y="26" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">③ 組み込みレプリカ</text>
<rect x="15" y="40" width="210" height="154" rx="10" fill="none" stroke="#94a3b8" stroke-dasharray="6 4"/>
<rect x="255" y="40" width="210" height="154" rx="10" fill="none" stroke="#94a3b8" stroke-dasharray="6 4"/>
<rect x="495" y="40" width="210" height="154" rx="10" fill="none" stroke="#94a3b8" stroke-dasharray="6 4"/>
<text x="23" y="56" font-size="10" fill="#64748b">アプリが動く PC</text>
<text x="263" y="56" font-size="10" fill="#64748b">アプリが動く PC</text>
<text x="503" y="56" font-size="10" fill="#64748b">アプリが動く PC</text>
<rect x="50" y="64" width="140" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="88" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリ（Rust）</text>
<rect x="290" y="64" width="140" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="88" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリ（Rust）</text>
<rect x="530" y="64" width="140" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="600" y="88" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリ（Rust）</text>
<rect x="40" y="140" width="160" height="42" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="158" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">手元のファイル</text>
<text x="120" y="174" font-size="10" fill="#475569" text-anchor="middle">u08_local.db</text>
<line x1="95" y1="138" x2="95" y2="106" stroke="#3b82f6" stroke-width="2" marker-end="url(#ar-07-081)"/>
<line x1="145" y1="104" x2="145" y2="136" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-081)"/>
<text x="88" y="126" font-size="11" fill="#1e40af" text-anchor="end">読む</text>
<text x="152" y="126" font-size="11" fill="#c2410c">書く</text>
<text x="120" y="250" font-size="11" fill="#64748b" text-anchor="middle">（ネットにはつながない）</text>
<rect x="280" y="232" width="160" height="48" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="252" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Turso（primary）</text>
<text x="360" y="270" font-size="10" fill="#475569" text-anchor="middle">ネットの向こうの本体</text>
<line x1="335" y1="230" x2="335" y2="106" stroke="#3b82f6" stroke-width="2" marker-end="url(#ar-07-081)"/>
<line x1="385" y1="104" x2="385" y2="228" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-081)"/>
<text x="328" y="160" font-size="11" fill="#1e40af" text-anchor="end">読む</text>
<text x="392" y="160" font-size="11" fill="#c2410c">書く</text>
<rect x="505" y="140" width="120" height="42" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="565" y="158" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">手元のコピー</text>
<text x="565" y="174" font-size="10" fill="#475569" text-anchor="middle">replica.db</text>
<line x1="565" y1="138" x2="565" y2="106" stroke="#3b82f6" stroke-width="2" marker-end="url(#ar-07-081)"/>
<text x="558" y="126" font-size="11" fill="#1e40af" text-anchor="end">読む</text>
<line x1="650" y1="104" x2="650" y2="228" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-081)"/>
<text x="657" y="126" font-size="11" fill="#c2410c">書く</text>
<rect x="520" y="232" width="160" height="48" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="252" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Turso（primary）</text>
<text x="600" y="270" font-size="10" fill="#475569" text-anchor="middle">ネットの向こうの本体</text>
<line x1="565" y1="230" x2="565" y2="186" stroke="#10b981" stroke-width="2" stroke-dasharray="5 3" marker-end="url(#ar-07-081)"/>
<text x="572" y="212" font-size="11" font-weight="700" fill="#047857">sync</text>
<text x="572" y="225" font-size="9" fill="#475569">（コピーを最新に）</text>
<text x="120" y="304" font-size="11" fill="#0f172a" text-anchor="middle">速い・ネットが無くても動く</text>
<text x="120" y="321" font-size="11" fill="#64748b" text-anchor="middle">データはその PC の中だけ</text>
<text x="360" y="304" font-size="11" fill="#0f172a" text-anchor="middle">どのサーバーからも同じ DB</text>
<text x="360" y="321" font-size="11" fill="#64748b" text-anchor="middle">読むたびにネットを往復</text>
<text x="600" y="304" font-size="11" fill="#0f172a" text-anchor="middle">読むのは手元なので速い</text>
<text x="600" y="321" font-size="11" fill="#64748b" text-anchor="middle">書くのは Turso へ・sync が必要</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: DB の置き場所の3つの形。点線の枠がアプリの動く PC。ローカルは読み書きとも手元、リモートは読み書きとも Turso、組み込みレプリカは読むのが手元のコピーで書くのが Turso</div>
</div>

組み込みレプリカでは、「誰が書いた行が、いつ見えるか」に注意が必要です。Turso の公式ドキュメントでは、次のように説明されています。

- 書き込みは、まず Turso の primary に送られます。書き込みが成功すると、**書いたアプリ自身の手元のコピーには、すぐに反映されます**。これを **read-your-writes**（自分の書き込みは、すぐ読める）と言います。libsql では最初から有効です。
- **ほかのアプリ（別のサーバー）の手元のコピー**には、そのアプリが `sync` するまで届きません。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-082" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="10" y="54" font-size="12" font-weight="700" fill="#1e40af">サーバー A</text>
<text x="10" y="70" font-size="10" fill="#475569">（手元のコピー）</text>
<text x="10" y="124" font-size="12" font-weight="700" fill="#047857">Turso</text>
<text x="10" y="140" font-size="10" fill="#475569">（primary）</text>
<text x="10" y="194" font-size="12" font-weight="700" fill="#7c3aed">サーバー B</text>
<text x="10" y="210" font-size="10" fill="#475569">（手元のコピー）</text>
<line x1="120" y1="60" x2="705" y2="60" stroke="#93c5fd" stroke-width="3"/>
<line x1="120" y1="130" x2="705" y2="130" stroke="#6ee7b7" stroke-width="3"/>
<line x1="120" y1="200" x2="705" y2="200" stroke="#c4b5fd" stroke-width="3"/>
<line x1="150" y1="62" x2="186" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-082)"/>
<text x="140" y="96" font-size="11" fill="#c2410c" text-anchor="end">① A が書く</text>
<line x1="200" y1="128" x2="236" y2="64" stroke="#10b981" stroke-width="2" marker-end="url(#ar-07-082)"/>
<text x="238" y="98" font-size="10" fill="#047857">② すぐ A に反映</text>
<text x="238" y="111" font-size="10" fill="#047857">（read-your-writes）</text>
<circle cx="330" cy="60" r="6" fill="#3b82f6"/>
<text x="330" y="44" font-size="11" fill="#1e40af" text-anchor="middle">③ A が読む → 見える</text>
<circle cx="330" cy="200" r="6" fill="#a78bfa"/>
<text x="330" y="226" font-size="11" fill="#6d28d9" text-anchor="middle">③ B が読む → まだ見えない</text>
<line x1="470" y1="132" x2="490" y2="196" stroke="#10b981" stroke-width="2" stroke-dasharray="5 3" marker-end="url(#ar-07-082)"/>
<text x="488" y="160" font-size="11" font-weight="700" fill="#047857">④ B が sync</text>
<circle cx="610" cy="200" r="6" fill="#7c3aed"/>
<text x="610" y="226" font-size="11" fill="#6d28d9" text-anchor="middle">⑤ B が読む → 見える</text>
<text x="705" y="20" font-size="11" fill="#64748b" text-anchor="end">時間の流れ →</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 2台のサーバーが組み込みレプリカを使うときの時間の流れ。A が書いた行は A にはすぐ見えるが、B には B が sync するまで見えない</div>
</div>

> **注意:** この巻の練習では、Turso の本物の DB は用意しません。図のような「本当に Turso とやり取りする」動きは公式ドキュメントで確かめた内容で、この単元の実行結果は、手元のファイルと、手元の何も動いていないポートにつないだときのものです。

### まず動かす最小コード

#### 例8-1: ローカルのファイルとメモリの DB を、開き直して比べる

> **▼ このコードがやること（先に日本語で）:** 手元のファイル `u08_local.db` と、メモリの中だけの DB（`:memory:`）の2つについて、同じことを試します。まず DB を開いて表を作り、メモを1行足して数を表示し、いったん閉じます。次に同じ名前で開き直して、メモが残っているかを数えます。最後に、ローカルの DB で `sync` を呼ぶとどうなるかを確かめます。

```rust
use libsql::{Builder, Connection};

// メモの数を数える
async fn count(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM memos", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    for path in ["u08_local.db", ":memory:"] {
        // ① 1回目: 開いて、表を作り、1行足す
        {
            let db = Builder::new_local(path).build().await?;
            let conn = db.connect()?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)",
                (),
            )
            .await?;
            conn.execute("INSERT INTO memos (text) VALUES (?1)", ["ローカル"])
                .await?;
            println!("{path}: 1回目に開いたとき {} 件", count(&conn).await?);
            // ここで db と conn が捨てられ、DB が閉じられる
        }

        // ② 2回目: 同じ名前で開き直す
        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        match count(&conn).await {
            Ok(n) => println!("{path}: 開き直したとき {n} 件"),
            Err(e) => println!("{path}: 開き直したとき エラー: {e}"),
        }

        // ③ ローカルの DB で sync を呼ぶと……
        match db.sync().await {
            Ok(_) => println!("{path}: sync 成功"),
            Err(e) => println!("{path}: sync エラー: {e}"),
        }
    }
    Ok(())
}
```

`src/bin/u08_local.rs` として保存し、**2回**実行します。

```powershell
cargo run -q --bin u08_local
cargo run -q --bin u08_local
```

**▼ 実行結果**（1回目）

```text
u08_local.db: 1回目に開いたとき 1 件
u08_local.db: 開き直したとき 1 件
u08_local.db: sync エラー: Sync is not supported in databases opened in File mode.
:memory:: 1回目に開いたとき 1 件
:memory:: 開き直したとき エラー: SQLite failure: `no such table: memos`
:memory:: sync エラー: Sync is not supported in databases opened in Memory mode.
```

**▼ 実行結果**（2回目）

```text
u08_local.db: 1回目に開いたとき 2 件
u08_local.db: 開き直したとき 2 件
u08_local.db: sync エラー: Sync is not supported in databases opened in File mode.
:memory:: 1回目に開いたとき 1 件
:memory:: 開き直したとき エラー: SQLite failure: `no such table: memos`
:memory:: sync エラー: Sync is not supported in databases opened in Memory mode.
```

ファイルの DB は、閉じても、プログラムを実行し直しても、行が残りました（2回目は 2 件）。メモリの DB は、閉じたとたんに表ごと消え、開き直すと「`memos` という表が無い」と言われました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: new_local に渡すのは「ファイルの場所」か「:memory:」

- `Builder::new_local("u08_local.db")` は、手元のファイルを DB として開きます。ファイルが無ければ作られます。場所を `data/app.db` のようにフォルダ付きで書くこともでき、フォルダ名を書かなければ、`cargo run` を実行したフォルダ（ここでは `sql-lab`）に作られます。
- `Builder::new_local(":memory:")` は、ファイルを作らず、**メモリの中だけ**に DB を作ります。道具 `sql` の既定で、この巻の SQL の練習はほとんどこれで動かしてきました。

##### 解説2: { } のブロックの終わりで DB が閉じる

- ①は `{ … }` で囲んであります。ブロックの中で作った `db` と `conn` は、ブロックの終わりで捨てられ（drop され）、DB が閉じられます。
- ②で同じ名前を開き直すのは、「アプリを止めて、もう一度起動した」場面の再現です。

##### 解説3: メモリの DB は、閉じると消える

- `:memory:` の DB は、開くたびに**別の、空の DB**ができます。閉じると中身は消えます。②で `no such table: memos` になったのはこのためです。
- テストでは、毎回まっさらな DB から始められるので便利です（第11巻で使います）。逆に、残しておきたいデータを `:memory:` に入れてはいけません。

##### 解説4: sync はレプリカのためのもの

- `db.sync()` は、組み込みレプリカの手元のコピーを最新にする命令です。ローカルの DB には取り寄せる相手がいないので、`Sync is not supported in databases opened in File mode.`（ファイルの形で開いた DB では sync できない）というエラーになります。メモリの DB では `Memory mode` です。
- エラーの文の `File mode`・`Memory mode` から、「どの形で開いた DB か」が分かります。sync を呼んだのにこのエラーが出たら、レプリカのつもりで `new_local` を使っていないかを確かめましょう。

### もう一歩: 実務に近い形

#### 例8-2: 環境変数で、DB の形を選ぶ

実際のアプリでは、自分の PC では手元のファイル、公開したサーバーでは Turso、というように、**同じプログラムを、場所によって違う DB につなぎます**。そのたびにコードを書き換えるのではなく、**環境変数**で切り替えます。Turso の URL や秘密のトークンをコードに書かずに済む（Git に入らない）のも大事な点です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 385" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-083" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="260" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="150" y="35" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">DATABASE_URL を読む</text>
<text x="150" y="52" font-size="10" fill="#475569" text-anchor="middle">無ければ u08_app.db（手元のファイル）</text>
<line x1="150" y1="60" x2="150" y2="84" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<rect x="20" y="88" width="260" height="50" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="150" y="108" font-size="12" font-weight="700" fill="#854d0e" text-anchor="middle">ネットの URL？</text>
<text x="150" y="127" font-size="10" fill="#475569" text-anchor="middle">libsql:// ・ https:// ・ http:// で始まる</text>
<line x1="280" y1="113" x2="346" y2="113" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="313" y="106" font-size="11" fill="#475569" text-anchor="middle">いいえ</text>
<rect x="350" y="91" width="170" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="435" y="110" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">ローカル</text>
<text x="435" y="127" font-size="10" fill="#475569" text-anchor="middle">new_local(パス)</text>
<line x1="150" y1="138" x2="150" y2="164" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="158" y="156" font-size="11" fill="#475569">はい</text>
<rect x="20" y="168" width="260" height="50" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="150" y="188" font-size="12" font-weight="700" fill="#854d0e" text-anchor="middle">トークンが要るのに空？</text>
<text x="150" y="207" font-size="10" fill="#475569" text-anchor="middle">http:// 以外で DATABASE_AUTH_TOKEN が空</text>
<line x1="280" y1="193" x2="346" y2="193" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="313" y="186" font-size="11" fill="#475569" text-anchor="middle">はい</text>
<rect x="350" y="171" width="170" height="44" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="435" y="190" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">設定エラー</text>
<text x="435" y="207" font-size="10" fill="#475569" text-anchor="middle">すぐに止める（終了コード 1）</text>
<line x1="150" y1="218" x2="150" y2="244" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="158" y="236" font-size="11" fill="#475569">いいえ</text>
<rect x="20" y="248" width="260" height="50" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="150" y="268" font-size="12" font-weight="700" fill="#854d0e" text-anchor="middle">コピーを置く場所がある？</text>
<text x="150" y="287" font-size="10" fill="#475569" text-anchor="middle">DATABASE_REPLICA_PATH が設定されている</text>
<line x1="280" y1="273" x2="346" y2="273" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="313" y="266" font-size="11" fill="#475569" text-anchor="middle">はい</text>
<rect x="350" y="251" width="170" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="435" y="270" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">組み込みレプリカ</text>
<text x="435" y="287" font-size="10" fill="#475569" text-anchor="middle">new_remote_replica(…)</text>
<line x1="150" y1="298" x2="150" y2="324" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-083)"/>
<text x="158" y="316" font-size="11" fill="#475569">いいえ</text>
<rect x="65" y="328" width="170" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="150" y="347" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">リモート</text>
<text x="150" y="364" font-size="10" fill="#475569" text-anchor="middle">new_remote(URL, トークン)</text>
<rect x="545" y="150" width="165" height="96" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="627" y="174" font-size="11" font-weight="700" fill="#0f172a" text-anchor="middle">どの形を選んでも</text>
<text x="627" y="194" font-size="11" fill="#475569" text-anchor="middle">この後の connect・</text>
<text x="627" y="212" font-size="11" fill="#475569" text-anchor="middle">query・execute は</text>
<text x="627" y="230" font-size="11" fill="#475569" text-anchor="middle">同じ書き方で使える</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: 例8-2 のプログラムが、環境変数から DB の形を選ぶ流れ。トークンが足りないときは、DB を開く前に止める</div>
</div>

> **▼ このコードがやること（先に日本語で）:** 環境変数 `DATABASE_URL`・`DATABASE_AUTH_TOKEN`・`DATABASE_REPLICA_PATH` を読み、上の図の順に「ローカル」「リモート」「組み込みレプリカ」のどれで開くかを決めます。選んだ形の名前だけを表示し（トークンは表示しません）、その形で DB を開いて `SELECT 1 + 1` を実行し、かかった時間と結果を表示します。

```rust
use std::env;
use std::time::{Duration, Instant};

use libsql::{Builder, Database};

// DB の置き場所の3つの形
#[derive(Debug)]
enum Mode {
    // 手元のファイル（開発・テスト）
    Local {
        path: String,
    },
    // ネットワークの向こうの DB に毎回問い合わせる
    Remote {
        url: String,
        token: String,
    },
    // 手元にコピーを持ち、読むのはコピー・書くのは向こう
    Replica {
        path: String,
        url: String,
        token: String,
    },
}

// 環境変数から、どの形で開くかを決める
fn mode_from_env() -> Result<Mode, String> {
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| "u08_app.db".to_string());
    let token = env::var("DATABASE_AUTH_TOKEN").unwrap_or_default();
    let replica_path = env::var("DATABASE_REPLICA_PATH").ok();

    let is_network = ["libsql://", "https://", "http://"]
        .iter()
        .any(|prefix| url.starts_with(prefix));
    if !is_network {
        return Ok(Mode::Local { path: url });
    }
    // インターネット上の Turso にはトークンが必要（http:// は手元のテスト用サーバー）
    if !url.starts_with("http://") && token.is_empty() {
        return Err(format!("{url} につなぐには DATABASE_AUTH_TOKEN が必要です"));
    }
    Ok(match replica_path {
        Some(path) => Mode::Replica { path, url, token },
        None => Mode::Remote { url, token },
    })
}

// 決めた形で DB を開く
async fn open(mode: Mode) -> Result<Database, libsql::Error> {
    match mode {
        Mode::Local { path } => Builder::new_local(path).build().await,
        Mode::Remote { url, token } => Builder::new_remote(url, token).build().await,
        Mode::Replica { path, url, token } => {
            Builder::new_remote_replica(path, url, token)
                // 60 秒ごとに、裏で自動的に sync する
                .sync_interval(Duration::from_secs(60))
                .build()
                .await
        }
    }
}

#[tokio::main]
async fn main() {
    let mode = match mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("設定エラー: {message}");
            std::process::exit(1);
        }
    };
    // トークンが表示されないよう、形の名前だけを出す
    let name = match &mode {
        Mode::Local { .. } => "ローカル",
        Mode::Remote { .. } => "リモート",
        Mode::Replica { .. } => "組み込みレプリカ",
    };
    println!("選んだ形: {name}");

    let started = Instant::now();
    let result = async {
        let db = open(mode).await?;
        let conn = db.connect()?;
        let mut rows = conn.query("SELECT 1 + 1", ()).await?;
        let row = rows.next().await?.expect("1行ある");
        row.get::<i64>(0)
    }
    .await;
    let ms = started.elapsed().as_millis();
    match result {
        Ok(n) => println!("SELECT 1 + 1 = {n}（{ms} ミリ秒）"),
        Err(e) => println!("失敗（{ms} ミリ秒）: {e}"),
    }
}
```

`src/bin/u08_open.rs` として保存し、環境変数を変えながら4回実行します。PowerShell で設定した環境変数は、その PowerShell の窓を閉じるまで残るので、最後に消しておきます。

```powershell
# ① 何も設定しない
cargo run -q --bin u08_open

# ② Turso の URL だけを設定し、トークンを忘れる
$env:DATABASE_URL = "libsql://rustnote-example.turso.io"
cargo run -q --bin u08_open

# ③ 手元の、何も動いていないポート 9 をリモートの DB として指定する
$env:DATABASE_URL = "http://127.0.0.1:9"
cargo run -q --bin u08_open

# ④ ③に、コピーを置く場所を足す
$env:DATABASE_REPLICA_PATH = "u08_replica.db"
cargo run -q --bin u08_open

# 後片付け: 設定した環境変数を消す
Remove-Item Env:DATABASE_URL, Env:DATABASE_REPLICA_PATH
```

**▼ 実行結果**（① 何も設定しない）

```text
選んだ形: ローカル
SELECT 1 + 1 = 2（12 ミリ秒）
```

**▼ 実行結果**（② トークンを忘れた）

```text
設定エラー: libsql://rustnote-example.turso.io につなぐには DATABASE_AUTH_TOKEN が必要です
```

**▼ 実行結果**（③ リモート）

```text
選んだ形: リモート
失敗（2033 ミリ秒）: Hrana: `http error: `error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)``
```

**▼ 実行結果**（④ 組み込みレプリカ）

```text
選んだ形: 組み込みレプリカ
失敗（2030 ミリ秒）: sync error: error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)
```

ミリ秒の数字は、PC や実行のたびに少し変わります。①は手元のファイル `u08_app.db` を作るところから始めたので 12 ミリ秒でしたが、2回目以降はもっと短くなります。②はネットに一切つながず、DB を開く前に止まりました（終了コードは 1 です。`$LASTEXITCODE` で確かめられます）。③と④は、どちらも約2秒待ってから、接続を拒否されて失敗しました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: enum Mode に「形」と「その形に必要な値」をまとめる

- `Mode` は3つの形を表す enum です。ローカルにはファイルの場所だけ、リモートには URL とトークン、組み込みレプリカにはその両方が必要です。
- enum にしておくと、「URL が無いリモート」のような、**必要な値がそろっていない形を作れません**。どの形でも、必要な値は必ずそろっています。

##### 解説2: 環境変数は env::var で読む

- `env::var("DATABASE_URL")` は、環境変数の値を `Result<String, _>` で返します。設定されていなければ `Err` です。
- `.unwrap_or_else(|_| "u08_app.db".to_string())` は「無ければ `u08_app.db`」、`.unwrap_or_default()` は「無ければ空の文字列」、`.ok()` は「無ければ `None` の `Option`」に変えます。値が無いときの扱いを、変数ごとに選べます。
- 何も設定しなければ手元のファイルで動くので、自分の PC では環境変数を何も準備しなくても練習できます。

##### 解説3: URL の始まりで、ネットの向こうかどうかを見分ける

- Turso の DB の URL は `libsql://rustnote-example.turso.io` のように `libsql://` で始まります。`https://` で始まる同じ場所の URL も使えます。
- `http://` は暗号化しない通信です。インターネットの向こうの Turso には使わず、**手元で動かすテスト用の libSQL サーバー**（Turso CLI の `turso dev` が `http://127.0.0.1:8080` で起動するもの）につなぐときに使います。手元のテスト用なので、トークンを求めないことにしています。
- どれにも当てはまらなければ、ファイルの場所としてローカルで開きます。

##### 解説4: トークンが足りなければ、DB を開く前に止める

- `libsql://` や `https://` なのにトークンが空なら、`Err` を返し、`main` が `設定エラー: …` を表示して `std::process::exit(1)` で止まります。
- 設定の間違いは、最初のリクエストが来てから気づくより、**起動した瞬間に止まって知らせる**ほうが、ずっと早く直せます。②では、Turso にも何も送っていません。

##### 解説5: 3つの Builder と sync_interval

- ローカルは `Builder::new_local(パス)`、リモートは `Builder::new_remote(URL, トークン)`、組み込みレプリカは `Builder::new_remote_replica(コピーのパス, URL, トークン)` です。どれも最後は `.build().await` で `Database` になります。
- `.sync_interval(Duration::from_secs(60))` は、「60 秒ごとに、裏で自動的に sync する」設定です。裏の sync は、できた `Database` が生きている間だけ続き、`Database` が捨てられると止まります。
- `Database` ができた後の `connect`・`query`・`execute` は、3つの形で**まったく同じ書き方**です。だから、形を選ぶ部分を `open` の1か所に閉じ込めれば、ほかのコードは形を気にせずに書けます。

##### 解説6: トークンを表示しない・失敗するまでの時間

- `main` では、`mode` を `{:?}` で表示せず、`match` で**形の名前だけ**を取り出して表示しています。`Mode` は `#[derive(Debug)]` なので、`{mode:?}` と書くとトークンまで画面に出てしまいます（練習問題3）。
- 失敗するまでに約2秒かかっています。この本の PC（Windows）では、何も動いていないポートへの接続は、拒否されるまでに約2秒かかりました。
- リモート（③）は、`build` の時点ではまだつなぎに行かず、**最初の SQL を送ったときに初めて**接続して失敗しました（`build` だけなら 4 ミリ秒で成功することも確かめました）。組み込みレプリカ（④）は、**`build` の中で** Turso とやり取りするので、`build` の時点で `sync error` になりました。このとき、コピーのファイル `u08_replica.db` は作られませんでした。

#### 例8-3: 組み込みレプリカの使い方（Turso の本物の DB 向け）

この例は、Turso の本物の DB と、その URL・トークンがあるときの形です。この本の PC では Turso の DB を用意していないので、**コンパイルが通ることだけ**を確かめ、実行結果は載せていません。

> **▼ このコードがやること（先に日本語で）:** 環境変数から URL とトークンを読み、手元のファイル `replica.db` を Turso の DB のコピー置き場にして開きます。起動したらまず1回 sync してコピーを最新にし、メモを1行書き込んで（書き込みは Turso へ送られる）、手元のコピーからメモの数を読みます。

```rust
use std::env;
use std::time::Duration;

use libsql::Builder;

// 組み込みレプリカの使い方の例（Turso の本物の DB が必要。この本の PC ではコンパイルだけ確認）
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL とトークンは環境変数から読む（コードに書かない）
    let url = env::var("DATABASE_URL")?;
    let token = env::var("DATABASE_AUTH_TOKEN")?;

    // 手元のファイル replica.db を、Turso の DB のコピー置き場にする
    let db = Builder::new_remote_replica("replica.db", url, token)
        // 60 秒ごとに、裏で自動的にコピーを最新にする
        .sync_interval(Duration::from_secs(60))
        .build()
        .await?;
    // 起動したら、まず1回コピーを最新にする
    db.sync().await?;

    let conn = db.connect()?;
    // 書き込みは Turso（向こうの本体）へ送られる
    conn.execute("INSERT INTO memos (text) VALUES (?1)", ["レプリカから追加"])
        .await?;
    // 読み込みは手元のコピーから。自分が書いた行はすぐ見える（read-your-writes）
    let mut rows = conn.query("SELECT COUNT(*) FROM memos", ()).await?;
    if let Some(row) = rows.next().await? {
        println!("メモの数: {}", row.get::<i64>(0)?);
    }
    Ok(())
}
```

（`src/bin/u08_replica.rs`。`cargo build --bin u08_replica` でコンパイルできることを確かめています。）

- `main` の戻り値を `Box<dyn std::error::Error>` にしたので、`env::var` の失敗（`VarError`）と `libsql::Error` の両方を `?` で返せます。
- 書き込みの `INSERT` は Turso の primary に送られ、成功すると、read-your-writes で手元のコピーにもすぐ反映されます。続く `SELECT` は手元のコピーを読むので、ネットを往復しません。
- Turso の公式ドキュメントは、次のことにも注意するよう書いています。
  - sync している最中に、手元のコピーのファイルを**別のプログラムで開かない**こと（ファイルが壊れることがある）。道具 `sql` などで中を見たいときは、アプリを止めてからにします。
  - ファイルを置けない環境（ファイルを保存できないサーバーレスの環境など）では使えないこと。
  - 手元のファイルを消すと、最初から全部を取り寄せ直すことがあること。

> **ヒント:** 本物の Turso の URL とトークンは、Turso CLI（WSL に入れて使います）で、`turso db show <DB名> --url` と `turso db tokens create <DB名>` で取り出せます。トークンには `--expiration 7d`（7日で期限切れ）や `--read-only`（読み込み専用）を付けられます。トークンの管理は UNIT 10 でくわしく扱います。

<!-- -->

> **ヒント:** Turso は、これから同期の仕組みを作る新しいプロジェクトには、別の仕組みの「Turso Sync」を勧めています。組み込みレプリカも、本番で引き続きサポートされています（2026年10月時点の公式ドキュメント）。この巻では、RustNote と同じ `libsql` クレートの組み込みレプリカを扱います。

#### どの形を選ぶか

| 形 | データの場所 | 読む | 書く | 向いている場面 |
|---|---|---|---|---|
| ローカル | アプリと同じ PC のファイル | 手元（速い） | 手元（速い） | 自分の PC での開発・テスト。ディスクを持つサーバー1台だけで動かすアプリ |
| リモート | Turso だけ | 毎回ネット往復 | 毎回ネット往復 | サーバーが複数台ある、またはサーバーにデータを残せない（入れ替わる）とき |
| 組み込みレプリカ | Turso と、手元のコピー | 手元（速い） | Turso へ | 読み込みがとても多く、サーバーにファイルを置けるとき |

- サーバーが1台で、そのサーバーのディスクが消えないなら、ローカルのファイルが一番簡単で速い形です。ただし、そのサーバーが壊れたときの備え（backup）が必要です（UNIT 10）。
- Fly.io のように、サーバー（マシン）を複数台に増やしたり、入れ替えたりする場所では、どのサーバーからも同じ DB を見られるリモートが分かりやすい形です。
- 組み込みレプリカは速さと引きかえに、「ほかのサーバーが書いた行が、sync まで見えない」ことを受け入れる必要があります（図の B の場面）。

### よくあるエラーと読み方

#### エラー1: os error 10061 — 接続を拒否された

例8-2 の③④で見たエラーです。

```text
失敗（2033 ミリ秒）: Hrana: `http error: `error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)``
```

- **何が起きたか:** `tcp connect error` は「相手の PC につながらなかった」、`os error 10061` は Windows の「接続が拒否された」という番号です。指定した場所で、DB のサーバーが待ち受けていません。
- **読み方:** 先頭の `Hrana` は、libsql がリモートの DB とやり取りするときの通信の仕組みの名前です。リモートでは、接続の失敗も、サーバーが返した SQL のエラーも、この `Hrana(…)` の中に入って返ってくる作りになっています（libsql 0.10 のソースで確認）。UNIT 07 の `SqliteFailure(番号, …)` では見分けられないのはこのためです。組み込みレプリカの `build` で起きたときは、`sync error: …` で始まります。
- **直し方:** URL のつづりとポートの番号を確かめます。手元のテスト用サーバーなら、`turso dev` が動いているかを確かめます。

#### エラー2: debug ビルドで、リモートへの execute がスタックあふれで止まる

この本の PC（Windows）では、debug ビルドの `main` の中で、リモートの DB に `execute` を送ると、プログラムが落ちることがありました。

> **▼ このコードがやること（先に日本語で）:** 何も動いていないポート 9 をリモートの DB として開き、`CREATE TABLE` を `execute` で送って、結果を表示します。

```rust
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 手元で何も待ち受けていないポート 9 を、リモートの DB として開く
    let db = Builder::new_remote("http://127.0.0.1:9".to_string(), String::new())
        .build()
        .await?;
    let conn = db.connect()?;
    // 書き込みの文をリモートへ送る
    let result = conn
        .execute("CREATE TABLE memos (id INTEGER PRIMARY KEY)", ())
        .await;
    println!("結果: {result:?}");
    Ok(())
}
```

```powershell
cargo run -q --bin u08_err_stack
```

**▼ 実行結果**（debug ビルド）

```text
thread 'main' (28564) has overflowed its stack
```

- **何が起きたか:** `has overflowed its stack` は「スタック（関数の作業用のメモリ）があふれた」という意味です。`結果:` の行が表示される前に、プログラムが異常終了しました。`$LASTEXITCODE` は `-1073741571`（16進数で `0xC00000FD`、Windows の「スタックあふれ」の番号）でした。
- **考えられる理由:** debug ビルドは最適化をしないので、async の処理が大きなスタックを使います。Windows の `main` のスレッドはスタックが小さめ（1 MiB）なので、そこに収まりませんでした。tokio が作る作業用のスレッドは、既定で 2 MiB のスタックを持っています。

同じプログラムを release ビルドで動かすと、正しくエラーが返ります。

```powershell
cargo run -q --release --bin u08_err_stack
```

**▼ 実行結果**（release ビルド）

```text
結果: Err(Hrana(Http("error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)")))
```

debug ビルドのまま直すなら、DB とのやり取りを `tokio::spawn` で作業用のスレッドに任せます。

```rust
// DB とのやり取りを tokio::spawn で作業用のスレッドに任せ、終わるのを待つ
let task = tokio::spawn(async move {
    conn.execute("CREATE TABLE memos (id INTEGER PRIMARY KEY)", ())
        .await
});
let result = task.await.expect("作業が panic していない");
println!("結果: {result:?}");
```

（`src/bin/u08_stack_spawn.rs`。前半の DB を開くところは上と同じです。）

**▼ 実行結果**（debug ビルド・tokio::spawn の中）

```text
結果: Err(Hrana(Http("error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)")))
```

- **直し方のまとめ:** release ビルドで動かすか、DB とのやり取りを `tokio::spawn` の中で行います。Web アプリでは、リクエストの処理はもともと作業用のスレッドで動くので、ふつうはこの問題に当たりません。ただし、起動したときの migration のように、`main` の中で直接リモートに書き込む処理では、同じことが起きるかもしれません。自分の PC から Turso の DB を試すときは、`cargo run --release` で動かすと安全です。
- 例8-2 の③（`main` の中でリモートに `query`）では起きませんでした。どの命令で起きるかは、処理の大きさしだいです。

#### エラー3: E0599 — new_remote が見つからない

巻頭のヒントで紹介した「clang なしでビルドする形」（`default-features = false, features = ["core"]`）のまま、リモートの DB を開こうとしたときのエラーです。

```rust
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // features = ["core"] だけの libsql で、リモートの DB を開こうとする
    let db = Builder::new_remote(
        "libsql://rustnote-example.turso.io".to_string(),
        String::new(),
    )
    .build()
    .await?;
    let _conn = db.connect()?;
    Ok(())
}
```

```text
error[E0599]: no associated function or constant named `new_remote` found for struct `libsql::Builder<T>` in the current scope
  --> src\main.rs:6:23
   |
 6 |     let db = Builder::new_remote(
   |                       ^^^^^^^^^^ associated function or constant not found in `libsql::Builder<_>`
   |
note: if you're trying to build a new `libsql::Builder<_>`, consider using `libsql::Builder::new_local` which returns `libsql::Builder<libsql::database::builder::Local>`
```

（実際の出力から要点を抜き出したもの。この後に、ここから続いて起きた `connect` のエラーも表示されますが、最初のエラーを直せば消えます。）

- **何が起きたか:** 「`Builder` に `new_remote` という関数が無い」と言われました。libsql は、機能ごとに **features** という部品に分かれていて、`core` だけでは `new_remote` そのものが作られません。`note:` は「`new_local` なら使えますよ」という提案です。
- **直し方:** リモートや組み込みレプリカを使うなら、`Cargo.toml` を `libsql = "0.10.0"`（既定の features）に戻します。ARM64 版の Windows では clang が必要です（巻頭）。

| features | 使えるようになるもの |
|---|---|
| `core` | `Builder::new_local`（手元のファイル・メモリの DB） |
| `remote` | `Builder::new_remote`（リモート） |
| `replication` | `Builder::new_remote_replica`（組み込みレプリカ） |
| `tls` | `https://`・`libsql://` の暗号化した通信（ARM64 版の Windows では clang が必要） |
| 既定（何も書かない） | 上の全部と `sync`（`core`・`replication`・`remote`・`sync`・`tls`） |

#### エラー4: Sync is not supported — ローカルの DB で sync した

例8-1 の③で見たエラーです。

```text
u08_local.db: sync エラー: Sync is not supported in databases opened in File mode.
```

- **何が起きたか:** `new_local` で開いた DB で `sync` を呼びました。ローカルの DB には、取り寄せる相手（primary）がいません。
- **直し方:** 組み込みレプリカを使いたいなら `new_remote_replica` で開きます。ローカルとレプリカを環境変数で切り替えるプログラムでは、レプリカのときだけ `sync` を呼ぶようにします。

#### エラー5: 設定エラー — トークンを渡し忘れた

例8-2 の②で見た、自分で用意したエラーです。

```text
設定エラー: libsql://rustnote-example.turso.io につなぐには DATABASE_AUTH_TOKEN が必要です
```

- **何が起きたか:** Turso の URL を設定したのに、トークンが空でした。
- **直し方:** `$env:DATABASE_AUTH_TOKEN = "…"` でトークンを設定します。公開したサーバーでは、トークンをファイルに書かず、そのサービスの「秘密の値」の仕組み（Fly.io なら `fly secrets set`）で渡します（第12巻）。

### RustNote ではここで使う

RustNote は、**自分の PC ではローカルのファイル、公開したサーバーではリモート（Turso）**を使います。形を選ぶのは `Rust/rustnote/src/config.rs` と `src/db.rs` の2か所です。

`config.rs` は、例8-2 の `mode_from_env` とほぼ同じ規則で、環境変数から設定を作ります。

```rust
/// ネットワークの向こうの DB（Turso / sqld）につなぐなら true
pub fn is_remote(&self) -> bool {
    ["libsql://", "https://", "http://", "wss://", "ws://"]
        .iter()
        .any(|prefix| self.database_url.starts_with(prefix))
}

/// Turso（インターネット上の DB）ならトークンが必須
fn requires_auth_token(&self) -> bool {
    ["libsql://", "https://", "wss://"]
        .iter()
        .any(|prefix| self.database_url.starts_with(prefix))
}
```

- `DATABASE_URL` が無ければ `rustnote.db`（ローカル）を使い、`libsql://`・`https://` なのにトークンが無ければ `ConfigError::MissingAuthToken` で起動を止めます。例8-2 の②と同じ考え方です。
- `wss://`・`ws://` は、ネットの向こうの DB とやり取りする、もう1つの通信の形（WebSocket）の URL です。
- トークンは、読んだらすぐ `Secret` 型に包みます。`Secret` の `Debug` は `Secret(********)` としか表示しないので、設定全体を `{:?}` でログに出しても、トークンは漏れません（練習問題3と同じ形）。

`db.rs` の `Db::open` は、設定に従って Builder を選びます。

```rust
/// 設定に従って、ローカルファイルかリモート（Turso）の DB を開く
pub async fn open(config: &Config) -> Result<Db, libsql::Error> {
    if config.is_remote() {
        // 秘密のトークンは、接続するこの場所でだけ取り出す
        let token = config
            .database_auth_token
            .as_ref()
            .map(|secret| secret.expose().to_string())
            .unwrap_or_default();
        let database = Builder::new_remote(config.database_url.clone(), token)
            .build()
            .await?;
        Ok(Db {
            database: Arc::new(database),
            is_local: false,
        })
    } else {
        Db::open_local(&config.database_url).await
    }
}
```

- トークンの中身を取り出す `expose()` は、DB を開くこの1か所でだけ呼びます。わざと目立つ名前にしてあるので、「どこで秘密を使っているか」を検索ですぐに見つけられます。
- ローカルのときだけ、`PRAGMA journal_mode = WAL`（UNIT 05）や、接続ごとの `PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;` を実行します（`is_local` で見分けています）。
- 開いた後のリポジトリ（`src/repository/`）は、ローカルかリモートかを一切気にせず、`connect`・`query`・`execute` を使います。図の「どの形を選んでも同じ書き方」を、そのまま生かした作りです。
- RustNote は、話を簡単にするために組み込みレプリカを使っていません。読み込みが増えて速さが足りなくなったら、`Db::open` のリモートの枝を `new_remote_replica` に変えるのが次の一手になります。

公開するときは、Fly.io に URL とトークンを秘密の値として渡します（`fly.toml` のコメントより）。

```text
fly secrets set DATABASE_URL=libsql://<DB名>-<組織名>.turso.io DATABASE_AUTH_TOKEN=<トークン>
```

> **注意:** RustNote の `Dockerfile` は、`DATABASE_URL` の既定値を `/data/rustnote.db`（サーバーの中のファイル）にしています。`fly secrets set` を忘れると、エラーにならずにローカルのファイルで動いてしまいます。公開する前に、`fly secrets list` で `DATABASE_URL` と `DATABASE_AUTH_TOKEN` が設定されているかを確かめましょう（第12巻）。

### 練習問題

**問題1:** 例8-2 の `u08_open` を、次の (a)〜(c) の環境変数で実行すると、何と表示されるでしょうか。先に予想してから、実行して確かめてください（それぞれ、前の設定は消してから試します）。

- (a) `DATABASE_REPLICA_PATH` だけを `copy.db` にする（`DATABASE_URL` は設定しない）
- (b) `DATABASE_URL` を `https://rustnote-example.turso.io` にする（トークンは設定しない）
- (c) `DATABASE_URL` を `data.db`、`DATABASE_AUTH_TOKEN` を `abc` にする

**問題2:** 2台のサーバー A と B が、同じ Turso の DB の組み込みレプリカを使っています。どちらも `sync_interval` は 60 秒です。10時00分00秒に、利用者が A でノートを1つ追加しました。

- (a) A の手元のコピーで、そのノートが読めるようになるのはいつですか。
- (b) B の手元のコピーで読めるようになるのは、遅くともいつですか。
- (c) 同じ利用者の次のリクエストが B に届くと、何が起きる可能性がありますか。

**問題3:** 例8-2 の `main` で、形の名前を表示する部分を `println!("選んだ形: {mode:?}");` の1行に書き換えると、トークンが画面に出てしまいます。`DATABASE_URL=http://127.0.0.1:9`・`DATABASE_AUTH_TOKEN=dummy-token-123` で実行して確かめ、`{mode:?}` のままでもトークンが表示されないように直してください（ヒント: RustNote の `Secret`）。

### 解答と解説

**解答1:**

```powershell
# (a)
$env:DATABASE_REPLICA_PATH = "copy.db"
cargo run -q --bin u08_open
Remove-Item Env:DATABASE_REPLICA_PATH

# (b)
$env:DATABASE_URL = "https://rustnote-example.turso.io"
cargo run -q --bin u08_open
Remove-Item Env:DATABASE_URL

# (c)
$env:DATABASE_URL = "data.db"
$env:DATABASE_AUTH_TOKEN = "abc"
cargo run -q --bin u08_open
Remove-Item Env:DATABASE_URL, Env:DATABASE_AUTH_TOKEN
```

**▼ 実行結果**（(a)）

```text
選んだ形: ローカル
SELECT 1 + 1 = 2（0 ミリ秒）
```

**▼ 実行結果**（(b)）

```text
設定エラー: https://rustnote-example.turso.io につなぐには DATABASE_AUTH_TOKEN が必要です
```

**▼ 実行結果**（(c)）

```text
選んだ形: ローカル
SELECT 1 + 1 = 2（1 ミリ秒）
```

- (a) は、`DATABASE_URL` が無いので `u08_app.db` のローカルになり、`DATABASE_REPLICA_PATH` は**使われません**。`copy.db` も作られませんでした。コピーの場所は、ネットの URL があるときだけ意味を持ちます。
- (b) は、`https://` もトークンが必要な URL なので、設定エラーで止まります。
- (c) は、`data.db` がファイルの場所なのでローカルになり、トークンは使われません。
- 「設定したのに使われない環境変数」は、エラーにならないので気づきにくい間違いです。大事な組み合わせ（URL とトークン）だけでも、起動時に確かめるようにしましょう。

**解答2:**

- (a) **すぐ（書き込みが成功した直後）**です。A が書いた行は、read-your-writes で A の手元のコピーに反映されます。
- (b) B が次に sync したときです。60 秒ごとの sync なら、**遅くとも 10時01分00秒ごろ**には読めます（B のアプリが `db.sync()` を自分で呼べば、そのときに読めます）。
- (c) B のコピーがまだ古いと、「追加したはずのノートが一覧に無い」と見えることがあります。データが消えたわけではなく、Turso の primary にはちゃんとあります。こうした食い違いを避けたいなら、リモートを使うか、書き込みの後に `sync` を呼ぶなどの工夫が必要です。

**解答3:** 書き換えた版の実行結果は、次のとおりです。

```text
選んだ形: Remote { url: "http://127.0.0.1:9", token: "dummy-token-123" }
失敗（2047 ミリ秒）: Hrana: `http error: `error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)``
```

トークンがそのまま表示されました。これがログに書かれると、ログを見られる人なら誰でも DB に入れてしまいます。直すには、トークンを `Secret` 型に包み、`Debug` を自分で書いて中身を隠します。

```rust
use std::fmt;

// 秘密の値。{:?} で表示されても中身を出さない
struct Secret(String);

impl Secret {
    // 中身が本当に必要な場所（DB を開くとき）だけで呼ぶ
    fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}
```

例8-2 からは、次のところを変えます（`src/bin/u08_q3.rs`）。

- `Mode` の `Remote` と `Replica` の `token: String` を `token: Secret` にする
- `mode_from_env` で `let token = Secret(env::var("DATABASE_AUTH_TOKEN").unwrap_or_default());` と、読んだらすぐ包む。空かどうかは `token.expose().is_empty()` で調べる
- `open` で Builder に渡すときだけ、`token.expose().to_string()` で中身を取り出す

```text
選んだ形: Remote { url: "http://127.0.0.1:9", token: Secret(********) }
失敗（2048 ミリ秒）: Hrana: `http error: `error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)``
```

`Mode` の `#[derive(Debug)]` は、各フィールドの `Debug` を使って表示します。`Secret` の `Debug` が `Secret(********)` を出すので、`{mode:?}` で表示しても、トークンは隠れたままです。`Secret` に `Display` を実装しないでおけば、`{}` でうっかり表示しようとしても、コンパイルエラーで止められます（RustNote の `Secret` もこの形です）。

### まとめ

- DB の置き場所には、手元のファイルを使う**ローカル**、Turso に毎回問い合わせる**リモート**、手元のコピーを読んで Turso へ書く**組み込みレプリカ**の3つの形があります。開くときの Builder が違うだけで、開いた後の `connect`・`query`・`execute` は同じ書き方です。
- 組み込みレプリカでは、自分の書き込みはすぐ読めますが（read-your-writes）、ほかのサーバーの書き込みは `sync` するまで見えません。`sync` はレプリカでだけ使えます。
- DB の URL とトークンは環境変数で渡し、コードにもログにも出しません。トークンは `Secret` 型に包み、足りないときは起動した瞬間に止めます。
- `os error 10061` は接続の拒否、`Hrana` はリモートとの通信のエラー、E0599 の `new_remote` は features の不足です。この本の PC では、debug ビルドの `main` の中でリモートに `execute` するとスタックがあふれたので、release ビルドか `tokio::spawn` で動かします。
