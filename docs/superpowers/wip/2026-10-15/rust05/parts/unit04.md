## UNIT 04: channel・backpressure・所有権の移動

### この単元のゴール

- `mpsc`（エムピーエスシー）の channel で、何人もの送り手から1人の受け手へ値を渡せる。値が**持ち主ごと**受け手へ移ることを説明できる
- 送る口（`Sender`）が全部片付くと、受け取りのループが終わるしくみを説明し、`drop(tx)` の書き忘れで処理が終わらなくなる間違いを直せる
- 容量の決まった channel で、受け手より速い送り手を待たせられる（**backpressure**：バックプレッシャー）。混んでいるときに `try_send` ですぐ断れる
- `oneshot` で返事を1回だけ受け取り、「依頼を受ける係」のタスク（アクター）を作れる
- `broadcast`（全員に配る）と `watch`（最新の値だけを共有する）を、目的に合わせて選べる
- E0382（送った値をまた使う）・`SendError`（受け手がいない）・`Lagged`（読み飛ばし）を読んで直せる

### たとえ話でつかむ

channel は、**郵便受け付きの仕事の受付箱**です。

- 送り手は、依頼の紙を箱に入れます（`send`）。紙は箱に入れた瞬間に、送り手の手元からなくなります。コピーが残るわけではありません。これが「**持ち主ごと移る**」です。
- 受け手は、箱から紙を1枚ずつ取り出して処理します（`recv`）。箱が空なら、次の紙が来るまで待ちます。
- 送り手が全員帰って、もう誰も紙を入れに来ないと分かったら、受け手は残りを片付けて帰ります（`recv` が `None` を返す）。だれか1人でも「まだ入れるかもしれない人」が残っていると、受け手はいつまでも待ち続けます。
- 箱には**入る枚数の上限**があります。いっぱいのときに来た送り手は、空きができるまで箱の前で待たされます。これが **backpressure**（背圧：後ろから押し返す力）です。受け手の処理が遅いと、その遅さが送り手に伝わって、仕事が入ってくる速さが自然に抑えられます。

> **用語:** channel（チャネル：タスクの間で値を渡すための通り道。送る口 `Sender` と受け取る口 `Receiver` の2つで1組）

<!-- -->

> **用語:** mpsc（multi-producer, single-consumer：送り手は何人でもよく、受け手は1人の channel）

<!-- -->

> **用語:** backpressure（バックプレッシャー：受け手が追いつかないとき、送り手を待たせたり断ったりして、入ってくる仕事の速さを抑えるしくみ）

### 図で理解する

まず、mpsc の channel です。送る口は `clone` で何人にも配れますが、受け取る口は1つだけです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-041" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="40" width="170" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="99" y="62" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">送り係 A</text>
<text x="99" y="80" font-size="10" fill="#475569" text-anchor="middle">tx.clone() を持つ</text>
<rect x="14" y="110" width="170" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="99" y="132" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">送り係 B</text>
<text x="99" y="150" font-size="10" fill="#475569" text-anchor="middle">tx.clone() を持つ</text>
<rect x="14" y="180" width="170" height="50" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4"/>
<text x="99" y="202" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">main の元の tx</text>
<text x="99" y="220" font-size="10" fill="#475569" text-anchor="middle">使わないので drop(tx)</text>
<rect x="270" y="80" width="200" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="370" y="102" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">箱（容量 8）</text>
<rect x="284" y="118" width="54" height="34" rx="4" fill="#ffffff" stroke="#f97316"/>
<text x="311" y="140" font-size="11" fill="#0f172a" text-anchor="middle">A-1</text>
<rect x="344" y="118" width="54" height="34" rx="4" fill="#ffffff" stroke="#f97316"/>
<text x="371" y="140" font-size="11" fill="#0f172a" text-anchor="middle">B-1</text>
<rect x="404" y="118" width="54" height="34" rx="4" fill="#ffffff" stroke="#f97316"/>
<text x="431" y="140" font-size="11" fill="#0f172a" text-anchor="middle">A-2</text>
<rect x="540" y="95" width="166" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="623" y="120" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">受け取り係</text>
<text x="623" y="140" font-size="10" fill="#475569" text-anchor="middle">rx は1つだけ</text>
<line x1="184" y1="65" x2="266" y2="108" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-041)"/>
<line x1="184" y1="135" x2="266" y2="135" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-041)"/>
<line x1="184" y1="205" x2="266" y2="162" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4"/>
<text x="225" y="58" font-size="10" fill="#475569" text-anchor="middle">send</text>
<text x="225" y="128" font-size="10" fill="#475569" text-anchor="middle">send</text>
<line x1="470" y1="125" x2="536" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-041)"/>
<text x="503" y="116" font-size="10" fill="#475569" text-anchor="middle">recv</text>
<text x="360" y="262" font-size="12" fill="#0f172a" text-anchor="middle">値は send で箱へ、recv で受け取り係へ、持ち主ごと移る（コピーは残らない）</text>
<text x="360" y="284" font-size="12" fill="#b91c1c" text-anchor="middle">送る口（tx）が全部片付くと、recv は None を返し、受け取りのループが終わる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: mpsc の channel。送る口は clone して何人にも配れ、受け取る口は1つ。送る口が全部なくなったら、受け取り係は「もう来ない」と分かって終われる</div>
</div>

- 送る口 `tx`（transmitter）は `clone` で増やせます。どの送り手が送った値も、同じ箱に入ります。
- 受け取る口 `rx`（receiver）は1つだけです。箱に入った順に、1件ずつ受け取ります。
- `main` が最初に作った `tx` も、送る口の1つに数えられます。使わないなら `drop(tx)` で片付けないと、受け取り係は「まだ送ってくる人がいる」と思って待ち続けます（「よくあるエラー4」）。

次は、受け取り係が遅いときに、送り係がどう待たされるかです。容量 2 の箱に、送り係が5件を一気に入れようとし、受け取り係は1件に 0.3 秒かかります（例4-2）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-042" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="120" y="10" width="14" height="12" fill="#fee2e2" stroke="#ef4444"/>
<text x="140" y="21" font-size="11" fill="#0f172a">送り係が待たされる（箱が満杯）</text>
<rect x="330" y="10" width="14" height="12" fill="#e2e8f0" stroke="#94a3b8"/>
<text x="350" y="21" font-size="11" fill="#0f172a">箱の中で順番待ち</text>
<rect x="480" y="10" width="14" height="12" fill="#ffedd5" stroke="#f97316"/>
<text x="500" y="21" font-size="11" fill="#0f172a">受け取り係が処理中</text>
<text x="100" y="55" font-size="12" fill="#0f172a" text-anchor="end">1 番</text>
<text x="100" y="87" font-size="12" fill="#0f172a" text-anchor="end">2 番</text>
<text x="100" y="119" font-size="12" fill="#0f172a" text-anchor="end">3 番</text>
<text x="100" y="151" font-size="12" fill="#0f172a" text-anchor="end">4 番</text>
<text x="100" y="183" font-size="12" fill="#0f172a" text-anchor="end">5 番</text>
<rect x="120" y="40" width="108" height="22" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="120" y="72" width="108" height="22" rx="4" fill="#e2e8f0" stroke="#94a3b8"/>
<rect x="228" y="72" width="108" height="22" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="120" y="104" width="216" height="22" rx="4" fill="#e2e8f0" stroke="#94a3b8"/>
<rect x="336" y="104" width="108" height="22" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="120" y="136" width="108" height="22" rx="4" fill="#fee2e2" stroke="#ef4444"/>
<rect x="228" y="136" width="216" height="22" rx="4" fill="#e2e8f0" stroke="#94a3b8"/>
<rect x="444" y="136" width="108" height="22" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<rect x="228" y="168" width="108" height="22" rx="4" fill="#fee2e2" stroke="#ef4444"/>
<rect x="336" y="168" width="216" height="22" rx="4" fill="#e2e8f0" stroke="#94a3b8"/>
<rect x="552" y="168" width="108" height="22" rx="4" fill="#ffedd5" stroke="#f97316" stroke-width="2"/>
<line x1="120" y1="206" x2="690" y2="206" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-042)"/>
<line x1="120" y1="200" x2="120" y2="212" stroke="#475569" stroke-width="2"/>
<line x1="228" y1="200" x2="228" y2="212" stroke="#475569" stroke-width="2"/>
<line x1="336" y1="200" x2="336" y2="212" stroke="#475569" stroke-width="2"/>
<line x1="444" y1="200" x2="444" y2="212" stroke="#475569" stroke-width="2"/>
<line x1="552" y1="200" x2="552" y2="212" stroke="#475569" stroke-width="2"/>
<line x1="660" y1="200" x2="660" y2="212" stroke="#475569" stroke-width="2"/>
<text x="120" y="228" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="228" y="228" font-size="11" fill="#475569" text-anchor="middle">0.3</text>
<text x="336" y="228" font-size="11" fill="#475569" text-anchor="middle">0.6</text>
<text x="444" y="228" font-size="11" fill="#475569" text-anchor="middle">0.9</text>
<text x="552" y="228" font-size="11" fill="#475569" text-anchor="middle">1.2</text>
<text x="660" y="228" font-size="11" fill="#475569" text-anchor="middle">1.5 秒</text>
<text x="360" y="256" font-size="12" fill="#475569" text-anchor="middle">箱の2件と処理中の1件を超えた分は、送り係が待たされる。仕事は受け取り係の速さでしか入ってこない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 容量 2 の箱で起きる backpressure。4番と5番は、箱に空きができるまで送り係が待たされる</div>
</div>

最後に、tokio にある4種類の channel を並べておきます。この単元で、全部を1回ずつ使います。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-043" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="10" width="340" height="120" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="30" y="34" font-size="13" font-weight="700" fill="#c2410c">mpsc（多 → 1）</text>
<circle cx="60" cy="56" r="8" fill="#3b82f6"/>
<circle cx="60" cy="80" r="8" fill="#3b82f6"/>
<rect x="130" y="56" width="60" height="24" rx="4" fill="#ffffff" stroke="#f97316"/>
<circle cx="270" cy="68" r="8" fill="#10b981"/>
<line x1="68" y1="58" x2="126" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="68" y1="80" x2="126" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="190" y1="68" x2="258" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<text x="30" y="108" font-size="11" fill="#0f172a">送り手は何人でも、受け手は1人。仕事の列に使う</text>
<text x="30" y="122" font-size="10" fill="#475569">例4-1・例4-2・例4-3</text>
<rect x="366" y="10" width="340" height="120" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="382" y="34" font-size="13" font-weight="700" fill="#1e40af">oneshot（1回だけ）</text>
<circle cx="412" cy="68" r="8" fill="#3b82f6"/>
<rect x="482" y="56" width="60" height="24" rx="4" fill="#ffffff" stroke="#3b82f6"/>
<circle cx="622" cy="68" r="8" fill="#10b981"/>
<line x1="420" y1="68" x2="478" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="542" y1="68" x2="610" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<text x="382" y="108" font-size="11" fill="#0f172a">値を1回だけ届ける封筒。依頼への返事に使う</text>
<text x="382" y="122" font-size="10" fill="#475569">例4-4</text>
<rect x="14" y="140" width="340" height="120" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="30" y="164" font-size="13" font-weight="700" fill="#047857">broadcast（1 → 全員）</text>
<circle cx="60" cy="198" r="8" fill="#3b82f6"/>
<rect x="130" y="186" width="60" height="24" rx="4" fill="#ffffff" stroke="#10b981"/>
<circle cx="270" cy="178" r="7" fill="#10b981"/>
<circle cx="270" cy="198" r="7" fill="#10b981"/>
<circle cx="270" cy="218" r="7" fill="#10b981"/>
<line x1="68" y1="198" x2="126" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="190" y1="196" x2="259" y2="180" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="190" y1="198" x2="259" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="190" y1="200" x2="259" y2="216" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<text x="30" y="242" font-size="11" fill="#0f172a">送った1件を、受け手の全員が受け取る。お知らせに使う</text>
<text x="30" y="256" font-size="10" fill="#475569">例4-5（遅れすぎた受け手は読み飛ばす: エラー3）</text>
<rect x="366" y="140" width="340" height="120" rx="10" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="382" y="164" font-size="13" font-weight="700" fill="#6d28d9">watch（最新の値だけ）</text>
<circle cx="412" cy="198" r="8" fill="#3b82f6"/>
<rect x="482" y="186" width="60" height="24" rx="4" fill="#ffffff" stroke="#8b5cf6"/>
<text x="512" y="203" font-size="10" fill="#6d28d9" text-anchor="middle">最新</text>
<circle cx="622" cy="182" r="7" fill="#10b981"/>
<circle cx="622" cy="214" r="7" fill="#10b981"/>
<line x1="420" y1="198" x2="478" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="542" y1="196" x2="611" y2="184" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<line x1="542" y1="200" x2="611" y2="212" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-043)"/>
<text x="382" y="242" font-size="11" fill="#0f172a">途中の値は飛ばしてよい。設定や「止めて」の合図に使う</text>
<text x="382" y="256" font-size="10" fill="#475569">例4-5・練習問題3</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: tokio の4種類の channel。仕事の列は mpsc、1回の返事は oneshot、全員へのお知らせは broadcast、最新の状態の共有は watch を使う</div>
</div>

### まず動かす最小コード

#### 例4-1: 2人の送り係から、1人の受け取り係へ依頼を渡す

> **▼ このコードがやること（先に日本語で）:** 「このノートを保存して」という依頼 `SaveRequest` を運ぶ channel を作ります。送り係のタスクを2つ（A と B）作り、それぞれ2件ずつ依頼を送ります。受け取り係のタスクは、箱から1件ずつ受け取って表示し、送る口が全部閉じたら、受け取った件数を返して終わります。

```rust
use tokio::sync::mpsc;

// 送るもの: 「このノートを保存して」という依頼
struct SaveRequest {
    title: String,
}

#[tokio::main]
async fn main() {
    // 箱に入るのは最大 8 件（bounded = 上限つき）。tx が送る口、rx が受け取る口
    let (tx, mut rx) = mpsc::channel::<SaveRequest>(8);

    // 受け取る側のタスク（1つだけ）
    let worker = tokio::spawn(async move {
        let mut saved = 0;
        // recv は「次の1件」を待つ。送る口が全部片付けられると None が返ってループが終わる
        while let Some(request) = rx.recv().await {
            println!("  受け取り: {}", request.title);
            saved += 1;
        }
        println!("  送る口がすべて閉じたので終わります");
        saved
    });

    // 送る側のタスクを2つ作る。送る口は clone して1つずつ渡す
    let mut senders = Vec::new();
    for who in ["A", "B"] {
        let tx = tx.clone();
        senders.push(tokio::spawn(async move {
            for n in 1..=2 {
                let title = format!("{who}-{n}");
                // send に渡した値は、受け取る側へ「持ち主ごと」移る
                tx.send(SaveRequest { title }).await.unwrap();
            }
        }));
    }
    // main が持っている元の送る口も片付ける（忘れると recv が永遠に待つ）
    drop(tx);

    for sender in senders {
        sender.await.unwrap();
    }
    let saved = worker.await.unwrap();
    println!("保存した件数: {saved}");
}
```

`src/bin/u04_mpsc.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_mpsc
```

**▼ 実行結果**

```text
  受け取り: B-1
  受け取り: A-1
  受け取り: A-2
  受け取り: B-2
  送る口がすべて閉じたので終わります
保存した件数: 4
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: mpsc::channel(容量) で、送る口と受け取る口を1組作る

- `mpsc::channel::<SaveRequest>(8)` は、「`SaveRequest` を最大8件まで入れておける箱」を作り、送る口 `tx` と受け取る口 `rx` を組で返します。`::<SaveRequest>` は運ぶ値の型の指定です。
- 容量の決まった channel を **bounded**（上限つき）と呼びます。上限の無い `mpsc::unbounded_channel` もありますが、送り係が待たされないので、受け取り係が遅いと箱が際限なく大きくなり、メモリを使い果たすおそれがあります。この巻では、上限つきを使います。

##### 解説2: 送る口は clone して配り、受け取る口は1つだけ

- `let tx = tx.clone();` で送る口を増やし、`async move` でそれぞれの送り係のタスクへ移しています（UNIT 02 の練習問題1と同じ形）。
- 受け取る口 `rx` は `clone` できません。受け取り係のタスクに `async move` で移して、そのタスクだけが受け取ります。
- 実行結果の順番（B-1 → A-1 → A-2 → B-2）は、2つの送り係のどちらが先に動いたかで決まり、実行するたびに変わることがあります。ただし、同じ送り係が送った A-1 と A-2 の順番は、必ず守られます。

##### 解説3: send した値は、持ち主ごと受け取り係へ移る

- `tx.send(SaveRequest { title })` に渡した値は、箱を通って受け取り係へ**移動**します。送った後の送り係の手元には、もう残っていません（送った後に使おうとすると E0382: 「よくあるエラー1」）。
- 受け取り係は、受け取った `request` の持ち主になります。だから、自由に読んだり書き換えたり、別の場所へ渡したりできます。コピーではないので、大きな値でも送るのは速く、2つのタスクが同じ値を同時にさわることもありません。
- `send` は、箱に空きがあればすぐに終わり、いっぱいなら空くまで待つ async 関数です（例4-2）。受け取り係がもういなければ `Err` を返します（「よくあるエラー2」）。

##### 解説4: 送る口が全部片付くと、recv が None を返す

- `rx.recv().await` は、次の1件が来るまで待ち、来たら `Some(値)` を返します。
- **送る口が1つ残らず片付けられ**、箱も空になると、`None` を返します。`while let Some(request) = ...` のループは、これで自然に終わります。
- 送る口は3つありました。送り係 A と B の `tx` は、それぞれのタスクが終わるときに片付けられます。`main` の元の `tx` は、`drop(tx)` で明示的に片付けています。

### もう一歩: 実務に近い形

#### 例4-2: 受け取り係が遅いと、送り係が待たされる（backpressure）

> **▼ このコードがやること（先に日本語で）:** 容量 2 の箱を作ります。受け取り係は1件の処理に 0.3 秒かかります。送り係は5件を一気に送ろうとします。送り係が箱に入れられた時刻と、受け取り係が処理を終えた時刻を表示して、送り係がどこで待たされたかを確かめます。

```rust
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 箱の容量は 2 件だけ
    let (tx, mut rx) = mpsc::channel::<u32>(2);

    // 遅い受け取り係: 1件の処理に 300ms かかる
    let consumer = tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            tokio::time::sleep(Duration::from_millis(300)).await;
            println!(
                "{:.1} 秒:   受け取り係 {job} 番を処理した",
                start.elapsed().as_secs_f64()
            );
        }
    });

    // 速い送り係: 5件を一気に送ろうとする
    for job in 1..=5 {
        // 箱がいっぱいなら、空きができるまでここで待たされる（= バックプレッシャー）
        tx.send(job).await.unwrap();
        println!(
            "{:.1} 秒: 送り係 {job} 番を箱に入れた",
            start.elapsed().as_secs_f64()
        );
    }
    drop(tx);
    consumer.await.unwrap();
}
```

`src/bin/u04_backpressure.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_backpressure
```

**▼ 実行結果**

```text
0.0 秒: 送り係 1 番を箱に入れた
0.0 秒: 送り係 2 番を箱に入れた
0.0 秒: 送り係 3 番を箱に入れた
0.3 秒:   受け取り係 1 番を処理した
0.3 秒: 送り係 4 番を箱に入れた
0.6 秒:   受け取り係 2 番を処理した
0.6 秒: 送り係 5 番を箱に入れた
0.9 秒:   受け取り係 3 番を処理した
1.2 秒:   受け取り係 4 番を処理した
1.5 秒:   受け取り係 5 番を処理した
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 容量 2 なのに、0 秒で3件入ったのはなぜか

- 1番は、箱に入った直後に受け取り係が `recv` で取り出し、処理を始めました。箱の中には残っていません。
- その後の2番と3番で、箱の2件分がいっぱいになります。だから 0 秒の時点で入れられたのは3件（処理中の1件と、箱の中の2件）です。図の「1番」から「3番」の行のとおりです。

##### 解説2: 4番と5番は、箱に空きができるまで待たされた

- 4番の `tx.send(4).await` は、箱がいっぱいなので、そこで止まって待ちます。0.3 秒に受け取り係が1番を終えて2番を取り出すと、箱に空きが1つでき、4番が入りました。5番も同じように 0.6 秒まで待たされています。
- 送り係は、**受け取り係の速さ（0.3 秒に1件）でしか先に進めなくなりました**。これが backpressure です。箱の中に貯まる仕事は、いつも2件までです。

##### 解説3: もし上限が無かったら

- 上限の無い channel では、送り係は待たされずに5件とも 0 秒で入れ終わり、箱の中に4件が貯まります。5件なら問題ありませんが、1秒に1000件の依頼が来て、受け取り係が1秒に3件しか処理できなければ、箱の中身は1秒に997件ずつ増え続けます。いつかメモリが足りなくなり、サーバーごと落ちます。
- 上限があれば、送り係の側（たとえば HTTP のリクエストを受けているタスク）が待たされるので、それ以上の仕事を受け付けられなくなります。「待たせる」代わりに「すぐ断る」こともできます（次の例4-3）。

#### 例4-3: 混んでいるときは、待たずにすぐ断る（try_send）

> **▼ このコードがやること（先に日本語で）:** 容量 2 の箱に、受け取り係が動き出す前に5件の依頼が一気に来たとします。`try_send` で入れようとし、箱がいっぱいなら待たずに「混んでいるので断りました」と表示します。最後に、受け付けた分だけを処理します。

```rust
use tokio::sync::mpsc::{self, error::TrySendError};

#[tokio::main]
async fn main() {
    // 箱の容量は 2 件
    let (tx, mut rx) = mpsc::channel::<u32>(2);

    // 受け取り係が動き出す前に、5 件の依頼が一気に来たとする
    for job in 1..=5 {
        // try_send は待たない。箱がいっぱいなら、すぐに Full のエラーを返す
        match tx.try_send(job) {
            Ok(()) => println!("{job} 番: 受け付けました"),
            Err(TrySendError::Full(job)) => {
                println!("{job} 番: 混んでいるので断りました（503 を返す）");
            }
            Err(TrySendError::Closed(job)) => println!("{job} 番: 受け取り係がいません"),
        }
    }
    drop(tx);

    // 受け付けた分だけを処理する
    while let Some(job) = rx.recv().await {
        println!("受け取り係: {job} 番を処理しました");
    }
}
```

`src/bin/u04_try_send.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_try_send
```

**▼ 実行結果**

```text
1 番: 受け付けました
2 番: 受け付けました
3 番: 混んでいるので断りました（503 を返す）
4 番: 混んでいるので断りました（503 を返す）
5 番: 混んでいるので断りました（503 を返す）
受け取り係: 1 番を処理しました
受け取り係: 2 番を処理しました
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: try_send は async ではなく、すぐに結果を返す

- `tx.try_send(値)` は `.await` を付けません。箱に空きがあれば入れて `Ok(())`、いっぱいなら待たずに `Err(TrySendError::Full(値))` を返します。受け取り係がもういなければ `Err(TrySendError::Closed(値))` です。
- どちらのエラーも、入れられなかった値を**返してくれます**（`Full(job)` の `job`）。値は持ち主ごと移る約束なので、失敗したときに値が消えてしまわないよう、送り手に戻ってくるのです。

##### 解説2: 「待たせる」か「断る」かは、場面で選ぶ

- Web サーバーでは、混雑したときに利用者を何十秒も待たせるより、すぐに「503 Service Unavailable（今は混んでいます）」を返して、少し後でやり直してもらうほうが親切なことがあります。`try_send` の `Full` は、その判断のきっかけになります。
- 一方、裏で動く集計のように、少しくらい遅れてもかまわない仕事なら、`send(...).await` で待たせて、確実に受け付けるほうが向いています。

> **注意:** `select!` の枝で `tx.send(値)` を使い、ほかの枝が先に終わると、送るはずだった値は**捨てられて失われます**（tokio 1.53.3 のソースに「the message is dropped and will be lost」と書かれています）。UNIT 03 のキャンセル安全の話です。一方、`rx.recv()` はキャンセル安全で、負けても箱の中の値は失われません。

#### 例4-4: 返事は oneshot で受け取る（アクターの形）

> **▼ このコードがやること（先に日本語で）:** 合計の数字を1つだけ持つ「カウンター係」のタスクを作ります。ほかのタスクは、mpsc の channel で「足して」「今の合計を教えて」と依頼を送ります。依頼には、返事を入れてもらう封筒（`oneshot` の送る口）を同封します。10・20・30 を順に足し、最後に合計を聞きます。

```rust
use tokio::sync::{mpsc, oneshot};

// 係（アクター）への依頼。返事を入れてもらう封筒（oneshot の送る口）を同封する
enum Command {
    Add {
        amount: u64,
        reply: oneshot::Sender<u64>,
    },
    Get {
        reply: oneshot::Sender<u64>,
    },
}

// カウンター係: 数字はこのタスクだけが持つので、Mutex がいらない
async fn counter_actor(mut rx: mpsc::Receiver<Command>) {
    let mut total = 0;
    while let Some(command) = rx.recv().await {
        match command {
            Command::Add { amount, reply } => {
                total += amount;
                // 返事は1回だけ送れる。依頼した側がもういなければ Err になるが、無視してよい
                let _ = reply.send(total);
            }
            Command::Get { reply } => {
                let _ = reply.send(total);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(16);
    tokio::spawn(counter_actor(rx));

    for amount in [10, 20, 30] {
        let (reply_tx, reply_rx) = oneshot::channel();
        tx.send(Command::Add {
            amount,
            reply: reply_tx,
        })
        .await
        .unwrap();
        // 返事が届くまで待つ
        let total = reply_rx.await.unwrap();
        println!("{amount} を足した → 合計 {total}");
    }

    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(Command::Get { reply: reply_tx }).await.unwrap();
    println!("最終的な合計: {}", reply_rx.await.unwrap());
}
```

`src/bin/u04_oneshot.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_oneshot
```

**▼ 実行結果**

```text
10 を足した → 合計 10
20 を足した → 合計 30
30 を足した → 合計 60
最終的な合計: 60
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-044" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="40" width="200" height="150" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="114" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">依頼する側（main）</text>
<text x="114" y="96" font-size="11" fill="#0f172a" text-anchor="middle">① oneshot::channel() で</text>
<text x="114" y="114" font-size="11" fill="#0f172a" text-anchor="middle">封筒を1組作る</text>
<text x="114" y="150" font-size="11" fill="#0f172a" text-anchor="middle">③ reply_rx.await で</text>
<text x="114" y="168" font-size="11" fill="#0f172a" text-anchor="middle">返事を待つ</text>
<rect x="474" y="40" width="232" height="150" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">カウンター係（アクター）</text>
<text x="590" y="96" font-size="11" fill="#0f172a" text-anchor="middle">total はこの係だけが持つ</text>
<text x="590" y="114" font-size="11" fill="#0f172a" text-anchor="middle">→ Mutex がいらない</text>
<text x="590" y="150" font-size="11" fill="#0f172a" text-anchor="middle">依頼を1つずつ順番に処理</text>
<line x1="214" y1="100" x2="470" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-044)"/>
<text x="342" y="88" font-size="11" fill="#0f172a" text-anchor="middle">② mpsc: Command::Add { amount, reply }</text>
<text x="342" y="118" font-size="10" fill="#475569" text-anchor="middle">返事用の封筒（reply）を同封</text>
<line x1="474" y1="156" x2="218" y2="156" stroke="#10b981" stroke-width="2" marker-end="url(#ar-05-044)"/>
<text x="346" y="146" font-size="11" fill="#047857" text-anchor="middle">④ oneshot: reply.send(total)</text>
<text x="346" y="174" font-size="10" fill="#475569" text-anchor="middle">封筒で1回だけ返事する</text>
<text x="360" y="222" font-size="12" fill="#475569" text-anchor="middle">依頼は mpsc の列に並ぶので、何人から同時に来ても、係は1つずつ順番に処理できる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: アクターの形。依頼は mpsc で係に届け、返事は依頼に同封した oneshot の封筒で受け取る。データは係のタスクだけが持つ</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: oneshot は、値を1回だけ届ける封筒

- `oneshot::channel()` は、値を**1回だけ**運べる channel の組（送る口と受け取る口）を作ります。送る口の `send` は1回しか呼べません（`send` が送る口を持ち主ごと受け取るので、2回目は書けない作りになっています）。
- 受け取る口 `reply_rx` は、それ自体を `.await` すると、返事が届くまで待って `Result` を返します。係が返事をせずに封筒（送る口）を捨てたら `Err` です。

##### 解説2: 依頼に封筒を同封すると、返事の行き先を迷わない

- `Command::Add { amount, reply: reply_tx }` のように、依頼そのものに返事用の送る口を入れて送ります。係は、受け取った依頼の `reply` に返事を入れるだけで、正しい依頼主に届きます。何人が同時に依頼しても、返事が入れ違うことはありません。
- `let _ = reply.send(total);` の `let _ =` は、「依頼主がもう待っていなくて送れなかった（`Err`）としても、気にしない」という意味です。

##### 解説3: アクターなら、共有のデータに鍵がいらない

- 合計 `total` を持っているのは、カウンター係のタスクだけです。ほかのタスクは、直接さわらずに、依頼を送って係にやってもらいます。依頼は channel の列に並ぶので、係は1つずつ順番に処理します。
- だから、第3巻 UNIT 10 で使った `Arc<Mutex<…>>` の鍵が要りません。「データを持つ係を1人決めて、メッセージで頼む」このやり方を**アクター**（actor）と呼びます。共有のデータを守るもう1つの方法（鍵を使う方法）は、UNIT 05 で学びます。

> **用語:** アクター（actor：自分だけのデータを持ち、メッセージで届いた依頼を1つずつ処理するタスク。データを共有せずに済むので、鍵がいらない）

#### 例4-5: 全員に配る broadcast と、最新の値だけを共有する watch

> **▼ このコードがやること（先に日本語で）:** 前半では、broadcast の channel で「ノートが作られました」「削除されました」の2つのお知らせを送り、メール係とログ係の**両方**が受け取ることを確かめます。後半では、watch の channel で「ログの細かさ」の設定を `info` → `debug` → `warn` → `error` と変え、見張り役のタスクが受け取った値を表示します。

```rust
use std::time::Duration;
use tokio::sync::{broadcast, watch};

#[tokio::main]
async fn main() {
    // ---- broadcast: 送った1件を、受け取り手「全員」が受け取る ----
    let (news_tx, _) = broadcast::channel::<String>(16);
    let mut handles = Vec::new();
    for name in ["メール係", "ログ係"] {
        // 受け取り口は subscribe で増やす（送る前に増やしておく）
        let mut news_rx = news_tx.subscribe();
        handles.push(tokio::spawn(async move {
            while let Ok(news) = news_rx.recv().await {
                println!("  {name} が受け取り: {news}");
            }
            println!("  {name}: 送り手がいなくなったので終わり");
        }));
    }
    news_tx
        .send(String::from("ノート 1 が作られました"))
        .unwrap();
    news_tx
        .send(String::from("ノート 1 が削除されました"))
        .unwrap();
    drop(news_tx);
    for handle in handles {
        handle.await.unwrap();
    }

    // ---- watch: 「最新の値」だけを共有する。途中の値は飛ばされてもよい ----
    let (level_tx, mut level_rx) = watch::channel("info");
    let watcher = tokio::spawn(async move {
        // changed: 値が変わるまで待つ。送り手がいなくなると Err
        while level_rx.changed().await.is_ok() {
            println!("  ログの細かさが変わった: {}", *level_rx.borrow());
        }
    });
    level_tx.send("debug").unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    // 立て続けに2回変えると、見張り役が見るのは最新の "error" だけのことが多い
    level_tx.send("warn").unwrap();
    level_tx.send("error").unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    drop(level_tx);
    watcher.await.unwrap();
}
```

`src/bin/u04_broadcast_watch.rs` として保存して実行します。

```powershell
cargo run -q --bin u04_broadcast_watch
```

**▼ 実行結果**

```text
  メール係 が受け取り: ノート 1 が作られました
  メール係 が受け取り: ノート 1 が削除されました
  ログ係 が受け取り: ノート 1 が作られました
  メール係: 送り手がいなくなったので終わり
  ログ係 が受け取り: ノート 1 が削除されました
  ログ係: 送り手がいなくなったので終わり
  ログの細かさが変わった: debug
  ログの細かさが変わった: error
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: broadcast は、受け取り口ごとに全部のお知らせが届く

- `broadcast::channel::<String>(16)` は、最大16件まで覚えておけるお知らせの channel です。受け取り口は `news_tx.subscribe()` で、受け手の数だけ作ります。最初に返ってくる受け取り口（`_`）は使わないので、すぐ捨てています。
- メール係もログ係も、2件のお知らせを**両方とも**受け取りました。mpsc では1件は1人にしか届きませんが、broadcast では全員に届きます。そのため、運ぶ値は `Clone` できる型でなければなりません（受け手ごとに写しが渡されます）。
- `subscribe` で作った受け取り口には、**作った後に**送られたお知らせだけが届きます。お知らせを送る前に、受け取り口を作っておく必要があります。
- 2つの係の表示が混ざっているのは、2つのタスクが別々に動いているからです。それぞれの係の中では、送った順番（作られた → 削除された）が守られています。

##### 解説2: watch は「今の値」だけを覚えている

- `watch::channel("info")` は、最初の値 `"info"` を持った channel を作ります。`level_tx.send("debug")` で値を書き換えると、見張り役の `level_rx.changed().await` が「変わったよ」と知らせてくれます。`*level_rx.borrow()` で今の値を読みます。
- `"warn"` と `"error"` を立て続けに送ったので、見張り役が起きたときには、値はもう `"error"` になっていました。`"warn"` は表示されていません。watch は「途中の値は飛ばしてもよいから、最新の値を知りたい」ときのための channel です。ログの細かさのような設定や、「止めて」という合図（練習問題3）に向いています。

> **注意:** `level_rx.borrow()` が返すものを持ったまま `.await` しないようにしましょう。`borrow` は、値を読んでいる間、中で読み込み用の鍵をかけています。持ったまま待つと、送り手が値を書き換えられずに待たされます。UNIT 05 の「鍵を持ったまま `.await` しない」と同じ注意です。

### よくあるエラーと読み方

#### エラー1: 送った値を、もう一度使う（E0382）

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8);
    let title = String::from("買い物メモ");
    tx.send(title).await.unwrap();
    // 送ったあとで、もう一度 title を使おうとする
    println!("送ったもの: {title}");
    println!("受け取ったもの: {:?}", rx.recv().await);
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0382]: borrow of moved value: `title`
 --> src\bin\e04_send_moved.rs:9:23
  |
6 |     let title = String::from("買い物メモ");
  |         ----- move occurs because `title` has type `String`, which does not implement the `Copy` trait
7 |     tx.send(title).await.unwrap();
  |             ----- value moved here
8 |     // 送ったあとで、もう一度 title を使おうとする
9 |     println!("送ったもの: {title}");
  |                            ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |     tx.send(title.clone()).await.unwrap();
  |                  ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `async_lab` (bin "e04_send_moved") due to 1 previous error
```

- **何が起きたか:** 「移動した後の値 `title` を借りようとしています」というエラーです。`value moved here` の線が、`tx.send(title)` で値が箱へ移ったことを示しています。第3巻 UNIT 02 で学んだ「移動」が、channel でもそのまま起きています。
- **直し方:** 送った後にも使いたいなら、`help:` のとおり `tx.send(title.clone())` で写しを送ります。ただ、多くの場合は、**送る前に**表示などを済ませて、送った後は使わないように順番を入れ替えるのが自然です。「送ったら、もう自分の物ではない」と考えましょう。

#### エラー2: 受け取り係がいない channel に送る（SendError: channel closed）

コンパイルは通り、実行すると panic する例です。

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<String>(8);
    // 受け取る側が先にいなくなった（たとえば受け取り係のタスクが panic して終わった）
    drop(rx);

    // ① エラーを確かめる書き方
    match tx.send(String::from("ノート1")).await {
        Ok(()) => println!("送れました"),
        Err(error) => println!("送れません: {error} / 戻ってきた値: {}", error.0),
    }
    // ② unwrap してしまう書き方
    tx.send(String::from("ノート2")).await.unwrap();
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target\debug\e04_send_closed.exe`
送れません: channel closed / 戻ってきた値: ノート1

thread 'main' (25592) panicked at src\bin\e04_send_closed.rs:15:44:
called `Result::unwrap()` on an `Err` value: SendError { .. }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\e04_send_closed.exe` (exit code: 101)
```

- **何が起きたか:** 受け取る口 `rx` が片付けられた後なので、`send` は `Err(SendError)` を返しました。①では `channel closed`（channel は閉じています）と表示し、送れなかった値（`error.0` の「ノート1」）も戻ってきています。②では、その `Err` を `unwrap` したので panic しました。
- **直し方:** 受け取り係のタスクは、panic などで先に終わることがあります。`send` の結果は `unwrap` せず、`Err` なら「受け取り係がいない」として扱います。サーバーなら、ログに残して 500（サーバーの中の問題）を返すのがふつうです。
- 例4-1の `unwrap` は、受け取り係が送り手より先に終わらない作りなので、練習のために省略していました。実際のアプリでは、`expect("…")` で理由を書くか、エラーとして返しましょう。

#### エラー3: broadcast で、遅れた受け手が読み飛ばす（RecvError::Lagged）

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // 容量 2 の broadcast
    let (tx, mut rx) = broadcast::channel::<u32>(2);
    // 受け取り手が読む前に 5 件送ってしまう
    for n in 1..=5 {
        tx.send(n).unwrap();
    }
    // 読み始める
    loop {
        match rx.recv().await {
            Ok(n) => println!("受け取り: {n}"),
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                println!("追いつけず {skipped} 件を読み飛ばしました");
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
        // 箱の中に読むものが残っていなければ終わる
        if rx.is_empty() {
            break;
        }
    }
}
```

```text
追いつけず 3 件を読み飛ばしました
受け取り: 4
受け取り: 5
```

（`cargo run` の `Compiling ...` などの行は省いています。）

- **何が起きたか:** broadcast は、mpsc と違って送り手を待たせません。容量（2件）を超えて送ると、**古いお知らせから上書きして**消していきます。遅れていた受け手が次に `recv` すると、まず `Err(RecvError::Lagged(3))`（3件を読み飛ばしました）が返り、その後で残っている新しい2件（4 と 5）を受け取れます。
- 1件の送り手のせいで全員が止まらないように、遅れた受け手のほうが読み飛ばす作りになっているのです。
- **直し方:** 読み飛ばしてもよいお知らせなら、`Lagged` を受け取ったらログに残して続けます。1件も落としてはいけない仕事なら、broadcast ではなく、受け手ごとに mpsc の channel を用意するなど、別の作りにします。容量を大きくすると、読み飛ばしは起きにくくなります。

#### エラー4（エラーが出ない間違い）: drop(tx) を忘れて、受け取り係が終わらない

```rust
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8);

    let worker = tokio::spawn(async move {
        let mut saved = 0;
        while let Some(title) = rx.recv().await {
            println!("  受け取り: {title}");
            saved += 1;
        }
        saved
    });

    let sender = {
        let tx = tx.clone();
        tokio::spawn(async move {
            for n in 1..=2 {
                tx.send(format!("A-{n}")).await.unwrap();
            }
        })
    };
    sender.await.unwrap();
    // drop(tx) を書き忘れた。main の tx が残っているので、箱は閉じない

    // 受け取り係の終わりを、1 秒だけ待ってみる
    tokio::select! {
        saved = worker => println!("保存した件数: {}", saved.unwrap()),
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
            println!("1 秒待っても、受け取り係が終わりません");
        }
    }
}
```

`src/bin/u04_forgot_drop.rs` として実行した結果です（そのままだと永遠に終わらないので、UNIT 03 の `select!` で1秒だけ待つようにしています）。

```text
  受け取り: A-1
  受け取り: A-2
1 秒待っても、受け取り係が終わりません
```

- **何が起きたか:** 2件は受け取れましたが、受け取り係は終わりません。送り係のタスクの `tx` は片付けられましたが、`main` が最初に作った `tx` が残っているので、受け取り係の `recv` は「まだ送ってくるかもしれない」と待ち続けています。エラーも panic も出ず、プログラムが止まったように見えます。
- **直し方:** 送り係に `clone` を配り終えたら、`main` の元の `tx` を `drop(tx)` で片付けます（例4-1）。あるいは、`clone` せずに元の `tx` をそのまま最後の送り係へ移す書き方もあります。
- `select!` の枝に `worker`（`JoinHandle`）をそのまま書けたのは、`JoinHandle` が `Unpin` な Future だからです（UNIT 03 のエラー2のような `pin!` は要りません）。

> **ヒント:** 「受け取りのループが終わらない」ときは、まず「送る口（`tx`）をまだ持っている人はだれか？」を数えましょう。`clone` した数と、片付けた数が合っていないことがほとんどです。

### RustNote ではここで使う

RustNote のコードそのものには、channel は出てきません。1つのリクエストの中で、ほかのタスクへ仕事を渡す場面がまだ無いからです（UNIT 10 のバックグラウンドジョブで、仕事を渡すときに使います）。

一方、RustNote を動かしている Topcoat の `internal_serve`（`topcoat-router-0.10.0/src/service.rs`）では、watch の channel が「止める合図」として使われています。Topcoat 0.10.0 のソースから、channel を作る部分と、合図を出す部分を抜き出します（コメントは日本語に置き換え、関係の無い部分は省略しています）。

```rust
// どの channel も「値を送る」のではなく「閉じる」ことで合図する。
// 接続のタスクは受け取り口の写しを持ち、送り口が片付くと changed() が終わる。
// 止め始めの合図
let (drain_tx, drain_rx) = watch::channel(());
// 猶予の終わりの合図
let (cutoff_tx, cutoff_rx) = watch::channel(());
// 生きているタスクを数える。各タスクが受け取り口の写しを持ち、
// 最後の1つが片付くと done_tx.closed() が終わる
let (done_tx, done_rx) = watch::channel(());

// （接続を受け付けるループ。接続ごとに drain_rx などの clone をタスクへ渡す。省略）

// 止め始めの合図を出す: 送り口を片付ける
drop(drain_rx);
drop(drain_tx);
drop(done_rx);
// 全部のタスクが終わるか、猶予の時間が過ぎるまで待つ
tokio::select! {
    () = done_tx.closed() => {}
    () = tokio::time::sleep(service.shutdown_timeout) => {}
}
```

- **送り口を片付けることが合図になっています。** 例4-1で、送る口が全部片付くと `recv` が `None` を返したのと同じく、watch でも送り口が片付くと、受け取り口の `changed()` が `Err` を返して終わります。値（ここでは中身の無い `()`）を送る必要さえありません。
- **受け取り口の数で、生きているタスクを数えています。** 接続のタスクは `done_rx` の写しを1つずつ持ち、タスクが終わると写しが片付きます。最後の1つが片付くと、`done_tx.closed()` が終わります。「全部の接続が終わるまで待つ」が、channel の受け取り口の数だけで書けています。
- 猶予の時間（`shutdown_timeout`）と `select!` で競争させて、待ちすぎないようにしています。この止め方の全体は、UNIT 06 の graceful shutdown で読みます。

### 練習問題

**問1:** 例4-2の `u04_backpressure.rs` で、箱の容量を `2` から `5` に変えると、送り係が箱に入れる時刻と、受け取り係が最後の5番を処理し終える時刻はどう変わるでしょうか。予想してから実行してください（`src/bin/u04_q1.rs`）。

**問2:** 例4-4のカウンター係に、「合計を 0 に戻し、戻す前の合計を返事する」依頼 `Reset` を足してください。10 を足す → 20 を足す → リセット → 5 を足す、の順に依頼し、それぞれの返事を表示します（`src/bin/u04_q2.rs`）。返事の封筒を作って送って待つ、の3行は、毎回同じなので関数にまとめてみましょう。

**問3:** `watch::channel(false)` を「止める合図」に使います。仕事係のタスクは、0.1 秒ごとに「仕事 N 回目」と表示し、合図の値が `true` になったら止まって、仕事をした回数を返します。`main` は 0.35 秒たったら合図を送り、仕事係が止まるのを待ってください。UNIT 03 の `select!` を使います（`src/bin/u04_q3.rs`）。

### 解答と解説

**問1の解答:** `mpsc::channel::<u32>(5)` に変えて実行した結果です（コメントも「箱の容量を 5 件に増やす」に直しました。ほかは例4-2と同じです）。

```text
0.0 秒: 送り係 1 番を箱に入れた
0.0 秒: 送り係 2 番を箱に入れた
0.0 秒: 送り係 3 番を箱に入れた
0.0 秒: 送り係 4 番を箱に入れた
0.0 秒: 送り係 5 番を箱に入れた
0.3 秒:   受け取り係 1 番を処理した
0.6 秒:   受け取り係 2 番を処理した
0.9 秒:   受け取り係 3 番を処理した
1.2 秒:   受け取り係 4 番を処理した
1.6 秒:   受け取り係 5 番を処理した
```

- 箱に5件とも入るので、送り係は待たされずに 0 秒で全部を入れ終わりました。
- それでも、受け取り係が最後の5番を終えるのは、容量 2 のときと同じ約 1.5 秒です（1.6 秒になっているのは、タイマーの遅れが積み重なったためです）。**容量を大きくしても、全体の処理は速くなりません。** 変わるのは「送り係が待つか」と「箱の中に何件まで貯まるか（メモリ）」だけです。全体を速くしたいなら、受け取り係を速くするか、増やす必要があります。

**問2の解答:**

```rust
use tokio::sync::{mpsc, oneshot};

// 係（アクター）への依頼
enum Command {
    Add {
        amount: u64,
        reply: oneshot::Sender<u64>,
    },
    // 合計を 0 に戻し、戻す前の合計を返事する
    Reset {
        reply: oneshot::Sender<u64>,
    },
}

async fn counter_actor(mut rx: mpsc::Receiver<Command>) {
    let mut total = 0;
    while let Some(command) = rx.recv().await {
        match command {
            Command::Add { amount, reply } => {
                total += amount;
                let _ = reply.send(total);
            }
            Command::Reset { reply } => {
                let old = total;
                total = 0;
                let _ = reply.send(old);
            }
        }
    }
}

// 依頼を送り、返事を待つ。返事の封筒は、依頼を作る関数 make で中に入れてもらう
async fn ask(
    tx: &mpsc::Sender<Command>,
    make: impl FnOnce(oneshot::Sender<u64>) -> Command,
) -> u64 {
    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(make(reply_tx)).await.expect("係がいません");
    reply_rx.await.expect("係が返事をしませんでした")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(16);
    tokio::spawn(counter_actor(rx));

    let total = ask(&tx, |reply| Command::Add { amount: 10, reply }).await;
    println!("10 を足した → 合計 {total}");
    let total = ask(&tx, |reply| Command::Add { amount: 20, reply }).await;
    println!("20 を足した → 合計 {total}");
    let old = ask(&tx, |reply| Command::Reset { reply }).await;
    println!("リセットした（それまでの合計 {old}）");
    let total = ask(&tx, |reply| Command::Add { amount: 5, reply }).await;
    println!("5 を足した → 合計 {total}");
}
```

```text
10 を足した → 合計 10
20 を足した → 合計 30
リセットした（それまでの合計 30）
5 を足した → 合計 5
```

- `Command` に `Reset` を足し、係の `match` に「`old` に今の合計を取っておき、0 に戻して、`old` を返事する」枝を足しました。`match` は全部の場合を書かないとコンパイルエラーになるので、依頼の種類を足したのに係の処理を書き忘れる、ということが起きません（第2巻 UNIT 08）。
- `ask` 関数は、「封筒を作る → 依頼に入れて送る → 返事を待つ」をまとめたものです。依頼の種類ごとに中身が違うので、「封筒を受け取って `Command` を作る関数」（クロージャ）を `make` として渡しています。`|reply| Command::Reset { reply }` は、「封筒 `reply` を受け取って、`Command::Reset { reply }` を作る」クロージャです（第2巻 UNIT 10）。
- 送る口を `&mpsc::Sender<Command>` で借りているのは、`send` が `&self`（借りるだけ）で呼べるからです。

**問3の解答:**

```rust
use std::time::{Duration, Instant};
use tokio::sync::watch;

// 止める合図（true）が来るまで、0.1 秒ごとに仕事をする
async fn worker(mut stop: watch::Receiver<bool>, start: Instant) -> u32 {
    let mut count = 0;
    loop {
        tokio::select! {
            changed = stop.changed() => {
                // 送り手がいなくなった（Err）か、値が true になったら止まる
                if changed.is_err() || *stop.borrow() {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                count += 1;
                println!("{:.1} 秒: 仕事 {count} 回目", start.elapsed().as_secs_f64());
            }
        }
    }
    count
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (stop_tx, stop_rx) = watch::channel(false);
    let handle = tokio::spawn(worker(stop_rx, start));

    tokio::time::sleep(Duration::from_millis(350)).await;
    println!(
        "{:.2} 秒: 止める合図を送ります",
        start.elapsed().as_secs_f64()
    );
    stop_tx.send(true).unwrap();

    let count = handle.await.unwrap();
    println!(
        "{:.2} 秒: 止まりました（仕事は {count} 回）",
        start.elapsed().as_secs_f64()
    );
}
```

```text
0.1 秒: 仕事 1 回目
0.2 秒: 仕事 2 回目
0.3 秒: 仕事 3 回目
0.36 秒: 止める合図を送ります
0.36 秒: 止まりました（仕事は 3 回）
```

- 仕事係は、`select!` で「合図が変わった」と「0.1 秒たった」を競争させています。合図を受けると、すぐに（0.36 秒のうちに）止まりました。4回目の仕事の `sleep` は、途中で捨てられています（UNIT 03 の例3-5と同じです）。
- `stop.changed()` を毎回のループで作り直してよいのは、tokio のソースで「`changed` はキャンセル安全です」と明記されているからです。負けても、合図を見逃すことはありません。
- `changed.is_err()` も確かめているのは、送り手 `stop_tx` が片付けられたときに、`changed()` がすぐ `Err` を返し続けるからです。これを確かめないと、ループが空回りし続けます。送り手がいなくなったことも、「止まれ」の合図として扱います。
- この「watch の合図で、仕事係を止める」形は、UNIT 06 でサーバーを止めるときに使う形の、小さな版です。

### まとめ

- mpsc の channel は、何人もの送り手から1人の受け手へ値を**持ち主ごと**渡す。送る口が全部片付くと `recv` が `None` を返して受け取りのループが終わるので、`main` の `tx` の `drop` を忘れない。
- 容量の決まった channel は、受け手が遅いと送り手を待たせる（backpressure）。`try_send` なら待たずに断れる。容量を増やしても全体は速くならない。
- 1回の返事は oneshot（依頼に封筒を同封するとアクターが作れる）、全員へのお知らせは broadcast（遅れると `Lagged`）、最新の値や止める合図は watch を使う。
