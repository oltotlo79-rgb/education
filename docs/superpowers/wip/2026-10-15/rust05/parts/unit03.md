## UNIT 03: JoinSet・select!・同時に動かす数の管理

### この単元のゴール

- `JoinSet`（ジョインセット）で子のタスクをまとめて作り、**終わった順に**結果を受け取れる
- 親が `?` などで先に抜けると、`JoinSet` に残った子のタスクが取り消されることを、実行結果で確かめられる
- 「1つ失敗したら全体を止める」集め方と、「失敗しても全部集める」集め方を書き分けられる
- 同時に動かすタスクの数に、上限を付けられる
- `tokio::select!` で、先に終わったほうの結果だけを使い、負けたほうが捨てられることを説明できる
- ループの中の `select!` で Future を `pin!` で留めて使い回せる。E0271・E0277（`Unpin`）のエラーと、「仕事がいつまでも終わらない」間違いを直せる

### たとえ話でつかむ

この単元では、3つの場面を使います。

- **JoinSet は、遠足の班です。** 先生（親のタスク）は、子どもたち（子のタスク）を班にして連れて行きます。戻ってきた子から順に点呼を取ります（`join_next`）。先生が帰ると決めたら、まだ遊んでいる子も**全員連れて帰ります**（取り消し）。だれかを置き去りにすることはありません。
- **select! は、2つの店に出す出前の注文です。** 同じ料理を2つの店に頼み、先に届いたほうを食べます。遅れたほうには「もういりません」と電話して、作るのをやめてもらいます（負けた Future は捨てられる）。
- **同時に動かす数の上限は、試着室です。** 試着室が2つしかないお店では、3人目は、どちらかが空くまで外で待ちます。待たせることで、お店の中があふれません。

> **用語:** JoinSet（ジョインセット：子のタスクをまとめて持つ袋。`spawn` で入れ、`join_next` で終わった順に1つずつ取り出す。袋を捨てると、残った子は取り消される）

<!-- -->

> **用語:** 構造化された並行処理（structured concurrency：子のタスクが親より長生きしないように、親が子の始まりと終わりを責任を持って管理する書き方）

<!-- -->

> **用語:** select!（セレクト：複数の Future を同時に進め、最初に終わったものの枝だけを実行するマクロ。残りの Future は捨てる）

### 図で理解する

まず、JoinSet です。親のタスクが袋を持ち、子のタスクを入れて、終わった順に受け取ります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-031" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="30" width="400" height="200" rx="12" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="30" y="54" font-size="13" font-weight="700" fill="#0f172a">親のタスク（fetch_all）</text>
<rect x="34" y="70" width="360" height="96" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="48" y="92" font-size="12" font-weight="700" fill="#1e40af">JoinSet（子のタスクの袋）</text>
<rect x="48" y="106" width="100" height="46" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="98" y="125" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">子 A</text>
<text x="98" y="143" font-size="10" fill="#475569" text-anchor="middle">0.1 秒で成功</text>
<rect x="166" y="106" width="100" height="46" rx="8" fill="#ffffff" stroke="#ef4444" stroke-width="2"/>
<text x="216" y="125" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">子 B</text>
<text x="216" y="143" font-size="10" fill="#475569" text-anchor="middle">0.2 秒で失敗</text>
<rect x="284" y="106" width="100" height="46" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2" stroke-dasharray="5 4"/>
<text x="334" y="125" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">子 C</text>
<text x="334" y="143" font-size="10" fill="#475569" text-anchor="middle">1.0 秒かかる</text>
<text x="48" y="194" font-size="11" fill="#0f172a">set.spawn(…) で入れる</text>
<text x="48" y="214" font-size="11" fill="#0f172a">set.join_next().await で、終わった順に受け取る</text>
<line x1="394" y1="118" x2="446" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-031)"/>
<text x="578" y="54" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">join_next() が返す順番</text>
<rect x="450" y="68" width="256" height="34" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="578" y="90" font-size="11" fill="#047857" text-anchor="middle">1番目: A の結果（0.1 秒）</text>
<rect x="450" y="110" width="256" height="34" rx="8" fill="#fef2f2" stroke="#ef4444"/>
<text x="578" y="132" font-size="11" fill="#b91c1c" text-anchor="middle">2番目: B の Err（0.2 秒）→ ? で親が抜ける</text>
<rect x="450" y="152" width="256" height="34" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="578" y="174" font-size="11" fill="#475569" text-anchor="middle">C は受け取らないまま終わる</text>
<rect x="14" y="246" width="692" height="44" rx="10" fill="#fef2f2" stroke="#ef4444"/>
<text x="360" y="273" font-size="12" fill="#b91c1c" text-anchor="middle">親が抜けると袋ごと捨てられ、残った子 C は取り消される。子は親より長生きしない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: JoinSet のしくみ。子のタスクを袋に入れ、終わった順に受け取る。親が途中で抜けると、袋が捨てられて残りの子も取り消される</div>
</div>

- UNIT 02 の `tokio::spawn` だけで作ったタスクは、引換券を捨てると、親とは関係なく動き続けました（例2-4）。親が先に終わっても、子は残ります。
- JoinSet に入れたタスクは、**袋が捨てられると一緒に取り消されます**。親の関数が `return` や `?` で抜けると、関数の中の変数として持っていた袋も片付けられるので、「親が終わったのに、子だけが裏で動き続ける」ことが起きません。

次に、`select!` です。2つの Future を同時に進め、先に終わったほうを選びます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-032" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="64" font-size="12" font-weight="700" fill="#1e40af">primary_db()</text>
<text x="20" y="80" font-size="10" fill="#475569">0.5 秒かかる</text>
<text x="20" y="128" font-size="12" font-weight="700" fill="#047857">cache()</text>
<text x="20" y="144" font-size="10" fill="#475569">0.1 秒で終わる</text>
<rect x="160" y="50" width="100" height="32" rx="6" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="260" y="50" width="400" height="32" rx="6" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 5"/>
<text x="460" y="71" font-size="11" fill="#64748b" text-anchor="middle">本当は 0.5 秒まで続くはずだった部分</text>
<text x="260" y="74" font-size="20" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<rect x="160" y="112" width="100" height="32" rx="6" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<text x="272" y="133" font-size="12" font-weight="700" fill="#047857">← 先に終わった。この枝の「=&gt; の後ろ」を実行する</text>
<line x1="260" y1="40" x2="260" y2="178" stroke="#ef4444" stroke-width="2" stroke-dasharray="4 4"/>
<text x="268" y="170" font-size="11" fill="#b91c1c">0.1 秒: select! が終わり、primary_db の Future は捨てられる（drop）</text>
<line x1="160" y1="196" x2="680" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-032)"/>
<line x1="160" y1="190" x2="160" y2="202" stroke="#475569" stroke-width="2"/>
<line x1="260" y1="190" x2="260" y2="202" stroke="#475569" stroke-width="2"/>
<line x1="660" y1="190" x2="660" y2="202" stroke="#475569" stroke-width="2"/>
<text x="160" y="218" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="260" y="218" font-size="11" fill="#475569" text-anchor="middle">0.1 秒</text>
<text x="660" y="218" font-size="11" fill="#475569" text-anchor="middle">0.5 秒</text>
<text x="360" y="242" font-size="12" fill="#475569" text-anchor="middle">select! は、負けた枝の仕事を「止めて捨てる」。結果を待ってはくれない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: select! で2つの Future を競争させたところ。先に終わった cache の枝だけが実行され、primary_db の Future は 0.1 秒の時点で捨てられる</div>
</div>

- `select!` は、渡した Future を**同じタスクの中で**同時に進めます（`join!` と同じく並行で、並列ではありません）。
- どれか1つが終わったら、その枝の `=>` の後ろだけを実行し、ほかの Future は drop します。drop された Future は二度と poll されないので、**途中で止まります**（UNIT 01 の例1-3と同じキャンセルです）。

### まず動かす最小コード

#### 例3-1: JoinSet で子のタスクを作り、終わった順に受け取る

> **▼ このコードがやること（先に日本語で）:** 画像を縮小する仕事の代わりに、0.3 秒・0.1 秒・0.2 秒かかる3つの子のタスクを JoinSet に入れます。`join_next` で、終わった順に結果を1つずつ受け取り、そのときの時刻を表示します。

```rust
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 画像の縮小などの仕事の代わり: 指定の時間だけ待ってから結果を返す
async fn make_thumbnail(name: &'static str, millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{name} の縮小画像")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // JoinSet: 子タスクをまとめて持つ「親の袋」
    let mut set = JoinSet::new();
    set.spawn(make_thumbnail("a.png", 300));
    set.spawn(make_thumbnail("b.png", 100));
    set.spawn(make_thumbnail("c.png", 200));
    println!("子タスクの数: {}", set.len());

    // join_next: 「終わった順に」1つずつ結果を受け取る。全部受け取ると None
    while let Some(joined) = set.join_next().await {
        let thumbnail = joined.expect("子タスクが panic しました");
        println!("{:.1} 秒: {thumbnail}", start.elapsed().as_secs_f64());
    }
    println!("全部終わりました（残り {} 個）", set.len());
}
```

`src/bin/u03_joinset.rs` として保存して実行します。

```powershell
cargo run -q --bin u03_joinset
```

**▼ 実行結果**

```text
子タスクの数: 3
0.1 秒: b.png の縮小画像
0.2 秒: c.png の縮小画像
0.3 秒: a.png の縮小画像
全部終わりました（残り 0 個）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: set.spawn は、袋の中にタスクを作る

- `JoinSet::new()` で空の袋を作り、`set.spawn(Future)` で子のタスクを作って袋に入れます。`tokio::spawn` と同じく、入れた瞬間から動き始めます。
- `set.len()` は、袋の中に残っている（まだ受け取っていない）タスクの数です。

##### 解説2: join_next は「終わった順」

- UNIT 02 の例2-1では、引換券を作った順番に `.await` したので、結果も作った順に受け取りました。JoinSet の `join_next()` は、**いちばん早く終わったもの**から返します。実行結果が b → c → a の順になっているのはそのためです。
- 先に終わったものから順に処理を進められるので、「全部そろうまで何もできない」ことがありません。
- 戻り値は `Option<Result<T, JoinError>>` です。袋が空になると `None` が返るので、`while let Some(joined) = ...` で「袋が空になるまで繰り返す」と書けます。中の `Result` は、UNIT 02 と同じく「panic したかどうか」です。

##### 解説3: 袋に入れられるのは、同じ型を返すタスクだけ

- `JoinSet<String>` のように、袋には「何の型を返すタスクの袋か」が決まっています。ここでは `make_thumbnail` が `String` を返すので、`JoinSet<String>` になりました（型は Rust が推論しています）。
- 違う型を返すタスクを入れるとコンパイルエラーになります（「よくあるエラー1」）。

> **ヒント:** 全部の結果を一度にまとめて受け取ればよいときは、`set.join_all().await` も使えます。全部が終わるのを待ち、結果を `Vec` で返します（例3-4で使います）。ただし、`join_all` は子のどれかが panic すると、自分も panic します（tokio 1.53.3 のソースで、panic をそのまま投げ直す作りになっていることを確かめました）。1つずつ確かめたいときは `join_next` を使います。

#### 例3-2: select! で、先に終わったほうを使う

> **▼ このコードがやること（先に日本語で）:** 遅い「本命の DB」と速い「予備のキャッシュ」に、同時に問い合わせます。`select!` で先に終わったほうの結果を使い、負けたほうが途中で片付けられることを、`Drop` で表示して確かめます。①は キャッシュが速い場合、②は DB が速い場合です。

```rust
use std::time::{Duration, Instant};

struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  片付け: {}", self.0);
    }
}

// 本命の DB（遅い）
async fn primary_db(millis: u64) -> &'static str {
    let _announce = Announce("primary_db");
    tokio::time::sleep(Duration::from_millis(millis)).await;
    println!("  primary_db: 最後まで終わりました");
    "本命の DB の結果"
}

// 予備のキャッシュ（速い）
async fn cache(millis: u64) -> &'static str {
    let _announce = Announce("cache");
    tokio::time::sleep(Duration::from_millis(millis)).await;
    println!("  cache: 最後まで終わりました");
    "キャッシュの結果"
}

async fn race(db_millis: u64, cache_millis: u64) {
    let start = Instant::now();
    // 2つの Future を同時に進め、先に終わったほうの枝だけを実行する
    let winner = tokio::select! {
        value = primary_db(db_millis) => format!("DB が勝ち: {value}"),
        value = cache(cache_millis) => format!("キャッシュが勝ち: {value}"),
    };
    println!("{winner}（{:.1} 秒）", start.elapsed().as_secs_f64());
}

#[tokio::main]
async fn main() {
    println!("① DB 500ms / キャッシュ 100ms");
    race(500, 100).await;
    println!("② DB 100ms / キャッシュ 500ms");
    race(100, 500).await;
}
```

`src/bin/u03_select.rs` として保存して実行します。

```powershell
cargo run -q --bin u03_select
```

**▼ 実行結果**

```text
① DB 500ms / キャッシュ 100ms
  cache: 最後まで終わりました
  片付け: cache
  片付け: primary_db
キャッシュが勝ち: キャッシュの結果（0.1 秒）
② DB 100ms / キャッシュ 500ms
  primary_db: 最後まで終わりました
  片付け: primary_db
  片付け: cache
DB が勝ち: 本命の DB の結果（0.1 秒）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: select! の枝は「パターン = Future => 式」

- `select!` の中の1行1行を**枝**と呼びます。`value = primary_db(db_millis) => format!(...)` は、「`primary_db(...)` が終わったら、その結果を `value` に入れて、`=>` の後ろの式を実行する」という意味です。
- 先に終わった枝の式の値が、`select!` 全体の値になります。ここでは、どちらの枝も `String` を返すので、`winner` は `String` です。すべての枝の式は、同じ型でなければなりません。

##### 解説2: 負けた Future は、その場で捨てられる

- ①では、0.1 秒でキャッシュが終わりました。`片付け: cache` はキャッシュの関数が最後まで動いて自然に片付いた印、`片付け: primary_db` は、**負けた DB の Future が途中で捨てられた**印です。「primary_db: 最後まで終わりました」は表示されていません。
- ②では、勝ち負けが逆になりました。どちらの場合も、全体は勝ったほうの 0.1 秒で終わり、負けたほうの残りの時間は待っていません。

##### 解説3: Drop で「片付け」を目に見えるようにする

- `Announce` は、片付けられるときに表示するだけの小さな struct です。第3巻 UNIT 01 で学んだ `Drop` trait を実装しています。
- `let _announce = Announce("cache");` のように関数の最初で作っておくと、関数が最後まで動いて終わったときも、途中で Future ごと捨てられたときも、必ず `drop` が呼ばれます。キャンセルされたかどうかを調べるときに便利な道具です。
- 変数名を `_announce` にしているのは、「使わないけれど、関数の終わりまで持っておく」ためです。`_` 1文字だけにすると、その場ですぐに片付けられてしまうので注意しましょう。

> **注意:** `select!` で負けた Future は、途中のどこで止まるか分かりません。「ファイルに半分だけ書いた」「お金を引き落としたのに、記録を書く前に止まった」のように、途中で止まると困る処理を、そのまま `select!` の枝に入れないようにしましょう。途中で捨てられても困らない性質を**キャンセル安全**（cancel safety）と呼びます。tokio のドキュメントには、関数ごとに「この関数はキャンセル安全です（This method is cancel safe）」のような説明が書いてあります。

### もう一歩: 実務に近い形

#### 例3-3: 1つ失敗したら、残りの子をまとめて取り消す

> **▼ このコードがやること（先に日本語で）:** 3つの外部サービス（A は 0.1 秒、B は 0.2 秒、C は 1.0 秒）に同時に問い合わせます。B だけが失敗します。1つでも失敗したら全体を失敗にし、まだ終わっていない C は待たずに取り消します。C が本当に途中で取り消されたかを、片付けの表示と時刻で確かめます。

```rust
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 片付けのときに表示する印。タスクが途中で取り消されると、ここで分かる
struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  片付け: {}", self.0);
    }
}

// 外部サービスへの問い合わせの代わり。name が "B" のときだけ失敗する
async fn fetch(name: &'static str, millis: u64) -> Result<String, String> {
    let _announce = Announce(name);
    tokio::time::sleep(Duration::from_millis(millis)).await;
    if name == "B" {
        return Err(format!("{name} は応答しませんでした"));
    }
    Ok(format!("{name} の結果"))
}

async fn fetch_all() -> Result<Vec<String>, String> {
    let mut set = JoinSet::new();
    set.spawn(fetch("A", 100));
    set.spawn(fetch("B", 200));
    set.spawn(fetch("C", 1000));

    let mut results = Vec::new();
    while let Some(joined) = set.join_next().await {
        // 1つ目の ? は「panic したか」、2つ目の ? は「Err を返したか」
        let value = joined.map_err(|e| e.to_string())??;
        results.push(value);
    }
    Ok(results)
    // 途中で return Err した場合も、ここで set が片付けられ、残りの子タスクは取り消される
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let result = fetch_all().await;
    println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
    // 取り消された子タスクの片付けが表示される時間を少しだけ待つ
    tokio::time::sleep(Duration::from_millis(50)).await;
    println!(
        "main が終わります（{:.1} 秒）",
        start.elapsed().as_secs_f64()
    );
}
```

`src/bin/u03_joinset_fail.rs` として保存して実行します。

```powershell
cargo run -q --bin u03_joinset_fail
```

**▼ 実行結果**

```text
  片付け: A
  片付け: B
Err("B は応答しませんでした")（0.2 秒）
  片付け: C
main が終わります（0.3 秒）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ?? で「panic」と「アプリのエラー」の両方を返す

- `joined` の型は `Result<Result<String, String>, JoinError>` です（UNIT 02 の練習問題3と同じ形）。
- `joined.map_err(|e| e.to_string())` で、外側のエラー（`JoinError`）を `String` に変えています。これで外側も内側もエラーの型が `String` にそろうので、`?` を2回続けて書けます。1つ目の `?` で panic を、2つ目の `?` で `fetch` の `Err` を、`fetch_all` のエラーとして返します。

##### 解説2: 親が抜けると、袋の中の子は取り消される

- 0.2 秒で B の `Err` を受け取ると、2つ目の `?` で `fetch_all` から抜けます。そのとき、関数の中の変数 `set` が片付けられ、JoinSet は**中に残っていた子（C）を全部取り消します**。
- 実行結果の `片付け: C` は、`main が終わります（0.3 秒）` より前に表示されています。C は 1.0 秒かかるはずだったのに、0.3 秒より前に片付けられました。プログラムが終わったからではなく、JoinSet が捨てられたことで取り消されたのです。tokio 1.53.3 のソースでも、JoinSet が片付けられるときに、中のタスクを1つずつ `abort` する作りになっていることを確かめました。
- `片付け: C` が結果の行より後に表示されるのは、取り消しの合図を受けた C が、ランタイムの上で少し遅れて片付けられるからです。`main` の最後で 0.05 秒待っているのは、その表示を見るためだけです。

##### 解説3: 「全部そろわないと意味がない」ときの形

- 画面に A・B・C の3つの情報を並べるページで、1つでも欠けたらエラーページを出す、というような場面では、この「1つ失敗したら全部やめる」形が向いています。失敗が分かった時点で残りを取り消すので、むだな問い合わせが続きません。
- 失敗しても、そろった分だけで画面を出したいときは、エラーも結果として集める形にします（練習問題3）。

#### 例3-4: 同時に動かす数を、2つまでに抑える

> **▼ このコードがやること（先に日本語で）:** 0.2 秒かかるダウンロードを6つ行います。ただし、同時に動かすのは2つまでにします。袋に2つ入っているときは、どれか1つが終わるまで待ってから、次を入れます。それぞれを始めた時刻と、全体の時間を表示します。

```rust
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 外部へのリクエストの代わり。どれも 200ms かかる
async fn download(id: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(200)).await;
    id
}

#[tokio::main]
async fn main() {
    // 同時に動かすのは最大 2 個まで
    const LIMIT: usize = 2;
    let start = Instant::now();
    let mut set = JoinSet::new();
    let mut done = Vec::new();

    for id in 1..=6 {
        // 袋がいっぱいなら、どれか1つ終わるまで待ってから次を入れる
        if set.len() >= LIMIT {
            // 袋には LIMIT 個入っているので、join_next は必ず Some を返す
            let joined = set.join_next().await.expect("袋は空ではない");
            done.push(joined.expect("子タスクが panic しました"));
        }
        println!("{:.2} 秒: {id} 番を開始", start.elapsed().as_secs_f64());
        set.spawn(download(id));
    }
    // 残りを全部受け取る。join_all は全部終わるのを待って Vec で返す
    done.extend(set.join_all().await);

    done.sort();
    println!("{done:?}（{:.2} 秒）", start.elapsed().as_secs_f64());
}
```

`src/bin/u03_limited.rs` として保存して実行します。

```powershell
cargo run -q --bin u03_limited
```

**▼ 実行結果**

```text
0.00 秒: 1 番を開始
0.00 秒: 2 番を開始
0.21 秒: 3 番を開始
0.21 秒: 4 番を開始
0.41 秒: 5 番を開始
0.41 秒: 6 番を開始
[1, 2, 3, 4, 5, 6]（0.63 秒）
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-033" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="20" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">同時に動くのは最大2つ（LIMIT = 2）</text>
<text x="60" y="51" font-size="12" fill="#0f172a" text-anchor="end">1 番</text>
<text x="60" y="83" font-size="12" fill="#0f172a" text-anchor="end">2 番</text>
<text x="60" y="115" font-size="12" fill="#0f172a" text-anchor="end">3 番</text>
<text x="60" y="147" font-size="12" fill="#0f172a" text-anchor="end">4 番</text>
<text x="60" y="179" font-size="12" fill="#0f172a" text-anchor="end">5 番</text>
<text x="60" y="211" font-size="12" fill="#0f172a" text-anchor="end">6 番</text>
<rect x="80" y="36" width="180" height="22" rx="5" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="80" y="68" width="180" height="22" rx="5" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="260" y="100" width="180" height="22" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="260" y="132" width="180" height="22" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="440" y="164" width="180" height="22" rx="5" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<rect x="440" y="196" width="180" height="22" rx="5" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<rect x="80" y="100" width="180" height="22" rx="5" fill="none" stroke="#94a3b8" stroke-dasharray="4 4"/>
<rect x="80" y="132" width="180" height="22" rx="5" fill="none" stroke="#94a3b8" stroke-dasharray="4 4"/>
<text x="170" y="116" font-size="10" fill="#64748b" text-anchor="middle">試着室の空き待ち</text>
<text x="170" y="148" font-size="10" fill="#64748b" text-anchor="middle">試着室の空き待ち</text>
<line x1="260" y1="30" x2="260" y2="228" stroke="#64748b" stroke-width="1" stroke-dasharray="4 4"/>
<line x1="440" y1="30" x2="440" y2="228" stroke="#64748b" stroke-width="1" stroke-dasharray="4 4"/>
<text x="632" y="116" font-size="11" fill="#475569">1つ終わったら</text>
<text x="632" y="132" font-size="11" fill="#475569">次を入れる</text>
<line x1="80" y1="240" x2="680" y2="240" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-033)"/>
<line x1="80" y1="234" x2="80" y2="246" stroke="#475569" stroke-width="2"/>
<line x1="260" y1="234" x2="260" y2="246" stroke="#475569" stroke-width="2"/>
<line x1="440" y1="234" x2="440" y2="246" stroke="#475569" stroke-width="2"/>
<line x1="620" y1="234" x2="620" y2="246" stroke="#475569" stroke-width="2"/>
<text x="80" y="262" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="260" y="262" font-size="11" fill="#475569" text-anchor="middle">約 0.2 秒</text>
<text x="440" y="262" font-size="11" fill="#475569" text-anchor="middle">約 0.4 秒</text>
<text x="620" y="262" font-size="11" fill="#475569" text-anchor="middle">約 0.6 秒</text>
<text x="360" y="284" font-size="12" fill="#475569" text-anchor="middle">上限が無ければ全体は 0.2 秒、1つずつなら 1.2 秒。上限 2 なら約 0.6 秒</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 同時に動かす数を2つまでにしたときの動き。3番以降は、前のどれかが終わるまで始まらない。待たせることで、相手のサーバーに一度に押し寄せない</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 袋の中の数で、次を入れるかどうかを決める

- ループのたびに `set.len()`（袋の中の、動いているタスクの数）を見ます。`LIMIT`（2）以上なら、`join_next().await` で1つ終わるのを待ってから、次を入れます。袋の中は、いつも2つ以下に保たれます。
- 実行結果では、1・2番が 0 秒、3・4番が約 0.2 秒、5・6番が約 0.4 秒に始まっています。全体は約 0.6 秒です。
- 0.2 秒ではなく 0.21 秒のように少し遅れているのは、タイマーの都合で、少し遅れて起こされることがあるためです。パソコンや OS によって数字は少し変わります。

##### 解説2: なぜわざわざ待たせるのか

- 相手のサーバーや DB には、一度に受けられる数に限りがあります。1万件のダウンロードを一度に始めると、相手に断られたり、自分のメモリや接続が足りなくなったりします。
- 上限を付けると、全体の時間は少し長くなりますが、**一度に使う資源の量が上限で決まる**ので、件数が増えても安全に動きます。「全部同時」と「1つずつ」の間の、ちょうどよいところを選べるのが、この形のよいところです。

##### 解説3: 残りは join_all でまとめて受け取る

- ループが終わった時点で、袋にはまだ最後の2つが残っています。`set.join_all().await` は、残りの全部が終わるのを待ち、結果を `Vec` で返します。
- `join_next` も `join_all` も、結果は**終わった順**に並びます。番号の順に並べたいので、最後に `done.sort()` で並べ替えています。

> **ヒント:** 同時に動かす数の上限は、`Semaphore`（セマフォ）という道具を使っても作れます。たくさんのタスクが、それぞれ別の場所から「入ってよいか」を確かめたいときは、そちらが便利です。UNIT 05 で学びます。

#### 例3-5: 締め切りまで、仕事を繰り返す（select! をループの中で使う）

> **▼ このコードがやること（先に日本語で）:** 1 秒後の締め切りを1つ作り、それまでの間、0.3 秒かかる仕事を繰り返します。ループのたびに `select!` で「締め切り」と「次の仕事」を競争させ、締め切りが来たらループを抜けます。

```rust
use std::pin::pin;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 全体の締め切り（1 秒後）。pin! で留めておくと、&mut で何度でも見張れる
    let mut deadline = pin!(tokio::time::sleep(Duration::from_secs(1)));
    let mut ticks = 0;
    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("{:.1} 秒: 締め切りです", start.elapsed().as_secs_f64());
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(300)) => {
                ticks += 1;
                println!("{:.1} 秒: {ticks} 回目の仕事", start.elapsed().as_secs_f64());
            }
        }
    }
}
```

`src/bin/u03_deadline.rs` として保存して実行します。

```powershell
cargo run -q --bin u03_deadline
```

**▼ 実行結果**

```text
0.3 秒: 1 回目の仕事
0.6 秒: 2 回目の仕事
0.9 秒: 3 回目の仕事
1.0 秒: 締め切りです
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 締め切りは、ループの外で1回だけ作る

- 締め切りの `sleep` は、ループの**外**で1回だけ作っています。もしループの中で `sleep(Duration::from_secs(1))` を毎回作ると、毎回「今から1秒」になってしまい、締め切りがいつまでも来ません。
- ループの中では、`&mut deadline` で締め切りを**借りて**見張ります。`select!` の別の枝が勝っても、借りていただけなので、締め切りの Future そのものは捨てられず、次のループで続きから見張れます。

##### 解説2: &mut で借りて poll するには、pin! で留める必要がある

- `pin!(...)` は、UNIT 01 の例1-4で使ったのと同じ「動かないように留める」マクロです（`std::pin::pin`）。
- `tokio::time::sleep` が作る Future（`Sleep`）は、poll している途中で場所が動くと困る種類の Future です。`&mut` で借りて何度も poll するには、先に留めておく必要があります。留めずに書くとどうなるかは、「よくあるエラー2」で見ます。

##### 解説3: 0.9 秒の次の仕事は、締め切りで捨てられた

- 4回目の仕事は 0.9 秒に始まり、1.2 秒に終わるはずでした。しかし 1.0 秒に締め切りの枝が勝ったので、4回目の `sleep(300ms)` は捨てられ、表示されていません。
- 締め切りを守るために、途中の仕事を捨てる、という形です。捨てられて困る仕事なら、締め切りを過ぎても最後まで終わらせるようにします（UNIT 06 の graceful shutdown）。

### よくあるエラーと読み方

#### エラー1: JoinSet に違う型を返すタスクを入れる（E0271）

```rust
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async { 42 });
    // 1つ目と違う型の値を返すタスクを入れる
    set.spawn(async { String::from("文字列") });
    while let Some(joined) = set.join_next().await {
        println!("{:?}", joined);
    }
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0271]: expected `{async block@e03_joinset_types.rs:8:15}` to be a future that resolves to `{integer}`, but it resolves to `String`
   --> src\bin\e03_joinset_types.rs:8:15
    |
  8 |     set.spawn(async { String::from("文字列") });
    |         ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected integer, found `String`
    |         |
    |         required by a bound introduced by this call
    |
note: required by a bound in `JoinSet::<T>::spawn`
   --> ...\tokio-1.53.3\src\task\join_set.rs:144:19
    |
142 |     pub fn spawn<F>(&mut self, task: F) -> AbortHandle
    |            ----- required by a bound in this associated function
143 |     where
144 |         F: Future<Output = T>,
    |                   ^^^^^^^^^^ required by this bound in `JoinSet::<T>::spawn`

For more information about this error, try `rustc --explain E0271`.
error: could not compile `async_lab` (bin "e03_joinset_types") due to 1 previous error
```

- **何が起きたか:** 「この async ブロックは整数（`{integer}`）を返す Future のはずなのに、`String` を返しています」というエラーです。1つ目の `set.spawn(async { 42 })` で、Rust は「これは整数を返すタスクの袋だ」と決めました。そこに `String` を返すタスクを入れようとしたので、型が合いません。
- `F: Future<Output = T>` は、「入れるタスクの戻り値の型（`Output`）は、袋の型 `T` と同じでなければならない」という条件です。
- **直し方:** 戻り値の型をそろえます。違う種類の結果を1つの袋で扱いたいときは、`enum` を作って包みます（第2巻 UNIT 08）。たとえば `enum Answer { Count(i32), Title(String) }` を作り、`async { Answer::Count(42) }` と `async { Answer::Title(String::from("文字列")) }` のように入れます。

#### エラー2: 締め切りを留めずに &mut で見張る（E0277: PhantomPinned cannot be unpinned）

例3-5の `pin!` を付け忘れた場合です。

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 全体の締め切り（1 秒後）
    let deadline = tokio::time::sleep(Duration::from_secs(1));
    let mut ticks = 0;
    loop {
        tokio::select! {
            // 同じ締め切りを、ループのたびに見張りたい
            _ = &mut deadline => {
                println!("締め切りです");
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(300)) => {
                ticks += 1;
                println!("{ticks} 回目の仕事");
            }
        }
    }
}
```

同じ形のエラーが3つ出ます。1つ目だけを載せます。

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0277]: `PhantomPinned` cannot be unpinned
  --> src\bin\e03_select_unpin.rs:9:9
   |
 9 | /         tokio::select! {
10 | |             // 同じ締め切りを、ループのたびに見張りたい
11 | |             _ = &mut deadline => {
12 | |                 println!("締め切りです");
...  |
19 | |         }
   | |         ^
   | |         |
   | |_________within `(PhantomData<&()>, PhantomPinned)`, the trait `Unpin` is not implemented for `PhantomPinned`
   |           required by a bound introduced by this call
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required because it appears within the type `(PhantomData<&()>, PhantomPinned)`
   = note: required for `Sleep` to implement `Unpin`
   = note: required for `&mut Sleep` to implement `Future`
   = note: required for `&mut Sleep` to implement `IntoFuture`
   = note: this error originates in the macro `$crate::select` which comes from the expansion of the macro `tokio::select` (in Nightly builds, run with -Z macro-backtrace for more info)
```

難しそうな言葉が並んでいますが、下から順に読むと筋が通ります。

- `required for &mut Sleep to implement Future`: 「`&mut Sleep`（借りた締め切り）を Future として使うには、条件があります」。
- `required for Sleep to implement Unpin`: その条件は「`Sleep` が `Unpin`（動かしても平気）であること」です。
- `the trait Unpin is not implemented for PhantomPinned`: でも `Sleep` の中には「動かしてはいけない」という印（`PhantomPinned`）が入っているので、`Unpin` ではありません。
- `note: consider using the pin! macro`: だから「`pin!` マクロを使ってください」。これが直し方です。

- **直し方:** 例3-5のとおり、`let mut deadline = pin!(tokio::time::sleep(...));` のように、作るときに `pin!` で留めます。留めた Future は、`&mut deadline` で借りて何度でも poll できます。
- `Unpin` は「poll の途中で場所を動かしても平気な型」に付く印です。多くの型は自動で `Unpin` ですが、`Sleep` や async fn が作る Future の多くは `Unpin` ではありません。「`&mut` で借りて select! に渡すときは、`pin!` が要る」と覚えておけば十分です。

#### エラー3（エラーが出ない間違い）: ループの中の select! で、仕事が毎回最初からやり直しになる

コンパイルは通り、panic もしないのに、仕事がいつまでも終わらない例です。0.5 秒かかる仕事を進めながら、0.2 秒ごとに見回りの表示を出すつもりで書きました。

```rust
use std::time::{Duration, Instant};

// 0.5 秒かかる仕事
async fn long_job() -> &'static str {
    println!("  long_job: 始めます");
    tokio::time::sleep(Duration::from_millis(500)).await;
    "仕事の結果"
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    for round in 1..=4 {
        tokio::select! {
            // ループのたびに、新しい long_job() を作ってしまう
            result = long_job() => {
                println!("{:.1} 秒: {result}", start.elapsed().as_secs_f64());
                return;
            }
            _ = tokio::time::sleep(Duration::from_millis(200)) => {
                println!("{:.1} 秒: 見回り {round} 回目", start.elapsed().as_secs_f64());
            }
        }
    }
    println!(
        "{:.1} 秒: 仕事は一度も終わりませんでした",
        start.elapsed().as_secs_f64()
    );
}
```

`src/bin/u03_select_restart.rs` として実行した結果です（終わらないと困るので、ループは4回までにしています）。

```text
  long_job: 始めます
0.2 秒: 見回り 1 回目
  long_job: 始めます
0.4 秒: 見回り 2 回目
  long_job: 始めます
0.6 秒: 見回り 3 回目
  long_job: 始めます
0.8 秒: 見回り 4 回目
0.8 秒: 仕事は一度も終わりませんでした
```

- **何が起きたか:** 「始めます」が4回も表示されています。`select!` の枝に `long_job()` と書いたので、**ループのたびに新しい `long_job` の Future が作られ**、0.2 秒で見回りの枝が勝つたびに、途中まで進んだ仕事が捨てられていました。0.5 秒かかる仕事は、0.2 秒より長く続けてもらえないので、永遠に終わりません。
- **直し方:** 例3-5の締め切りと同じく、仕事の Future をループの外で1回だけ作り、`pin!` で留めて `&mut` で借ります。

```rust
use std::pin::pin;
use std::time::{Duration, Instant};

// 0.5 秒かかる仕事
async fn long_job() -> &'static str {
    println!("  long_job: 始めます");
    tokio::time::sleep(Duration::from_millis(500)).await;
    "仕事の結果"
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 仕事の Future はループの外で1回だけ作り、pin! で留める
    let mut job = pin!(long_job());
    for round in 1..=4 {
        tokio::select! {
            // &mut で借りて進める。負けても捨てられず、次のループで続きから進む
            result = &mut job => {
                println!("{:.1} 秒: {result}", start.elapsed().as_secs_f64());
                return;
            }
            _ = tokio::time::sleep(Duration::from_millis(200)) => {
                println!("{:.1} 秒: 見回り {round} 回目", start.elapsed().as_secs_f64());
            }
        }
    }
    println!(
        "{:.1} 秒: 仕事は一度も終わりませんでした",
        start.elapsed().as_secs_f64()
    );
}
```

`src/bin/u03_select_resume.rs` として実行した結果です。

```text
  long_job: 始めます
0.2 秒: 見回り 1 回目
0.4 秒: 見回り 2 回目
0.5 秒: 仕事の結果
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-034" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="100" y="24" font-size="12" font-weight="700" fill="#b91c1c">ループのたびに long_job() を作る（u03_select_restart）</text>
<rect x="100" y="36" width="140" height="28" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="240" y="36" width="140" height="28" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="380" y="36" width="140" height="28" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="520" y="36" width="140" height="28" rx="5" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<text x="170" y="55" font-size="11" fill="#c2410c" text-anchor="middle">始めます → 捨てる</text>
<text x="310" y="55" font-size="11" fill="#c2410c" text-anchor="middle">始めます → 捨てる</text>
<text x="450" y="55" font-size="11" fill="#c2410c" text-anchor="middle">始めます → 捨てる</text>
<text x="590" y="55" font-size="11" fill="#c2410c" text-anchor="middle">始めます → 捨てる</text>
<text x="240" y="84" font-size="16" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<text x="380" y="84" font-size="16" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<text x="520" y="84" font-size="16" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<text x="660" y="84" font-size="16" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<text x="100" y="100" font-size="11" fill="#b91c1c">0.5 秒続けてもらえないので、一度も終わらない</text>
<text x="100" y="134" font-size="12" font-weight="700" fill="#047857">ループの外で1回だけ作り、pin! で留める（u03_select_resume）</text>
<rect x="100" y="146" width="350" height="28" rx="5" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<text x="275" y="165" font-size="11" fill="#047857" text-anchor="middle">見回りの間も、続きから進む</text>
<text x="470" y="165" font-size="12" font-weight="700" fill="#047857">✓ 0.5 秒で結果</text>
<line x1="240" y1="176" x2="240" y2="184" stroke="#64748b" stroke-width="2"/>
<line x1="380" y1="176" x2="380" y2="184" stroke="#64748b" stroke-width="2"/>
<text x="240" y="194" font-size="10" fill="#475569" text-anchor="middle">見回り1</text>
<text x="380" y="194" font-size="10" fill="#475569" text-anchor="middle">見回り2</text>
<line x1="100" y1="214" x2="680" y2="214" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-034)"/>
<line x1="100" y1="208" x2="100" y2="220" stroke="#475569" stroke-width="2"/>
<line x1="240" y1="208" x2="240" y2="220" stroke="#475569" stroke-width="2"/>
<line x1="380" y1="208" x2="380" y2="220" stroke="#475569" stroke-width="2"/>
<line x1="450" y1="208" x2="450" y2="220" stroke="#475569" stroke-width="2"/>
<line x1="520" y1="208" x2="520" y2="220" stroke="#475569" stroke-width="2"/>
<line x1="660" y1="208" x2="660" y2="220" stroke="#475569" stroke-width="2"/>
<text x="100" y="236" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="240" y="236" font-size="11" fill="#475569" text-anchor="middle">0.2</text>
<text x="380" y="236" font-size="11" fill="#475569" text-anchor="middle">0.4</text>
<text x="450" y="236" font-size="11" fill="#475569" text-anchor="middle">0.5</text>
<text x="520" y="236" font-size="11" fill="#475569" text-anchor="middle">0.6</text>
<text x="660" y="236" font-size="11" fill="#475569" text-anchor="middle">0.8 秒</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: ループの中の select! の2つの書き方。枝の中で毎回 Future を作ると、負けるたびに捨てられて最初からやり直しになる。外で作って &amp;mut で借りれば、続きから進む</div>
</div>

- 「始めます」は1回だけで、見回りを2回はさんで、0.5 秒で仕事が終わりました。`&mut job` で借りているので、見回りの枝が勝っても `job` は捨てられず、次のループでは**続きから**進みます。
- `select!` をループの中で使うときは、「この枝の Future は、ループのたびに作り直されていないか？」を必ず確かめましょう。作り直してよいのは、例3-5の `sleep(300ms)` のように、毎回最初からでかまわない Future だけです。

### RustNote ではここで使う

RustNote のコードそのものには、`JoinSet` も `select!` も出てきません。1つのリクエストの処理は、DB に順番に問い合わせるだけで済んでいるからです。一方で、RustNote を動かしている Topcoat の中では、この単元の形がそのまま使われています。UNIT 02 で読んだ Topcoat 0.10.0 の `internal_serve`（`topcoat-router-0.10.0/src/service.rs`）を、もう少し詳しく見てみましょう（コメントは日本語に置き換え、関係の無い部分は省略しています）。

```rust
// 止める合図の Future を、ループの外で1回だけ留める
let mut shutdown = pin!(shutdown);

loop {
    let accepted = tokio::select! {
        // 新しい接続を受け付ける
        accepted = listener.accept() => accepted,
        // 止める合図（Ctrl+C）が来たら、ループを抜ける
        () = &mut shutdown => break,
    };
    let (stream, remote_addr) = accepted?;
    // （1つの接続を、1つのタスクで処理する。UNIT 02 で読んだ部分。省略）
}
```

- 止める合図 `shutdown` は、ループの外で `pin!` で留め、ループの中では `&mut shutdown` で借りています。例3-5の締め切りと、まったく同じ形です。もし枝の中で毎回作り直していたら、エラー3のように、合図を見逃し続けるかもしれません。
- 接続を受け付ける `listener.accept()` は、ループのたびに新しく作っています。これが許されるのは、tokio の `TcpListener::accept` がキャンセル安全だからです。tokio 1.53.3 のソースには「このメソッドはキャンセル安全です。select! の枝で使い、ほかの枝が先に終わったときは、新しい接続を受け付けていないことが保証されます」と書かれています。途中で捨てても、接続を取りこぼしません。
- 止める合図を受けたら、処理中の接続をしばらく待ってから終わります。その部分は、UNIT 04 の channel と UNIT 06 の graceful shutdown で読みます。

> **ヒント:** 自分のアプリで「1つの画面のために、関係の無い3つの問い合わせをする」ようになったら、UNIT 01 の `try_join!` か、この単元の JoinSet を思い出しましょう。問い合わせの数が決まっているなら `try_join!`、数が実行するまで分からないなら JoinSet が向いています。

### 練習問題

**問1:** 例3-4の `u03_limited.rs` で、`LIMIT` を `2` から `3` に変えると、それぞれの番号が始まる時刻と、全体の時間はどうなるでしょうか。予想してから実行してください（`src/bin/u03_q1.rs`）。

**問2:** 0.8 秒かかる問い合わせ `slow_query` に、「指定した時間を過ぎたら時間切れ」のエラーを返す仕組みを、`select!` で作ってください。300 ミリ秒と 1000 ミリ秒の2通りで試します（`src/bin/u03_q2.rs`）。

```rust
// 遅い問い合わせ（0.8 秒かかる）
async fn slow_query() -> &'static str {
    tokio::time::sleep(Duration::from_millis(800)).await;
    "問い合わせの結果"
}
```

**問3:** 例3-3の `fetch_all` を書き換えて、B が失敗しても止めずに、A・B・C の全部の結果（成功も失敗も）を `Vec<Result<String, String>>` で返すようにしてください。全体で何秒かかるかも予想しましょう（`src/bin/u03_q3.rs`）。

### 解答と解説

**問1の解答:** `const LIMIT: usize = 3;` に変えて実行した結果です（コメントの「最大 2 個まで」も「最大 3 個まで」に直しました。ほかは例3-4と同じです）。

```text
0.00 秒: 1 番を開始
0.00 秒: 2 番を開始
0.00 秒: 3 番を開始
0.21 秒: 4 番を開始
0.21 秒: 5 番を開始
0.21 秒: 6 番を開始
[1, 2, 3, 4, 5, 6]（0.43 秒）
```

- 最初の3つが同時に始まり、約 0.2 秒後に残りの3つが始まります。6つを3つずつ2回に分けたので、全体は約 0.4 秒です。
- 上限を上げるほど全体は速くなりますが、一度に使う資源（相手のサーバーへの接続など）も増えます。上限は「速さ」と「相手への負担」の兼ね合いで決めます。

**問2の解答:**

```rust
use std::time::{Duration, Instant};

// 遅い問い合わせ（0.8 秒かかる）
async fn slow_query() -> &'static str {
    tokio::time::sleep(Duration::from_millis(800)).await;
    "問い合わせの結果"
}

// 問い合わせと limit のタイマーを競争させ、タイマーが勝ったら時間切れにする
async fn query_with_limit(limit: Duration) -> Result<&'static str, String> {
    tokio::select! {
        value = slow_query() => Ok(value),
        _ = tokio::time::sleep(limit) => Err(format!("{} ミリ秒で時間切れ", limit.as_millis())),
    }
}

#[tokio::main]
async fn main() {
    for millis in [300, 1000] {
        let start = Instant::now();
        let result = query_with_limit(Duration::from_millis(millis)).await;
        println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
    }
}
```

```text
Err("300 ミリ秒で時間切れ")（0.3 秒）
Ok("問い合わせの結果")（0.8 秒）
```

- 問い合わせとタイマーを競争させ、タイマーが勝ったら `Err` を返します。300 ミリ秒ではタイマーが勝ち、0.3 秒で時間切れになりました。1000 ミリ秒では問い合わせが先に終わり、0.8 秒で結果が返りました。
- 時間切れのとき、`slow_query` の Future は途中で捨てられます（例3-2と同じです）。
- 「時間を区切って待つ」はとてもよく使うので、tokio には専用の関数 `tokio::time::timeout` が用意されています。UNIT 06 で学びます。中でやっていることは、この解答とほぼ同じです。

**問3の解答:**

```rust
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 外部サービスへの問い合わせの代わり。name が "B" のときだけ失敗する
async fn fetch(name: &'static str, millis: u64) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    if name == "B" {
        return Err(format!("{name} は応答しませんでした"));
    }
    Ok(format!("{name} の結果"))
}

// 失敗があっても止めずに、全部の結果を集める
async fn fetch_all() -> Vec<Result<String, String>> {
    let mut set = JoinSet::new();
    set.spawn(fetch("A", 100));
    set.spawn(fetch("B", 200));
    set.spawn(fetch("C", 1000));

    let mut results = Vec::new();
    while let Some(joined) = set.join_next().await {
        // panic は Err に変えて、アプリのエラーと同じように集める
        let result = joined.unwrap_or_else(|e| Err(e.to_string()));
        results.push(result);
    }
    results
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    for result in fetch_all().await {
        println!("{result:?}");
    }
    println!("{:.1} 秒", start.elapsed().as_secs_f64());
}
```

```text
Ok("A の結果")
Err("B は応答しませんでした")
Ok("C の結果")
1.0 秒
```

- `?` を使わずに、`Result` をそのまま `results` に入れていきます。B の `Err` も1つの結果として集めるので、ループは止まらず、C（1.0 秒）まで待ちます。全体は、いちばん長い C の 1.0 秒です。
- `joined.unwrap_or_else(|e| Err(e.to_string()))` は、「panic していなければ中身の `Result` をそのまま使い、panic していたら（`JoinError` なら）それを `Err` の文字列に変える」という意味です。panic した子も、1つの失敗として数えられます。
- 例3-3（1つ失敗したら止める）と、この解答（全部集める）のどちらを選ぶかは、画面の作りで決まります。「一部が欠けても表示できる」なら、この形のほうが利用者に親切です。

### まとめ

- JoinSet は子のタスクの袋で、`join_next` で終わった順に受け取れる。袋が捨てられると残った子は取り消されるので、子が親より長生きしない。同時に動かす数の上限も、袋の中の数で作れる。
- `select!` は、先に終わった枝だけを実行し、負けた Future をその場で捨てる。途中で捨てられて困る処理（キャンセル安全でない処理）は、枝に入れない。
- ループの中の `select!` で同じ Future を使い続けるときは、ループの外で作って `pin!` で留め、`&mut` で借りる。留め忘れは E0277（`Unpin`）、作り直しは「いつまでも終わらない」間違いになる。
