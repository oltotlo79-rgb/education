## UNIT 06: timeout・キャンセル・graceful shutdown

### この単元のゴール

- `tokio::time::timeout` で待つ時間に上限を付け、時間切れ（`Elapsed`）を自分のエラーの型に変えて返せる
- timeout や `select!` で Future が途中で捨てられる**キャンセル**によって、データが中途半端に壊れる場所を見つけ、`.await` をはさまない形や、別のタスクに任せる形で守れる
- timeout が**効かない**場合（スレッドごと眠る処理・重い計算）を説明できる
- `CancellationToken`（キャンセレーション・トークン）で停止の合図を全員に配り、`TaskTracker`（タスク・トラッカー）で全員が終わるのを待てる
- Ctrl+C を受けたら「受け付けを止める → 合図を出す → 上限つきで待つ → 間に合わなければ打ち切る」の順で止まる **graceful shutdown**（グレースフル・シャットダウン）を書ける。Topcoat が同じ順番で止まることを、ソースで確かめられる

### たとえ話でつかむ

- **timeout は、電話の保留の上限です。** 「30秒待っても相手が出なければ、いったん切ってかけ直す」と決めておけば、相手が出ないまま一日中受話器を持ち続けることはありません。外のサービスや DB が返事をしないとき、上限が無いと、そのリクエストを処理しているタスクは永遠に待ち続けます。
- **キャンセルは、作っている途中の料理の注文取り消しです。** 取り消されたら、店員はそこで手を止めます。そのとき、コンロに火がついたままだったら困ります。「どこで止められても、台所が中途半端な状態にならない」作り方が必要です。
- **graceful shutdown は、お店の閉店の手順です。**
  1. 入口に「本日の受付は終了しました」の札を出す（新しいお客さんを入れない）
  2. 店内に「まもなく閉店です」とアナウンスする（停止の合図）
  3. 食事中のお客さんが食べ終わるのを待つ。ただし、30分までと決めておく（上限つきで待つ）
  4. 30分たっても残っているお客さんには、申し訳ないけれど出てもらう（打ち切り）

  これに対して、いきなり店の電気を消して全員を追い出すのが、**強制終了**です。食べかけの料理も、会計の途中のお客さんも、そのまま放り出されます。

> **用語:** timeout（タイムアウト：待つ時間の上限。上限までに終わらなければ、待つのをやめて「時間切れ」として扱う）

<!-- -->

> **用語:** キャンセル（cancellation：Future が途中で drop され、二度と poll されなくなること。UNIT 01 の `try_join!`、UNIT 03 の `select!`、この単元の `timeout` で起きる）

<!-- -->

> **用語:** graceful shutdown（グレースフル・シャットダウン：処理中の仕事をできるだけ終わらせてから止まる、行儀のよい止め方。強制終了の反対）

### 図で理解する

まず、`timeout` です。中身は UNIT 03 の練習問題2で作った「仕事とタイマーの `select!`」と同じです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-061" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="30" font-size="12" font-weight="700" fill="#0f172a">timeout(500ms, 仕事)</text>
<text x="20" y="66" font-size="11" fill="#047857">仕事が 100ms で終わる</text>
<rect x="170" y="50" width="60" height="26" rx="5" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<text x="244" y="68" font-size="11" fill="#047857">→ Ok(結果)。0.1 秒で返る</text>
<text x="20" y="116" font-size="11" fill="#b91c1c">仕事が 800ms かかる</text>
<rect x="170" y="100" width="300" height="26" rx="5" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<rect x="470" y="100" width="180" height="26" rx="5" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 5"/>
<text x="470" y="120" font-size="18" font-weight="700" fill="#ef4444" text-anchor="middle">✕</text>
<text x="560" y="118" font-size="10" fill="#64748b" text-anchor="middle">捨てられて、続きは動かない</text>
<text x="320" y="146" font-size="11" fill="#b91c1c" text-anchor="middle">500ms で上限 → Err(Elapsed)。仕事の Future はその場で drop</text>
<line x1="470" y1="40" x2="470" y2="160" stroke="#ef4444" stroke-width="2" stroke-dasharray="4 4"/>
<text x="476" y="52" font-size="10" fill="#b91c1c">上限 500ms</text>
<line x1="170" y1="182" x2="690" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-061)"/>
<line x1="170" y1="176" x2="170" y2="188" stroke="#475569" stroke-width="2"/>
<line x1="230" y1="176" x2="230" y2="188" stroke="#475569" stroke-width="2"/>
<line x1="470" y1="176" x2="470" y2="188" stroke="#475569" stroke-width="2"/>
<line x1="650" y1="176" x2="650" y2="188" stroke="#475569" stroke-width="2"/>
<text x="170" y="204" font-size="11" fill="#475569" text-anchor="middle">0</text>
<text x="230" y="204" font-size="11" fill="#475569" text-anchor="middle">0.1</text>
<text x="470" y="204" font-size="11" fill="#475569" text-anchor="middle">0.5</text>
<text x="650" y="204" font-size="11" fill="#475569" text-anchor="middle">0.8 秒</text>
<text x="360" y="236" font-size="12" fill="#475569" text-anchor="middle">上限までに終われば Ok、終わらなければ Err(Elapsed)。どちらでも、待つのは最大 0.5 秒</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: timeout のしくみ。上限までに仕事が終われば Ok(結果)、終わらなければ Err(Elapsed) が返り、仕事の Future は上限の時点で捨てられる</div>
</div>

次に、graceful shutdown の順番です。この単元のコード（例6-4）と、RustNote を動かしている Topcoat の中の止め方を並べました。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-062" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="128" height="70" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="74" y="46" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">① 止める合図</text>
<text x="74" y="66" font-size="10" fill="#475569" text-anchor="middle">Ctrl+C を受け取る</text>
<rect x="152" y="20" width="128" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="216" y="46" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">② 受付を止める</text>
<text x="216" y="66" font-size="10" fill="#475569" text-anchor="middle">新しい仕事を入れない</text>
<rect x="294" y="20" width="128" height="70" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="358" y="46" font-size="12" font-weight="700" fill="#854d0e" text-anchor="middle">③ 全員に知らせる</text>
<text x="358" y="66" font-size="10" fill="#475569" text-anchor="middle">「片付けて終わって」</text>
<rect x="436" y="20" width="128" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="500" y="46" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">④ 上限つきで待つ</text>
<text x="500" y="66" font-size="10" fill="#475569" text-anchor="middle">全員が終わるまで</text>
<rect x="578" y="12" width="132" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="644" y="37" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">⑤ 全員終わった</text>
<rect x="578" y="60" width="132" height="40" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="644" y="85" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">⑤ 時間切れ → 打ち切り</text>
<line x1="138" y1="55" x2="148" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-062)"/>
<line x1="280" y1="55" x2="290" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-062)"/>
<line x1="422" y1="55" x2="432" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-062)"/>
<line x1="564" y1="48" x2="574" y2="34" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-062)"/>
<line x1="564" y1="62" x2="574" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-062)"/>
<text x="10" y="132" font-size="12" font-weight="700" fill="#0f172a">この単元のコード（例6-4）</text>
<text x="74" y="156" font-size="10" fill="#0f172a" text-anchor="middle">signal::ctrl_c()</text>
<text x="216" y="156" font-size="10" fill="#0f172a" text-anchor="middle">tracker.close()</text>
<text x="358" y="156" font-size="10" fill="#0f172a" text-anchor="middle">token.cancel()</text>
<text x="500" y="156" font-size="10" fill="#0f172a" text-anchor="middle">timeout(5秒, tracker.wait())</text>
<text x="644" y="156" font-size="10" fill="#0f172a" text-anchor="middle">Ok / Err(Elapsed)</text>
<line x1="10" y1="172" x2="710" y2="172" stroke="#cbd5e1"/>
<text x="10" y="200" font-size="12" font-weight="700" fill="#0f172a">Topcoat 0.10 の中（RustNote）</text>
<text x="74" y="224" font-size="10" fill="#0f172a" text-anchor="middle">Ctrl+C / SIGTERM</text>
<text x="216" y="224" font-size="10" fill="#0f172a" text-anchor="middle">drop(listener)</text>
<text x="358" y="224" font-size="10" fill="#0f172a" text-anchor="middle">drop(drain_tx)</text>
<text x="500" y="224" font-size="10" fill="#0f172a" text-anchor="middle">done_tx.closed() と</text>
<text x="500" y="240" font-size="10" fill="#0f172a" text-anchor="middle">sleep(30秒) の select!</text>
<text x="644" y="224" font-size="10" fill="#0f172a" text-anchor="middle">drop(cutoff_tx) で</text>
<text x="644" y="240" font-size="10" fill="#0f172a" text-anchor="middle">残りの接続を切る</text>
<text x="360" y="284" font-size="12" fill="#475569" text-anchor="middle">道具は違っても、順番は同じ。「待つ」には必ず上限を付け、最後は打ち切れるようにしておく</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: graceful shutdown の順番。合図を受けたら、受付を止め、全員に知らせ、上限つきで待ち、間に合わなければ打ち切る。Topcoat も同じ順番で止まる</div>
</div>

- ②で受付を止めるのは、止めている最中に新しい仕事が入ってくると、いつまでも終われないからです。
- ④で上限を付けるのは、返事をしない相手を待っている仕事が1つでもあると、永遠に止まれないからです。止められないサーバーは、結局、外から強制終了されてしまいます。

### まず動かす最小コード

#### 例6-1: 外のサービスの返事を、最大 500 ミリ秒だけ待つ

> **▼ このコードがやること（先に日本語で）:** 外のサービスの代わりに、決まった時間たってから答える関数 `call_service` を作ります。100 ミリ秒で答える場合と、800 ミリ秒かかる場合のそれぞれに、`timeout` で 500 ミリ秒の上限を付けて呼び、結果と、かかった時間を表示します。

```rust
use std::time::{Duration, Instant};
use tokio::time::timeout;

// 外部サービスの代わり: millis ミリ秒たってから答える
async fn call_service(millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{millis}ms で応答")
}

#[tokio::main]
async fn main() {
    for millis in [100, 800] {
        let start = Instant::now();
        // 最大 500ms だけ待つ。間に合えば Ok(結果)、間に合わなければ Err(Elapsed)
        match timeout(Duration::from_millis(500), call_service(millis)).await {
            Ok(answer) => println!("成功: {answer}（{:.1} 秒）", start.elapsed().as_secs_f64()),
            Err(elapsed) => println!(
                "時間切れ: {elapsed} / {elapsed:?}（{:.1} 秒）",
                start.elapsed().as_secs_f64()
            ),
        }
    }
}
```

`src/bin/u06_timeout.rs` として保存して実行します。

```powershell
cargo run -q --bin u06_timeout
```

**▼ 実行結果**

```text
成功: 100ms で応答（0.1 秒）
時間切れ: deadline has elapsed / Elapsed(())（0.5 秒）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: timeout(上限, Future) は、Result を返す Future

- `tokio::time::timeout(Duration::from_millis(500), call_service(millis))` は、「`call_service` の Future を、最大 500 ミリ秒だけ待つ」という新しい Future を作ります。これも Future なので、`.await` して初めて動きます（UNIT 01）。
- 結果は `Result<中身の結果, Elapsed>` です。上限までに終われば `Ok(答え)`、終わらなければ `Err(Elapsed)` になります。
- 800 ミリ秒かかる場合も、0.5 秒で返ってきました。待つ時間が、上限で必ず区切られています。

##### 解説2: Elapsed は「時間切れでした」という印

- `Elapsed` は、時間切れを表すエラーの型です。`{elapsed}`（`Display`）で表示すると `deadline has elapsed`（締め切りが過ぎました）、`{elapsed:?}`（`Debug`）で表示すると `Elapsed(())` になります。
- `Elapsed` には、「どれだけ待ったか」や「何を待っていたか」は入っていません。ログやエラーメッセージには、自分で「何を、何ミリ秒待ったか」を書き足しましょう（次の例6-2）。

##### 解説3: 時間切れのとき、中の Future は捨てられる

- 時間切れになると、`call_service(800)` の Future は、その場で drop されます（UNIT 03 の `select!` で負けた枝と同じ）。800 ミリ秒の `sleep` の残りは動きません。
- 「待つのをやめる」だけでなく、「**仕事そのものも途中で止まる**」ことに注意しましょう。途中で止まって困る仕事のときの守り方は、例6-3と練習問題2で扱います。

> **ヒント:** 外のサービスや DB を呼ぶときは、基本的に、必ず上限を付けます。上限の長さは「利用者が待てる時間」から決めます。画面の表示に使う問い合わせなら、数百ミリ秒〜数秒が目安です。HTTP クライアントの reqwest には、上限の設定が最初から用意されています（UNIT 08）。

#### 例6-2: 時間切れを、自分の関数のエラーに変えて返す

> **▼ このコードがやること（先に日本語で）:** エラーを `String` で返す関数 `fetch` の中で、800 ミリ秒かかるサービスを `timeout` で呼びます。時間切れなら「500 ミリ秒で時間切れ」という文字列のエラーに変えて、`?` で返します。上限 500 ミリ秒と 1000 ミリ秒の2通りで試します。

```rust
use std::time::Duration;
use tokio::time::timeout;

async fn call_service() -> String {
    tokio::time::sleep(Duration::from_millis(800)).await;
    String::from("応答")
}

async fn fetch(limit: Duration) -> Result<String, String> {
    // 時間切れ（Elapsed）を、この関数のエラーの型（String）に変えてから ? で返す
    let answer = timeout(limit, call_service())
        .await
        .map_err(|_| format!("{} ミリ秒で時間切れ", limit.as_millis()))?;
    Ok(answer)
}

#[tokio::main]
async fn main() {
    println!("{:?}", fetch(Duration::from_millis(500)).await);
    println!("{:?}", fetch(Duration::from_millis(1000)).await);
}
```

`src/bin/u06_fix_elapsed.rs` として保存して実行します。

```powershell
cargo run -q --bin u06_fix_elapsed
```

**▼ 実行結果**

```text
Err("500 ミリ秒で時間切れ")
Ok("応答")
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: map_err でエラーの型をそろえてから ? を付ける

- `timeout(...).await` の型は `Result<String, Elapsed>` です。`fetch` のエラーの型は `String` なので、そのまま `?` を付けるとコンパイルエラーになります（「よくあるエラー1」）。
- `.map_err(|_| format!(...))` は、「`Err` のときだけ、中身を別の値に変える」メソッドです（第2巻 UNIT 09）。`Elapsed` を捨てて、「何ミリ秒で時間切れか」を書いた `String` に変えています。これで型がそろい、`?` で返せます。

##### 解説2: 実際のアプリでは、アプリのエラー型へ変える

- RustNote のような実際のアプリでは、`String` ではなく、第4巻 UNIT 05 で作ったようなアプリのエラー型を使います。たとえば、エラーの `enum` に「外のサービスが時間内に答えなかった」という種類を1つ足し、`map_err` でその種類に変えます。そうすると、画面には「時間をおいてもう一度お試しください」を出し、ログには何を何ミリ秒待ったかを残す、という使い分けができます。

### もう一歩: 実務に近い形

#### 例6-3: キャンセルされても、データが壊れない形にする

> **▼ このコードがやること（先に日本語で）:** 2つの口座の残高（どちらも 1000）を `Mutex` で守り、0番から1番へ 300 を移す「振り込み」を2通りの書き方で作ります。①は「引く → 相手の銀行への連絡（0.3 秒）を `.await` → 足す」、②は「連絡を先に `.await` → 引くと足すを一気に」です。どちらも `timeout` で 0.1 秒で打ち切り、そのときの残高を表示します。

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::timeout;

// 口座の残高（共有）
type Balances = Arc<Mutex<Vec<i64>>>;

// ① 悪い例: 「引く」と「足す」の間に .await がある
async fn transfer_unsafe(balances: Balances, from: usize, to: usize, amount: i64) {
    balances.lock().unwrap()[from] -= amount;
    println!("  {from} 番から {amount} 引いた");
    // 相手の銀行へ連絡する（時間がかかる）
    tokio::time::sleep(Duration::from_millis(300)).await;
    balances.lock().unwrap()[to] += amount;
    println!("  {to} 番に {amount} 足した");
}

// ② 良い例: 時間のかかる連絡を先に済ませ、残高の変更は .await をはさまずに一気に行う
async fn transfer_safe(balances: Balances, from: usize, to: usize, amount: i64) {
    tokio::time::sleep(Duration::from_millis(300)).await;
    let mut b = balances.lock().unwrap();
    b[from] -= amount;
    b[to] += amount;
    println!("  {from} 番 → {to} 番へ {amount} 移した");
}

#[tokio::main]
async fn main() {
    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    // 100ms で打ち切る（= 途中で Future が片付けられる）
    let result = timeout(
        Duration::from_millis(100),
        transfer_unsafe(Arc::clone(&balances), 0, 1, 300),
    )
    .await;
    println!("① {result:?} → 残高 {:?}", balances.lock().unwrap());

    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    let result = timeout(
        Duration::from_millis(100),
        transfer_safe(Arc::clone(&balances), 0, 1, 300),
    )
    .await;
    println!("② {result:?} → 残高 {:?}", balances.lock().unwrap());
}
```

`src/bin/u06_cancel.rs` として保存して実行します。

```powershell
cargo run -q --bin u06_cancel
```

**▼ 実行結果**

```text
  0 番から 300 引いた
① Err(Elapsed(())) → 残高 [700, 1000]
② Err(Elapsed(())) → 残高 [1000, 1000]
```

①では、合計が 2000 から 1700 に減りました。**300 がどこかに消えています。**

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-063" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="26" font-size="12" font-weight="700" fill="#b91c1c">① 引く → .await → 足す</text>
<rect x="20" y="38" width="110" height="34" rx="6" fill="#fee2e2" stroke="#ef4444" stroke-width="2"/>
<text x="75" y="60" font-size="11" fill="#b91c1c" text-anchor="middle">0 番から引く</text>
<line x1="240" y1="30" x2="240" y2="86" stroke="#ef4444" stroke-width="2" stroke-dasharray="4 4"/>
<rect x="140" y="38" width="300" height="34" rx="6" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<text x="290" y="60" font-size="11" fill="#1e40af" text-anchor="middle">相手の銀行へ連絡（.await 0.3 秒）</text>
<rect x="450" y="38" width="110" height="34" rx="6" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4"/>
<text x="505" y="60" font-size="11" fill="#64748b" text-anchor="middle">1 番に足す</text>
<text x="240" y="100" font-size="11" fill="#b91c1c" text-anchor="middle">0.1 秒で打ち切り ✕</text>
<text x="580" y="52" font-size="11" fill="#b91c1c">残高 [700, 1000]</text>
<text x="580" y="68" font-size="11" fill="#b91c1c">300 が消えた</text>
<text x="20" y="140" font-size="12" font-weight="700" fill="#047857">② .await → 引くと足すを一気に（.await をはさまない）</text>
<line x1="120" y1="144" x2="120" y2="200" stroke="#ef4444" stroke-width="2" stroke-dasharray="4 4"/>
<rect x="20" y="152" width="300" height="34" rx="6" fill="#dbeafe" stroke="#3b82f6" stroke-width="2"/>
<text x="170" y="174" font-size="11" fill="#1e40af" text-anchor="middle">相手の銀行へ連絡（.await 0.3 秒）</text>
<rect x="330" y="152" width="200" height="34" rx="6" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4"/>
<text x="430" y="174" font-size="11" fill="#64748b" text-anchor="middle">引く＋足す（途中で止まらない）</text>
<text x="120" y="214" font-size="11" fill="#b91c1c" text-anchor="middle">0.1 秒で打ち切り ✕</text>
<text x="550" y="166" font-size="11" fill="#047857">残高 [1000, 1000]</text>
<text x="550" y="182" font-size="11" fill="#047857">何も起きていない</text>
<text x="360" y="234" font-size="11" fill="#475569" text-anchor="middle">止められるのは .await の所だけ。「全部か、何もしないか」にしたい変更の間に .await を置かない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: キャンセルされる場所とデータの壊れ方。①は引いた直後の .await で止められて、足す前で終わった。②は変更の前に止められたので、何も変わっていない</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: Future が止められるのは、.await の所だけ

- `timeout` が 0.1 秒で時間切れになると、`transfer_unsafe` の Future は drop されます。drop された Future は、**止まっていた `.await` の所から先が、二度と動きません**。
- ①は、0番から 300 を引いた直後の `.await`（0.3 秒の連絡）で止まっていました。だから「引く」は済んだのに「足す」は起きず、300 が消えました。
- ②は、最初の `.await`（連絡）の途中で止められました。引くのも足すのもその後なので、残高は何も変わっていません。

##### 解説2: 「全部か、何もしないか」の間に .await を置かない

- 引くと足すは、両方が起きるか、どちらも起きないか、のどちらかでなければなりません。②では、この2つを `.await` をはさまずに続けて書いているので、途中で止められることがありません。キャンセルは `.await` の区切りでしか起きないからです（UNIT 02 の例2-3の解説3）。
- 時間のかかる処理（連絡）は先に済ませ、状態の変更は最後に一気に行う。これが、キャンセルされても壊れない書き方の基本です。

##### 解説3: 本当のお金なら、DB の transaction で守る

- この例はメモリの中の残高ですが、本当のアプリでは残高は DB にあります。DB への「引く」と「足す」も、それぞれ `.await` で待つ処理です。だから、間に `.await` を置かない、という書き方ができません。
- そこで使うのが、第7巻 UNIT 05 で学んだ **transaction** です。「全部確定か、全部取り消しか」を DB が守ってくれます。libsql の `Transaction` は、`commit` する前に drop されると、自動で取り消されます（第7巻で確かめました）。Future が途中で捨てられても、確定していない変更は残りません。

> **注意:** キャンセルは、`timeout` だけでなく、`select!` で負けたとき（UNIT 03）、`try_join!` でほかが失敗したとき（UNIT 01）、`JoinSet` が捨てられたとき（UNIT 03）、タスクが `abort` されたとき（UNIT 02）にも起きます。そして、**利用者がブラウザでページの読み込みを途中でやめたとき**にも、Web フレームワークによってはリクエストの処理の Future が捨てられることがあります。「どこで止められても壊れないか」は、async で書くときにいつも確かめることです。

#### 例6-4: Ctrl+C を受けたら、裏の係に知らせて、片付けを待ってから止まる

この例では、tokio に加えて tokio-util を使います。`async_lab` フォルダで次を実行します。

```powershell
cargo add tokio-util@=0.7.20 --features rt
```

```text
    Updating crates.io index
      Adding tokio-util =0.7.20 to dependencies
             Features:
             + futures-util
             + rt
             - __docs_rs
             - codec
             - compat
             - full
             - futures-io
             - hashbrown
             - io
             - io-util
             - join-map
             - libc
             - net
             - slab
             - time
             - tracing
    Updating crates.io index
     Locking 7 packages to highest Rust 1.98 compatible versions
      Adding futures-core v0.3.34
      Adding futures-macro v0.3.34
      Adding futures-sink v0.3.34
      Adding futures-task v0.3.34
      Adding futures-util v0.3.34
      Adding slab v0.4.12
      Adding tokio-util v0.7.20
```

- tokio-util は、tokio の作者たちが作っている、tokio と一緒に使う便利な道具の集まりです。`rt` の機能に、この例で使う `TaskTracker` が入っています（`CancellationToken` は機能を選ばなくても使えます）。
- `Locking 7 packages to highest Rust 1.98 compatible versions` は、`Cargo.toml` の `rust-version = "1.98"` に合う版の中から選んだ、という意味です。

> **▼ このコードがやること（先に日本語で）:** 0.5 秒ごとに仕事をする裏の係（worker）を2つ動かします。`main` は Ctrl+C を待ち、受け取ったら、①新しいタスクの受付を止め、②全員に停止の合図を出し、③最大 5 秒だけ全員が終わるのを待ちます。係は合図を受け取ったら、0.2 秒かけて片付けてから終わります。

```rust
use std::time::Duration;
use tokio::time::{interval, sleep, timeout};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

// 裏で動き続ける係。0.5 秒ごとに仕事をし、停止の合図が来たら片付けて終わる
async fn worker(id: u32, token: CancellationToken) {
    let mut ticker = interval(Duration::from_millis(500));
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("  worker{id}: 停止の合図を受け取った。片付けます");
                // 片付けにも少し時間がかかる（書きかけのデータを保存するなど）
                sleep(Duration::from_millis(200)).await;
                println!("  worker{id}: 終了");
                return;
            }
            _ = ticker.tick() => {
                println!("  worker{id}: 仕事をしました");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // 停止の合図を全員に配るための札
    let token = CancellationToken::new();
    // 作ったタスクを覚えておき、全部終わるのを待てるようにする係
    let tracker = TaskTracker::new();
    for id in 1..=2 {
        tracker.spawn(worker(id, token.child_token()));
    }

    println!("起動しました。Ctrl+C で止まります");
    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl+C を待ち受けられません");
    println!("Ctrl+C を受け取りました。停止を始めます");

    // ① もう新しいタスクは受け付けない
    tracker.close();
    // ② 全員に停止の合図を出す
    token.cancel();
    // ③ 最大 5 秒だけ、全員が終わるのを待つ
    match timeout(Duration::from_secs(5), tracker.wait()).await {
        Ok(()) => println!("全員きれいに終わりました"),
        Err(_) => println!("5 秒待っても終わらないタスクがあるので、打ち切ります"),
    }
}
```

`src/bin/u06_shutdown.rs` として保存して実行し、1秒ほどたったら、ターミナルで Ctrl+C を押します。

```powershell
cargo run -q --bin u06_shutdown
```

**▼ 実行結果**（起動してから約 1.3 秒後に Ctrl+C を送ったときの結果です）

```text
起動しました。Ctrl+C で止まります
  worker2: 仕事をしました
  worker1: 仕事をしました
  worker1: 仕事をしました
  worker2: 仕事をしました
Ctrl+C を受け取りました。停止を始めます
  worker2: 停止の合図を受け取った。片付けます
  worker1: 停止の合図を受け取った。片付けます
  worker1: 終了
  worker2: 終了
全員きれいに終わりました
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-064" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="250" y="14" width="220" height="50" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="360" y="36" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">親の札 token</text>
<text x="360" y="54" font-size="10" fill="#475569" text-anchor="middle">token.cancel() で全員に届く</text>
<rect x="60" y="120" width="200" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="160" y="142" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">子の札（worker1）</text>
<text x="160" y="160" font-size="10" fill="#475569" text-anchor="middle">token.child_token()</text>
<rect x="460" y="120" width="200" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="560" y="142" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">子の札（worker2）</text>
<text x="560" y="160" font-size="10" fill="#475569" text-anchor="middle">token.child_token()</text>
<line x1="320" y1="64" x2="190" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-064)"/>
<line x1="400" y1="64" x2="530" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-064)"/>
<text x="230" y="86" font-size="10" fill="#475569" text-anchor="middle">親が取り消されると</text>
<text x="490" y="86" font-size="10" fill="#475569" text-anchor="middle">子も取り消される</text>
<text x="360" y="204" font-size="12" fill="#475569" text-anchor="middle">子の札を取り消しても、親やほかの子には届かない。合図は親から子への一方通行</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: CancellationToken の親と子。親の札を取り消すと、そこから作った子の札にも合図が届く。子の札だけを取り消すと、その子だけが止まる</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: tokio::signal::ctrl_c で、Ctrl+C を待つ

- `tokio::signal::ctrl_c().await` は、ターミナルで Ctrl+C が押されるまで待つ Future です（tokio の `signal` の機能。`full` に入っています）。押されると `Ok(())` が返り、次の行へ進みます。
- ふつうのプログラムは、Ctrl+C を押されると、すぐに強制終了されます。`ctrl_c()` で待ち受けているときは、強制終了されずに、プログラムが自分で止まり方を決められます。

##### 解説2: CancellationToken は「止まって」の合図を配る札

- `CancellationToken::new()` で親の札を作り、`token.child_token()` で係ごとに子の札を作って渡しています。親の札で `token.cancel()` を呼ぶと、子の札の全員に合図が届きます。
- 係の側では、`token.cancelled()`（合図が来るまで待つ Future）を `select!` の枝に入れています（UNIT 03・UNIT 04 の練習問題3と同じ形）。合図が来ると、その枝で片付けをして `return` します。
- UNIT 04 の練習問題3では、watch の channel で同じことをしました。`CancellationToken` は「止める合図」専用の道具で、`clone` や `child_token` で何人にでも配れ、一度取り消されたら取り消されたままになります。

##### 解説3: TaskTracker は、作ったタスクを覚えておき、全部の終わりを待つ係

- `tracker.spawn(...)` は `tokio::spawn` と同じようにタスクを作り、そのタスクを覚えておきます。
- `tracker.wait().await` は、「**受付が閉じられていて（`close` 済み）、かつ、覚えているタスクが全部終わった**」ときに終わる Future です。だから、先に `tracker.close()` を呼びます。呼び忘れると、タスクが全部終わっても `wait` が終わりません（「よくあるエラー3」）。
- UNIT 03 の JoinSet との違いは、結果を受け取らないことと、タスクを捨てても取り消さないことです。「結果は要らないが、終わるまでは待ちたい」裏の仕事には、TaskTracker が向いています。

##### 解説4: 待つのにも上限を付ける

- ③では、`tracker.wait()` に `timeout` で 5 秒の上限を付けています。係の片付けがどこかで止まってしまっても、プログラムは 5 秒で止まれます。
- この実行では、係が 0.2 秒で片付けを終えたので、「全員きれいに終わりました」になりました。上限に間に合わないとどうなるかは、練習問題3で確かめます。

> **ヒント:** 実行結果の「仕事をしました」が、Ctrl+C の前に2回ずつ出ているのは、`interval` の最初の `tick()` がすぐに（0 秒で）終わるからです。0 秒と 0.5 秒と 1.0 秒の3回のうち、どこまで表示されるかは、Ctrl+C を押す時刻によって変わります。

### よくあるエラーと読み方

#### エラー1: timeout の結果に、そのまま ? を付ける（E0277: `?` couldn't convert the error）

```rust
use std::time::Duration;
use tokio::time::timeout;

async fn call_service() -> String {
    tokio::time::sleep(Duration::from_millis(800)).await;
    String::from("応答")
}

// エラーを String で返す関数の中で、timeout の結果に ? を付ける
async fn fetch() -> Result<String, String> {
    let answer = timeout(Duration::from_millis(500), call_service()).await?;
    Ok(answer)
}

#[tokio::main]
async fn main() {
    println!("{:?}", fetch().await);
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0277]: `?` couldn't convert the error to `String`
  --> src\bin\e06_elapsed_question.rs:11:75
   |
11 |     let answer = timeout(Duration::from_millis(500), call_service()).await?;
   |                  ---------------------------------------------------------^ the trait `From<Elapsed>` is not implemented for `String`
   |                  |
   |                  this can't be annotated with `?` because it has type `Result<_, Elapsed>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: `String` implements trait `From<T>`:
             From<&String>
             From<&mut str>
             From<&str>
             From<Box<str>>
             From<Cow<'_, str>>
             From<char>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `async_lab` (bin "e06_elapsed_question") due to 1 previous error
```

- **何が起きたか:** 「`?` が、エラーを `String` に変換できませんでした」というエラーです。`?` は、`Err` を返すときに、`From` trait でエラーの型を関数の戻り値の型に変換します（`note:` の行。第4巻 UNIT 05）。`Elapsed` から `String` への変換（`From<Elapsed> for String`）は用意されていないので、変換できません。
- `help:` には、`String` が受け取れる変換の元（`&str` や `char` など）が並んでいます。`Elapsed` は入っていません。
- **直し方:** 例6-2のとおり、`.map_err(|_| ...)` で `Elapsed` を `String` に変えてから `?` を付けます。アプリのエラー型を使っているなら、そのエラー型に `From<Elapsed>` を実装しておく方法もあります（第4巻 UNIT 05 の `#[from]`）。

#### エラー2（エラーが出ない間違い）: timeout が効かない

`timeout` で 100 ミリ秒の上限を付けたのに、中で `std::thread::sleep`（スレッドごと眠る）を使った場合です。

```rust
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 100ms で打ち切るつもりで、中で「スレッドごと眠る」処理をしてしまう
    let result = timeout(Duration::from_millis(100), async {
        std::thread::sleep(Duration::from_secs(1));
        "1 秒の処理が終わった"
    })
    .await;
    println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
}
```

`src/bin/u06_blocking_timeout.rs` として実行した結果です。

```text
Ok("1 秒の処理が終わった")（1.0 秒）
```

- **何が起きたか:** 上限は 0.1 秒なのに、1.0 秒かかって、しかも `Ok` が返りました。時間切れになっていません。
- `timeout` は、中の Future を poll して、`Pending` が返ってきたら、タイマーと競争させます。ところが、中の `std::thread::sleep` は `.await` ではなく、**poll の中でスレッドごと1秒眠って**しまいます。その間、`timeout` は何もできません。1秒後に poll から戻ってきたときには、中身はもう `Ready` になっていました。
- 「Future が止められるのは `.await` の所だけ」（例6-3）の裏返しです。`.await` の無い長い処理は、timeout でも、`select!` でも、`abort` でも止められません。
- **直し方:** async の中では、待つなら `tokio::time::sleep(...).await` を使います（UNIT 01 の解説3）。重い計算や、どうしても同期で待つ処理は、`spawn_blocking` で別のスレッドに任せます（UNIT 07）。そうすれば、待つ側は `.await` で席を譲りながら待てるので、上限も効きます。

#### エラー3（エラーが出ない間違い）: tracker.close() を忘れて、wait が終わらない

```rust
use std::time::Duration;
use tokio::time::timeout;
use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() {
    let tracker = TaskTracker::new();
    for id in 1..=2 {
        tracker.spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("  タスク{id}: 終了");
        });
    }
    // tracker.close() を書き忘れた
    match timeout(Duration::from_secs(1), tracker.wait()).await {
        Ok(()) => println!("全員終わりました"),
        Err(_) => println!(
            "1 秒待っても wait が終わりません（残りのタスク: {}）",
            tracker.len()
        ),
    }
}
```

```text
  タスク2: 終了
  タスク1: 終了
1 秒待っても wait が終わりません（残りのタスク: 0）
```

（`cargo run` の `Compiling ...` などの行は省いています。）

- **何が起きたか:** 2つのタスクは 0.1 秒で終わり、残りのタスクは 0 個なのに、`wait` は1秒待っても終わりませんでした。
- `TaskTracker` の `wait` は、「もう新しいタスクは来ない（`close` 済み）」と分かるまで終わらない作りです。タスクが0個になっただけでは、「これから新しいタスクが足されるかもしれない」ので、待ち続けます。UNIT 04 の「送る口を `drop` し忘れて、受け取り係が終わらない」と同じ考え方です。
- **直し方:** `wait` の前に `tracker.close()` を呼びます（例6-4の①）。

### RustNote ではここで使う

RustNote の `main.rs` には、Ctrl+C を待つコードも、停止の合図を配るコードもありません。`topcoat::start(router).await` の中で、Topcoat が graceful shutdown をしてくれるからです。Topcoat 0.10.0 のソースで、そのことを確かめます。

まず `topcoat-0.10.0/src/serve.rs` です。`topcoat::start` が使う `serve` 関数の説明には、「Ctrl+C か、Unix では SIGTERM を受け取ると graceful shutdown を始める。処理中のリクエストは、`RouterService::shutdown_timeout` の時間が過ぎるまでに終わればよい」と書かれています。止める合図を待つ関数は次のとおりです（コメントは日本語に置き換えています）。

```rust
// プロセスが止める合図（Ctrl+C か、Unix では SIGTERM）を受け取ったら終わる
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install the Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install the SIGTERM signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}
```

- 例6-4と同じ `tokio::signal::ctrl_c()` を使っています。Linux などの Unix では、それに加えて **SIGTERM**（「止まってください」という OS の合図）も待ち、`select!` でどちらか先に来たほうを受け取ります。Windows には SIGTERM が無いので、`std::future::pending()`（永遠に終わらない Future）を代わりに置いています。
- RustNote を Fly.io で動かすとき、アプリを止めたり入れ替えたりする際に、Fly.io から合図が送られてきます。合図の種類と待ち時間の設定は、第12巻で扱います。

次に、合図を受けた後の流れです（`topcoat-router-0.10.0/src/service.rs` の `internal_serve` の後半。UNIT 04 でも一部を読みました）。

```rust
// 受け付けをやめる。待ち受けのポートを空けて、入れ替わりのプロセスが使えるようにする
drop(listener);

// 全部の接続のタスクに graceful shutdown を始めさせ、猶予の時間を与える
drop(drain_rx);
drop(drain_tx);
drop(done_rx);
tokio::select! {
    () = done_tx.closed() => {}
    () = tokio::time::sleep(service.shutdown_timeout) => {}
}

// 残っている接続を切り、それらのタスクが終わるのを待つ
drop(cutoff_rx);
drop(cutoff_tx);
done_tx.closed().await;
```

- `drop(listener)` が②受付を止める、`drop(drain_tx)` が③全員に知らせる、`select!` で「全部の接続が終わった」と「猶予の時間」を競争させるのが④上限つきで待つ、`drop(cutoff_tx)` が⑤打ち切り、です。図の下の段のとおりです。
- 猶予の時間 `shutdown_timeout` の既定は **30 秒**です（同じファイルに `const DEFAULT_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);` とあります）。RustNote は既定のまま使っています。
- 接続のタスクの側は、合図を受けると「今のリクエストを最後まで処理してから、接続を閉じる」動きをします（UNIT 02 で読んだ、接続ごとのタスクの中の `select!`）。だから、RustNote でノートを保存している最中に Ctrl+C を押しても、その保存が猶予の 30 秒以内に終われば、最後まで行われてから止まります。

RustNote の中で「時間の上限」が使われているのは、DB の設定の1か所です。ローカルの DB ファイルを使うとき、`src/db.rs` の `connect` で、接続ごとに次の設定をしています。

```rust
// busy_timeout … ほかの接続が書き込み中なら最大 5 秒待つ
conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;")
    .await?;
```

`busy_timeout` は、tokio の `timeout` ではなく、SQLite の中の「ほかの接続が書き込みの鍵を持っているとき、最大何ミリ秒待つか」の設定です（第7巻 UNIT 05 の `database is locked`）。RustNote のコードには、tokio の `timeout` は出てきません。Turso（リモート）への問い合わせに上限を付けたくなったら、Repository の関数の中で、この単元の `timeout` と `map_err` の形で包むことになります。

### 練習問題

**問1:** 100・400・200 ミリ秒かかる3つの呼び出し A・B・C に、それぞれ 300 ミリ秒の上限を付けて、`tokio::join!` で同時に呼んでください。時間切れは「B は時間切れ」のような `Err(String)` にします。どれが時間切れになり、全体で何秒かかるかを予想してから実行しましょう（`src/bin/u06_q1.rs`）。

**問2:** 例6-3の①（`transfer_unsafe`）を書き換えずに、「呼ぶ側が 0.1 秒で待つのをやめても、振り込みそのものは最後まで行われる」ようにしてください。少し後で残高を表示して、合計が 2000 のままか確かめます（`src/bin/u06_q2.rs`）。ヒント: UNIT 02 で学んだとおり、`tokio::spawn` したタスクは、引換券を待つのをやめても動き続けます。

**問3:** 例6-4の `u06_shutdown.rs` で、係の片付けにかかる時間を 0.2 秒から **3 秒**に、全員を待つ上限を 5 秒から **1 秒**に変えると、Ctrl+C の後の表示はどうなるでしょうか。予想してから実行してください（`src/bin/u06_q3.rs`）。

### 解答と解説

**問1の解答:**

```rust
use std::time::{Duration, Instant};
use tokio::time::timeout;

// 外部サービスの代わり: millis ミリ秒たってから答える
async fn call_service(name: &str, millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{name} の応答")
}

// 1つの呼び出しに、300ms の上限を付ける
async fn call_with_limit(name: &str, millis: u64) -> Result<String, String> {
    timeout(Duration::from_millis(300), call_service(name, millis))
        .await
        .map_err(|_| format!("{name} は時間切れ"))
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        call_with_limit("A", 100),
        call_with_limit("B", 400),
        call_with_limit("C", 200)
    );
    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
    println!("全体 {:.1} 秒", start.elapsed().as_secs_f64());
}
```

```text
Ok("A の応答")
Err("B は時間切れ")
Ok("C の応答")
全体 0.3 秒
```

- 300 ミリ秒に間に合わなかった B だけが時間切れです。全体は、B の上限の 0.3 秒で終わりました。上限が無ければ、B を待って 0.4 秒かかるところです。
- `try_join!` ではなく `join!` を使ったので、B が失敗しても A と C の結果は受け取れています（UNIT 03 の練習問題3と同じ考え方）。「一部が時間切れでも、そろった分で画面を出す」ときの形です。

**問2の解答:**

```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::timeout;

type Balances = Arc<Mutex<Vec<i64>>>;

// 例6-3 の悪い例と同じ: 「引く」と「足す」の間に .await がある
async fn transfer_unsafe(balances: Balances, from: usize, to: usize, amount: i64) {
    balances.lock().unwrap()[from] -= amount;
    println!("  {from} 番から {amount} 引いた");
    tokio::time::sleep(Duration::from_millis(300)).await;
    balances.lock().unwrap()[to] += amount;
    println!("  {to} 番に {amount} 足した");
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let balances: Balances = Arc::new(Mutex::new(vec![1000, 1000]));
    // 仕事は別のタスクとして動かし、timeout は「待つこと」だけに付ける
    let handle = tokio::spawn(transfer_unsafe(Arc::clone(&balances), 0, 1, 300));
    match timeout(Duration::from_millis(100), handle).await {
        Ok(_) => println!("{:.1} 秒: 間に合いました", start.elapsed().as_secs_f64()),
        Err(_) => println!(
            "{:.1} 秒: 待つのはやめました（仕事は続いています）",
            start.elapsed().as_secs_f64()
        ),
    }
    // 少し後で残高を見る
    tokio::time::sleep(Duration::from_millis(400)).await;
    println!(
        "{:.1} 秒: 残高 {:?}",
        start.elapsed().as_secs_f64(),
        balances.lock().unwrap()
    );
}
```

```text
  0 番から 300 引いた
0.1 秒: 待つのはやめました（仕事は続いています）
  1 番に 300 足した
0.5 秒: 残高 [700, 1300]
```

- 振り込みを `tokio::spawn` で別のタスクにし、`timeout` は、引換券（`JoinHandle`）を待つことだけに付けました。時間切れになって捨てられるのは「引換券を待つ Future」だけで、振り込みのタスクは捨てられません（UNIT 02 の例2-4のとおり、引換券を捨ててもタスクは取り消されません）。
- 0.1 秒で待つのをやめた後も、振り込みは続き、0.3 秒で「足した」まで進みました。合計は 700 + 1300 = 2000 のままです。
- 「利用者を長く待たせたくないが、始めた仕事は最後までやり遂げたい」ときの形です。ただし、このタスクは `main` が終わると捨てられます（UNIT 02 の例2-4）。サーバーなら、例6-4の TaskTracker で作っておけば、止めるときに終わりを待てます。

**問3の解答:** 片付けの `sleep` を `Duration::from_secs(3)` に、`timeout` を `Duration::from_secs(1)` に変え、表示の「5 秒」も「1 秒」に直したものを、例6-4と同じように約 1.3 秒後に Ctrl+C を送って実行した結果です。

```text
起動しました。Ctrl+C で止まります
  worker2: 仕事をしました
  worker1: 仕事をしました
  worker2: 仕事をしました
  worker1: 仕事をしました
Ctrl+C を受け取りました。停止を始めます
  worker2: 停止の合図を受け取った。片付けます
  worker1: 停止の合図を受け取った。片付けます
1 秒待っても終わらないタスクがあるので、打ち切ります
```

- 係は片付けに 3 秒かかるので、1 秒の上限に間に合わず、`timeout` が `Err` を返して「打ち切ります」が表示されました。
- 「worker1: 終了」「worker2: 終了」は表示されていません。`main` がそこで終わったので、ランタイムが片付けられ、片付けの途中だった係のタスクも捨てられたからです（UNIT 02 の例2-4）。**打ち切りは、強制終了と同じく、途中の仕事を失います。**
- だから、上限は「ふつうの片付けが十分に終わる長さ」にし、片付けそのものも短く終わるように作ります。どうしても長くかかる仕事（大量のデータの保存など）は、サーバーを止める直前に始めるのではなく、ふだんから少しずつ済ませておくか、UNIT 10 のバックグラウンドジョブのように、止まっても後で続きからやり直せる形にします。

### まとめ

- 外を待つときは `timeout` で上限を付け、`Elapsed` は `map_err` で自分のエラーに変えて返す。時間切れになると、中の Future は `.await` の所で捨てられる（キャンセル）。`.await` の無い長い処理には timeout が効かない。
- 「全部か、何もしないか」にしたい変更の間に `.await` を置かない。途中で止めたくない仕事は `tokio::spawn` で別のタスクにし、待つことだけに上限を付ける。DB の変更は transaction で守る。
- graceful shutdown は「合図を受ける → 受付を止める（`close`）→ 全員に知らせる（`CancellationToken`）→ 上限つきで待つ（`timeout` + `TaskTracker::wait`）→ 間に合わなければ打ち切る」。Topcoat も同じ順番で、既定 30 秒の猶予で止まる。
