## この巻のまとめ

この巻では、たくさんの仕事を同時にさばくサーバーの土台を、async の仕組みと Tokio のタスクから始めて、タスクの間の受け渡し・守り方・止め方、外とのやり取りと見守り方、画面の外で動く仕事まで、順に学びました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、コードを動かし直しましょう。表示の順番を予想してから動かすのを忘れずに。

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

**渡す・守る・止める**

- ☐ mpsc の channel で、`clone` した送る口から1つの受け取り口へ値を持ち主ごと渡せる。送る口が全部片付くと `recv` が `None` を返すことを使って、受け取りのループを終わらせられる（UNIT 04）
- ☐ 容量の決まった channel で受け手が遅いと送り手が待たされる（backpressure）ことを、時刻の表示で示せる。`try_send` の `Full` で混雑時にすぐ断れる。容量を増やしても全体が速くならない理由を説明できる（UNIT 04）
- ☐ 依頼に oneshot の封筒を同封して、データを1つのタスクだけが持つアクターを作れる（UNIT 04）
- ☐ broadcast（全員に配る・遅れると `Lagged`）と watch（最新の値だけ・止める合図）を目的に合わせて選べる。`borrow` を持ったまま `.await` しない（UNIT 04）
- ☐ E0382（送った値の再利用）・`SendError`（`channel closed`）・`RecvError::Lagged` と、`drop(tx)` の書き忘れで受け取り係が終わらない間違いを直せる（UNIT 04）
- ☐ async の中でも標準の `std::sync::Mutex` を使い、「鍵を持ったまま `.await` しない」を `{ }` のブロックか async でない関数で守れる（UNIT 05）
- ☐ `MutexGuard` を持ったまま `.await` したときの「future cannot be sent between threads safely」を読み、`drop(guard)` では直らない理由（変数の有効範囲で判断される）を説明できる（UNIT 05）
- ☐ spawn しないとコンパイルが通り、1本のスレッドではデッドロックすることを知っていて、clippy の `await_holding_lock` で見つけられる。同じタスクで2回 `lock` しない（UNIT 05）
- ☐ 鍵を持ったまま `.await` する必要があるときだけ `tokio::sync::Mutex` を選べる。RwLock で読む人を同時に通し、Semaphore の許可証で同時に入れる数を決められる（UNIT 05）
- ☐ 消えてはいけないデータや、複数のサーバーで共有するデータは、メモリの鍵ではなく DB で守る理由を説明できる（UNIT 05）
- ☐ `tokio::time::timeout` で待つ時間に上限を付け、`Elapsed` を `map_err` で自分のエラーに変えて `?` で返せる。E0277（`?` couldn't convert the error）を読める（UNIT 06）
- ☐ キャンセルが `.await` の所でしか起きないことから、「全部か、何もしないか」の変更の間に `.await` を置かない書き方と、`tokio::spawn` で仕事を続けさせて待つことだけに上限を付ける書き方ができる（UNIT 06）
- ☐ `std::thread::sleep` や重い計算のように `.await` の無い処理には timeout が効かないことを、実行結果で説明できる（UNIT 06）
- ☐ `CancellationToken`（親と子の札）と `TaskTracker`（`close` してから `wait`）で、Ctrl+C を受けて「受付停止 → 合図 → 上限つきで待つ → 打ち切り」の順に止まるプログラムを書ける。Topcoat が既定 30 秒の猶予で同じ順に止まることを知っている（UNIT 06）

**外とつなぎ・見守る**

- ☐ 重い計算や同期の I/O を async の中でそのまま行うと同じスレッドのタスクが止まることを、「心臓の鼓動」で示せる。CPU バウンドと I/O バウンドの違いを説明できる（UNIT 07）
- ☐ `spawn_blocking` で blocking スレッドに任せ、`Ok(Ok)`・`Ok(Err)`・`Err(JoinError)` の3つに分けて結果を扱える。借り物を渡したときの E0521 を `to_string`・`clone` と `move` で直せる（UNIT 07）
- ☐ Semaphore と `spawn_blocking` で、Argon2 のような重い計算の同時実行数に上限を付けられる。上限が無いと blocking スレッドが 512 本まで増え、メモリと CPU を使い切る理由を説明できる（UNIT 07）
- ☐ `spawn_blocking` の仕事は取り消せず、ランタイムの片付けでも終わりを待たれることを知っていて、必ず終わる仕事だけを渡せる。速さを測るときに debug と release を区別できる（UNIT 07）
- ☐ reqwest の `Client` を1つ作って使い回し、`connect_timeout` と `timeout` で上限を付けて GET できる。`send()` の `Ok` は「答えが届いた」で、4xx・5xx は `error_for_status` で `Err` にすることを知っている（UNIT 08）
- ☐ `#[derive(Deserialize)]` の struct で JSON の答えを受け取り、`missing field` などの形の違いを、エラーの `source()` の連なりから読める（UNIT 08）
- ☐ `is_connect`・`is_timeout` で通信の失敗を見分け、5xx・429・時間切れ・接続できない、だけを、指数バックオフ＋ジッター＋回数の上限で再試行できる（UNIT 08）
- ☐ 時間切れは「失敗」ではなく「分からない」なので、冪等でない操作は冪等キーが無ければ自動で再試行しない理由を説明できる。RustNote の PRG（303）の意味が分かる（UNIT 08）
- ☐ tracing の `info!`・`warn!`・`error!`・`debug!` でフィールド付きのログを出し、subscriber を `main` の最初に1回だけ登録できる。登録し忘れると何も出ず、2回登録すると panic する（`try_init`）ことを知っている（UNIT 09）
- ☐ `with_env_filter` と `RUST_LOG` でログの細かさを変え、`.json()` で機械が読める形にできる。`fmt().init()` は `RUST_LOG` を読まないことを知っている（UNIT 09）
- ☐ `#[instrument]` と `info_span!`・`.instrument(span)` でスパンを作り、`request_id` で同時のリクエストのログを見分けられる。busy と idle の違いを説明でき、spawn では `.in_current_span()` でスパンを引き継げる（UNIT 09）
- ☐ パスワードやトークンを `skip` でログから外せる。`Debug` が無いときに `#[derive(Debug)]` で直してはいけない理由を説明できる（UNIT 09）
- ☐ リクエスト数・エラー数・応答時間のヒストグラムを atomic 型で数え、Prometheus の形で書き出して読める（UNIT 09）
- ☐ 画面の応答と切り離せる仕事をバックグラウンドジョブにし、メモリの列では止めると消える仕事を DB の `jobs` 表に保存して、再起動の後に続きから処理できる（UNIT 10）
- ☐ `UPDATE … WHERE id = (SELECT … LIMIT 1) RETURNING …` の1つの文で仕事を取り出し、worker が何人いても二重に取らないようにできる。文を分けると二重に取る理由を説明できる（UNIT 10）
- ☐ 失敗した仕事を `run_at` を先にして倍々の間隔でやり直し、`max_attempts` で `dead` にして理由を残せる。`CHECK` で状態の値を守れる（UNIT 10）
- ☐ lease（持ち時間）で落ちた worker の仕事を取り直せることと、そのため仕事が「少なくとも1回」動くので中身を冪等にする理由を説明できる。worker の数を設定で決め、止めるときは仕事の切れ目で止まれる（UNIT 10）

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
| Mutex | ミューテックス。中のデータを同時に1人だけがさわれるようにする鍵（標準と tokio の2種類） | UNIT 05 |
| ガード（MutexGuard） | `lock()` が返す「鍵を持っている証拠」。片付けられると鍵が返る | UNIT 05 |
| デッドロック | お互いに鍵が返るのを待って、だれも先へ進めなくなった状態 | UNIT 05 |
| await_holding_lock | 標準の Mutex のガードを持ったまま `.await` していることを知らせる clippy の警告 | UNIT 05 |
| RwLock | 読み書き鍵。読む人は何人でも同時に、書く人は1人だけ | UNIT 05 |
| Semaphore | セマフォ。決まった枚数の許可証で、同時に入れる数を決める道具 | UNIT 05 |
| permit | 許可証。Semaphore からもらい、片付けられるときに返る | UNIT 05 |
| `LazyLock` | 最初に使われたときに1回だけ値を作る、標準ライブラリの道具 | UNIT 05 |
| atomic 型 | `AtomicU64` など。鍵を使わずに、割り込まれない1回の操作で数を書き換えられる型 | UNIT 05 |
| timeout | タイムアウト。待つ時間の上限。`tokio::time::timeout(上限, Future)` | UNIT 06 |
| `Elapsed` | timeout の時間切れを表すエラーの型。表示は `deadline has elapsed` | UNIT 06 |
| graceful shutdown | グレースフル・シャットダウン。処理中の仕事をできるだけ終わらせてから止まる止め方 | UNIT 06 |
| 強制終了 | 処理中の仕事を待たずに、その場でプログラムを止めること | UNIT 06 |
| `tokio::signal::ctrl_c` | Ctrl+C が押されるまで待つ Future。待ち受け中は強制終了されない | UNIT 06 |
| SIGTERM | Linux などで、OS やサービスがプログラムに「止まってください」と伝える合図 | UNIT 06 |
| tokio-util | tokio と一緒に使う道具の集まり。`CancellationToken`・`TaskTracker` など | UNIT 06 |
| CancellationToken | 停止の合図を配る札。`child_token` で子の札を作り、`cancel` で取り消す | UNIT 06 |
| TaskTracker | 作ったタスクを覚えておき、`close` の後の `wait` で全部の終わりを待つ係 | UNIT 06 |
| blocking | ブロッキング。終わるまでスレッドを手放さずに占有し続けること | UNIT 07 |
| CPU バウンド / I/O バウンド | 計算そのものに時間がかかる処理／返事を待つ時間がほとんどの処理 | UNIT 07 |
| blocking スレッド | `spawn_blocking` の仕事のために tokio が用意するスレッド。既定の上限は 512 本 | UNIT 07 |
| `spawn_blocking` | 同期のクロージャを blocking スレッドで動かし、引換券を返す関数。始まったら取り消せない | UNIT 07 |
| Argon2id | アルゴン・ツー・アイディー。わざと時間とメモリがかかるように作られた、パスワードのハッシュの方式 | UNIT 07 |
| PHC 文字列 | `$argon2id$v=19$m=…$塩$ハッシュ` の形で、方式・設定・塩・ハッシュをまとめた保存用の文字列 | UNIT 07 |
| release ビルド | 最適化を有効にしたビルド（`--release`）。debug ビルドより計算がずっと速い | UNIT 07 |
| HTTP クライアント | HTTP のリクエストを送り、答えを受け取る側のプログラム | UNIT 08 |
| reqwest | リクエスト。Rust でよく使われる HTTP クライアントのクレート | UNIT 08 |
| ステータスコード | HTTP の答えに付く3桁の数字。2xx 成功・4xx 頼んだ側の問題・5xx 答える側の問題 | UNIT 08 |
| `error_for_status` | 4xx・5xx の答えを `Err` に変える reqwest のメソッド | UNIT 08 |
| serde / `Deserialize` | データの形の変換を自動で書く道具。`Deserialize` は JSON などから struct を作る | UNIT 08 |
| 原因の連なり（source） | エラーを包んでいる奥のエラー。`error.source()` で1つずつたどれる | UNIT 08 |
| 再試行（retry） | 失敗した操作を、もう一度行うこと | UNIT 08 |
| 指数バックオフ | 再試行の間隔を倍々にのばしていくこと。上限を付けて使う | UNIT 08 |
| ジッター | 再試行の間隔に足す、ばらつき。やり直しの時刻を散らす | UNIT 08 |
| 冪等 | べきとう。同じ操作を1回しても何回しても、結果が同じになる性質 | UNIT 08 |
| 冪等キー | 操作ごとの整理番号。同じキーの2回目を、受ける側が見分けられる | UNIT 08 |
| PRG | Post/Redirect/Get。POST の成功後に 303 で GET のページへ移し、再読み込みで二重に送られないようにする形 | UNIT 08 |
| ログ | プログラムが、何が起きたかを書き残す記録 | UNIT 09 |
| tracing | トレーシング。ログやスパンを出す側のクレート | UNIT 09 |
| subscriber | サブスクライバー。ログを受け取り、絞り込み・形の決定・書き出しをする係。プログラムに1つ | UNIT 09 |
| レベル | ログの重要度。ERROR・WARN・INFO・DEBUG・TRACE | UNIT 09 |
| フィールド | ログに付ける「名前 = 値」の欄 | UNIT 09 |
| 構造化ログ | 決まった欄に値を書くログ。検索や集計ができる | UNIT 09 |
| スパン | 処理の区間。中で出したログに情報を引き継ぎ、木の形にできる | UNIT 09 |
| `#[instrument]` | 関数の呼び出しをスパンにする印。引数がフィールドになる（`skip` で外せる） | UNIT 09 |
| `RUST_LOG` | ログの細かさを決める環境変数（`with_env_filter` で読む） | UNIT 09 |
| request_id | リクエストごとの受付番号。同時のリクエストのログを見分ける | UNIT 09 |
| メトリクス | リクエスト数・エラー数・応答時間など、数で表した集計 | UNIT 09 |
| ヒストグラム | 値を範囲ごとの箱に分けて数える数え方。応答時間の分布を見る | UNIT 09 |
| Prometheus | プロメテウス。メトリクスを集めてグラフにする、定番のソフト | UNIT 09 |
| バックグラウンドジョブ | 利用者への応答とは別に、裏で行う仕事 | UNIT 10 |
| worker | ワーカー。仕事の列から仕事を取り出して処理する係 | UNIT 10 |
| ジョブキュー | 仕事の列。この巻では DB の `jobs` 表で作った | UNIT 10 |
| claim | 仕事を取り出して自分のものにすること。1つの `UPDATE … RETURNING` で行う | UNIT 10 |
| lease | リース。worker が仕事を取り出したときの持ち時間。過ぎるとほかの worker が取り直せる | UNIT 10 |
| dead-letter | デッドレター。上限までやり直しても成功しなかった仕事の置き場。人が確かめて入れ直す | UNIT 10 |
| ポーリング | 決まった間隔で「仕事は無いか」を見に行くやり方 | UNIT 10 |
| 少なくとも1回（at-least-once） | 仕事が失われないかわりに、2回以上動くことがある、という約束 | UNIT 10 |
| release_command | Fly.io で、新しい版を公開する前に1回だけ動かすコマンド（第12巻） | UNIT 10 |
