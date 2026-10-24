## UNIT 05: Mutex・RwLock・Semaphore と .await

### この単元のゴール

- 第3巻 UNIT 10 の `std::sync::Mutex` を async の中で使うときの決まり「**鍵を持ったまま `.await` しない**」を守り、`{ }` のブロックか、async でない関数で、鍵を持つ範囲を区切れる
- 鍵（`MutexGuard`）を持ったまま `.await` したときのエラー「future cannot be sent between threads safely」を読み、`drop(guard)` を書いても直らない理由と、本当の直し方を説明できる
- spawn しないとエラーにならず、プログラムが本当に止まってしまう（デッドロック）ことを実際に確かめ、clippy の `await_holding_lock` で見つけられる
- 鍵を持ったまま `.await` する必要があるときに、`tokio::sync::Mutex` を選べる
- `RwLock`（読み書き鍵）で読む人を同時に通し、`Semaphore`（セマフォ）で同時に入れる数に上限を付けられる

### たとえ話でつかむ

**Mutex は、1人ずつしか入れない倉庫の鍵**です（第3巻 UNIT 10 と同じたとえです）。鍵を持っている人だけが倉庫に入り、中の品物（データ）を数えたり並べ替えたりできます。用が済んだら鍵を返し、次の人が入ります。

async の世界では、ここに1つ、気をつけることが加わります。

- `.await` は「待つ間、ほかの仕事に席を譲る」印でした（UNIT 01）。鍵を持ったまま `.await` するのは、**倉庫の鍵をポケットに入れたまま、昼ごはんを食べに出かける**ようなものです。その間、だれも倉庫に入れません。
- 標準ライブラリの `std::sync::Mutex` で鍵を待つ人は、**扉の前に立ったまま、一歩も動きません**（スレッドごと止まる）。その人が、昼ごはんから戻ってくるはずの店員本人だったら、もう二度と鍵は返ってきません。これが**デッドロック**です。
- tokio の `tokio::sync::Mutex` で鍵を待つ人は、**番号札を取って、別の仕事をしに行きます**（`.await` で席を譲る）。鍵が空いたら呼び出してもらえます。

ほかに2つの道具を使います。

- **RwLock は、図書館の閲覧室**です。本を読むだけの人は、何人でも同時に入れます。掲示物を張り替える人（書く人）は、読んでいる人が全員出てから、1人で入ります。
- **Semaphore は、入場整理券**です。券は決まった枚数しかなく、券を持っている人だけが中に入れます。出るときに券を返すと、次の人が入れます。

> **用語:** Mutex（ミューテックス：mutual exclusion＝相互排除。中のデータを、同時に1つのスレッドやタスクだけがさわれるようにする鍵）

<!-- -->

> **用語:** ガード（guard：`lock()` が返す「鍵を持っている証拠」。ガードを通して中のデータをさわり、ガードが片付けられると鍵が返る）

<!-- -->

> **用語:** デッドロック（deadlock：お互いに相手が鍵を返すのを待っていて、だれも先へ進めなくなった状態。エラーも出ずに、プログラムが止まったように見える）

<!-- -->

> **用語:** Semaphore（セマフォ：決まった枚数の許可証（permit）を配る道具。許可証を持っている数だけが、同時に中に入れる）

### 図で理解する

まず、鍵とガードの関係をおさらいします。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-051" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="50" width="170" height="44" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="105" y="70" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">タスク B</text>
<text x="105" y="86" font-size="10" fill="#475569" text-anchor="middle">lock() で順番待ち</text>
<rect x="20" y="110" width="170" height="44" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="105" y="130" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">タスク C</text>
<text x="105" y="146" font-size="10" fill="#475569" text-anchor="middle">lock() で順番待ち</text>
<line x1="190" y1="102" x2="300" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-051)"/>
<text x="245" y="94" font-size="10" fill="#475569" text-anchor="middle">鍵が空くのを待つ</text>
<rect x="304" y="30" width="400" height="150" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="504" y="54" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Mutex の中のデータ（倉庫）</text>
<rect x="304" y="84" width="12" height="40" fill="#f97316"/>
<text x="330" y="108" font-size="11" fill="#c2410c">扉</text>
<rect x="360" y="74" width="150" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="435" y="98" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">タスク A</text>
<text x="435" y="116" font-size="10" fill="#475569" text-anchor="middle">ガードを持っている</text>
<text x="435" y="132" font-size="10" fill="#475569" text-anchor="middle">＝ 鍵を持っている</text>
<rect x="530" y="74" width="160" height="70" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="610" y="98" font-size="11" fill="#0f172a" text-anchor="middle">"/notes" → 75</text>
<text x="610" y="118" font-size="11" fill="#0f172a" text-anchor="middle">"/login" → 25</text>
<text x="20" y="210" font-size="12" fill="#0f172a">① lock() で鍵を取る → ② ガードを通して中をさわる → ③ ガードが片付く（} を抜ける）と、鍵が次の人へ渡る</text>
<text x="20" y="238" font-size="12" fill="#b91c1c">async での決まり: ガードを持ったまま .await しない（鍵を持ったまま出かけない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: Mutex の鍵とガード。ガードを持っている1人だけが中のデータをさわれる。ガードが片付くと、待っていた次の人が鍵を取る</div>
</div>

次に、**標準の Mutex の鍵を持ったまま `.await` すると何が起きるか**です。スレッドが1本だけのランタイムで、タスク A と B が同じ鍵を使う場合を考えます（「よくあるエラー4」で実際に動かします）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-052" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="14" y="22" font-size="12" font-weight="700" fill="#b91c1c">std::sync::Mutex の鍵を持ったまま .await（スレッドは1本）</text>
<rect x="14" y="34" width="156" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="92" y="58" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">A: 鍵を取る</text>
<text x="92" y="78" font-size="10" fill="#475569" text-anchor="middle">lock().unwrap()</text>
<rect x="194" y="34" width="156" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="272" y="58" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">A: .await で席を譲る</text>
<text x="272" y="78" font-size="10" fill="#b91c1c" text-anchor="middle">鍵はポケットの中</text>
<rect x="374" y="34" width="156" height="58" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="452" y="58" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">B: lock() で待つ</text>
<text x="452" y="78" font-size="10" fill="#b91c1c" text-anchor="middle">スレッドごと止まる</text>
<rect x="554" y="34" width="152" height="58" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="630" y="58" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">✕ A を進める</text>
<text x="630" y="78" font-size="10" fill="#b91c1c" text-anchor="middle">スレッドがもう無い</text>
<line x1="170" y1="63" x2="190" y2="63" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<line x1="350" y1="63" x2="370" y2="63" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<line x1="530" y1="63" x2="550" y2="63" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<text x="360" y="116" font-size="11" fill="#b91c1c" text-anchor="middle">A は鍵を返せず、B は鍵を待ち続ける → デッドロック（エラーも出ずに止まる）</text>
<text x="14" y="150" font-size="12" font-weight="700" fill="#047857">tokio::sync::Mutex なら（スレッドは1本）</text>
<rect x="14" y="162" width="126" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="77" y="186" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">A: 鍵を取る</text>
<text x="77" y="206" font-size="10" fill="#475569" text-anchor="middle">lock().await</text>
<rect x="156" y="162" width="126" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="219" y="186" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">A: .await</text>
<text x="219" y="206" font-size="10" fill="#475569" text-anchor="middle">席を譲る</text>
<rect x="298" y="162" width="126" height="58" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="361" y="186" font-size="11" font-weight="700" fill="#854d0e" text-anchor="middle">B: lock().await</text>
<text x="361" y="206" font-size="10" fill="#475569" text-anchor="middle">Pending で席を譲る</text>
<rect x="440" y="162" width="126" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="503" y="186" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">A: 続きを進める</text>
<text x="503" y="206" font-size="10" fill="#475569" text-anchor="middle">終わって鍵を返す</text>
<rect x="582" y="162" width="124" height="58" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="644" y="186" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">✓ B: 鍵を取る</text>
<text x="644" y="206" font-size="10" fill="#475569" text-anchor="middle">起こされて続き</text>
<line x1="140" y1="191" x2="152" y2="191" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<line x1="282" y1="191" x2="294" y2="191" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<line x1="424" y1="191" x2="436" y2="191" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<line x1="566" y1="191" x2="578" y2="191" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-052)"/>
<text x="360" y="250" font-size="11" fill="#047857" text-anchor="middle">B は待つ間もスレッドを手放すので、A が戻ってきて鍵を返せる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 標準の Mutex と tokio の Mutex の違い。標準の Mutex で鍵を待つとスレッドごと止まり、鍵を返すはずのタスクが動けなくなる。tokio の Mutex は待つ間も席を譲る</div>
</div>

- 標準の `std::sync::Mutex` の `lock()` は、鍵が空くまで**スレッドそのものを止めて**待ちます。第3巻のように、1つのスレッドで1つの仕事をしていたときは、それで困りませんでした。
- async では、1本のスレッドがたくさんのタスクを切り替えて進めています（UNIT 02）。そのスレッドが止まると、同じスレッドで動くはずだったタスクも全部止まります。鍵を持ったまま席を譲ったタスクも、その中に入っています。
- だから決まりは1つです。**標準の Mutex の鍵は、`.await` の前に必ず返す。** 鍵を持ったまま `.await` したいときだけ、`tokio::sync::Mutex` を使います。

### まず動かす最小コード

#### 例5-1: 標準の Mutex で数え、鍵を返してから .await する

> **▼ このコードがやること（先に日本語で）:** ページごとのアクセス数を数える表（`HashMap`）を、標準の `Mutex` で守って、100個のタスクで共有します。それぞれのタスクは、鍵を取って数を1足し、**鍵を返してから**、時間のかかる処理（DB の代わりの 10 ミリ秒の待ち）を `.await` します。最後に、ページごとの数を表示します。

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ページごとのアクセス数を数える（標準ライブラリの Mutex で守る）
type Counter = Arc<Mutex<HashMap<String, u64>>>;

async fn handle_request(counter: Counter, path: &str) {
    // 鍵を取って数を足す。{ } を抜けた所でガードが片付けられ、鍵が返る
    {
        let mut map = counter.lock().unwrap();
        *map.entry(path.to_string()).or_insert(0) += 1;
    }
    // 鍵を返してから、時間のかかる処理（DB や外部 API の代わり）を待つ
    tokio::time::sleep(Duration::from_millis(10)).await;
}

#[tokio::main]
async fn main() {
    let counter: Counter = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();
    for i in 0..100 {
        let counter = Arc::clone(&counter);
        let path = if i % 4 == 0 { "/login" } else { "/notes" };
        handles.push(tokio::spawn(async move {
            handle_request(counter, path).await;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let map = counter.lock().unwrap();
    let mut paths: Vec<_> = map.iter().collect();
    paths.sort();
    println!("{paths:?}");
}
```

`src/bin/u05_std_mutex.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_std_mutex
```

**▼ 実行結果**

```text
[("/login", 25), ("/notes", 75)]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Arc で共有し、Mutex で守る（第3巻 UNIT 10 と同じ形）

- `type Counter = Arc<Mutex<HashMap<String, u64>>>;` は、長い型に `Counter` という別名を付けています（`type` の別名）。
- `Arc` は「何人もで同じものを持つ」ための入れ物、`Mutex` は「同時に1人だけがさわれる」ようにする鍵です。100個のタスクに `Arc::clone` で持ち手を配り、全員が同じ表を数えます。
- `counter.lock().unwrap()` の `unwrap` は、第3巻 UNIT 10 で学んだポイズニング（鍵を持った人が panic した）のときだけ `Err` になる部分です。この例では panic しないので `unwrap` で済ませています。
- 100個のうち、4で割り切れる25個が `/login`、残りの75個が `/notes` です。数え漏れなく、ぴったり合っています。

##### 解説2: { } で囲んで、鍵を持つ範囲をはっきり区切る

- 鍵を使う2行を `{ }` で囲んでいます。`map`（ガード）は `{ }` の中で作った変数なので、`}` を抜けたところで片付けられ、鍵が返ります。
- その**後で** `sleep(...).await` しています。待っている間、鍵はだれも持っていないので、ほかのタスクは自由に数を足せます。
- もし `{ }` で囲まずに、鍵を持ったまま `.await` すると、コンパイルエラーになります（「よくあるエラー1」）。

##### 解説3: async の中でも、標準の Mutex を使ってよい

- 「async の中では tokio の Mutex を使わなければならない」と思われがちですが、そうではありません。tokio のドキュメントにも「よく誤解されているが、async のコードの中で標準ライブラリの Mutex を使うのは問題なく、むしろ好ましいことが多い」と書かれています（tokio 1.53.3 の `sync/mutex.rs`）。
- 標準の Mutex のほうが軽くて速いからです。条件は1つ、**鍵を持ったまま `.await` しないこと**。数を足す・表を書き換えるのように、鍵の中の仕事が短く、`.await` を含まないなら、標準の Mutex で十分です。

> **注意:** この例のように、アクセス数をメモリの中の表で数えると、サーバーを再起動したときに0に戻ります。Fly.io などで同じアプリを2台以上動かすと、台ごとに別々の表になり、合計も合いません。消えてはいけないデータや、台をまたいで共有したいデータは、DB に置きます（第7巻）。

### もう一歩: 実務に近い形

#### 例5-2: 鍵を使う部分を、async でない関数に分ける

> **▼ このコードがやること（先に日本語で）:** 数を1足して、足した後の数を返す処理を、async でないふつうの関数 `bump` に分けます。async の関数 `handle_request` は、`bump` を呼んでから `.await` で待ち、何回目かを表示します。2回呼んで確かめます。

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

// 鍵を使う部分だけを、async でないふつうの関数に分ける。
// この関数の中には .await が書けないので、「鍵を持ったまま待つ」ことが起きようがない
fn bump(counter: &Mutex<u64>) -> u64 {
    let mut count = counter.lock().unwrap();
    *count += 1;
    *count
}

async fn handle_request(counter: Arc<Mutex<u64>>) {
    let now = bump(&counter);
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{now} 回目");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    tokio::spawn(handle_request(Arc::clone(&counter)))
        .await
        .unwrap();
    tokio::spawn(handle_request(Arc::clone(&counter)))
        .await
        .unwrap();
}
```

`src/bin/u05_sync_helper.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_sync_helper
```

**▼ 実行結果**

```text
1 回目
2 回目
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: async でない関数の中には、.await を書けない

- `bump` は `async` の付いていない関数です。UNIT 01 の「よくあるエラー2」（E0728）のとおり、この中には `.await` を書けません。
- だから、`bump` の中で取った鍵は、`bump` が戻るときに必ず返されます。「鍵を持ったまま待つ」ことが、**書こうとしても書けない**形になりました。

##### 解説2: 鍵の中身は、必要な値だけ取り出して持ち出す

- `bump` は、足した後の数 `*count`（`u64` の値）を返しています。ガードそのものではなく、**中の値の写し**を返すので、`handle_request` に戻ったときには鍵はもう返っています。
- 鍵の外で使いたいものは、こうして値として取り出してから鍵を返します。中身が `String` や `Vec` なら、必要な分だけ `clone` して取り出します（第3巻 UNIT 10 の「第5巻への橋渡し」で予告した形です）。
- `{ }` で囲む書き方（例5-1）と、関数に分ける書き方（この例）は、どちらも正解です。鍵を使う処理が数行なら `{ }`、何か所からも使うなら関数にするとよいでしょう。

#### 例5-3: 鍵を持ったまま .await したいときは、tokio の Mutex

> **▼ このコードがやること（先に日本語で）:** 「同時に1人しか使えない接続」を `tokio::sync::Mutex` で守り、3つのタスクで共有します。それぞれのタスクは鍵を取り、鍵を持ったまま、送るのに 0.1 秒かかるメッセージの送信を `.await` します。鍵を取った時刻と、送った順番、全体の時間を表示します。

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

// 「1本しかない接続」の代わり。同時に1人しか使えない
struct Connection {
    sent: Vec<String>,
}

impl Connection {
    // 送るのに 100ms かかる（.await を含む）
    async fn send(&mut self, message: String) {
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.sent.push(message);
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let connection = Arc::new(Mutex::new(Connection { sent: Vec::new() }));
    let mut handles = Vec::new();
    for n in 1..=3 {
        let connection = Arc::clone(&connection);
        handles.push(tokio::spawn(async move {
            // tokio の Mutex は lock も .await で待つ。待っている間、スレッドは他の仕事をする
            let mut conn = connection.lock().await;
            println!(
                "{:.1} 秒: {n} 番が鍵を取った",
                start.elapsed().as_secs_f64()
            );
            // ガードを持ったまま .await してもよい（そのための Mutex）
            conn.send(format!("メッセージ{n}")).await;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("送った順: {:?}", connection.lock().await.sent);
    println!("合計 {:.1} 秒", start.elapsed().as_secs_f64());
}
```

`src/bin/u05_tokio_mutex.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_tokio_mutex
```

**▼ 実行結果**

```text
0.0 秒: 1 番が鍵を取った
0.1 秒: 2 番が鍵を取った
0.2 秒: 3 番が鍵を取った
送った順: ["メッセージ1", "メッセージ2", "メッセージ3"]
合計 0.3 秒
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: tokio の Mutex は、lock も .await で待つ

- `tokio::sync::Mutex` の `lock()` は async 関数で、`connection.lock().await` のように `.await` を付けます。鍵が空いていなければ、`Pending` を返して席を譲り、鍵が空いたら起こしてもらえます。待っている間も、スレッドはほかのタスクを進められます。
- 標準の Mutex と違って、戻り値は `Result` ではなく、ガードそのものです。tokio の Mutex にはポイズニングが無いので、`unwrap` が要りません。

##### 解説2: ガードを持ったまま .await してよいのが、tokio の Mutex

- `conn.send(...).await` は、鍵（ガード `conn`）を持ったまま 0.1 秒待っています。tokio の Mutex のガードは、`.await` をまたいでも `Send` なので、`tokio::spawn` に渡してもコンパイルエラーになりません。
- 「接続を使い始めてから送り終わるまで、ほかの人に割り込まれたくない」のように、**`.await` をはさむ一続きの仕事を、まとめて1人でやりたい**ときに使います。

##### 解説3: そのかわり、全体は1人ずつの速さになる

- 3つのタスクは、鍵を 0.1 秒ずつ順番に持ったので、全体で 0.3 秒かかりました。UNIT 02 のように待ちが重なることはありません。鍵を持ったまま待つと、その間はだれも入れないからです。
- tokio の Mutex は、標準の Mutex より重い道具です。tokio のドキュメントも「使える場面では、標準の Mutex を選ぶべき」としています。鍵を持ったまま `.await` する必要が本当にあるときだけ使いましょう。共有の接続のようなものは、UNIT 04 のアクター（係のタスクに依頼する形）で作るほうがすっきりすることも多いです。

#### 例5-4: 読む人は同時に、書く人は1人で（RwLock）

> **▼ このコードがやること（先に日本語で）:** 画面の設定（テーマ）を `tokio::sync::RwLock` で守ります。読む人のタスクを3つ作り、それぞれ読み込みの鍵を取って 0.2 秒持ちます。少し遅れて書く人のタスクを作り、書き込みの鍵でテーマを書き換えます。それぞれが鍵を取れた時刻を表示します。

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 読む人は何人でも同時に、書く人は1人だけ
    let settings = Arc::new(RwLock::new(String::from("テーマ: ライト")));
    let mut handles = Vec::new();

    for n in 1..=3 {
        let settings = Arc::clone(&settings);
        handles.push(tokio::spawn(async move {
            let value = settings.read().await;
            println!(
                "{:.1} 秒: 読む人{n}: {}",
                start.elapsed().as_secs_f64(),
                *value
            );
            tokio::time::sleep(Duration::from_millis(200)).await;
        }));
    }
    // 読む人たちが先に鍵を取れるよう、少しだけ遅れて書く人を始める
    tokio::time::sleep(Duration::from_millis(10)).await;
    let writer = {
        let settings = Arc::clone(&settings);
        tokio::spawn(async move {
            let mut value = settings.write().await;
            *value = String::from("テーマ: ダーク");
            println!(
                "{:.1} 秒: 書く人: 書き換えた",
                start.elapsed().as_secs_f64()
            );
        })
    };
    for handle in handles {
        handle.await.unwrap();
    }
    writer.await.unwrap();
    println!("最後の値: {}", settings.read().await);
}
```

`src/bin/u05_rwlock.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_rwlock
```

**▼ 実行結果**

```text
0.0 秒: 読む人1: テーマ: ライト
0.0 秒: 読む人3: テーマ: ライト
0.0 秒: 読む人2: テーマ: ライト
0.2 秒: 書く人: 書き換えた
最後の値: テーマ: ダーク
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-053" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="36" font-size="12" fill="#0f172a" text-anchor="end">読む人1</text>
<text x="110" y="64" font-size="12" fill="#0f172a" text-anchor="end">読む人2</text>
<text x="110" y="92" font-size="12" fill="#0f172a" text-anchor="end">読む人3</text>
<text x="110" y="128" font-size="12" fill="#0f172a" text-anchor="end">書く人</text>
<rect x="130" y="22" width="300" height="20" rx="4" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="130" y="50" width="300" height="20" rx="4" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="130" y="78" width="300" height="20" rx="4" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<text x="280" y="37" font-size="10" fill="#1e40af" text-anchor="middle">読み込みの鍵（同時に3人）</text>
<text x="280" y="65" font-size="10" fill="#1e40af" text-anchor="middle">読み込みの鍵</text>
<text x="280" y="93" font-size="10" fill="#1e40af" text-anchor="middle">読み込みの鍵</text>
<rect x="145" y="114" width="285" height="20" rx="4" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 4"/>
<text x="287" y="129" font-size="10" fill="#475569" text-anchor="middle">write().await で、読む人が全員出るのを待つ</text>
<rect x="430" y="114" width="60" height="20" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<text x="500" y="129" font-size="11" fill="#c2410c">書き換える（1人だけ）</text>
<line x1="130" y1="156" x2="690" y2="156" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-053)"/>
<line x1="130" y1="150" x2="130" y2="162" stroke="#475569" stroke-width="2"/>
<line x1="430" y1="150" x2="430" y2="162" stroke="#475569" stroke-width="2"/>
<text x="130" y="178" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="430" y="178" font-size="11" fill="#475569" text-anchor="middle">0.2 秒</text>
<text x="360" y="202" font-size="12" fill="#475569" text-anchor="middle">読む人どうしは待ち合わない。書く人は、読む人がいなくなってから1人で入る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: RwLock の動き。3人の読む人は同時に鍵を取れる。書く人は、読む人が全員鍵を返す 0.2 秒まで待ってから、1人で書き換える</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: read() は何人でも、write() は1人だけ

- `settings.read().await` は**読み込みの鍵**です。ほかに読んでいる人がいても、同時に取れます。3人の読む人は、全員 0 秒で鍵を取れました。
- `settings.write().await` は**書き込みの鍵**です。読んでいる人も書いている人も1人もいないときにだけ取れます。書く人は 0.01 秒に来ましたが、読む人が全員 0.2 秒まで鍵を持っていたので、0.2 秒まで待たされました。
- 読み込みの鍵（`value`）を通しては読むことしかできず、書き込みの鍵（`mut value`）を通すと書き換えられます。`*value = ...` で中身を入れ替えています。

##### 解説2: 書く人が待っている間は、新しく来た読む人も待たされる

- tokio の RwLock は「書く人を優先する」作りです（tokio 1.53.3 のドキュメントに「公平（書き込み優先）」とあります）。書く人が待ち始めた後に来た読む人は、書く人が終わるまで待たされます。
- そうしないと、読む人が次々に来る間、書く人が永遠に入れない（飢える）ことがあるからです。

##### 解説3: Mutex とどちらを使うか

- 読む回数がとても多く、書く回数がたまにしかないデータ（設定・一覧の写しなど）なら、RwLock にすると、読む人どうしが待ち合わずに済みます。
- 読むのも書くのも同じくらいなら、仕組みが簡単な Mutex で十分です。標準ライブラリにも `std::sync::RwLock` があり、`.await` をまたがないなら、標準のものを使えます（Mutex と同じ考え方です）。

#### 例5-5: 同時に入れる数を、Semaphore で2つまでにする

> **▼ このコードがやること（先に日本語で）:** 許可証が2枚の `Semaphore` を作り、5つのタスクで共有します。それぞれのタスクは許可証を1枚もらってから 0.3 秒の仕事をし、終わったら許可証を返します。開始と終了の時刻を表示して、同時に動いているのが2つまでになっていることを確かめます。

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 許可証（permit）は 2 枚。同時に動けるのは 2 つまで
    let permits = Arc::new(Semaphore::new(2));
    let mut handles = Vec::new();

    for n in 1..=5 {
        let permits = Arc::clone(&permits);
        handles.push(tokio::spawn(async move {
            // 許可証が空くまで待つ。手に入れた許可証は _permit が片付けられるときに返る
            let _permit = permits.acquire_owned().await.unwrap();
            println!("{:.1} 秒: {n} 番 開始", start.elapsed().as_secs_f64());
            tokio::time::sleep(Duration::from_millis(300)).await;
            println!("{:.1} 秒: {n} 番 終了", start.elapsed().as_secs_f64());
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
```

`src/bin/u05_semaphore.rs` として保存して実行します。

```powershell
cargo run -q --bin u05_semaphore
```

**▼ 実行結果**

```text
0.0 秒: 1 番 開始
0.0 秒: 2 番 開始
0.3 秒: 1 番 終了
0.3 秒: 3 番 開始
0.3 秒: 2 番 終了
0.3 秒: 5 番 開始
0.6 秒: 3 番 終了
0.6 秒: 4 番 開始
0.6 秒: 5 番 終了
0.9 秒: 4 番 終了
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: acquire_owned で許可証をもらい、片付けで返す

- `Semaphore::new(2)` は、許可証が2枚の Semaphore を作ります。`Arc` で包んで、全部のタスクで共有しています。
- `permits.acquire_owned().await` は、許可証が残っていればすぐに1枚もらい、無ければ、だれかが返すまで待ちます。戻り値の `_permit` が許可証で、**`_permit` が片付けられるときに自動で返されます**（ガードと同じ形です）。返し忘れが起きません。
- `_owned` が付いた版は、`Arc<Semaphore>` から、借り物ではない許可証を作ります。タスクの中で使うときは、この版が扱いやすいです。`unwrap` は、Semaphore が閉じられたとき（`close()`）にだけ `Err` になる部分です。

##### 解説2: 同時に動いているのは、いつも2つまで

- 実行結果を時刻ごとに見ると、0 秒に1・2番、0.3 秒に3・5番、0.6 秒に4番が始まっています。どの時点でも、「開始した」のに「終了していない」ものは2つまでです。5つ全部が終わるまで、約 0.9 秒でした。
- 5番が4番より先に始まっているのは、5番のタスクのほうが先に `acquire_owned` を呼んで、列に並んだからです。tokio の Semaphore は公平で、**許可証を求めた順番**に配ります（tokio 1.53.3 のドキュメントより）。どのタスクが先に列に並ぶかは、実行するたびに変わります。

##### 解説3: UNIT 03 の JoinSet の上限との違い

- UNIT 03 の例3-4では、親が「袋の中の数」を数えて、上限を守っていました。これは、親が1人で全部のタスクを作るときに向いています。
- Semaphore は、**あちこちのタスクが、それぞれ自分で「入ってよいか」を確かめる**ときに向いています。たとえば、たくさんのリクエストの処理が、それぞれ外の API を呼ぶとき、「外の API を同時に呼ぶのは全体で10まで」を、Semaphore を1つ共有するだけで守れます。

最後に、この単元の道具の選び方を図にまとめます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-054" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="260" y="10" width="200" height="40" rx="10" fill="#0f172a"/>
<text x="360" y="35" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">何を守りたい？</text>
<rect x="20" y="80" width="230" height="40" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="135" y="105" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">共有するデータ</text>
<rect x="270" y="80" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="105" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">同時に入る数</text>
<rect x="470" y="80" width="230" height="40" rx="8" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="585" y="105" font-size="12" font-weight="700" fill="#6d28d9" text-anchor="middle">データを持つ係を1人にできる</text>
<path d="M300,50 L135,76" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<line x1="360" y1="50" x2="360" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<path d="M420,50 L585,76" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<rect x="20" y="140" width="230" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="135" y="165" font-size="12" fill="#0f172a" text-anchor="middle">鍵を持ったまま .await する？</text>
<line x1="135" y1="120" x2="135" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<rect x="20" y="200" width="230" height="40" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="135" y="218" font-size="11" fill="#0f172a" text-anchor="middle">しない → std::sync::Mutex</text>
<text x="135" y="233" font-size="10" fill="#475569" text-anchor="middle">（読む人が多いなら RwLock）</text>
<rect x="20" y="250" width="230" height="40" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="135" y="268" font-size="11" fill="#0f172a" text-anchor="middle">する → tokio::sync::Mutex</text>
<text x="135" y="283" font-size="10" fill="#475569" text-anchor="middle">（重いので、本当に必要なときだけ）</text>
<line x1="135" y1="180" x2="135" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<path d="M250,160 H262 V270 H254" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<rect x="270" y="200" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="218" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Semaphore</text>
<text x="360" y="233" font-size="10" fill="#475569" text-anchor="middle">許可証の枚数で上限を決める</text>
<line x1="360" y1="120" x2="360" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
<rect x="470" y="200" width="230" height="40" rx="8" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="585" y="218" font-size="12" font-weight="700" fill="#6d28d9" text-anchor="middle">アクター（UNIT 04）</text>
<text x="585" y="233" font-size="10" fill="#475569" text-anchor="middle">鍵がいらない。依頼は channel で</text>
<line x1="585" y1="120" x2="585" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-054)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 守りたいものによる道具の選び方。データは標準の Mutex が基本で、鍵を持ったまま待つときだけ tokio の Mutex。数の上限は Semaphore、係を1人にできるならアクター</div>
</div>

### よくあるエラーと読み方

#### エラー1: 標準の Mutex の鍵を持ったまま .await して spawn する（future cannot be sent between threads safely）

例5-1の `{ }` を外して、鍵を持ったまま `.await` した場合です。

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

type Counter = Arc<Mutex<HashMap<String, u64>>>;

async fn handle_request(counter: Counter, path: &str) {
    let mut map = counter.lock().unwrap();
    *map.entry(path.to_string()).or_insert(0) += 1;
    // 鍵（ガード map）を持ったまま .await してしまう
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{path}: {} 回目", map[path]);
}

#[tokio::main]
async fn main() {
    let counter: Counter = Arc::new(Mutex::new(HashMap::new()));
    let handle = tokio::spawn(handle_request(Arc::clone(&counter), "/notes"));
    handle.await.unwrap();
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error: future cannot be sent between threads safely
   --> src\bin\e05_guard_await.rs:18:31
    |
 18 |     let handle = tokio::spawn(handle_request(Arc::clone(&counter), "/notes"));
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ future returned by `handle_request` is not `Send`
    |
    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, HashMap<String, u64>>`
note: future is not `Send` as this value is used across an await
   --> src\bin\e05_guard_await.rs:11:51
    |
  8 |     let mut map = counter.lock().unwrap();
    |         ------- has type `std::sync::MutexGuard<'_, HashMap<String, u64>>` which is not `Send`
...
 11 |     tokio::time::sleep(Duration::from_millis(10)).await;
    |                                                   ^^^^^ await occurs here, with `mut map` maybe used later
note: required by a bound in `tokio::spawn`
   --> ...\tokio-1.53.3\src\task\spawn.rs:176:21
    |
174 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
175 |     where
176 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

error: could not compile `async_lab` (bin "e05_guard_await") due to 1 previous error
```

- **何が起きたか:** UNIT 02 の「よくあるエラー2」（`Rc` を持ったまま `.await`）と、まったく同じ形のエラーです。今度の原因の型は `std::sync::MutexGuard`（標準の Mutex のガード）です。標準の Mutex のガードは `Send` ではありません。鍵は「取ったスレッドで返す」約束になっているので、`.await` の後で別のスレッドに移ってから返されては困るからです。
- `note: future is not Send as this value is used across an await` の下で、8行目で作ったガード `map` が、11行目の `.await` の後（12行目）でも使われていることが示されています。
- **直し方:** 例5-1のように `{ }` で囲んで、ガードを `.await` の前に片付けます。`.await` の後で使いたい値（ここでは何回目か）は、`{ }` の中で取り出しておきます。

#### エラー2: drop(guard) を書いても、エラーが消えない

「ガードを `.await` の前に片付ければよい」と聞いて、`drop` を足した場合です。

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn handle_request(counter: Arc<Mutex<u64>>) {
    let mut count = counter.lock().unwrap();
    *count += 1;
    let now = *count;
    // .await の前に、ガードを明示的に片付ける
    drop(count);
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{now} 回目");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    tokio::spawn(handle_request(Arc::clone(&counter))).await.unwrap();
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error: future cannot be sent between threads safely
   --> src\bin\e05_drop_guard.rs:17:18
    |
 17 |     tokio::spawn(handle_request(Arc::clone(&counter))).await.unwrap();
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ future returned by `handle_request` is not `Send`
    |
    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, u64>`
note: future is not `Send` as this value is used across an await
   --> src\bin\e05_drop_guard.rs:10:51
    |
  5 |     let mut count = counter.lock().unwrap();
    |         --------- has type `std::sync::MutexGuard<'_, u64>` which is not `Send`
...
 10 |     tokio::time::sleep(Duration::from_millis(10)).await;
    |                                                   ^^^^^ await occurs here, with `mut count` maybe used later
note: required by a bound in `tokio::spawn`
   --> ...\tokio-1.53.3\src\task\spawn.rs:176:21
    |
174 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
175 |     where
176 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

error: could not compile `async_lab` (bin "e05_drop_guard") due to 1 previous error
```

- **何が起きたか:** 9行目で `drop(count)` しているのに、「`count` が `.await` の後で使われるかもしれない（`maybe used later`）」と言われています。Rust 1.99.0 でも、このとおりのエラーになりました。
- コンパイラは、Future の中にしまう変数を決めるとき、`drop` を呼んだかどうかまでは見ずに、**変数の有効範囲（`{ }` のどこで作られ、どこで終わるか）**で判断しています。`count` は関数の最後まで有効な変数なので、`.await` をまたいでいる、と扱われるのです。
- **直し方:** `drop` ではなく、変数の有効範囲そのものを `.await` の前で終わらせます。`{ }` のブロックで囲むか（例5-1）、async でない関数に分けます（例5-2）。実際に直すのは、練習問題1で自分の手でやってみましょう。

#### エラー3: spawn しないと、コンパイルが通ってしまう（clippy の await_holding_lock）

同じ「鍵を持ったまま `.await`」でも、`tokio::spawn` に渡さずに直接 `.await` すると、`Send` が求められないので、コンパイルが通ってしまいます。

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn handle_request(counter: Arc<Mutex<u64>>) {
    let mut count = counter.lock().unwrap();
    *count += 1;
    // 鍵を持ったまま .await する
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{} 回目", *count);
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    // spawn せずに、main の中で直接 .await する
    handle_request(Arc::clone(&counter)).await;
}
```

実行すると、何事もなかったように動きます。

```text
1 回目
```

この見落としを見つけてくれるのが clippy です。`cargo clippy --bin e05_guard_no_spawn` の結果です。

```text
    Checking async_lab v0.1.0 (...\async_lab)
warning: this `MutexGuard` is held across an await point
 --> src\bin\e05_guard_no_spawn.rs:5:9
  |
5 |     let mut count = counter.lock().unwrap();
  |         ^^^^^^^^^
  |
  = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling `await`
note: these are all the await points this lock is held through
 --> src\bin\e05_guard_no_spawn.rs:8:51
  |
8 |     tokio::time::sleep(Duration::from_millis(10)).await;
  |                                                   ^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#await_holding_lock
  = note: `#[warn(clippy::await_holding_lock)]` on by default

warning: `async_lab` (bin "e05_guard_no_spawn") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
```

- **何が起きたか:** clippy が「この `MutexGuard` は、`.await` の地点をまたいで持たれています」と警告しています。`help:` は「async 用の Mutex を使うか、`.await` の前にガードを片付けてください」、`note:` は、鍵を持ったまままたいでいる `.await` の場所（8行目）です。
- このプログラムが無事に動いたのは、鍵を使うタスクが1つしかなかったからです。同じ鍵を使うタスクが2つ以上あると、次のエラー4のように、本当に止まることがあります。
- **直し方:** エラー1と同じく、ガードを `.await` の前に片付けます。「spawn していないからエラーにならない」は、安全という意味ではありません。`cargo clippy -- -D warnings`（UNIT 01）で、警告も残さないようにしましょう。

#### エラー4（エラーが出ない間違い）: 本当に止まる（デッドロック）

スレッドが1本だけのランタイム（`flavor = "current_thread"`、UNIT 02 のヒント）で、タスク A と B が同じ標準の Mutex を使い、どちらも鍵を持ったまま `.await` します。`tokio::join!` で同時に進めるので、`Send` は求められず、コンパイルは通ります（clippy は、エラー3と同じ `await_holding_lock` の警告を出します）。

```rust
use std::sync::Mutex;
use std::time::Duration;

async fn task(name: &str, counter: &Mutex<u64>) {
    println!("{name}: 鍵を取りに行きます");
    let mut count = counter.lock().unwrap();
    println!("{name}: 鍵を取りました");
    *count += 1;
    // 標準の Mutex の鍵を持ったまま .await して、スレッドを手放してしまう
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("{name}: 終わりました（{}）", *count);
}

// スレッドが1本だけのランタイムで動かす
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let counter = Mutex::new(0);
    // spawn せずに join! で2つを同時に進めるので、Send は求められずコンパイルが通る
    tokio::join!(task("A", &counter), task("B", &counter));
    println!("全部終わりました");
}
```

このプログラムは終わらないので、Git Bash の `timeout 5`（5 秒たったら止める）を付けて実行しました。

```text
A: 鍵を取りに行きます
A: 鍵を取りました
B: 鍵を取りに行きます
```

5 秒たっても続きが表示されず、`timeout` に止められました（終了コード 124 は「時間切れで止めた」という意味です）。

- **何が起きたか:** 図の上の段のとおりです。A が鍵を取り、`.await` で席を譲りました。同じスレッドで B が動き出し、`counter.lock()` で鍵を待ちます。標準の Mutex の `lock` は**スレッドごと止めて**待つので、A を進めるスレッドがもうありません。A は鍵を返せず、B は鍵を待ち続けます。エラーも panic も出ずに、ただ止まりました。
- スレッドが何本もあるランタイムなら、運よく別のスレッドで A が進んで動くこともあります。しかし、同時に来るリクエストが増えると、全部のスレッドが同じように止まり、サーバー全体が応答しなくなります。「たまにしか起きない」ぶん、見つけるのがとても難しい間違いです。
- **直し方:** いちばんよいのは、エラー1と同じく、ガードを `.await` の前に片付けることです。どうしても鍵を持ったまま待つ必要があるなら、tokio の Mutex に変えます。`use tokio::sync::Mutex;` にして、`counter.lock().unwrap()` を `counter.lock().await` に変えたものが次のコードです。

```rust
use std::time::Duration;
use tokio::sync::Mutex;

async fn task(name: &str, counter: &Mutex<u64>) {
    println!("{name}: 鍵を取りに行きます");
    // tokio の Mutex は、鍵が空くのを .await で待つ。待つ間、スレッドはほかの仕事をする
    let mut count = counter.lock().await;
    println!("{name}: 鍵を取りました");
    *count += 1;
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("{name}: 終わりました（{}）", *count);
}

// スレッドが1本だけのランタイムで動かす
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let counter = Mutex::new(0);
    tokio::join!(task("A", &counter), task("B", &counter));
    println!("全部終わりました");
}
```

`src/bin/u05_fix_deadlock.rs` として実行した結果です。

```text
A: 鍵を取りに行きます
A: 鍵を取りました
B: 鍵を取りに行きます
A: 終わりました（1）
B: 鍵を取りました
B: 終わりました（2）
全部終わりました
```

B は鍵を待つ間も席を譲ったので、A が戻ってきて鍵を返し、その後で B が鍵を取れました（図の下の段）。

#### エラー5（エラーが出ない間違い）: 同じタスクの中で、もう一度 lock する

tokio の Mutex でも、**自分が持っている鍵を、自分でもう一度取ろうとする**と、永遠に待ちます。

```rust
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let notes = Arc::new(Mutex::new(vec![String::from("ノート1")]));

    let first = notes.lock().await;
    println!("1回目の lock: 取れた（{} 件）", first.len());

    // first を返さないまま、同じタスクでもう一度 lock する
    let second = tokio::time::timeout(Duration::from_secs(1), notes.lock()).await;
    match second {
        Ok(_) => println!("2回目の lock: 取れた"),
        Err(_) => println!("2回目の lock: 1 秒待っても取れない（自分が持っている鍵を待っている）"),
    }
}
```

```text
1回目の lock: 取れた（1 件）
2回目の lock: 1 秒待っても取れない（自分が持っている鍵を待っている）
```

（`cargo run` の `Compiling ...` などの行は省いています。）

- `tokio::time::timeout(時間, Future)` は、「この時間までに終わらなければ `Err` を返す」関数です。UNIT 03 の練習問題2で `select!` を使って作ったものと同じで、UNIT 06 でくわしく学びます。ここでは、永遠に待たないように1秒で区切っています。
- **何が起きたか:** `first` が鍵を持ったままなので、2回目の `lock` は、`first` が鍵を返すのを待ちます。ところが `first` を持っているのは自分自身なので、返されることはありません。
- **直し方:** 1つのタスクの中では、鍵は1回だけ取り、取ったガードを使い回します。関数 A が鍵を取ったまま、同じ鍵を取る関数 B を呼んでいないか、を確かめましょう。鍵を取る場所を1か所（例5-2のような小さな関数）にまとめておくと、この間違いは起きにくくなります。

### RustNote ではここで使う

RustNote のコードには、`Mutex` も `RwLock` も `Semaphore` も出てきません。これは偶然ではなく、設計で避けています。

- **変わるデータは、全部 DB に置いています。** ノートもユーザーもログイン中のセッションも、`notes`・`users`・`sessions` の表にあります。メモリの中で共有して書き換えるデータが無いので、鍵が要りません。同時に書き込んだときの食い違いは、DB の transaction と `version` 列が防ぎます（第7巻 UNIT 05）。
- **共有している `AppState` は、作った後に書き換えません。** UNIT 02 で見たとおり、`AppState` は `Arc<dyn NoteRepository + Send + Sync>` などを持っているだけです。だれも書き換えないものは、`Arc` で配るだけで、鍵なしに何人でも同時に読めます。
- このやり方なら、Fly.io で同じアプリを2台以上動かしても、データが台ごとに分かれることがありません（例5-1の注意）。**鍵をかけたくなったら、まず「そのデータは DB に置くべきではないか？」と考える**のが、Web アプリでの基本です。

唯一、標準ライブラリの「1回だけ作る」道具が使われています。`src/auth.rs` の、ログインに使うダミーのハッシュです。

```rust
/// 登録されていないメールでも同じくらい時間をかけるための、ダミーのハッシュ。
/// （すぐ返すと「このメールは未登録だ」と応答時間から推測されてしまう）
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    Argon2::default()
        .hash_password(b"rustnote-dummy-password")
        .expect("ダミーのハッシュを作れません")
        .to_string()
});
```

- `std::sync::LazyLock` は、**最初に使われたときに1回だけ**中のクロージャを実行して値を作り、それ以降は作った値を何人にでも読ませる道具です。2つのリクエストが同時に初めて使っても、作られるのは1回だけです。中では鍵のようなしくみで「1回だけ」を守っていますが、作り終わった後は、読むだけなので待ち合いません。
- 一方で、RustNote には「パスワードのハッシュ（Argon2）を同時に何個まで計算するか」の上限がありません（改善の候補として引き継ぎに記録しています）。Argon2 はわざと重く作られた計算なので、ログインが一度に大量に来ると、CPU を使い切ってしまいます。この上限こそ、例5-5の Semaphore の出番です。重い計算をどこで動かすかと合わせて、UNIT 07 で扱います。

### 練習問題

**問1:** 「よくあるエラー2」の `e05_drop_guard.rs` を、`drop` を使わずに直して、`tokio::spawn` に渡せるようにしてください。表示は「1 回目」になります（`src/bin/u05_q1.rs`）。

**問2:** 同時に使える DB の接続が3本しかないとします。300 ミリ秒かかる問い合わせを6つのタスクから行い、Semaphore で同時に3つまでにしてください。それぞれの問い合わせを始めた時刻と、全体の時間を表示します。全体で何秒になるか予想してから実行しましょう（`src/bin/u05_q2.rs`）。

**問3:** 次の4つの場面で、この単元の道具（標準の Mutex・tokio の Mutex・RwLock・Semaphore）とアクター（UNIT 04）、DB（第7巻）のどれを使うのがよいか、理由と一緒に答えてください。

1. 1台のサーバーの中だけで、「今、何件のリクエストを処理中か」を数えて、管理画面に表示したい。数を足したり引いたりするだけで、その間に `.await` はしない。
2. 外の地図 API は「同時に5件まで」と決められている。たくさんのリクエストの処理が、それぞれ地図 API を呼ぶ。
3. 起動時に読み込んだ「禁止ワードの一覧」を、全部のリクエストが毎回読む。管理者が、1日に1回くらい一覧を入れ替える。
4. ユーザーごとの「今月の投稿数」を数えて、上限を超えたら投稿を断りたい。サーバーは2台で動いている。

### 解答と解説

**問1の解答:**

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn handle_request(counter: Arc<Mutex<u64>>) {
    // 鍵を使う部分を { } で囲み、ガードが .await より前に片付くことを形で示す
    let now = {
        let mut count = counter.lock().unwrap();
        *count += 1;
        *count
    };
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{now} 回目");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    tokio::spawn(handle_request(Arc::clone(&counter)))
        .await
        .unwrap();
}
```

```text
1 回目
```

- `{ }` のブロックの中でガード `count` を作り、ブロックの最後の式 `*count` で、足した後の数をブロックの値として外へ出しています（第2巻 UNIT 03 の「ブロックも式」）。`count` の有効範囲は `}` で終わるので、`.await` をまたぎません。コンパイラにも、それが形で伝わります。
- 例5-2のように、`fn bump(counter: &Mutex<u64>) -> u64` という async でない関数に分けても正解です。

**問2の解答:**

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

// DB への問い合わせの代わり。300ms かかる
async fn query(n: u32, start: Instant) {
    println!(
        "{:.1} 秒: {n} 番 問い合わせ開始",
        start.elapsed().as_secs_f64()
    );
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 同時に使える DB の接続は 3 本まで
    let connections = Arc::new(Semaphore::new(3));
    let mut handles = Vec::new();
    for n in 1..=6 {
        let connections = Arc::clone(&connections);
        handles.push(tokio::spawn(async move {
            let _permit = connections.acquire_owned().await.unwrap();
            query(n, start).await;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!(
        "全部終わりました（{:.1} 秒）",
        start.elapsed().as_secs_f64()
    );
}
```

```text
0.0 秒: 1 番 問い合わせ開始
0.0 秒: 2 番 問い合わせ開始
0.0 秒: 3 番 問い合わせ開始
0.3 秒: 4 番 問い合わせ開始
0.3 秒: 5 番 問い合わせ開始
0.3 秒: 6 番 問い合わせ開始
全部終わりました（0.6 秒）
```

- 許可証を3枚にしたので、最初の3つが 0 秒に、残りの3つが 0.3 秒に始まり、全体で 0.6 秒でした。番号の順番は、実行するたびに入れ替わることがあります（例5-5の解説2）。
- `_permit` は `query(n, start).await` が終わるまで持たれ、タスクの最後で片付けられて許可証が返ります。もし `let _ = connections.acquire_owned()...` と `_` 1文字にすると、その場ですぐに許可証が返ってしまい、上限が効かなくなります（UNIT 03 の例3-2の解説3と同じ注意です）。

**問3の解答:**

1. **標準の Mutex**（または、数を数えるだけなら標準の atomic 型 `AtomicU64`）。1台の中だけの、`.await` をはさまない短い書き換えなので、軽い標準の Mutex で十分です。再起動で0に戻ってもかまわない「今の様子」の数なので、メモリに置いてよい例です。
2. **Semaphore**。許可証を5枚にした Semaphore を1つ作り、`Arc` で全部のリクエストの処理に配ります。地図 API を呼ぶ前に許可証をもらい、終わったら返します（例5-5）。
3. **RwLock**。全部のリクエストが毎回読み、書き換えは1日に1回なので、読む人どうしが待ち合わない RwLock が向いています。読むときに `.await` をはさまないなら、標準の `std::sync::RwLock` で十分です。
4. **DB**。サーバーが2台あるので、メモリの中で数えると台ごとに別々の数になり、上限を正しく守れません。数は DB の表に置き、「数えて、上限を確かめて、足す」を transaction の中で行います（第7巻 UNIT 05）。メモリの鍵では、台をまたいだ食い違いは防げません。

> **ヒント:** 問3の1で出てきた atomic 型（`std::sync::atomic::AtomicU64` など）は、数を足す・引く・読むだけなら、鍵を使わずに、割り込まれない1回の操作で書き換えられる型です。第3巻 UNIT 10 の表に出てきた `Arc` の数え方も、この仕組みを使っています。

### まとめ

- async の中でも、データを守るのは標準の `std::sync::Mutex` が基本。決まりは「鍵を持ったまま `.await` しない」で、`{ }` のブロックか async でない関数で鍵の範囲を区切る。`drop(guard)` では、コンパイラに伝わらない。
- spawn すると「future cannot be sent」のエラーで止めてもらえるが、spawn しないと通ってしまい、スレッドが止まってデッドロックすることがある。clippy の `await_holding_lock` で見つける。鍵を持ったまま待つ必要があるときだけ tokio の Mutex を使う。
- 読む人が多いデータは RwLock、同時に入れる数の上限は Semaphore（許可証は片付けで返る）。消えてはいけないデータや台をまたぐデータは、鍵ではなく DB で守る。
