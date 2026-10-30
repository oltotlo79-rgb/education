## UNIT 07: tracing・メトリクス・health・SLO

### この単元のゴール

- RustNote のアクセスログを、tracing の**構造化ログ**（項目に分けた JSON の1行）にして、あとから項目で絞り込める形にできる
- 状態コードの種類ごとの数と、応答時間の分布（ヒストグラム）を数え、`/metrics` で Prometheus の形で返せる
- `/health` が知らせること（DB まで届くか）と、知らせないこと（速いか・エラーが多いか）を言える
- SLO（成功の割合と速さの目標）と**エラーの予算**を決め、`/metrics` の数から、守れたかを計算できる

### たとえ話でつかむ

旅客機の操縦室を考えます。

- **航海日誌（ログ）:** 「10 時 02 分、高度 1 万メートルに着いた」「10 時 15 分、右のエンジンの温度が上がった」と、出来事を1件ずつ書きます。あとから「何が起きたか」を調べるのに使います。項目（時刻・高さ・エンジン）を決めて書くと、「右のエンジンの記録だけ」と探しやすくなります（構造化ログ）。
- **計器（メトリクス）:** 速さ・高さ・燃料の**数**を、いつも表示しています。1件ずつではなく、まとめた数です。針が赤い所に入ったら、すぐ分かります。
- **エンジンのランプ（health）:** 「エンジンがかかっているか」だけを、緑か赤で知らせます。かかっていても、燃料が少ないかどうかは、ランプでは分かりません。
- **定時の目標（SLO）:** 「1 か月の便の 99% は、定時に着く」と決めます。1 か月 1000 便なら、遅れてよいのは 10 便まで。これが**エラーの予算**です。予算を使い切ったら、新しい便を増やすより先に、遅れの原因を直します。

| 旅客機 | RustNote |
|---|---|
| 航海日誌 | tracing の構造化ログ（例7-1） |
| 計器 | `/metrics` の数（例7-2） |
| エンジンのランプ | `/health`（DB に `SELECT 1` が届くか） |
| 定時の目標 | SLO（成功が 99.9% 以上・95% が 50 ms 以内、など。例7-3） |
| 遅れてよい便の数 | エラーの予算（1000 回なら 1 回） |

> **用語:** 構造化ログ（ログを、文の1行ではなく、`"status":500` のような**項目**の集まりで書いたもの。あとから項目で絞り込んだり、数えたりできる）

<!-- -->

> **用語:** メトリクス（動いている様子を表す**数**。リクエストの数・失敗の数・応答時間の分布など。ログより小さく、グラフにしやすい）

<!-- -->

> **用語:** SLO（Service Level Objective：サービスの目標。「成功の割合が 99.9% 以上」のように、利用者から見た良さを、測れる数で決めたもの）

<!-- -->

> **用語:** エラーの予算（error budget：SLO の残り。99.9% なら、0.1% までは失敗してよい。1000 回なら 1 回）

### 図で理解する

3つの見え方と、それぞれの使い道です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="16" width="220" height="220" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="42" font-size="14" font-weight="700" fill="#1d4ed8" text-anchor="middle">ログ（tracing）</text>
<text x="120" y="62" font-size="11" fill="#1d4ed8" text-anchor="middle">1件ずつ・くわしい</text>
<text x="24" y="92" font-size="10" fill="#0f172a" font-family="Consolas, monospace">{"status":500,</text>
<text x="24" y="108" font-size="10" fill="#0f172a" font-family="Consolas, monospace"> "path":"/notes",…}</text>
<text x="24" y="140" font-size="11" fill="#0f172a">・何が起きたかを、あとで調べる</text>
<text x="24" y="164" font-size="11" fill="#0f172a">・項目で絞り込む</text>
<text x="24" y="188" font-size="11" fill="#b91c1c">・量が多い（全部のリクエスト）</text>
<text x="24" y="212" font-size="11" fill="#b91c1c">・秘密を書かない（第8巻）</text>
<rect x="250" y="16" width="220" height="220" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="42" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">メトリクス（/metrics）</text>
<text x="360" y="62" font-size="11" fill="#047857" text-anchor="middle">まとめた数</text>
<text x="264" y="92" font-size="10" fill="#0f172a" font-family="Consolas, monospace">requests_total{class="5xx"} 3</text>
<text x="264" y="108" font-size="10" fill="#0f172a" font-family="Consolas, monospace">duration_bucket{le="0.05"} …</text>
<text x="264" y="140" font-size="11" fill="#0f172a">・グラフにする・見張る</text>
<text x="264" y="164" font-size="11" fill="#0f172a">・SLO を計算する（例7-3）</text>
<text x="264" y="188" font-size="11" fill="#b91c1c">・どのリクエストかは分からない</text>
<text x="264" y="212" font-size="11" fill="#b91c1c">・だれでも見られないようにする</text>
<rect x="490" y="16" width="220" height="220" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="42" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">health（/health）</text>
<text x="600" y="62" font-size="11" fill="#c2410c" text-anchor="middle">動いているか</text>
<text x="504" y="92" font-size="10" fill="#0f172a" font-family="Consolas, monospace">200 {"status":"ok"}</text>
<text x="504" y="108" font-size="10" fill="#0f172a" font-family="Consolas, monospace">503 {"status":"unavailable"}</text>
<text x="504" y="140" font-size="11" fill="#0f172a">・Fly.io が、送り先を決める</text>
<text x="504" y="164" font-size="11" fill="#0f172a">・DB まで届くかを見る</text>
<text x="504" y="188" font-size="11" fill="#b91c1c">・遅い・エラーが多いは</text>
<text x="504" y="206" font-size="11" fill="#b91c1c">　分からない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: ログ・メトリクス・health。ログは「何が起きたか」、メトリクスは「どのくらい起きているか」、health は「今、受け付けられるか」を答える</div>
</div>

SLO とエラーの予算です（例7-3の数）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
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
</div>

### まず動かす最小コード

#### 例7-1: アクセスログを、tracing の構造化ログにする

RustNote のアクセスログは、`eprintln!` で「`POST /login -> 401 Unauthorized (398ms)`」という文を書いています。人が読むには十分ですが、「500 だけ」「`/login` だけ」と絞り込むには、文を切り分ける必要があります。これを、`tracing` の構造化ログにします。練習用のコピーに、2つのクレートを足します。

```powershell
cargo add tracing@=0.1.44
cargo add tracing-subscriber@=0.3.23 --features json,env-filter
```

```text
      Adding tracing =0.1.44 to dependencies
      Adding tracing-subscriber =0.3.23 to dependencies
      Adding matchers v0.2.0
      Adding nu-ansi-term v0.50.3
      Adding sharded-slab v0.1.7
      Adding thread_local v1.1.10
      Adding tracing-log v0.2.0
      Adding tracing-serde v0.2.0
      Adding tracing-subscriber v0.3.23
      Adding valuable v0.1.1
```

アクセスログの層（`src/routes.rs`）と、500 の原因を書く所（`src/views.rs`）を、`tracing` の書き方に替えます。

```diff
--- a/src/routes.rs
+++ b/src/routes.rs
@@ -19,6 +19,7 @@
 pub mod health;
 pub mod login;
 pub mod logout;
+pub mod metrics;
 pub mod notes;
 pub mod signup;
 
@@ -64,26 +65,28 @@
 // error_boundary（404 の画面）まで届ける
 not_found!();
 
-/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
+/// アクセスログ: 1つのリクエストを、項目に分けた1件の記録（構造化ログ）にし、数える
 #[layer]
 async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
     let started = std::time::Instant::now();
     let result = next.run(cx, body).await;
-    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
-    if uri(cx).path() != "/health" {
-        let status = match &result {
-            Ok(response) => response.status().to_string(),
+    // ヘルスチェックと、数を読みに来る /metrics は、ログにも数にも入れない
+    if uri(cx).path() != "/health" && uri(cx).path() != "/metrics" {
+        let elapsed = started.elapsed();
+        let status: u16 = match &result {
+            Ok(response) => response.status().as_u16(),
             // ページが Err(see_other(..)) で返したリダイレクトは、ここではまだ Err に見える
-            Err(error) if error.downcast_ref::<SeeOther>().is_some() => "303 See Other".to_string(),
-            Err(error) => format!("error ({error})"),
+            Err(error) if error.downcast_ref::<SeeOther>().is_some() => 303,
+            Err(_) => 500,
         };
-        eprintln!(
-            "{} {} -> {} ({:?})",
-            method(cx),
-            uri(cx).path(),
+        tracing::info!(
+            method = %method(cx),
+            path = uri(cx).path(),
             status,
-            started.elapsed()
+            elapsed_ms = elapsed.as_secs_f64() * 1000.0,
+            "request"
         );
+        metrics::METRICS.record(status, elapsed);
     }
     result
 }
```

```diff
--- a/src/views.rs
+++ b/src/views.rs
@@ -244,6 +244,6 @@
 fn log_if_internal(status: StatusCode, error: &Error) {
     if status == StatusCode::INTERNAL_SERVER_ERROR {
         let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();
-        eprintln!("[error] {}", causes.join(" <- "));
+        tracing::error!(causes = causes.join(" <- "), "internal error");
     }
 }
```

> **▼ このコードがやること（先に日本語で）:** `tracing::info!(method = %method(cx), path = …, status, elapsed_ms = …, "request")` は、「`request` という出来事を、`method`・`path`・`status`・`elapsed_ms` の4つの項目つきで記録する」という意味です。`%` は「Display（ふだんの表示）で書く」印です。記録を、どこに・どんな形で書くかは、**subscriber**（受け取る人）が決めます。下のテストは、subscriber に「標準出力へ、JSON で」を入れてから、いくつかのリクエストを送ります。

```rust
//! 第11巻 UNIT 07: tracing の構造化ログ。リクエストを送り、JSON の1行ずつの記録を見る。
//! （RustNote のコピーに、tracing と /metrics を足した版で動かす）
//!
//! 実行: cargo test --test u07_logs -- --nocapture

mod common;

use common::{encode, test_app};

#[tokio::test]
async fn structured_logs() {
    // 受け取る人（subscriber）を入れる。テストでは、標準出力に JSON で書く
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stdout)
        .init();

    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;
    let form = format!("email={}&password=wrong", encode("alice@example.com"));
    app.post_form("/login", &form, None).await;
    app.get("/notes/99", Some(&alice)).await;
    app.post_form("/notes", "title=%00&body=", Some(&alice)).await;
}
```

```powershell
cargo test --test u07_logs -- --nocapture
```

**▼ 実行結果**

```text
{"timestamp":"2026-10-30T19:30:38.309230Z","level":"INFO","fields":{"message":"request","method":"POST","path":"/signup","status":303,"elapsed_ms":957.9676000000001},"target":"rustnote::routes"}
{"timestamp":"2026-10-30T19:30:38.686341Z","level":"INFO","fields":{"message":"request","method":"POST","path":"/login","status":401,"elapsed_ms":376.7552},"target":"rustnote::routes"}
{"timestamp":"2026-10-30T19:30:38.732042Z","level":"INFO","fields":{"message":"request","method":"GET","path":"/notes/99","status":404,"elapsed_ms":45.3889},"target":"rustnote::routes"}
{"timestamp":"2026-10-30T19:30:38.739513Z","level":"ERROR","fields":{"message":"internal error","causes":"サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`"},"target":"rustnote::views"}
{"timestamp":"2026-10-30T19:30:38.739745Z","level":"INFO","fields":{"message":"request","method":"POST","path":"/notes","status":500,"elapsed_ms":7.3384},"target":"rustnote::routes"}
```

- 1件のリクエストが、JSON の1行になりました。`"status":401` のように、項目が名前つきで入っているので、「`status` が 500 以上の行だけ」「`path` が `/login` の行だけ」と、文を切り分けずに絞り込めます。たくさんのサーバーのログを集める道具（ログの置き場）も、この形を読めます。
- 500 のときは、`"level":"ERROR"` の行（原因の連なり `causes`）と、`"level":"INFO"` のアクセスログの行の、2行が出ました。`level` で、「ERROR だけ」を見ることもできます。
- `elapsed_ms` の `957.9676000000001` のような端数は、小数（`f64`）のかけ算でできる、ふつうの誤差です。読む人の画面では、丸めて表示します。
- `timestamp` は、世界の標準の時刻（UTC。日本の時刻から 9 時間引いたもの）です。たくさんのサーバーのログを並べても、時刻がずれないようにするためです。

### もう一歩: 実務に近い形

#### 例7-2: `/metrics` で、数を返す

ログは1件ずつなので、「今、1分に何回 500 が出ているか」を知るには、全部を数える必要があります。**メトリクス**は、はじめから数えておいた数を、決まった URL で返します。ここでは、数を集めて保存する道具 Prometheus が読める文字の形で、`/metrics` を返す部品を、練習用のコピーに足しました。

> **▼ このコードがやること（先に日本語で）:** `METRICS` は、アプリ全体で1つの数の入れ物です。`AtomicU64`（いくつのスレッドから同時に足しても、数がこわれない整数）で、状態コードの種類（1xx〜5xx）ごとの数と、応答時間が「5 ms 以下・10 ms 以下・…・1 秒以下」だった数（ヒストグラム）を数えます。例7-1のアクセスログの層が、リクエストのたびに `METRICS.record(status, elapsed)` を呼びます。`GET /metrics` は、`render()` で、その数を文字にして返します。

```rust
//! GET /metrics : 動いている様子を数で返す（第11巻 UNIT 07 で、RustNote のコピーに足して試す）。
//!
//! Prometheus（数を集めて保存する道具）が読める、文字の形（text exposition format）で返す。
//! - rustnote_requests_total{class="2xx"} … 状態コードの種類ごとの、リクエストの数
//! - rustnote_request_duration_seconds … 応答にかかった時間の分布（ヒストグラム）

use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use topcoat::{Result, context::Cx, router::route};

/// 応答時間の区切り（秒）。「この秒数以下だった数」を数える
const BUCKETS: [f64; 6] = [0.005, 0.01, 0.05, 0.1, 0.5, 1.0];

/// 数えるための入れ物。どのスレッドからも同時に足せるよう、AtomicU64 にする
pub struct Metrics {
    by_class: [AtomicU64; 5],
    buckets: [AtomicU64; BUCKETS.len()],
    count: AtomicU64,
    sum_micros: AtomicU64,
}

/// アプリ全体で1つの入れ物（AtomicU64::new は const fn なので、static で作れる）
pub static METRICS: Metrics = Metrics {
    by_class: [const { AtomicU64::new(0) }; 5],
    buckets: [const { AtomicU64::new(0) }; BUCKETS.len()],
    count: AtomicU64::new(0),
    sum_micros: AtomicU64::new(0),
};

impl Metrics {
    /// 1つのリクエストの結果を数える
    pub fn record(&self, status: u16, elapsed: Duration) {
        let class = usize::from(status / 100).clamp(1, 5) - 1;
        self.by_class[class].fetch_add(1, Ordering::Relaxed);
        let seconds = elapsed.as_secs_f64();
        for (bucket, limit) in self.buckets.iter().zip(BUCKETS) {
            if seconds <= limit {
                bucket.fetch_add(1, Ordering::Relaxed);
            }
        }
        self.count.fetch_add(1, Ordering::Relaxed);
        let micros = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
        self.sum_micros.fetch_add(micros, Ordering::Relaxed);
    }

    /// Prometheus の文字の形にする
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# TYPE rustnote_requests_total counter\n");
        for (i, counter) in self.by_class.iter().enumerate() {
            let value = counter.load(Ordering::Relaxed);
            let _ = writeln!(out, "rustnote_requests_total{{class=\"{}xx\"}} {value}", i + 1);
        }
        out.push_str("# TYPE rustnote_request_duration_seconds histogram\n");
        for (bucket, limit) in self.buckets.iter().zip(BUCKETS) {
            let value = bucket.load(Ordering::Relaxed);
            let _ = writeln!(out, "rustnote_request_duration_seconds_bucket{{le=\"{limit}\"}} {value}");
        }
        let count = self.count.load(Ordering::Relaxed);
        let sum = self.sum_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        let _ = writeln!(out, "rustnote_request_duration_seconds_bucket{{le=\"+Inf\"}} {count}");
        let _ = writeln!(out, "rustnote_request_duration_seconds_sum {sum}");
        let _ = writeln!(out, "rustnote_request_duration_seconds_count {count}");
        out
    }
}

#[route(GET)]
pub async fn metrics(_cx: &Cx) -> Result<String> {
    Ok(METRICS.render())
}
```

本物のサーバーでも確かめます。`main.rs` で、起動のときに subscriber を入れます（`RUST_LOG` で量を変えられるようにします）。

```diff
--- a/src/main.rs
+++ b/src/main.rs
@@ -22,6 +22,14 @@
 async fn main() -> anyhow::Result<()> {
     // .env が無くてもエラーにしない（本番では環境変数を直接設定する）
     dotenvy::dotenv().ok();
+    // ログを JSON の1行ずつで標準出力へ。RUST_LOG で量を変えられる（既定は info）
+    tracing_subscriber::fmt()
+        .json()
+        .with_env_filter(
+            tracing_subscriber::EnvFilter::try_from_default_env()
+                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
+        )
+        .init();
 
     let config = Config::from_env().context("設定を読み込めませんでした")?;
 
```

`topcoat asset bundle` と `cargo build` のあと、サーバーを `127.0.0.1:3012` で起動し、`GET /login`・`GET /notes`（ログインしていないので 303）・`GET /nothing-here`（404）を送ってから、`/metrics` を読みました。

```powershell
curl.exe -i http://127.0.0.1:3012/metrics
```

```text
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
vary: accept-encoding
content-length: 748
# TYPE rustnote_requests_total counter
rustnote_requests_total{class="1xx"} 0
rustnote_requests_total{class="2xx"} 1
rustnote_requests_total{class="3xx"} 1
rustnote_requests_total{class="4xx"} 1
rustnote_requests_total{class="5xx"} 0
# TYPE rustnote_request_duration_seconds histogram
rustnote_request_duration_seconds_bucket{le="0.005"} 3
rustnote_request_duration_seconds_bucket{le="0.01"} 3
rustnote_request_duration_seconds_bucket{le="0.05"} 3
rustnote_request_duration_seconds_bucket{le="0.1"} 3
rustnote_request_duration_seconds_bucket{le="0.5"} 3
rustnote_request_duration_seconds_bucket{le="1"} 3
rustnote_request_duration_seconds_bucket{le="+Inf"} 3
rustnote_request_duration_seconds_sum 0.001177
rustnote_request_duration_seconds_count 3
```

- 2xx・3xx・4xx が1回ずつで、3回とも 5 ms 以内でした。`le="0.005"` の `3` は「0.005 秒以下だったのが 3 回」、`+Inf` は全部の回数です。区切りは「以下の数」を数えるので、右へいくほど数が増えます（減ることはありません）。
- `/health` と `/metrics` 自身は、数えていません。見張りの道具が数十秒ごとに読みに来るので、数えると、利用者のリクエストの数が分からなくなるからです。

サーバーの標準出力と、標準エラー出力です。

```text
{"timestamp":"2026-10-30T19:31:33.585966Z","level":"INFO","fields":{"message":"request","method":"GET","path":"/login","status":200,"elapsed_ms":0.5281},"target":"rustnote::routes"}
{"timestamp":"2026-10-30T19:31:33.698385Z","level":"INFO","fields":{"message":"request","method":"GET","path":"/notes","status":303,"elapsed_ms":0.2939},"target":"rustnote::routes"}
{"timestamp":"2026-10-30T19:31:33.817840Z","level":"INFO","fields":{"message":"request","method":"GET","path":"/nothing-here","status":404,"elapsed_ms":0.35609999999999997},"target":"rustnote::routes"}
マイグレーションを適用しました: 0001_users
マイグレーションを適用しました: 0002_notes
マイグレーションを適用しました: 0003_sessions
RustNote を起動しました: http://127.0.0.1:3012
```

- アクセスログは、JSON の行として**標準出力**に出ました。起動のときの「マイグレーションを適用しました」などは、`eprintln!` のままなので、**標準エラー出力**に出ています。ログの置き場に送るときは、両方を集めるか、起動のときの行も `tracing` にそろえます。

#### 例7-3: SLO とエラーの予算を、数から計算する

**SLO** は、「利用者から見て、どのくらい良ければ十分か」を、測れる数で決めたものです。ここでは、次の2つを決めます。

| SLO | 目標 | 数え方 |
|---|---|---|
| 成功の割合 | 99.9% 以上 | 5xx 以外の数 ÷ 全部の数（4xx は利用者のまちがいなので、成功に数える） |
| 速さ | 95% 以上が 50 ms 以内 | `le="0.05"` の数 ÷ 全部の数 |

> **▼ このコードがやること（先に日本語で）:** alice を登録してから、1000 回のリクエストを送ります。900 回は一覧を見る、97 回はノートを作る、3 回は NUL で始まる題（UNIT 04 で見つけたバグ）です。そのあと `/metrics` を読み、全部の数・5xx の数・50 ms 以内の数から、成功の割合・速さ・エラーの予算を計算して表示します。

```rust
//! 第11巻 UNIT 07: 1日分のつもりのリクエスト（1000 回）を送り、/metrics の数から、
//! SLO（守ると決めた目標）を守れたかと、エラーの予算（あと何回まで失敗してよいか）を計算する。
//! （RustNote のコピーに、tracing と /metrics を足した版で動かす）
//!
//! 実行: cargo test --test u07_slo -- --nocapture

mod common;

use common::{body_text, test_app};

/// /metrics の本文から、名前とラベルが合う行の数を読む
fn read(metrics: &str, name: &str) -> f64 {
    metrics
        .lines()
        .find(|line| line.starts_with(name) && line[name.len()..].starts_with(' '))
        .and_then(|line| line.rsplit(' ').next())
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

#[tokio::test]
async fn one_day_of_traffic() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;

    // 900 回は一覧を見る、97 回はノートを作る、3 回は NUL で始まる題（UNIT 04 のバグ）
    for i in 0..1000 {
        if i % 100 == 0 && i < 300 {
            app.post_form("/notes", "title=%00&body=", Some(&alice)).await;
        } else if i % 10 == 0 {
            let form = format!("title=memo{i}&body=");
            app.post_form("/notes", &form, Some(&alice)).await;
        } else {
            app.get("/notes", Some(&alice)).await;
        }
    }

    let metrics = body_text(app.get("/metrics", None).await).await;
    println!("{metrics}");

    let total = read(&metrics, "rustnote_request_duration_seconds_count");
    let errors = read(&metrics, r#"rustnote_requests_total{class="5xx"}"#);
    let under_50ms = read(&metrics, r#"rustnote_request_duration_seconds_bucket{le="0.05"}"#);
    let availability = (total - errors) / total * 100.0;
    let fast = under_50ms / total * 100.0;
    // SLO: 成功が 99.9% 以上・95% のリクエストが 50 ms 以内
    let budget = total * (1.0 - 0.999);
    println!("--- SLO の確かめ（登録の1回を含めて {total} 回）");
    println!("成功の割合: {availability:.2}%（目標 99.9% 以上）");
    println!("50 ms 以内の割合: {fast:.1}%（目標 95% 以上）");
    println!("エラーの予算: {budget:.0} 回まで → 使った回数 {errors} 回（{:.0}%）", errors / budget * 100.0);
}
```

```powershell
cargo test --test u07_slo -- --nocapture
```

**▼ 実行結果**

```text
rustnote_requests_total{class="2xx"} 900
rustnote_requests_total{class="3xx"} 98
rustnote_requests_total{class="4xx"} 0
rustnote_requests_total{class="5xx"} 3
rustnote_request_duration_seconds_bucket{le="0.005"} 360
rustnote_request_duration_seconds_bucket{le="0.01"} 842
rustnote_request_duration_seconds_bucket{le="0.05"} 979
--- SLO の確かめ（登録の1回を含めて 1001 回）
成功の割合: 99.70%（目標 99.9% 以上）
50 ms 以内の割合: 97.8%（目標 95% 以上）
エラーの予算: 1 回まで → 使った回数 3 回（300%）
```

- 速さの SLO（95% が 50 ms 以内）は、97.8% で守れました。成功の割合は、NUL の題の 500 が3回あったので 99.70% で、目標の 99.9% に届きませんでした。
- **エラーの予算:** 1001 回の 0.1% は約 1 回です。3 回の 500 で、予算の 300% を使いました。予算を使い切ったら、新しい機能を足すより先に、5xx の原因を直す、と**前もって決めておく**のが、エラーの予算の使い方です（図の2つ目）。
- UNIT 04 の直し方（題の制御文字を断る）も当てて、もう一度動かしました。

```text
rustnote_requests_total{class="4xx"} 3
rustnote_requests_total{class="5xx"} 0
成功の割合: 100.00%（目標 99.9% 以上）
50 ms 以内の割合: 97.3%（目標 95% 以上）
エラーの予算: 1 回まで → 使った回数 0 回（0%）
```

- 3回の NUL の題は 400 になり、5xx は 0 回、成功の割合は 100.00% になりました。速さの数は、動かすたびに少し変わります（97.8% と 97.3%）。
- 本物では、1日・1か月のような**期間**で数えます。たとえば 1 か月に 300 万回なら、99.9% の予算は 3000 回です。テストの 1000 回は、計算の練習のための小さな数です。

**health について:** RustNote の `/health` は、DB に `SELECT 1` を送り、届けば `200 {"status":"ok"}`、届かなければ `503` を返します（第9巻 UNIT 10）。Fly.io は、これを見て、そのマシンにリクエストを送るかを決めます。ただし、`/health` は、**遅い・500 が多い**ことは知らせません。上の SLO の計算では、5xx が 3 回ありましたが、`/health` の作り（DB に `SELECT 1` を送るだけ）からすると、その間も 200 を返し続けます。「動いているか」は health、「良く動いているか」はメトリクスと SLO で見ます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: subscriber を入れずに、ログが出ると思った

```rust
//! 第11巻 UNIT 07 エラー1: subscriber を入れずに、リクエストを送った。
//! 実行: cargo test --test e07_no_subscriber -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn logs_without_subscriber() {
    let app = test_app().await;
    app.get("/login", None).await;
    app.get("/notes/99", None).await;
    println!("2つのリクエストを送りました");
}
```

```text
running 1 test
2つのリクエストを送りました
test logs_without_subscriber ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s
```

- **何が起きたか:** 2つのリクエストを送りましたが、アクセスログの JSON は1行も出ませんでした。`tracing::info!` は、「記録を作って、受け取る人（subscriber）に渡す」だけで、受け取る人がいなければ、何も書きません。エラーにもなりません。
- **直し方:** プログラムの最初（`main` や、テストの最初）で、`tracing_subscriber::fmt()…init()` を1回だけ呼びます。本番で「ログが出ない」ときは、まず subscriber を入れたか、`RUST_LOG` で絞りすぎていないか（問1）を確かめます。

#### エラー2: テストごとに subscriber を入れた（2回目の init で panic）

```rust
//! 第11巻 UNIT 07 エラー2: テストごとに subscriber を入れた（同じテストのファイルに2つ）。
//! 実行: cargo test --test e07_init_twice

mod common;

use common::test_app;

fn start_logging() {
    tracing_subscriber::fmt().json().init();
}

#[tokio::test]
async fn first() {
    start_logging();
    test_app().await.get("/login", None).await;
}

#[tokio::test]
async fn second() {
    start_logging();
    test_app().await.get("/signup", None).await;
}
```

```text
test first ... FAILED
test second ... ok
thread 'first' (220928) panicked at ...\mod.rs:520:14:
Unable to install global subscriber: SetGlobalDefaultError("a global default trace dispatcher has already been set")
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

- **何が起きたか:** `init()` は、プログラム全体で1つの subscriber（global default）を入れます。同じテストのファイルの2つのテストは、同じプログラムの中で同時に動くので、2回目の `init()` が「もう入っています」（`a global default trace dispatcher has already been set`）と panic しました。先に入れたほうは通り、`/signup` のログも出ています。どちらが先かは、その時しだいです。
- **直し方:** テストでは `init()` の代わりに `try_init()` を使い、2回目の失敗を無視します（`let _ = tracing_subscriber::fmt().json().try_init();`）。または、例7-1のように、1つのテストのファイルには、subscriber を入れるテストを1つだけにします。

#### エラー3: 数える入れ物を、`HashMap` の `static` で作ろうとした（E0015）

```rust
//! 第11巻 UNIT 07 エラー3: 数える入れ物を、HashMap の static で作ろうとした。
//! 実行: cargo test --test e07_static

use std::collections::HashMap;
use std::sync::Mutex;

/// 状態コードごとの数
static COUNTS: Mutex<HashMap<u16, u64>> = Mutex::new(HashMap::new());

#[test]
fn count_statuses() {
    for status in [200, 200, 404] {
        *COUNTS.lock().unwrap().entry(status).or_insert(0) += 1;
    }
    println!("{:?}", COUNTS.lock().unwrap());
}
```

```text
error[E0015]: cannot call non-const associated function `HashMap::<u16, u64>::new` in statics
  |
8 | static COUNTS: Mutex<HashMap<u16, u64>> = Mutex::new(HashMap::new());
  |                                                      ^^^^^^^^^^^^^^
  |
  = note: calls in statics are limited to constant functions, tuple structs and tuple variants
  = note: consider wrapping this expression in `std::sync::LazyLock::new(|| ...)`
error: could not compile `rustnote` (test "e07_static") due to 1 previous error
```

- **何が起きたか:** `static` の値は、プログラムが動き出す前に決まっていなければならないので、`const fn`（組み立てのときに計算できる関数）しか呼べません。`HashMap::new()` は `const fn` ではないので、E0015 になりました。
- **直し方:** `note:` のとおり、`std::sync::LazyLock::new(|| Mutex::new(HashMap::new()))` で、初めて使うときに作ります。例7-2の `METRICS` は、`AtomicU64::new(0)` が `const fn` なので、そのまま `static` で作れました。数える種類が決まっているなら、`HashMap` より、配列の `AtomicU64` のほうが、ロックも要らず速いです。

### RustNote ではここで使う

- 今の RustNote は、アクセスログ（`src/routes.rs` の `access_log`）・500 の原因（`src/views.rs` の `log_if_internal`）・health の失敗（`src/routes/health.rs`）の3か所で、`eprintln!` の文を書いています。メトリクスと SLO の計算は、ありません。
- この単元の `tracing` と `/metrics` は、練習用のコピーにだけ足しました（教材の `Rust/rustnote` は変えていません）。足すときの注意（改善点）:
  - `/metrics` は、だれでも読めると、リクエストの数や 500 の数が外から分かります（問2）。Fly.io では、内側の通信（6PN）だけで読めるようにする、決まった人しか読めない別のポートにする、などの方法があります。
  - 構造化ログにも、パスワード・トークン・メールアドレスを、そのまま書きません（第8巻 UNIT 10）。項目に分けると、うっかり `?form` のように構造体ごと書きやすくなるので、注意します。
  - 起動のときの `eprintln!` も、`tracing` にそろえると、ログの置き場で1つの形として扱えます。
- 公開したあとの見張り（数を集めて、グラフと知らせにする）は、第12巻で扱います。

### 練習問題

**問1:** 例7-2のサーバーを、`RUST_LOG=warn` を付けて起動し、同じリクエストを送ると、標準出力のログはどうなるでしょうか。500 のときの `ERROR` の行は、どうでしょうか。

**問2:** `/metrics` を、インターネットのだれでも読めるようにしておくと、何が困るでしょうか。

**問3:** 1日に 10 万回のリクエストがあるサービスで、成功の割合の SLO を 99.9%（1 か月 30 日）にすると、1 か月のエラーの予算は何回でしょうか。例7-3の NUL の題の 500 が、1日に 3 回ずつ起きると、何日で予算を使い切るでしょうか。

### 解答と解説

**問1の解答:**

```powershell
$env:RUST_LOG = "warn"
.\target\debug\rustnote.exe
```

標準出力（`GET /login` と `GET /nothing-here` を送ったあと）:

```text

```

標準エラー出力:

```text
マイグレーションを適用しました: 0001_users
マイグレーションを適用しました: 0002_notes
マイグレーションを適用しました: 0003_sessions
RustNote を起動しました: http://127.0.0.1:3012
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
- 予算が残っていても、5xx は、利用者にとっては、ノートが作れなかった失敗です。予算は「どこまで我慢するか」ではなく、「直す順番を決めるための物差し」として使います。

### まとめ

- `tracing` で、アクセスログを、`method`・`path`・`status`・`elapsed_ms` の項目つきの JSON の1行にした。項目で絞り込め、500 は `ERROR` の行と原因の連なりで分かる。subscriber を入れないと何も書かれず、`init()` を2回呼ぶと panic する（テストでは `try_init()`）。`RUST_LOG=warn` で、`info` のアクセスログは出なくなる。
- `/metrics` で、状態コードの種類ごとの数と、応答時間のヒストグラムを、Prometheus の形で返した。数える入れ物は、`AtomicU64` の配列なら `static` で作れる（`HashMap` は E0015、`LazyLock` を使う）。`/metrics` は、だれでも読めない場所に置く。
- SLO は、成功の割合と速さの目標を、数で決めたもの。1001 回のうち NUL の題の 500 が 3 回で、99.9% の予算（1 回）を 300% 使い、直すと 100.00% になった。`/health` は「動いているか」を答え、「良く動いているか」はメトリクスと SLO で見る。
