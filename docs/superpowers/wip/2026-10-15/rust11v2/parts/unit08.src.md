## UNIT 08: benchmark・profile・メモリの使い方

### この単元のゴール

- criterion で関数の速さを、何百万回もくり返して統計つきで測り、結果（3つの数・外れ値）を読める
- 「どこが遅いか」を部分ごとに測ってから直す、という順番を、ログインの例（時間の 86% が Argon2 の照合）で説明できる
- debug と release で速さが何倍も違うことを確かめ、速さは release で測る理由を言える
- メモリを借りる係（global allocator）を「数える係」に替えて、借りた回数とピークを測り、Argon2 が1回に 19 MiB 使うことを確かめられる

### たとえ話でつかむ

レストランの厨房で、「料理が出るのが遅い」と言われた場面を考えます。

- 料理長は、すぐに包丁を研いだりしません。まず、**下ごしらえ・焼く・盛りつけ**の時間を、ストップウォッチで測ります。すると、**焼く**が全体の 8 割以上でした。盛りつけを2倍速くしても、全体はほとんど変わりません。
- 測るのは、**本番と同じ火加減**でします。練習用の弱い火で測ると、焼く時間が何倍にもなって、どこが遅いのか見誤ります（debug と release）。
- 1回だけ測ると、たまたまオーブンが温まっていなかった、ということがあります。**何回も測って**、ばらつきも見ます（criterion）。
- 焼くには、大きな**天板**が要ります。同時にたくさん焼くと、天板の置き場（メモリ）が足りなくなります。1回に、どのくらいの大きさが要るかも測っておきます。

| 厨房 | RustNote |
|---|---|
| 料理が出るまでの時間 | `POST /login` 全体の時間 |
| 下ごしらえ・焼く・盛りつけ | DB でユーザーを探す・パスワードの照合（Argon2）・セッションを作る |
| ストップウォッチで部分ごとに | 部分ごとに `Instant` で測る（例8-2） |
| 本番の火加減 | release ビルド（エラー1） |
| 何回も測る | criterion（例8-1） |
| 天板の大きさ | 1回の照合で借りるメモリ（19 MiB。例8-3） |

> **用語:** ベンチマーク（benchmark：同じ処理を何回もくり返して、1回あたりの時間を測ること）

<!-- -->

> **用語:** プロファイル（profile：プログラムの時間やメモリが、**どこで**使われているかを調べること。全体の時間だけでなく、内訳を見る）

<!-- -->

> **用語:** release ビルド（`--release` を付けた組み立て。時間をかけて最適化し、速いプログラムを作る。ふだんの `cargo build`・`cargo test` は、速く組み立てられる代わりに遅い debug ビルド）

### 図で理解する

`POST /login` 1回の時間の内訳です（例8-2の数。5回のうち、いちばん短い回）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="36" font-size="13" font-weight="700" fill="#0f172a">debug</text>
<text x="20" y="54" font-size="11" fill="#475569">全体 450.69 ms</text>
<rect x="120" y="22" width="3" height="36" fill="#3b82f6"/>
<rect x="123" y="22" width="497" height="36" fill="#fecaca" stroke="#ef4444" stroke-width="1.5"/>
<text x="370" y="45" font-size="12" fill="#b91c1c" text-anchor="middle">パスワードの照合 386.72 ms（86%）</text>
<rect x="620" y="22" width="80" height="36" fill="#e2e8f0" stroke="#94a3b8" stroke-width="1.5"/>
<text x="660" y="45" font-size="10" fill="#334155" text-anchor="middle">その他</text>
<text x="20" y="116" font-size="13" font-weight="700" fill="#0f172a">release</text>
<text x="20" y="134" font-size="11" fill="#475569">全体 35.12 ms</text>
<rect x="120" y="102" width="2" height="36" fill="#3b82f6"/>
<rect x="122" y="102" width="26" height="36" fill="#fecaca" stroke="#ef4444" stroke-width="1.5"/>
<rect x="148" y="102" width="17" height="36" fill="#e2e8f0" stroke="#94a3b8" stroke-width="1.5"/>
<text x="180" y="125" font-size="11" fill="#0f172a">照合 20.27 ms（58%）・DB 2.20 ms・その他 約 13 ms</text>
<text x="120" y="172" font-size="11" fill="#1d4ed8">■ DB でユーザーを探す（1.43 ms / 2.20 ms）</text>
<text x="120" y="194" font-size="11" fill="#475569">横の長さは、debug と release で同じ縮尺（1 ms ＝ 同じ長さ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: ログインの時間の内訳。debug では照合が 86% を占める。release では全体が約 13 分の 1 になり、照合も約 19 倍速くなる。DB で探す時間は、どちらでも小さい</div>
</div>

### まず動かす最小コード

#### 例8-1: criterion で、parse の速さを測る

1回だけ `Instant` で測ると、ほかの仕事の影響や、たまたまの遅れで、数がぶれます。criterion は、決まった時間の中で何百万回もくり返し、統計を使って「1回あたり何秒か」と、そのばらつきを出します。練習用のコピーの dev-dependency に足します。

```powershell
cargo add --dev criterion@=0.8.2
```

```text
@@OUTGREP:u08_add:^(    Updating|      Adding criterion =|     Locking)@@
```

（`criterion` 自身を含めて、33 個のクレートが足されました。）

ベンチマークは `benches/` に置き、`Cargo.toml` に、次の3行を足します。

```toml
@@OUT:u08_bench_toml@@
```

> **▼ このコードがやること（先に日本語で）:** `NoteTitle::parse` に、半角の `a`・ひらがなの `あ`・絵文字の `😀` を、それぞれ 80 個並べた題を渡す速さと、`Email::parse` に、前後に空白のある大文字まじりのアドレスを渡す速さを測ります。`black_box` は、「この値を、組み立てのときに計算し終えてしまわないで」とコンパイラーに伝える印です（解説2）。

```rust
@@FILE:staging_u08/parse.rs@@
```

```powershell
cargo bench --bench parse
```

**▼ 実行結果**（抜き出しています）

```text
@@OUTGREP:u08_bench:^(    Finished|Gnuplot|Benchmarking NoteTitle::parse/ascii80|NoteTitle::parse/|Email::parse  |                        time|Found)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 結果の読み方

- `time: [61.085 ns 63.692 ns 66.979 ns]` は、1回あたりの時間です。真ん中が「いちばんありそうな値」、左右が「たぶん、この間に入る」という幅（信頼区間）です。`ns` はナノ秒（10 億分の 1 秒）です。
- `Collecting 100 samples in estimated 5.0002 s (111M iterations)` は、「5 秒で、1 億 1100 万回くり返し、100 個の測った値（サンプル）を集めた」という意味です。
- `Found 19 outliers` は、100 個のうち 19 個が、ほかから外れていた（外れ値）という知らせです。パソコンでほかの仕事も動いているので、ある程度は出ます。多いときは、ほかのプログラムを閉じて測り直します。
- `Gnuplot not found, using plotters backend` は、グラフを描く道具 Gnuplot が無いので、別の描き方を使う、という知らせで、エラーではありません。グラフつきの報告は `target/criterion/report/index.html` にできます。

##### 解説2: `black_box` で、最適化に消されないようにする

- release ビルドのコンパイラーは、「結果を使わない計算」や「毎回同じ答えになる計算」を、まるごと省くことがあります。すると、何も測っていないのに、とても速い数が出ます。
- `black_box(input)` は、「この値は、何か分からないもの」とコンパイラーに思わせ、計算を省かせない印です（標準ライブラリの `std::hint::black_box`）。criterion の `b.iter(|| …)` は、戻り値も `black_box` に通しています。

### もう一歩: 実務に近い形

#### 例8-2: ログインのどこが遅いかを、直す前に測る

「ログインが遅い」と感じたら、まず**内訳**を測ります。`POST /login` 全体と、その中の2つの部分（DB でユーザーを探す・パスワードの照合）を、別々に測ります。

> **▼ このコードがやること（先に日本語で）:** alice を登録してから、①DB でユーザーを探す（`find_by_email`）、②パスワードの照合（`verify_password`）、③`POST /login` 全体、の3つを、それぞれ 5 回ずつ測り、いちばん短い時間（ほかの仕事の影響がいちばん少ない回）を表示します。照合が全体のどのくらいかも計算します。同じテストを、debug と release で動かします。

```rust
@@FILE:staging_u08/u08_login_parts.rs@@
```

```powershell
cargo test --test u08_login_parts -- --nocapture
cargo test --release --test u08_login_parts -- --nocapture
```

**▼ 実行結果（debug）**

```text
@@OUTGREP:u08_login_debug:^(---|DB|パスワード|POST /login 全体|照合)@@
```

**▼ 実行結果（release）**

```text
@@OUTGREP:u08_login_release:^(---|DB|パスワード|POST /login 全体|照合)@@
```

- debug では、照合が全体の 86% でした。DB で探すのは 1.43 ms で、全体の 0.3% です。DB の問い合わせを速くしても、ログインはほとんど速くなりません。**時間の大きい所から**直します。
- release では、照合が 386.72 ms → 20.27 ms（約 19 倍）、全体が 450.69 ms → 35.12 ms（約 13 倍）になりました。照合の割合は 58% に下がり、残りの約 13 ms は、セッションを作って DB に書く所（`log_in`）などです。
- Argon2 の照合が遅いのは、**わざと**です。総当たりで当てにくくするため、1回の計算に時間とメモリがかかるように作られています（第8巻 UNIT 02）。だから、ここで「照合を速くする」のは、守りを弱めることになります（問2）。

#### 例8-3: メモリを借りる回数とピークを測る

Rust のプログラムは、`String` や `Vec` を作るたびに、メモリを借ります。借りる係（global allocator）は、ふつうは OS の係（`System`）ですが、`#[global_allocator]` で、自分の係に替えられます。ここでは、本物の係に任せつつ、**数だけ数える係**に替えて、主な操作が、メモリをどれだけ使うかを測ります。

> **▼ このコードがやること（先に日本語で）:** `Counting` は、借りる（`alloc`）たびに回数と量を足し、返す（`dealloc`）たびに量を引き、「同時に借りていた量」のいちばん多い所（ピーク）を覚えます。この係を、このテストのプログラムだけの global allocator にします。そのうえで、①20 件のノートの一覧（`GET /notes`）、②パスワードのハッシュ、③パスワードの照合、のそれぞれで、借りた回数と、始めより何 MiB 多く借りていたか（ピーク）を表示します。速さではなくメモリを見るので、release で動かします。

```rust
@@FILE:staging_u08/u08_memory.rs@@
```

```powershell
cargo test --release --test u08_memory -- --nocapture --test-threads=1
```

**▼ 実行結果**

```text
@@OUTGREP:u08_memory:^(GET /notes（|パスワード)@@
```

- 一覧の画面は、529 回借りましたが、ピークは 0.07 MiB（約 70 KB）でした。小さな `String` をたくさん作っては返しています。
- パスワードのハッシュと照合は、どちらも **19.00 MiB** でした。RustNote の Argon2 の設定は `m=19456`（19,456 KiB ＝ 19 MiB。第8巻 UNIT 02 の PHC 文字列の `m=19456`）なので、そのとおりのメモリを、1回ごとに借りています。借りた回数は少なく（8 回・4 回）、大きな1かたまりを借りています。
- **同時にログインする人の数 × 19 MiB** が、Argon2 だけで必要になります。10 人なら 190 MiB、50 人なら 950 MiB です。小さなマシン（Fly.io の 256 MiB など）では、同時のログインが多いと、メモリが足りなくなります。どうなるかは、UNIT 09 で、本物の負荷をかけて確かめます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 速さを debug ビルドで測った

同じ計算（`あ` を 80 個並べた題の `parse` を 10 万回）を、debug と release で測りました。

```rust
@@FILE:staging_u08/u08_debug_vs_release.rs@@
```

```text
@@OUTGREP:u08_err_debug:^debug@@
@@OUTGREP:u08_err_release:^release@@
```

- **何が起きたか:** debug は 1 回 393 ns、release は 47 ns で、**約 8 倍**違いました。例8-2の Argon2 では約 19 倍でした。どのくらい違うかは、処理によって変わります。
- **困ること:** debug で「この関数が遅い」と思って直しても、release では、もともと速かった、ということがあります。逆に、debug では目立たない所が、release で目立つこともあります。
- **直し方:** 速さは、本番と同じ release で測ります（`cargo bench` は、何も書かなくても release の設定（`bench` プロファイル）で組み立てます）。`cargo test` で測るときは `--release` を付けます。

#### エラー2（エラーが出ない間違い）: `harness = false` を書き忘れた

`benches/parse_noharness.rs`（中身は例8-1と同じ）を、`Cargo.toml` に `[[bench]] name = "parse_noharness"` だけ書いて（`harness = false` なし）動かした場合です。

```text
@@OUTGREP:u08_err_harness:^(     Running|running|test result)@@
```

- **何が起きたか:** `running 0 tests` と出て、何も測られずに終わりました。エラーにはなりません。
- **なぜか:** `harness = false` を書かないと、Rust の標準の試験の係（libtest）が、ベンチマークのプログラムの `main` を用意し、`#[bench]` や `#[test]` を探します。criterion の `criterion_main!` が作った `main` は使われず、探す物も無いので、0 個です。
- **直し方:** criterion のベンチマークには、必ず `harness = false` を書きます（例8-1の3行）。「速さの数が1つも出ない」ときは、まずここを確かめます。

#### エラー3（エラーが出ない間違い）: 1回だけ測った

例8-2の release の出力から、アクセスログの行（`POST /login` 1回ずつの時間）を見ます。

```text
@@OUTGREP:u08_login_release:^POST /login@@
```

- **何が起きたか:** 1回目は 89.52 ms、2回目は 104.77 ms で、4回目からは 35〜36 ms でした。始めのうちは、プログラムやデータが、まだ CPU の近くの速い記憶（キャッシュ）に入っていなかったり、ほかのテストの準備と重なったりして、遅くなります。
- **直し方:** 何回か測って、いちばん短い回か、真ん中の値（中央値）を使います。例8-2は 5 回の最短、criterion は「温める時間（Warming up）」をとってから、100 個のサンプルで統計をとります。

### RustNote ではここで使う

- ログインの時間の大部分は、Argon2 の照合です。これは守りのための遅さなので、ほかの所を速くしても、ログインはほとんど速くなりません。RustNote は、照合を `spawn_blocking` で別のスレッドに出しているので、照合の間も、ほかのリクエストは動けます（第8巻 UNIT 02）。
- 本番は release で動かします。RustNote の `Dockerfile` は `cargo build --release` で組み立てます（第12巻）。debug の時間（450 ms）で心配しすぎないようにします。
- メモリは、1回のログイン・登録ごとに 19 MiB です。同時のログインの数が、マシンのメモリの大きさを決めます（UNIT 09）。一覧の画面のような、ふつうのページは、1回 0.1 MiB もかかりません。
- この単元の `benches/` と、数える global allocator のテストは、練習用のコピーにだけ足しました（`criterion` も同じ）。

### 練習問題

**問1:** 例8-1で、`NoteTitle::parse` は、`a` が 63.692 ns、`あ` が 79.937 ns、`😀` が 86.609 ns でした。文字数は、どれも 80 です。なぜ時間が違うのでしょうか。

**問2:** ログインを速くするために、Argon2 の設定（`m`・`t`）を小さくしてよいでしょうか。

**問3:** 例8-3の数から、同時に 50 人がログインしたとき、Argon2 だけで何 MiB 要るかを計算してください。メモリが 256 MiB のマシンでは、どうなりそうでしょうか。

### 解答と解説

**問1の解答:**

- `parse` は、前後の空白を除き（`trim`）、`chars().count()` で文字を数え、`to_string()` で写します。どれも、文字ではなく**バイト**の数に比例して仕事が増えます。UTF-8 では、`a` は1バイト、`あ` は3バイト、`😀` は4バイトなので、80 文字は 80・240・320 バイトです。
- ただし、時間は 3 倍・4 倍にはなっていません（63.7 → 79.9 → 86.6 ns）。`String` を作るためにメモリを借りる時間など、バイト数に関係しない部分が、半分以上を占めているからです。測ってみると、「バイト数に比例するはず」という予想よりも小さな差でした。

**問2の解答:**

- 小さくしてはいけません。Argon2 の遅さとメモリは、DB が漏れたときに、攻撃する人がパスワードを総当たりで当てる時間を長くするためのものです（第8巻 UNIT 02・UNIT 10）。RustNote の `m=19456`・`t=2`・`p=1` は、OWASP が勧める**最低**の設定です。
- 速くしたいなら、ログインの回数そのものを減らす（セッションを使う。毎回のリクエストでは照合しない）方向で考えます。RustNote は、ログインのときだけ照合し、そのあとはセッションのトークンの SHA-256 を DB で探すだけなので、ふつうのページは速いままです。

**問3の解答:**

- 50 人 × 19 MiB ＝ **950 MiB** です。256 MiB のマシンでは、Argon2 だけで足りません。RustNote のプログラムそのものや DB の分もあるので、もっと少ない人数で足りなくなります。
- 同時に照合する数に上限を付ける（待たせる）か、マシンのメモリを増やす必要があります。実際に、何人で、どう壊れるかは、UNIT 09 で、負荷をかけて確かめます。

### まとめ

- criterion は、何百万回くり返して、1回あたりの時間と、その幅・外れ値を出す。`benches/` に置き、`harness = false` を書く（忘れると `running 0 tests` で何も測らない）。`black_box` で、計算を最適化に消されないようにする。
- 遅いと感じたら、まず部分ごとに測る。ログインは、debug で 86%、release で 58% が Argon2 の照合で、DB で探すのは数 ms だった。照合の遅さは守りのためなので、設定は下げない。速さは release で測る（parse は約 8 倍、照合は約 19 倍違った）。何回か測って、最短か中央値を使う。
- 数える global allocator で、一覧の画面はピーク 0.07 MiB、Argon2 のハッシュと照合は1回 19.00 MiB と分かった。同時のログインの数 × 19 MiB が、マシンのメモリの大きさを決める。
