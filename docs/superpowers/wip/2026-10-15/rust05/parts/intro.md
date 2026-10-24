<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 05 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rustバックエンド — async・並行処理・ジョブ・運用</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">Future の仕組みから、タスク・channel・timeout・再試行・ログまで、たくさんの仕事を同時に安全にさばくサーバーの土台を身に付ける</div>
</div>

# 第5巻: Rustバックエンド — async・並行処理・ジョブ・運用

## この巻で身に付けること

Web アプリのサーバーには、たくさんの人から同時にリクエストが届きます。ノートアプリ RustNote でも、Aさんがノートを保存している間に、Bさんは一覧を開き、Cさんはログインしようとしています。

1つのリクエストの処理時間のほとんどは、実は**待ち時間**です。DB の返事を待つ、外の API の返事を待つ、ネットワークの向こうからデータが届くのを待つ。待っている間、CPU はほとんど何もしていません。その待ち時間に**ほかの人の仕事を進める**のが、この巻で学ぶ **async**（エイシンク：非同期処理）です。

ただし、たくさんの仕事を同時に進めると、1つずつ順番に進めるときには起きなかった事故が起きます。

- 1件ずつ順番に処理していて、10人が同時に来ると、10人目は**10倍待たされる**
- パスワードのハッシュのような重い計算をしている間、**ほかの全員の応答が止まる**
- 外の API が返事をしないまま、**いつまでも待ち続ける**
- 仕事が一気に押し寄せて、受け付けた仕事でメモリがあふれ、**サーバーごと落ちる**
- 鍵（Mutex）を持ったまま待ってしまい、**全員が鍵を待ったまま動かなくなる**
- サーバーを止めた瞬間に、**保存の途中だった仕事が切れる**
- エラーが起きても、どのリクエストで何が起きたのかを**ログからたどれない**

この巻のテーマは、こうした事故を、**Future**（フューチャー）・**タスク**・**channel**（チャネル）・**Semaphore**（セマフォ）・**timeout**（タイムアウト）・**spawn_blocking**（スポーン・ブロッキング）・**再試行**・**tracing**（トレーシング）という仕組みで防ぐことです。Rust では、多くの間違いを**コンパイルの時点で**止めてくれます。この巻では、そのエラーメッセージを読み解く力も身に付けます。

### 到達目標

この巻を終えると、次のことができるようになります。

- `async fn` が返す **Future** の正体（「まだ始まっていない仕事の札」で、`poll` されて少しずつ進む状態機械）を説明できる
- `tokio::join!` と `tokio::try_join!` で、複数の待ち時間を重ねて、全体を速くできる
- Tokio の**ランタイム**の役割を説明し、`tokio::spawn` でタスクを作って結果を受け取れる。タスクに `'static` と `Send` が要る理由が分かる
- `JoinSet` と `select!` で、子のタスクをまとめて待ち、最初に終わったものを選べる。同時に動かす数に上限を付けられる
- channel でタスクの間に値を渡し、容量の決まった channel で**多すぎる仕事に「待った」をかけられる**（backpressure：バックプレッシャー）
- `Mutex`・`RwLock`・`Semaphore` で共有するデータと同時に入れる数を守り、「鍵を持ったまま `.await` しない」を守れる
- `timeout` で待つ時間に上限を付け、Ctrl+C を受けたら処理中の仕事を片付けてから止まれる（graceful shutdown：グレースフル・シャットダウン）
- 重い計算を `spawn_blocking` に逃がし、ほかのリクエストを止めない
- HTTP クライアントで外の API を呼び、一時的な失敗だけを間隔をあけて再試行できる。同じ操作を2回送っても困らない形（冪等性：べきとうせい）を説明できる
- tracing で、リクエストごとの情報（何の処理か・誰の処理か・何ミリ秒かかったか）を付けたログを出せる
- 画面の外で動く仕事（バックグラウンドジョブ）を、サーバーが落ちても続けられる形で設計できる

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">async の</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">土台</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">渡す・守る</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">止める</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">外とつなぎ</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">見守る</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Future</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">待つ間に進める</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">spawn</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">タスクを作る</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">JoinSet・select!</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">まとめて待つ</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="64" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="84" font-size="12" fill="#475569" text-anchor="middle">async fn main</text>
<text x="641" y="102" font-size="12" fill="#475569" text-anchor="middle">接続ごとのタスク</text>
<path d="M494,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 04</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">channel</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">値を受け渡す</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="347" y="190" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Mutex・Semaphore</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">共有と人数の上限</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">timeout・停止</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">上限と後片付け</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="174" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="194" font-size="12" fill="#475569" text-anchor="middle">Ctrl+C で処理中を</text>
<text x="641" y="212" font-size="12" fill="#475569" text-anchor="middle">待ってから止まる</text>
<path d="M494,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 07</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">spawn_blocking</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">重い計算を逃がす</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 08</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">HTTP・再試行</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">外の API を呼ぶ</text>
<rect x="429" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="494" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="494" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">tracing</text>
<text x="494" y="319" font-size="12" fill="#475569" text-anchor="middle">ログと計測</text>
<rect x="576" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="641" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="641" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ジョブ</text>
<text x="641" y="319" font-size="12" fill="#475569" text-anchor="middle">画面の外の仕事</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<line x1="559" y1="295" x2="574" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-001)"/>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-?: 第5巻の学習地図。async の土台から始めて、タスクの間の受け渡し・守り方・止め方、外とのやり取りと見守り方へ進む</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | async・Future・await の正体 | 「待つ間に、ほかの仕事を進める」仕組みを、中からのぞく |
| UNIT 02 | Tokio のランタイム・spawn・JoinHandle | 仕事（タスク）を作り、何人もの作業員で分け合う |
| UNIT 03 | JoinSet・select!・同時に動かす数の管理 | 子のタスクをまとめて待ち、最初に終わったものを選ぶ |
| UNIT 04 | channel・backpressure・所有権の移動 | タスクの間で値を受け渡し、多すぎる仕事に「待った」をかける |
| UNIT 05 | Mutex・RwLock・Semaphore と .await | 共有するデータと、同時に入れる数を守る |
| UNIT 06 | timeout・キャンセル・graceful shutdown | 待つ時間に上限を付け、止めるときは仕事を片付けてから止める |
| UNIT 07 | spawn_blocking と重い計算 | パスワードのハッシュのような重い仕事を、別の場所に逃がす |
| UNIT 08 | HTTP クライアント・再試行・冪等性（準備中） | 外の API を呼び、失敗したら賢くやり直す |
| UNIT 09 | tracing・構造化ログ・メトリクス（準備中） | 何が起きたかを、あとからたどれるように記録する |
| UNIT 10 | バックグラウンドジョブと運用（準備中） | 画面の外で動く仕事を、落ちても続けられる形にする |

> **お知らせ:** この版には UNIT 01〜07 を収録しています。UNIT 08〜10 は準備中で、次の更新から順に追加します。本文の中の「UNIT 08 で学びます」のような予告は、これから追加する単元を指しています。

### この巻に出てくる名前の整理

この巻には、Rust の言葉そのものと、クレート（部品）の名前が混ざって出てきます。最初に、どれが何なのかを分けておきましょう。

- **`async` / `.await` / `Future`**: Rust の言葉と標準ライブラリにある仕組みです。「待ちながら進む仕事」を**書く**ための道具で、どの Rust のプログラムでも使えます。
- **Tokio**（トーキョー）: その仕事を**実際に動かす**ためのクレートです。Rust の標準ライブラリには「Future を動かす係」が入っていないので、Tokio のような**ランタイム**を足して使います。RustNote が使う Topcoat も、第7巻の libsql も、Tokio の上で動きます。
- **そのほかのクレート**: Tokio の上で動く、目的別の部品です。この巻では、取り消しの合図に tokio-util、パスワードに argon2、HTTP クライアントに reqwest（リクエスト）、ログに tracing を使います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="40" width="210" height="220" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="121" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Rust の言葉と標準ライブラリ</text>
<text x="121" y="86" font-size="11" fill="#475569" text-anchor="middle">仕事を「書く」道具</text>
<text x="32" y="118" font-size="12" fill="#0f172a">async fn・async ブロック</text>
<text x="32" y="146" font-size="12" fill="#0f172a">.await（待つ印）</text>
<text x="32" y="174" font-size="12" fill="#0f172a">Future（仕事の札）</text>
<text x="32" y="202" font-size="12" fill="#0f172a">Poll（Pending / Ready）</text>
<text x="32" y="230" font-size="12" fill="#0f172a">Waker（起こし係）</text>
<rect x="276" y="40" width="210" height="220" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="381" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Tokio（ランタイム）</text>
<text x="381" y="86" font-size="11" fill="#475569" text-anchor="middle">仕事を「動かす」係</text>
<text x="292" y="118" font-size="12" fill="#0f172a">スケジューラー（割り振り）</text>
<text x="292" y="146" font-size="12" fill="#0f172a">タイマー（sleep・timeout）</text>
<text x="292" y="174" font-size="12" fill="#0f172a">I/O（ネットワークなど）</text>
<text x="292" y="202" font-size="12" fill="#0f172a">spawn・JoinSet・channel</text>
<text x="292" y="230" font-size="12" fill="#0f172a">Mutex・Semaphore</text>
<rect x="536" y="40" width="170" height="220" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="621" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ほかのクレート</text>
<text x="621" y="86" font-size="11" fill="#475569" text-anchor="middle">Tokio の上で動く部品</text>
<text x="550" y="118" font-size="12" fill="#0f172a">tokio-util（取り消し）</text>
<text x="550" y="146" font-size="12" fill="#0f172a">argon2（パスワード）</text>
<text x="550" y="174" font-size="12" fill="#0f172a">reqwest（HTTP）</text>
<text x="550" y="202" font-size="12" fill="#0f172a">tracing（ログ）</text>
<text x="550" y="230" font-size="12" fill="#0f172a">Topcoat・libsql</text>
<line x1="226" y1="150" x2="272" y2="150" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-002)"/>
<text x="250" y="140" font-size="10" fill="#475569" text-anchor="middle">渡す</text>
<line x1="536" y1="150" x2="490" y2="150" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-002)"/>
<text x="512" y="140" font-size="10" fill="#475569" text-anchor="middle">使う</text>
<text x="360" y="288" font-size="12" fill="#475569" text-anchor="middle">async fn が作った Future を Tokio が動かし、ほかのクレートも Tokio の力を借りて動く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-?: この巻に出てくる名前の関係。仕事を「書く」のは Rust の言葉、「動かす」のは Tokio、目的別の部品は Tokio の上で動く</div>
</div>

> **ヒント:** 第7巻（Turso・libSQL）では、`#[tokio::main]` を「`main` の中で `.await` を使えるようにするおまじない」、`.await` を「返事が来るまで待つ印」として使いました。その「おまじない」の中身を、UNIT 01 と UNIT 02 で開けて見ていきます。第7巻をまだ読んでいなくても、この巻は読み進められます。

### 前提知識（先に読んでおく巻）

- **第1巻（環境構築）:** Rust・VS Code・rust-analyzer がインストール済みで、`cargo new` と `cargo run` ができること。
- **第2巻（Rust 基礎）:** `struct` / `enum` / `match` / `Option` / `Result` / `?` / `Vec` / `String` / クロージャの基本。
- **第3巻（所有権・借用）:** 所有権の移動（move）と借用。`'static`（第3巻 UNIT 06）。そして特に、**第3巻 UNIT 10 の `Arc`・`Mutex`・`Send`・`Sync`** は、この巻で何度も使います。スレッドを作る `std::thread::spawn` の話を覚えていると、UNIT 02 の `tokio::spawn` がすっと入ります。
- **第4巻（設計）:** trait と generics、エラー型（thiserror と anyhow）、`Cargo.toml` の版の書き方（第4巻 UNIT 07）。

> **注意:** この巻では、スレッド・`Arc`・`Mutex` を「第3巻で学んだもの」として使います。`Send` と `Sync` の意味があいまいなときは、先に第3巻 UNIT 10 の「図で理解する」と「よくあるエラー」だけでも読み返しておくと、UNIT 02 のエラーメッセージがずっと読みやすくなります。

### この巻の進め方

各単元では、次の順番を1サイクルとして進めてください。async は、頭の中で考えるだけでは、どの順番で何が起きるのかが想像しにくい分野です。**表示の順番を予想してから動かす**のが、いちばんの近道です。

1. **予想する:** 実行する前に、どの行がどの順番で表示されるか、全体で何秒かかるかを予想します。
2. **そのまま動かす:** 教材のコードをそのまま実行し、結果が教材と同じか比べます。
3. **1か所だけ変える:** 待ち時間や `.await` の位置を1つだけ変えて、表示の順番と時間がどう変わるかを観察します。「よくあるエラー」の節のコードも実際にビルドして、エラーメッセージを自分の目で読みます。
4. **整える:** 単元の終わりには、次の3つのコマンドを実行する習慣をつけます。

```powershell
# コードの見た目を Rust の標準の形に整える
cargo fmt
# よくある書き間違い・もったいない書き方を指摘してもらう
cargo clippy
# テストを実行する
cargo test
```

> **ヒント:** この巻の実行結果には、かかった時間（`0.5 秒` など）や、スレッドの番号（`ThreadId(13)` など）が出てきます。時間はパソコンの速さやその時の混み具合で少し変わり、スレッドの番号は実行するたびに変わります。表示の順番が教材と入れ替わることもあります。どういうときに入れ替わるのかは、UNIT 02 で説明します。

### 使うツールと版

この巻のコードは、すべて次の環境で実際にコンパイル・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分だけ `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |
| edition | 2024（`cargo new` の既定値） |
| tokio | 1.53.3（`full` の機能つき。この巻の全単元で使う） |
| tokio-util | 0.7.20（UNIT 06。取り消しの合図と、タスクの見張り） |
| argon2 | 0.6.0（UNIT 07。パスワードのハッシュ） |
| reqwest | 0.13.5（UNIT 08。HTTP クライアント。ARM64 版の Windows では、ビルドに clang が必要） |
| serde / serde_json | 1.0.229 / 1.0.154（UNIT 08・09。JSON との変換） |
| tracing / tracing-subscriber | 0.1.44 / 0.3.23（UNIT 09。ログ） |

UNIT 01〜05 で使うクレートは tokio だけで、UNIT 06 で tokio-util を、UNIT 07 で argon2 を足します。ほかのクレートは、使う単元で `cargo add` します。

### 練習用プロジェクトを用意する

この巻では、練習用のプロジェクト `async_lab` を1つ作り、小さなプログラムを `src/bin/` に1つずつ足していきます。

```powershell
cargo new async_lab
cd async_lab
mkdir src/bin
cargo add tokio@=1.53.3 --features full
```

`cargo new` と `cargo add` の実際の出力は次のとおりです。

```text
    Creating binary (application) `async_lab` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    Updating crates.io index
      Adding tokio =1.53.3 to dependencies
             Features:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + net
             + parking_lot
             + process
             + rt
             + rt-multi-thread
             + signal
             + signal-hook-registry
             + socket2
             + sync
             + time
             + tokio-macros
             - io-uring
             - mio
             - schedule-latency
             - taskdump
             - test-util
             - tracing
             - windows-sys
    Updating crates.io index
     Locking 24 packages to highest Rust 1.99.0 compatible versions
      Adding tokio v1.53.3 (available: v1.54.0)
```

- `tokio@=1.53.3` の `=` は「ちょうど 1.53.3」という意味です（第4巻 UNIT 07）。最後の行の `available: v1.54.0` は「もっと新しい 1.54.0 も出ています」というお知らせです。`=` を付けずに `tokio@1.53.3` と書くと、Cargo は 1.x の中でいちばん新しい版を選ぶので、2026年10月22日の時点では 1.54.0 が入ります。この巻の出力と同じ結果になるように、版を固定しておきます。
- `--features full` は、Tokio の機能を**全部** ON にする指定です。`+` が付いた機能が ON になりました。`time`（待つ・時間の上限）、`sync`（channel や Mutex）、`macros`（`#[tokio::main]`）、`rt-multi-thread`（何本ものスレッドで動かす）、`signal`（Ctrl+C を受け取る）など、この巻で使う機能が全部入っています。
- 本番のアプリでは、使う機能だけを選ぶのがふつうです。RustNote の `Cargo.toml` は `features = ["rt-multi-thread", "macros"]` の2つだけです。機能を絞ると、ビルドが速くなり、使わない部品が入りません。

最後に、`Cargo.toml` の `[package]` の中、`edition = "2024"` の次の行に `rust-version = "1.98"` を手で1行足します。できあがった `Cargo.toml` は次のとおりです。

```toml
[package]
name = "async_lab"
version = "0.1.0"
edition = "2024"
rust-version = "1.98"

[dependencies]
tokio = { version = "=1.53.3", features = ["full"] }
```

一度 `cargo build` しておきましょう。最初のビルドでは Tokio とその部品をコンパイルするので、少し時間がかかります（この PC では約12秒でした）。

```text
   Compiling tokio v1.53.3
   Compiling async_lab v0.1.0 (...\async_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.89s
```

#### src/bin/ に置いたファイルは、それぞれが1つのプログラム

`src/bin/` フォルダに置いた `.rs` ファイルは、1つずつが別のプログラムになります。たとえば `src/bin/u01_lazy.rs` を作ったら、次のように名前（拡張子なし）を指定して実行します。

```powershell
cargo run -q --bin u01_lazy
```

- `--bin u01_lazy` は「`src/bin/u01_lazy.rs` のプログラムを実行して」という指定です。
- `-q`（quiet：静かに）を付けると、`Compiling ...` や `Running ...` のような Cargo の表示が消えて、プログラムが表示した内容だけが残ります。この巻の実行結果は、特に断りがなければ `-q` を付けて実行したものです。
- コンパイルエラーの例は、エラーの全文を見るために `-q` を付けずに `cargo build --bin 名前` や `cargo run --bin 名前` で実行しています。

> **ヒント:** VS Code で `src/bin/` のファイルを開くと、`fn main` の上に「▶ Run」の文字が出ます。これをクリックしても同じように実行できます（rust-analyzer の機能です）。
