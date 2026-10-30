"""unit07.src.md の @@名前@@ の下書きの所に、本文を入れる。"""
from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit07.src.md"
s = p.read_text(encoding="utf-8")

PARTS = {}

PARTS["SLOFIG"] = """<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="30" font-size="13" font-weight="700" fill="#0f172a">1001 回のリクエスト（SLO: 成功が 99.9% 以上）</text>
<rect x="20" y="44" width="680" height="34" rx="4" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="66" font-size="12" fill="#047857" text-anchor="middle">成功（2xx・3xx・4xx）: 998 回</text>
<rect x="640" y="44" width="60" height="34" rx="4" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="670" y="66" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">5xx 3</text>
<text x="20" y="110" font-size="12" fill="#0f172a">エラーの予算 = 1001 回 × 0.1% ≒ 1 回</text>
<rect x="20" y="122" width="40" height="28" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="40" y="141" font-size="11" fill="#c2410c" text-anchor="middle">1</text>
<rect x="70" y="122" width="120" height="28" rx="4" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="130" y="141" font-size="11" fill="#b91c1c" text-anchor="middle">使った 3（300%）</text>
<text x="210" y="141" font-size="11" fill="#b91c1c">→ 予算を使い切った: 新しい機能より先に、5xx の原因を直す</text>
<text x="20" y="182" font-size="12" fill="#047857">UNIT 04 の直し方（題の制御文字を断る）を当てると: 5xx 0 回・4xx 3 回・成功の割合 100.00%</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: SLO とエラーの予算。NUL で始まる題の 500 が3回あると、1001 回の 0.1%（1 回）の予算を 300% 使ってしまう。400 は「利用者のまちがい」なので、失敗に数えない</div>
</div>"""

PARTS["U07_LOGS_NOTES"] = """- 1件のリクエストが、JSON の1行になりました。`"status":401` のように、項目が名前つきで入っているので、「`status` が 500 以上の行だけ」「`path` が `/login` の行だけ」と、文を切り分けずに絞り込めます。たくさんのサーバーのログを集める道具（ログの置き場）も、この形を読めます。
- 500 のときは、`"level":"ERROR"` の行（原因の連なり `causes`）と、`"level":"INFO"` のアクセスログの行の、2行が出ました。`level` で、「ERROR だけ」を見ることもできます。
- `elapsed_ms` の `957.9676000000001` のような端数は、小数（`f64`）のかけ算でできる、ふつうの誤差です。読む人の画面では、丸めて表示します。
- `timestamp` は、世界の標準の時刻（UTC。日本の時刻から 9 時間引いたもの）です。たくさんのサーバーのログを並べても、時刻がずれないようにするためです。"""

PARTS["U07_METRICS"] = """ログは1件ずつなので、「今、1分に何回 500 が出ているか」を知るには、全部を数える必要があります。**メトリクス**は、はじめから数えておいた数を、決まった URL で返します。ここでは、数を集めて保存する道具 Prometheus が読める文字の形で、`/metrics` を返す部品を、練習用のコピーに足しました。

> **▼ このコードがやること（先に日本語で）:** `METRICS` は、アプリ全体で1つの数の入れ物です。`AtomicU64`（いくつのスレッドから同時に足しても、数がこわれない整数）で、状態コードの種類（1xx〜5xx）ごとの数と、応答時間が「5 ms 以下・10 ms 以下・…・1 秒以下」だった数（ヒストグラム）を数えます。例7-1のアクセスログの層が、リクエストのたびに `METRICS.record(status, elapsed)` を呼びます。`GET /metrics` は、`render()` で、その数を文字にして返します。

```rust
@@FILE:variants/metrics.rs@@
```

本物のサーバーでも確かめます。`main.rs` で、起動のときに subscriber を入れます（`RUST_LOG` で量を変えられるようにします）。

```diff
@@FILE:out/u07_diff_main.txt@@
```

`topcoat asset bundle` と `cargo build` のあと、サーバーを `127.0.0.1:3012` で起動し、`GET /login`・`GET /notes`（ログインしていないので 303）・`GET /nothing-here`（404）を送ってから、`/metrics` を読みました。

```powershell
curl.exe -i http://127.0.0.1:3012/metrics
```

```text
@@OUT:u07_server_metrics_head@@
@@OUT:u07_server_metrics@@
```

- 2xx・3xx・4xx が1回ずつで、3回とも 5 ms 以内でした。`le="0.005"` の `3` は「0.005 秒以下だったのが 3 回」、`+Inf` は全部の回数です。区切りは「以下の数」を数えるので、右へいくほど数が増えます（減ることはありません）。
- `/health` と `/metrics` 自身は、数えていません。見張りの道具が数十秒ごとに読みに来るので、数えると、利用者のリクエストの数が分からなくなるからです。

サーバーの標準出力と、標準エラー出力です。

```text
@@OUT:u07_server_stdout@@
@@OUT:u07_server_stderr@@
```

- アクセスログは、JSON の行として**標準出力**に出ました。起動のときの「マイグレーションを適用しました」などは、`eprintln!` のままなので、**標準エラー出力**に出ています。ログの置き場に送るときは、両方を集めるか、起動のときの行も `tracing` にそろえます。"""

PARTS["U07_SLO"] = """**SLO** は、「利用者から見て、どのくらい良ければ十分か」を、測れる数で決めたものです。ここでは、次の2つを決めます。

| SLO | 目標 | 数え方 |
|---|---|---|
| 成功の割合 | 99.9% 以上 | 5xx 以外の数 ÷ 全部の数（4xx は利用者のまちがいなので、成功に数える） |
| 速さ | 95% 以上が 50 ms 以内 | `le="0.05"` の数 ÷ 全部の数 |

> **▼ このコードがやること（先に日本語で）:** alice を登録してから、1000 回のリクエストを送ります。900 回は一覧を見る、97 回はノートを作る、3 回は NUL で始まる題（UNIT 04 で見つけたバグ）です。そのあと `/metrics` を読み、全部の数・5xx の数・50 ms 以内の数から、成功の割合・速さ・エラーの予算を計算して表示します。

```rust
@@FILE:staging_u07/u07_slo.rs@@
```

```powershell
cargo test --test u07_slo -- --nocapture
```

**▼ 実行結果**

```text
@@OUTGREP:u07_slo:^(rustnote_requests_total\\{class="[2-5]xx"\\}|rustnote_request_duration_seconds_bucket\\{le="0.0|---|成功|50 ms|エラーの予算)@@
```

- 速さの SLO（95% が 50 ms 以内）は、97.8% で守れました。成功の割合は、NUL の題の 500 が3回あったので 99.70% で、目標の 99.9% に届きませんでした。
- **エラーの予算:** 1001 回の 0.1% は約 1 回です。3 回の 500 で、予算の 300% を使いました。予算を使い切ったら、新しい機能を足すより先に、5xx の原因を直す、と**前もって決めておく**のが、エラーの予算の使い方です（図の2つ目）。
- UNIT 04 の直し方（題の制御文字を断る）も当てて、もう一度動かしました。

```text
@@OUTGREP:u07_slo_fixed:^(rustnote_requests_total\\{class="[45]xx"\\}|成功|50 ms|エラーの予算)@@
```

- 3回の NUL の題は 400 になり、5xx は 0 回、成功の割合は 100.00% になりました。速さの数は、動かすたびに少し変わります（97.8% と 97.3%）。
- 本物では、1日・1か月のような**期間**で数えます。たとえば 1 か月に 300 万回なら、99.9% の予算は 3000 回です。テストの 1000 回は、計算の練習のための小さな数です。

**health について:** RustNote の `/health` は、DB に `SELECT 1` を送り、届けば `200 {"status":"ok"}`、届かなければ `503` を返します（第9巻 UNIT 10）。Fly.io は、これを見て、そのマシンにリクエストを送るかを決めます。ただし、`/health` は、**遅い・500 が多い**ことは知らせません。上の SLO の計算では、5xx が 3 回ありましたが、その間も `/health` は 200 のままです。「動いているか」は health、「良く動いているか」はメトリクスと SLO で見ます。"""

PARTS["U07_ERRORS"] = """#### エラー1（エラーが出ない間違い）: subscriber を入れずに、ログが出ると思った

```rust
@@FILE:staging_u07/e07_no_subscriber.rs@@
```

```text
@@OUTGREP:u07_err_none:^(running|2つ|test )@@
```

- **何が起きたか:** 2つのリクエストを送りましたが、アクセスログの JSON は1行も出ませんでした。`tracing::info!` は、「記録を作って、受け取る人（subscriber）に渡す」だけで、受け取る人がいなければ、何も書きません。エラーにもなりません。
- **直し方:** プログラムの最初（`main` や、テストの最初）で、`tracing_subscriber::fmt()…init()` を1回だけ呼びます。本番で「ログが出ない」ときは、まず subscriber を入れたか、`RUST_LOG` で絞りすぎていないか（問1）を確かめます。

#### エラー2: テストごとに subscriber を入れた（2回目の init で panic）

```rust
@@FILE:staging_u07/e07_init_twice.rs@@
```

```text
@@OUTGREP:u07_err_twice:^(test |thread|Unable)@@
```

- **何が起きたか:** `init()` は、プログラム全体で1つの subscriber（global default）を入れます。同じテストのファイルの2つのテストは、同じプログラムの中で同時に動くので、2回目の `init()` が「もう入っています」（`a global default trace dispatcher has already been set`）と panic しました。先に入れたほうは通り、`/signup` のログも出ています。どちらが先かは、その時しだいです。
- **直し方:** テストでは `init()` の代わりに `try_init()` を使い、2回目の失敗を無視します（`let _ = tracing_subscriber::fmt().json().try_init();`）。または、例7-1のように、1つのテストのファイルには、subscriber を入れるテストを1つだけにします。

#### エラー3: 数える入れ物を、`HashMap` の `static` で作ろうとした（E0015）

```rust
@@FILE:staging_u07/e07_static.rs@@
```

```text
@@OUTGREP:u07_err_static:^(error|  -->| *[0-9]* \\||  = )@@
```

- **何が起きたか:** `static` の値は、プログラムが動き出す前に決まっていなければならないので、`const fn`（組み立てのときに計算できる関数）しか呼べません。`HashMap::new()` は `const fn` ではないので、E0015 になりました。
- **直し方:** `note:` のとおり、`std::sync::LazyLock::new(|| Mutex::new(HashMap::new()))` で、初めて使うときに作ります。例7-2の `METRICS` は、`AtomicU64::new(0)` が `const fn` なので、そのまま `static` で作れました。数える種類が決まっているなら、`HashMap` より、配列の `AtomicU64` のほうが、ロックも要らず速いです。"""

PARTS["U07_RUSTNOTE"] = """- 今の RustNote は、アクセスログ（`src/routes.rs` の `access_log`）・500 の原因（`src/views.rs` の `log_if_internal`）・health の失敗（`src/routes/health.rs`）の3か所で、`eprintln!` の文を書いています。メトリクスと SLO の計算は、ありません。
- この単元の `tracing` と `/metrics` は、練習用のコピーにだけ足しました（教材の `Rust/rustnote` は変えていません）。足すときの注意（改善点）:
  - `/metrics` は、だれでも読めると、リクエストの数や 500 の数が外から分かります（問2）。Fly.io では、内側の通信（6PN）だけで読めるようにする、決まった人しか読めない別のポートにする、などの方法があります。
  - 構造化ログにも、パスワード・トークン・メールアドレスを、そのまま書きません（第8巻 UNIT 10）。項目に分けると、うっかり `?form` のように構造体ごと書きやすくなるので、注意します。
  - 起動のときの `eprintln!` も、`tracing` にそろえると、ログの置き場で1つの形として扱えます。
- 公開したあとの見張り（数を集めて、グラフと知らせにする）は、第12巻で扱います。"""

PARTS["U07_Q"] = """**問1:** 例7-2のサーバーを、`RUST_LOG=warn` を付けて起動し、同じリクエストを送ると、標準出力のログはどうなるでしょうか。500 のときの `ERROR` の行は、どうでしょうか。

**問2:** `/metrics` を、インターネットのだれでも読めるようにしておくと、何が困るでしょうか。

**問3:** 1日に 10 万回のリクエストがあるサービスで、成功の割合の SLO を 99.9%（1 か月 30 日）にすると、1 か月のエラーの予算は何回でしょうか。例7-3の NUL の題の 500 が、1日に 3 回ずつ起きると、何日で予算を使い切るでしょうか。"""

PARTS["U07_A"] = """**問1の解答:**

```powershell
$env:RUST_LOG = "warn"
.\\target\\debug\\rustnote.exe
```

標準出力（`GET /login` と `GET /nothing-here` を送ったあと）:

```text
@@OUT:u07_warn_stdout@@
```

標準エラー出力:

```text
@@OUT:u07_warn_stderr@@
```

- 標準出力には、何も出ませんでした。アクセスログは `info` なので、`warn` 以上（`warn` と `error`）だけを書く設定では、捨てられます。起動のときの行は `eprintln!` なので、`RUST_LOG` と関係なく出ます。
- 500 のときの行は `tracing::error!` なので、`warn` でも書かれます。ふだんは `info` で動かし、ログが多すぎるときは `warn` にする、`/login` まわりだけ `debug` にする（`RUST_LOG=rustnote::routes::login=debug`）、のように、`EnvFilter` で細かく決められます。

**問2の解答:**

- 例7-2の `rustnote_requests_total{class="5xx"}` を見ると、「今、エラーが増えている」ことが、外から分かります。攻撃する人は、それを見て、「この入力で 500 が出る」と探す手がかりにできます。リクエストの数から、利用者の数の見当もつきます。
- 読みに来るリクエストが多すぎると、`render()` の仕事が、サーバーの負担になります。
- だから、`/metrics` は、見張りの道具だけが届く場所（内側の通信・別のポート・決まった人だけが知る合言葉）に置きます。

**問3の解答:**

- 1 か月の全部の回数は、10 万回 × 30 日 ＝ 300 万回です。99.9% の SLO なら、エラーの予算は 300 万回 × 0.1% ＝ **3000 回**です。
- 1日に 3 回ずつなら、1000 日（約 2 年 9 か月）かかるので、この 500 だけでは、予算を使い切りません。ただし、例7-3の 1001 回では 300% でした。予算は「割合」なので、全部の回数が少ないと、少しの失敗でも大きく見えます。回数の少ないサービスでは、期間を長くとるか、「5xx の回数そのもの」も見張ります。
- 予算が残っていても、5xx は、利用者にとっては、ノートが作れなかった失敗です。予算は「どこまで我慢するか」ではなく、「直す順番を決めるための物差し」として使います。"""

PARTS["U07_SUMMARY"] = """- `tracing` で、アクセスログを、`method`・`path`・`status`・`elapsed_ms` の項目つきの JSON の1行にした。項目で絞り込め、500 は `ERROR` の行と原因の連なりで分かる。subscriber を入れないと何も書かれず、`init()` を2回呼ぶと panic する（テストでは `try_init()`）。`RUST_LOG=warn` で、`info` のアクセスログは出なくなる。
- `/metrics` で、状態コードの種類ごとの数と、応答時間のヒストグラムを、Prometheus の形で返した。数える入れ物は、`AtomicU64` の配列なら `static` で作れる（`HashMap` は E0015、`LazyLock` を使う）。`/metrics` は、だれでも読めない場所に置く。
- SLO は、成功の割合と速さの目標を、数で決めたもの。1001 回のうち NUL の題の 500 が 3 回で、99.9% の予算（1 回）を 300% 使い、直すと 100.00% になった。`/health` は「動いているか」を答え、「良く動いているか」はメトリクスと SLO で見る。"""

for key, text in PARTS.items():
    marker = f"@@{key}@@"
    assert s.count(marker) == 1, key
    s = s.replace(marker, text)
assert "@@U07_" not in s and "@@SLOFIG@@" not in s
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")
