## この巻のまとめ

この巻では、たくさんの仕事を同時にさばくサーバーの土台として、async の仕組みと Tokio のタスクから学び始めました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、コードを動かし直しましょう。表示の順番を予想してから動かすのを忘れずに。

**async の土台**

- ☐ `async fn` を呼ぶと Future（仕事の札）が返るだけで、中身は `.await` したときに初めて動くことを、実行結果で説明できる（UNIT 01）
- ☐ `tokio::time::sleep(...).await` と `std::thread::sleep` の違いを説明し、async の中では前者を使える（UNIT 01）
- ☐ お互いに関係のない待ちを `tokio::join!` で重ね、全体の時間が「足し算」から「いちばん長いもの」になることを示せる。並行と並列の違いを説明できる（UNIT 01）
- ☐ `tokio::try_join!` が最初に起きた失敗をすぐに返し、残りの Future を途中で捨てる（キャンセルする）ことを説明できる（UNIT 01）
- ☐ Future が poll されて `Pending` / `Ready` を返し、Waker で起こされて進む流れを図で説明できる。`.await` をまたぐ変数が Future の中にしまわれることを知っている（UNIT 01）
- ☐ E0752（`async fn main`）・E0728（`async` でない関数の `.await`）・E0308（`.await` の付け忘れ）と、`unused implementer of Future` の警告を読んで直せる。警告を `cargo clippy -- -D warnings` で止められる（UNIT 01）
- ☐ ランタイムの中身（ワーカースレッド・スケジューラー・タイマーと I/O の係）と、`#[tokio::main]` が `Builder` と `block_on` の形に書き換わることを説明できる（UNIT 02）
- ☐ `tokio::spawn` でタスクを作り、引換券（`JoinHandle`）の `.await` で `Result<戻り値, JoinError>` を受け取れる。タスクは spawn した瞬間から動くことを知っている（UNIT 02）
- ☐ `JoinError` の `is_panic()` と `is_cancelled()` で、panic と `abort` を見分けられる。`Result<Result<T, E>, JoinError>` を3つに分けて扱える（UNIT 02）
- ☐ 引換券を捨てたタスクが `main` の終わりで途中で捨てられることを知っていて、最後までやり遂げてほしい仕事は待つようにできる（UNIT 02）
- ☐ `tokio::spawn` に `Send + 'static` が要る理由を説明し、E0373・E0382（ループの中の `move`）・「future cannot be sent between threads safely」を、`move`・`clone`・`Arc` で直せる（UNIT 02）
- ☐ ランタイムの中で `block_on` を呼ぶと `Cannot start a runtime from within a runtime` で panic する理由を説明できる（UNIT 02）
- ☐ `JoinSet` で子のタスクをまとめて作り、`join_next` で終わった順に受け取れる。袋が捨てられると残った子が取り消されることを、`Drop` の表示で確かめられる（UNIT 03）
- ☐ 「1つ失敗したら `??` で抜けて残りを取り消す」集め方と、「失敗も `Result` のまま全部集める」集め方を書き分けられる（UNIT 03）
- ☐ 袋の中の数を見て、同時に動かすタスクの数に上限を付けられる。上限が「速さ」と「相手への負担」の兼ね合いだと説明できる（UNIT 03）
- ☐ `tokio::select!` で先に終わった枝だけを実行し、負けた Future がその場で捨てられることと、キャンセル安全の意味を説明できる（UNIT 03）
- ☐ ループの中の `select!` で、締め切りや仕事の Future をループの外で作って `pin!` で留め、`&mut` で借りられる。E0271（JoinSet の型の食い違い）・E0277（`PhantomPinned cannot be unpinned`）と、「毎回やり直しになって終わらない」間違いを直せる（UNIT 03）
- ☐ mpsc の channel で、`clone` した送る口から1つの受け取り口へ値を持ち主ごと渡せる。送る口が全部片付くと `recv` が `None` を返すことを使って、受け取りのループを終わらせられる（UNIT 04）
- ☐ 容量の決まった channel で受け手が遅いと送り手が待たされる（backpressure）ことを、時刻の表示で示せる。`try_send` の `Full` で混雑時にすぐ断れる。容量を増やしても全体が速くならない理由を説明できる（UNIT 04）
- ☐ 依頼に oneshot の封筒を同封して、データを1つのタスクだけが持つアクターを作れる（UNIT 04）
- ☐ broadcast（全員に配る・遅れると `Lagged`）と watch（最新の値だけ・止める合図）を目的に合わせて選べる。`borrow` を持ったまま `.await` しない（UNIT 04）
- ☐ E0382（送った値の再利用）・`SendError`（`channel closed`）・`RecvError::Lagged` と、`drop(tx)` の書き忘れで受け取り係が終わらない間違いを直せる（UNIT 04）

> **お知らせ:** この版には UNIT 01〜04 を収録しています。UNIT 05（Mutex・RwLock・Semaphore）・UNIT 06（timeout・キャンセル・graceful shutdown）・UNIT 07（spawn_blocking）・UNIT 08（HTTP クライアント・再試行）・UNIT 09（tracing）・UNIT 10（バックグラウンドジョブ）は準備中で、次の更新から順に追加します。それまでの間、重い計算を逃がす `spawn_blocking` が RustNote でどう使われているかは、`Rust/rustnote/src/auth.rs` で確かめられます。

次の第6巻では、この巻で学んだタスクの上で動く Web フレームワーク Topcoat を使って、HTTP のリクエストと答えのしくみを学びます。

## 用語ミニ辞典

この巻で出てきた主な用語です。

| 用語 | 読み・意味 | 出てくる単元 |
|---|---|---|
| async（非同期処理） | エイシンク。待ち時間の間に、ほかの仕事を進められるように書いた処理 | 巻頭・UNIT 01 |
| Tokio | トーキョー。Future を動かすランタイムを提供するクレート | 巻頭・UNIT 02 |
| Future | フューチャー。「まだ終わっていない仕事」を表す値。`.await` か poll されるまで動かない | UNIT 01 |
| `.await` | Future が終わるまで待つ印。待つ間は、ほかの仕事に席を譲る | UNIT 01 |
| ランタイム（executor） | Future を poll して動かす係。実行係 | UNIT 01・02 |
| poll | ポール。Future に「進められるところまで進めて」と頼むこと | UNIT 01 |
| Pending / Ready | poll の答え。「まだです」と「できました（値つき）」 | UNIT 01 |
| Waker | ウェイカー。準備ができた Future を、もう一度 poll するよう知らせる起こし係 | UNIT 01 |
| 状態機械 | 「今どこまで進んだか」を状態として持ち、少しずつ進むしくみ。async fn はこれに作り変えられる | UNIT 01 |
| pin | ピン。Future をメモリの中で動かないように留めること | UNIT 01 |
| 並行（concurrency） | コンカレンシー。複数の仕事を切り替えながら少しずつ進めること | UNIT 01 |
| 並列（parallelism） | パラレリズム。複数の CPU で、本当に同時に動かすこと | UNIT 01・02 |
| `tokio::join!` | 複数の Future を同時に進め、全部の結果をタプルで返す | UNIT 01 |
| `tokio::try_join!` | `Result` を返す Future 用の `join!`。最初の失敗をすぐに返す | UNIT 01 |
| キャンセル | Future が途中で drop され、二度と poll されなくなること | UNIT 01・02 |
| タスク | ランタイムの上で動く、1つの独立した仕事。`tokio::spawn` で作る | UNIT 02 |
| ワーカースレッド | ランタイムがタスクを進めるために用意したスレッド | UNIT 02 |
| スケジューラー | 待ち行列のタスクを、ワーカースレッドに割り振る係 | UNIT 02 |
| `tokio::spawn` | Future をタスクとして動かし始め、引換券を返す関数 | UNIT 02 |
| JoinHandle | ジョインハンドル。タスクの引換券。`.await` で結果を受け取る | UNIT 02 |
| JoinError | タスクが panic したか、取り消されたことを表すエラー | UNIT 02 |
| abort | アボート。タスクを取り消すこと。次の `.await` の区切りで止まる | UNIT 02 |
| 切り離されたタスク | 引換券を捨てて、誰も結果を待っていないタスク（detached task） | UNIT 02 |
| `block_on` | Future が終わるまで、今のスレッドで待ち続ける関数。async の世界の入口で使う | UNIT 02 |
| `Send` | 別のスレッドへ渡してよい、という型の印（第3巻 UNIT 10） | UNIT 02 |
| `'static` | 借り物を持たず、いつまでも使える、という印。`tokio::spawn` に渡す仕事に求められる | UNIT 02 |
| `async move` | 外の変数を借りずに、async ブロックの中へ移す書き方 | UNIT 02 |
| coroutine | コルーチン。コンパイラの中での、async ブロックの呼び名。エラーメッセージに出てくる | UNIT 02 |
| JoinSet | ジョインセット。子のタスクの袋。`join_next` で終わった順に受け取り、捨てると残りを取り消す | UNIT 03 |
| `join_next` / `join_all` | JoinSet から結果を1つずつ／全部まとめて受け取るメソッド。どちらも終わった順 | UNIT 03 |
| 構造化された並行処理 | 子のタスクが親より長生きしないように、親が子の始まりと終わりを管理する書き方 | UNIT 03 |
| `tokio::select!` | 複数の Future を同時に進め、最初に終わった枝だけを実行するマクロ | UNIT 03 |
| 枝 | `select!` の中の「パターン = Future => 式」の1つ1つ | UNIT 03 |
| キャンセル安全 | 途中で捨てられても、データを失ったり中途半端な状態を残したりしない性質（cancel safety） | UNIT 03 |
| `Unpin` | poll の途中で場所を動かしても平気な型に付く印。`Sleep` などは `Unpin` ではない | UNIT 03 |
| `pin!` | Future を動かないように留めるマクロ。留めると `&mut` で借りて何度も poll できる | UNIT 01・03 |
| channel | チャネル。タスクの間で値を渡す通り道。送る口と受け取る口の1組 | UNIT 04 |
| mpsc | 送り手は何人でも、受け手は1人の channel（multi-producer, single-consumer） | UNIT 04 |
| bounded / unbounded | 箱に入る数の上限がある／無い channel。この巻では上限つきを使う | UNIT 04 |
| backpressure | バックプレッシャー。受け手が追いつかないとき、送り手を待たせたり断ったりするしくみ | UNIT 04 |
| `try_send` | 待たずに送る。箱がいっぱいなら、すぐに `Full` のエラーを返す | UNIT 04 |
| oneshot | 値を1回だけ届ける channel。依頼への返事に使う | UNIT 04 |
| アクター | 自分だけのデータを持ち、メッセージで届いた依頼を1つずつ処理するタスク | UNIT 04 |
| broadcast | 送った1件を、受け手の全員に届ける channel。遅れた受け手は読み飛ばす | UNIT 04 |
| `Lagged` | broadcast で、受け手が追いつけずに読み飛ばした件数を知らせるエラー | UNIT 04 |
| watch | 最新の値だけを共有する channel。設定の変更や「止めて」の合図に使う | UNIT 04 |
