## UNIT 08: HTTP クライアント・再試行・冪等性

### この単元のゴール

- reqwest（リクエスト）の `Client` を1つ作って使い回し、外の API に GET して、本文や JSON を受け取れる。接続と全体の時間に上限を付けられる
- 404 や 503 の答えでも `send()` は `Ok` を返す（通信としては成功）ことを知っていて、`error_for_status` で `Err` に変えられる
- 失敗の種類（接続できない・時間切れ・ステータス・JSON の形）を見分け、エラーの「原因の連なり」をたどって読める
- 一時的な失敗だけを、間隔を倍々に空けて（指数バックオフ）、ばらつき（ジッター）を足し、回数の上限つきで再試行できる
- 再試行で同じ操作が二重に行われないように、**冪等キー**（べきとうキー）を使う理由と形を説明できる

### たとえ話でつかむ

お店に電話で出前を頼む場面を考えます。電話がうまくいかない理由は、いくつもあります。

- **電話がつながらない**（相手が電話に出られる状態ではない）。少し時間をおけば、つながるかもしれません。
- **呼び出し音が鳴り続けて、だれも出ない**（時間切れ）。いつまでも受話器を持っていないで、上限を決めて切ります（UNIT 06）。
- **「その料理はメニューにありません」と言われる**（404 Not Found）。何回かけ直しても答えは同じです。かけ直すのは無駄です。
- **「今とても混んでいるので、少ししてからかけ直してください」と言われる**（503 Service Unavailable・429 Too Many Requests）。かけ直す価値があります。

かけ直すときにも、作法があります。

- **すぐにかけ直さず、少しずつ間を空けます。** 1回目は1分後、2回目は2分後、3回目は4分後…と、間隔を倍々にのばします（**指数バックオフ**）。
- **みんなが同じ時刻にかけ直さないように、間隔を少しずつずらします**（**ジッター**）。全員がちょうど1分後にかけ直したら、お店はまた混んでしまいます。
- **注文が通ったか分からないまま電話が切れたら**、もう一度注文すると、料理が2つ届くかもしれません。最初に「注文番号 123 です」と伝えておけば、お店は2回目の電話に「123 番はもう受けています」と答えられます。これが**冪等キー**です。

> **用語:** HTTP クライアント（HTTP のリクエストを送り、答えを受け取る側のプログラム。この単元では reqwest クレートを使う。答える側は HTTP サーバー）

<!-- -->

> **用語:** ステータスコード（HTTP の答えに付く3桁の数字。2xx は成功、4xx は頼んだ側の問題、5xx は答える側の問題。第6巻でくわしく学ぶ）

<!-- -->

> **用語:** 指数バックオフ（exponential backoff：再試行の間隔を 100 ミリ秒・200 ミリ秒・400 ミリ秒…のように倍々にのばしていくこと）

<!-- -->

> **用語:** 冪等（べきとう：同じ操作を1回しても何回しても、結果が同じになる性質。「ノートを読む」は冪等、「お金を送る」はふつうは冪等ではない）

### 図で理解する

1つのリクエストは、いくつもの段階を通ります。どの段階でも失敗することがあり、失敗した段階によって、エラーの種類が変わります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-081" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="54" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">① 名前を</text>
<text x="54" y="82" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">調べる</text>
<rect x="111" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="155" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">② 接続</text>
<text x="155" y="82" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">する</text>
<rect x="212" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="256" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">③ 暗号化</text>
<text x="256" y="82" font-size="10" fill="#475569" text-anchor="middle">（https のとき）</text>
<rect x="313" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="357" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">④ 頼む</text>
<text x="357" y="82" font-size="10" fill="#475569" text-anchor="middle">（送る）</text>
<rect x="414" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="458" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">⑤ 答えの</text>
<text x="458" y="82" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">見出し</text>
<rect x="515" y="40" width="88" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="559" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">⑥ 本文を</text>
<text x="559" y="82" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">受け取る</text>
<rect x="616" y="40" width="94" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="663" y="64" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">⑦ JSON を</text>
<text x="663" y="82" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">読む</text>
<line x1="98" y1="68" x2="108" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<line x1="199" y1="68" x2="209" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<line x1="300" y1="68" x2="310" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<line x1="401" y1="68" x2="411" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<line x1="502" y1="68" x2="512" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<line x1="603" y1="68" x2="613" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-081)"/>
<rect x="10" y="12" width="390" height="20" rx="4" fill="#fef2f2" stroke="#ef4444"/>
<text x="205" y="27" font-size="10" fill="#b91c1c" text-anchor="middle">つながらない: is_connect()（例: os error 10061 接続を拒否された）</text>
<rect x="111" y="112" width="492" height="22" rx="4" fill="#fefce8" stroke="#eab308"/>
<text x="357" y="128" font-size="10" fill="#854d0e" text-anchor="middle">上限の時間を過ぎた: is_timeout()（Client の timeout は ② から ⑥ まで全体にかかる）</text>
<rect x="414" y="146" width="189" height="40" rx="4" fill="#fff7ed" stroke="#f97316"/>
<text x="508" y="162" font-size="10" fill="#c2410c" text-anchor="middle">4xx・5xx が返った</text>
<text x="508" y="178" font-size="10" fill="#c2410c" text-anchor="middle">send() は Ok。error_for_status で Err</text>
<rect x="616" y="146" width="94" height="40" rx="4" fill="#f5f3ff" stroke="#8b5cf6"/>
<text x="663" y="162" font-size="10" fill="#6d28d9" text-anchor="middle">形が違う</text>
<text x="663" y="178" font-size="10" fill="#6d28d9" text-anchor="middle">decoding error</text>
<text x="360" y="222" font-size="12" fill="#475569" text-anchor="middle">「通信の失敗」（つながらない・時間切れ）と「答えの中身の失敗」（ステータス・形）を分けて考える</text>
<text x="360" y="242" font-size="11" fill="#475569" text-anchor="middle">この単元の練習では、自分のパソコンの中の http（127.0.0.1）だけを使うので、①と③は通らない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 1つのリクエストが通る段階と、失敗したときのエラーの種類。つながらない・時間切れは通信の失敗、4xx・5xx と JSON の形の違いは、答えは届いたが中身が期待と違う失敗</div>
</div>

次に、失敗したときに「やり直すか、やめるか」の決め方です。例8-4で、このとおりに作ります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-082" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="270" y="10" width="180" height="36" rx="8" fill="#0f172a"/>
<text x="360" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">リクエストの結果は？</text>
<rect x="10" y="80" width="160" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="90" y="104" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">2xx</text>
<text x="90" y="124" font-size="11" fill="#0f172a" text-anchor="middle">成功。ここで終わり</text>
<rect x="185" y="80" width="170" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="270" y="104" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">404・400・401 など</text>
<text x="270" y="124" font-size="11" fill="#0f172a" text-anchor="middle">やり直しても無駄 → やめる</text>
<rect x="370" y="80" width="170" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="455" y="104" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">5xx・429</text>
<text x="455" y="124" font-size="11" fill="#0f172a" text-anchor="middle">一時的かも → やり直す</text>
<rect x="555" y="80" width="155" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="632" y="104" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">時間切れ・接続不可</text>
<text x="632" y="124" font-size="11" fill="#0f172a" text-anchor="middle">一時的かも → やり直す</text>
<line x1="300" y1="46" x2="100" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<line x1="340" y1="46" x2="280" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<line x1="380" y1="46" x2="445" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<line x1="420" y1="46" x2="622" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<rect x="370" y="166" width="340" height="44" rx="8" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="540" y="185" font-size="11" font-weight="700" fill="#854d0e" text-anchor="middle">回数の上限に達した？ → 達したらやめる</text>
<text x="540" y="201" font-size="10" fill="#475569" text-anchor="middle">（上限が無いと、壊れた相手を永遠に叩き続ける）</text>
<line x1="455" y1="136" x2="455" y2="162" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<line x1="632" y1="136" x2="632" y2="162" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<rect x="370" y="234" width="340" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="253" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">待ってからやり直す: 100 → 200 → 400ms …</text>
<text x="540" y="269" font-size="10" fill="#475569" text-anchor="middle">倍々にのばし（上限あり）、ばらつきを足す</text>
<line x1="540" y1="210" x2="540" y2="230" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-082)"/>
<path d="M370,256 H178 V30 H266" fill="none" stroke="#3b82f6" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-05-082)"/>
<text x="274" y="250" font-size="10" fill="#1e40af" text-anchor="middle">もう一度</text>
<text x="12" y="200" font-size="11" fill="#b91c1c">POST など冪等でない操作は、</text>
<text x="12" y="218" font-size="11" fill="#b91c1c">冪等キーが無ければ</text>
<text x="12" y="236" font-size="11" fill="#b91c1c">やり直さない（例8-5）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 再試行するかどうかの決め方。一時的かもしれない失敗だけを、間隔を倍々にのばしながら、回数の上限までやり直す。冪等でない操作は、冪等キーが無ければやり直さない</div>
</div>

### まず動かす最小コード

#### 準備: reqwest と serde を入れ、練習用のサーバーを用意する

この単元では、HTTP クライアントの reqwest と、JSON を struct に変換する serde を使います。`async_lab` フォルダで次を実行します。

```powershell
cargo add reqwest@=0.13.5 --features json
cargo add serde@=1.0.229 --features derive
```

`cargo add reqwest` の実際の出力の始めの部分です（`Locking 147 packages` の後ろに、`Adding …` の行が 147 行続きますが、省いています）。

```text
    Updating crates.io index
      Adding reqwest =0.13.5 to dependencies
             Features:
             + __rustls
             + __rustls-aws-lc-rs
             + __tls
             + charset
             + default-tls
             + http2
             + json
             + rustls
             + system-proxy
             - __native-tls
             - __native-tls-alpn
             - blocking
             - brotli
             - cookies
             - deflate
             - form
             - gzip
             - hickory-dns
             - http3
             - multipart
             - native-tls
             - native-tls-no-alpn
             - native-tls-vendored
             - native-tls-vendored-no-alpn
             - query
             - rustls-no-provider
             - socks
             - stream
             - zstd
    Updating crates.io index
     Locking 147 packages to highest Rust 1.98 compatible versions
```

```text
    Updating crates.io index
      Adding serde =1.0.229 to dependencies
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
```

- reqwest は、HTTP クライアントとしていちばんよく使われているクレートです。`json` の機能を足すと、答えの本文を JSON として読めるようになります。
- `+ rustls` と `+ __rustls-aws-lc-rs` は、https の暗号化に使う部品です。147 個ものクレートが入るのは、この暗号の部品と、HTTP/2 などの部品のためです。最初のビルドには時間がかかります（この PC では約 33 秒でした）。
- 暗号の部品（aws-lc-sys）には C 言語で書かれた部分があり、ビルドに C のコンパイラを使います。この PC（ARM64 版の Windows）では、第7巻の巻頭で入れた LLVM（clang）が入っているので、そのままビルドできました。

> **注意:** ARM64 版の Windows で、clang が入っていないと、この暗号の部品のビルドで止まります。そのときは第7巻の巻頭「ARM64 版の Windows では clang が必要」の手順で LLVM を入れてから、ビルドし直してください。x64 版の Windows・macOS・Linux では、ふつうは追加の準備はいりません。

できあがった `Cargo.toml` の `[dependencies]` は次のとおりです。

```toml
[dependencies]
argon2 = "=0.6.0"
reqwest = { version = "=0.13.5", features = ["json"] }
serde = { version = "=1.0.229", features = ["derive"] }
tokio = { version = "=1.53.3", features = ["full"] }
tokio-util = { version = "=0.7.20", features = ["rt"] }
```

外の API の代わりに、自分のパソコンの中（127.0.0.1）だけで動く、とても小さな HTTP サーバーを用意します。インターネットには出ないので、どの例も安全に何度でも試せます。次の2つのファイルを作ってください。中身をくわしく理解する必要はありません（本物の Web サーバーは、第6巻で Topcoat を使って作ります）。「決まった答えを返すだけの、練習用の相手」として使います。

`src/lib.rs`（`src/bin/` のプログラムから `async_lab::mini_server` で使えるようにする）:

```rust
//! 練習用の部品をまとめたライブラリ（UNIT 08 で使う）。

pub mod mini_server;
```

`src/mini_server.rs`:

```rust
//! 練習用の、とても小さな HTTP サーバー。
//!
//! 自分のパソコンの中（127.0.0.1）だけで動き、インターネットには出ない。
//! 「決まった答えを返す」だけの最小限のもので、本物の Web アプリは第6巻の Topcoat で作る。

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// 受け取ったリクエスト（この練習で使う部分だけ）
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request {
    /// ヘッダーを名前で探す（大文字・小文字は区別しない）
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// 返す答え
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn new(status: u16, body: impl Into<String>) -> Response {
        Response {
            status,
            body: body.into(),
        }
    }
}

/// 127.0.0.1 の空いているポートでサーバーを起動し、そのアドレスを返す。
/// `handler` はリクエストが1つ来るたびに呼ばれ、答え（Response）を返す。
pub async fn start<F, Fut>(handler: F) -> SocketAddr
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    // ポート 0 を指定すると、OS が空いているポートを選んでくれる
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("ポートを開けません");
    let address = listener.local_addr().expect("アドレスを読めません");
    let handler = Arc::new(handler);

    // 接続を受け付け続けるタスク
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                continue;
            };
            let handler = Arc::clone(&handler);
            // 1つの接続を1つのタスクで処理する
            tokio::spawn(async move {
                serve_one(stream, handler.as_ref()).await;
            });
        }
    });
    address
}

/// 1つの接続から1つのリクエストを読み、答えを書いて閉じる
async fn serve_one<F, Fut>(mut stream: TcpStream, handler: &F)
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Response>,
{
    let Some(request) = read_request(&mut stream).await else {
        return;
    };
    let response = handler(request).await;
    let reason = match response.status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Unknown",
    };
    let text = format!(
        "HTTP/1.1 {} {reason}\r\ncontent-type: text/plain; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        response.status,
        response.body.len(),
        response.body
    );
    // 相手がもう待っていない（タイムアウトで切られた）こともあるので、失敗は無視する
    let _ = stream.write_all(text.as_bytes()).await;
}

/// ヘッダーの終わり（空行）まで読み、Content-Length の分だけ本文を読む
async fn read_request(stream: &mut TcpStream) -> Option<Request> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 1024];
    let header_end = loop {
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        buffer.extend_from_slice(&chunk[..n]);
        if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
            break pos;
        }
    };
    let head = String::from_utf8_lossy(&buffer[..header_end]).to_string();
    let mut lines = head.split("\r\n");
    let mut first = lines.next()?.split(' ');
    let method = first.next()?.to_string();
    let path = first.next()?.to_string();
    let headers: Vec<(String, String)> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect();
    let length: usize = headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.parse().ok())
        .unwrap_or(0);
    let mut body = buffer[header_end + 4..].to_vec();
    while body.len() < length {
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    Some(Request {
        method,
        path,
        headers,
        body: String::from_utf8_lossy(&body).to_string(),
    })
}
```

このサーバーで、この巻のこれまでの道具が使われていることに気づいたでしょうか。`start` は、接続を受け付けるたびに `tokio::spawn` でタスクを作ります（UNIT 02 で読んだ Topcoat と同じ形）。`handler` を `Arc` に入れて、どのタスクからも使えるようにしています（UNIT 05）。`F: Fn(Request) -> Fut + Send + Sync + 'static` は、「リクエストを受け取って Future を返す関数で、スレッドをまたいで共有できるもの」という条件です（UNIT 02 の `Send + 'static`）。

> **ヒント:** この先の実行結果では、サーバーのポート番号（`127.0.0.1:` の後ろの数字）を `PORT` に置き換えて載せています。ポート 0 を指定して OS に空いている番号を選んでもらっているので、実行するたびに番号が変わるからです。

#### 例8-1: GET で答えを受け取り、ステータスを確かめる

> **▼ このコードがやること（先に日本語で）:** 練習用のサーバーを起動します。`/hello` には 200 と挨拶を、それ以外には 404 を返します。reqwest の `Client` を時間の上限つきで1つ作り、①`/hello` に GET して本文を表示し、②無い URL に GET してステータスを確かめ、③`error_for_status` で 404 を `Err` に変えます。

```rust
use std::time::Duration;

use async_lab::mini_server::{self, Response};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 練習用サーバーを起動する。/hello には 200、それ以外には 404 を返す
    let address = mini_server::start(|request| async move {
        if request.path == "/hello" {
            Response::new(200, "こんにちは、RustNote です")
        } else {
            Response::new(404, "見つかりません")
        }
    })
    .await;
    println!("サーバー: http://{address}");

    // Client は1つ作って使い回す（中で接続をため込んで再利用する）
    let client = reqwest::Client::builder()
        // 接続を確立するまでの上限
        .connect_timeout(Duration::from_secs(2))
        // 1回のリクエスト全体（接続〜本文の受信まで）の上限
        .timeout(Duration::from_secs(5))
        .build()?;

    // ① 成功する GET
    let response = client.get(format!("http://{address}/hello")).send().await?;
    println!("① ステータス: {}", response.status());
    println!("① 本文: {}", response.text().await?);

    // ② 404 は「通信としては成功」なので、send() は Ok を返す
    let response = client
        .get(format!("http://{address}/nothing"))
        .send()
        .await?;
    println!(
        "② ステータス: {} / 成功?: {}",
        response.status(),
        response.status().is_success()
    );

    // ③ error_for_status: 4xx・5xx を Err に変える
    let result = client
        .get(format!("http://{address}/nothing"))
        .send()
        .await?
        .error_for_status();
    match result {
        Ok(_) => println!("③ 成功"),
        Err(error) => println!("③ エラー: {error}"),
    }
    Ok(())
}
```

`src/bin/u08_get.rs` として保存して実行します。

```powershell
cargo run -q --bin u08_get
```

**▼ 実行結果**

```text
サーバー: http://127.0.0.1:PORT
① ステータス: 200 OK
① 本文: こんにちは、RustNote です
② ステータス: 404 Not Found / 成功?: false
③ エラー: HTTP status client error (404 Not Found) for url (http://127.0.0.1:PORT/nothing)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Client は1つ作って、使い回す

- `reqwest::Client::builder()` で設定を決めて、`.build()?` で `Client` を作ります。`Client` の中には、相手のサーバーとの接続をため込んでおく場所（接続プール）があり、同じ相手への2回目以降のリクエストは、前の接続を使い回して速く送れます。
- だから、`Client` は**アプリ全体で1つ作って使い回す**のが基本です。リクエストのたびに作り直すと、毎回接続からやり直すことになります。`Client` は `clone` しても中身は共有されるので（`Arc` と同じ考え方）、タスクに配るときは `clone` します。

##### 解説2: 2種類の時間の上限

- `.connect_timeout(2 秒)` は、②の「接続する」までの上限です。相手のサーバーが見つからない・応答しないときに、早めに諦められます。
- `.timeout(5 秒)` は、1回のリクエスト全体（接続から本文を受け取り終わるまで）の上限です。UNIT 06 の `tokio::time::timeout` で包むのと同じ役目を、reqwest が中でやってくれます。
- reqwest の `Client` は、何も設定しないと全体の時間の上限がありません。外の API を呼ぶときは、必ず `.timeout(…)` を付けましょう。

##### 解説3: send() の Ok は「答えが届いた」という意味

- `client.get(URL).send().await` は、リクエストを送り、答えの見出し（ステータスとヘッダー）が届くまで待ちます。結果は `Result<Response, reqwest::Error>` です。
- ②のとおり、**404 でも `Ok` が返ります**。サーバーが「見つかりません」と答えてくれた、つまり通信そのものは成功しているからです。`Err` になるのは、つながらない・時間切れなど、答えが届かなかったときです（例8-3）。
- 4xx・5xx も失敗として扱いたいときは、③のように `.error_for_status()` を付けます。2xx なら `Ok(Response)`、4xx・5xx なら `Err` に変えてくれます。
- `response.text().await?` は、本文を最後まで受け取って `String` にします。本文の受け取りにも時間がかかるので、`.await` です。

#### 例8-2: JSON の答えを、struct に変換する

> **▼ このコードがやること（先に日本語で）:** 天気の API の代わりに、練習用のサーバーが `/weather` で `{"city":"東京","celsius":18.5}` を返すようにします。`/broken` は `celsius` が抜けた JSON を返します。どちらも GET して、答えを `Weather` という struct に変換し、成功なら内容を、失敗ならエラーと原因を表示します。

```rust
use std::error::Error;
use std::time::Duration;

use async_lab::mini_server::{self, Response};
use serde::Deserialize;

// 外の API が返す JSON の形を、struct で書いておく
#[derive(Debug, Deserialize)]
struct Weather {
    city: String,
    celsius: f64,
}

#[tokio::main]
async fn main() {
    // /weather は正しい JSON、/broken は celsius が抜けた JSON を返す
    let address = mini_server::start(|request| async move {
        match request.path.as_str() {
            "/weather" => Response::new(200, r#"{"city":"東京","celsius":18.5}"#),
            "/broken" => Response::new(200, r#"{"city":"東京"}"#),
            _ => Response::new(404, "見つかりません"),
        }
    })
    .await;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    for path in ["/weather", "/broken"] {
        let result = client
            .get(format!("http://{address}{path}"))
            .send()
            .await
            .and_then(|response| response.error_for_status());
        let weather = match result {
            // 本文を JSON として読み、Weather に変換する
            Ok(response) => response.json::<Weather>().await,
            Err(error) => Err(error),
        };
        match weather {
            Ok(weather) => println!("{path}: {} は {} 度", weather.city, weather.celsius),
            Err(error) => {
                println!("{path}: 失敗 {error}");
                if let Some(cause) = error.source() {
                    println!("    原因: {cause}");
                }
            }
        }
    }
}
```

`src/bin/u08_json.rs` として保存して実行します。

```powershell
cargo run -q --bin u08_json
```

**▼ 実行結果**

```text
/weather: 東京 は 18.5 度
/broken: 失敗 error decoding response body for url (http://127.0.0.1:PORT/broken)
    原因: missing field `celsius` at line 1 column 17
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[derive(Deserialize)] で、JSON の形を struct に書く

- `#[derive(Deserialize)]` を付けると、serde が「JSON などから、この struct を作る」コードを自動で書いてくれます（第4巻 UNIT 08 の DTO と同じ考え方）。JSON の `"city"` が `city: String` に、`"celsius"` が `celsius: f64` に入ります。
- `response.json::<Weather>().await` は、本文を最後まで受け取り、JSON として読んで `Weather` に変換します。`::<Weather>` は「何の型に変換するか」の指定です（ターボフィッシュ：第4巻 UNIT 03）。

##### 解説2: 形が違えば、どこが違うかを教えてくれる

- `/broken` の JSON には `celsius` がありません。変換は失敗し、`error decoding response body`（本文を読み解けませんでした）のエラーになりました。
- `error.source()` で、このエラーの**原因**になった、もう1つ奥のエラーを取り出せます。ここでは serde のエラー `missing field celsius at line 1 column 17`（1行目の17文字目のところで、`celsius` という欄が無いと分かりました）が出ています。
- 外の API は、こちらの都合とは関係なく、答えの形を変えることがあります。形が違えば必ずエラーになり、どの欄が違うかまで分かるので、知らないうちにおかしな値で動き続けることがありません。

##### 解説3: and_then で、2つの失敗を1つの流れにまとめる

- `.send().await` の結果（`Result`）に `.and_then(|response| response.error_for_status())` を続けています。`and_then` は、「`Ok` のときだけ次の処理をして、その結果（これも `Result`）を返す」メソッドです（第2巻 UNIT 09）。通信の失敗も、4xx・5xx も、同じ `Err(reqwest::Error)` の流れにまとまります。

> **ヒント:** 欄が無いこともある JSON なら、`celsius: Option<f64>` のように `Option` にしておくと、無いときは `None` として読めます。「必ずある欄」と「無いこともある欄」を、struct の型ではっきり書き分けるのが、serde の使い方の基本です。

### もう一歩: 実務に近い形

#### 例8-3: つながらない・時間切れを見分ける

> **▼ このコードがやること（先に日本語で）:** ①だれも待ち受けていないポートへ接続しようとし、②1 秒かかる遅いサーバーに、300 ミリ秒の上限で問い合わせます。どちらも失敗するので、エラーと、その「原因の連なり」を全部表示し、`is_connect`（つながらなかった）と `is_timeout`（時間切れ）で見分けます。

```rust
use std::error::Error;
use std::time::{Duration, Instant};

use async_lab::mini_server::{self, Response};

// エラーの「原因の連なり」（source）を全部表示する
fn show_chain(label: &str, error: &reqwest::Error) {
    println!("{label}: {error}");
    let mut source = error.source();
    while let Some(cause) = source {
        println!("    原因: {cause}");
        source = cause.source();
    }
    println!(
        "    is_connect = {} / is_timeout = {}",
        error.is_connect(),
        error.is_timeout()
    );
}

#[tokio::main]
async fn main() {
    // ① 誰も待ち受けていないポートへ接続する
    let unused = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.local_addr().unwrap()
        // ここで listener が片付けられ、ポートは閉じる
    };
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let start = Instant::now();
    if let Err(error) = client.get(format!("http://{unused}/")).send().await {
        show_chain("① 接続できない", &error);
    }
    println!("    （{:.1} 秒かかった）", start.elapsed().as_secs_f64());

    // ② 答えが遅すぎるサーバー（1 秒かかる）に、300ms の時間切れで問い合わせる
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(300))
        .build()
        .unwrap();
    let slow = mini_server::start(|_| async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Response::new(200, "遅くなりました")
    })
    .await;
    if let Err(error) = client.get(format!("http://{slow}/")).send().await {
        show_chain("② 時間切れ", &error);
    }
}
```

`src/bin/u08_errors.rs` として保存して実行します。

```powershell
cargo run -q --bin u08_errors
```

**▼ 実行結果**

```text
① 接続できない: error sending request for url (http://127.0.0.1:PORT/)
    原因: client error (Connect)
    原因: tcp connect error
    原因: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)
    is_connect = true / is_timeout = false
    （2.0 秒かかった）
② 時間切れ: error sending request for url (http://127.0.0.1:PORT/)
    原因: operation timed out
    is_connect = false / is_timeout = true
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: エラーは、原因の連なりになっている

- いちばん外のエラーは、どちらも `error sending request for url (…)`（リクエストを送れませんでした）で、これだけでは理由が分かりません。
- `error.source()` で1つ奥の原因を取り出し、さらにその `source()`…と、`None` になるまでたどっています（`while let`）。①では、「クライアントのエラー（接続）」→「TCP の接続エラー」→「対象のコンピューターによって拒否されたため、接続できませんでした。（os error 10061）」と、だんだんくわしくなりました。いちばん奥の文は Windows が日本語で出したメッセージです。
- 第4巻 UNIT 05 で、エラーに文脈を足して包む話をしました。reqwest のエラーも、同じように何重にも包まれています。ログには、この連なりの全部を残すと、原因を調べやすくなります。

##### 解説2: is_connect と is_timeout で、種類を見分ける

- `error.is_connect()` は「接続できなかった」、`error.is_timeout()` は「時間の上限を過ぎた」ときに `true` を返します。再試行するかどうかの判断（例8-4）に使えます。
- ①は、閉じたポートへの接続なので `is_connect = true` です。②は、接続はできたのに答えが 300 ミリ秒以内に来なかったので `is_timeout = true` です。

##### 解説3: 拒否されるまで、2 秒かかった

- ①は、だれもいないポートなのに、失敗が分かるまで 2.0 秒かかりました。この PC（Windows）では、閉じたポートへの接続は、何回試しても約 2 秒たってから「拒否された」の失敗が返りました。Windows の TCP は、断られてもすぐには諦めずに接続をやり直す作りになっているためです（Linux などでは、ふつうはすぐに失敗が返ります）。
- 接続の上限（`connect_timeout`）を 2 秒より短くしておけば、もっと早く諦められます。

#### 例8-4: 一時的な失敗だけを、間隔を空けて再試行する

> **▼ このコードがやること（先に日本語で）:** 最初の2回は 503（混んでいます）を返し、3回目から 200 を返す「調子の悪い」サーバーを作ります。再試行の関数 `get_with_retry` は、5xx・429・時間切れ・接続できない、のときだけ、100 ミリ秒・200 ミリ秒…と倍々に待つ時間をのばし（ばらつき付き）、最大5回までやり直します。①調子の悪い URL と、②無い URL（404）で試します。

```rust
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use async_lab::mini_server::{self, Response};

// 0 以上 max 未満の「ばらつき」を作る（標準ライブラリだけで作る簡易な乱数）
fn jitter(max: Duration) -> Duration {
    let random = RandomState::new().build_hasher().finish();
    let millis = max.as_millis() as u64;
    Duration::from_millis(if millis == 0 { 0 } else { random % millis })
}

// n 回目の失敗のあとに待つ時間: 100ms, 200ms, 400ms, ...（上限 2 秒）＋ばらつき
fn backoff(attempt: u32) -> Duration {
    let base = Duration::from_millis(100) * 2u32.pow(attempt - 1);
    let capped = base.min(Duration::from_secs(2));
    capped + jitter(capped / 2)
}

// この失敗は、もう一度やれば成功するかもしれないか
fn is_retryable(result: &Result<reqwest::Response, reqwest::Error>) -> bool {
    match result {
        // 503（一時的に使えない）・429（混んでいる）・その他の 5xx はやり直す価値がある
        Ok(response) => {
            let status = response.status();
            status.is_server_error() || status.as_u16() == 429
        }
        // 時間切れ・接続できない、はやり直す価値がある
        Err(error) => error.is_timeout() || error.is_connect(),
    }
}

async fn get_with_retry(
    client: &reqwest::Client,
    url: &str,
    max_attempts: u32,
) -> Result<String, String> {
    let start = Instant::now();
    for attempt in 1..=max_attempts {
        let result = client.get(url).send().await;
        let label = match &result {
            Ok(response) => response.status().to_string(),
            Err(error) => error.to_string(),
        };
        println!(
            "  {:.2} 秒: {attempt} 回目 → {label}",
            start.elapsed().as_secs_f64()
        );

        if !is_retryable(&result) {
            // 成功、または「やり直しても無駄な失敗」（404 など）はここで終わり
            let response = result.map_err(|e| e.to_string())?;
            let response = response.error_for_status().map_err(|e| e.to_string())?;
            return response.text().await.map_err(|e| e.to_string());
        }
        if attempt < max_attempts {
            let wait = backoff(attempt);
            println!("  {}ms 待ってやり直します", wait.as_millis());
            tokio::time::sleep(wait).await;
        }
    }
    Err(format!("{max_attempts} 回やっても成功しませんでした"))
}

#[tokio::main]
async fn main() {
    // 最初の2回は 503、3回目からは 200 を返す「調子の悪い」サーバー
    let count = Arc::new(AtomicU32::new(0));
    let server_count = Arc::clone(&count);
    let address = mini_server::start(move |request| {
        let count = Arc::clone(&server_count);
        async move {
            let n = count.fetch_add(1, Ordering::SeqCst) + 1;
            match request.path.as_str() {
                "/flaky" if n <= 2 => Response::new(503, "混んでいます"),
                "/flaky" => Response::new(200, "やっと成功"),
                _ => Response::new(404, "見つかりません"),
            }
        }
    })
    .await;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    println!("① 調子の悪いサーバー（503 が2回続く）");
    let result = get_with_retry(&client, &format!("http://{address}/flaky"), 5).await;
    println!("結果: {result:?}");

    println!("② 存在しない URL（404 はやり直さない）");
    let result = get_with_retry(&client, &format!("http://{address}/nothing"), 5).await;
    println!("結果: {result:?}");
}
```

`src/bin/u08_retry.rs` として保存して実行します。

```powershell
cargo run -q --bin u08_retry
```

**▼ 実行結果**

```text
① 調子の悪いサーバー（503 が2回続く）
  0.00 秒: 1 回目 → 503 Service Unavailable
  105ms 待ってやり直します
  0.13 秒: 2 回目 → 503 Service Unavailable
  205ms 待ってやり直します
  0.36 秒: 3 回目 → 200 OK
結果: Ok("やっと成功")
② 存在しない URL（404 はやり直さない）
  0.00 秒: 1 回目 → 404 Not Found
結果: Err("HTTP status client error (404 Not Found) for url (http://127.0.0.1:PORT/nothing)")
```

待つ時間（105ms・205ms）は、ばらつきを足しているので、実行するたびに変わります。

#### ▼ コードを1つずつ分解して解説

##### 解説1: やり直す価値がある失敗だけを、やり直す

- `is_retryable` は、図の決め方をそのままコードにしたものです。答えが届いた（`Ok`）なら、5xx（`is_server_error`）と 429 だけを、届かなかった（`Err`）なら、時間切れと接続できないときだけを、やり直します。
- ②の 404 は、何回やり直しても答えは変わらないので、1回目で終わっています。やり直しても無駄な失敗をやり直すと、相手のサーバーに無駄な負担をかけ、利用者を待たせるだけです。

##### 解説2: 間隔を倍々にのばし、上限を付ける

- `backoff(attempt)` は、`100ms × 2 の (attempt - 1) 乗` を計算します。1回目の失敗の後は 100ms、2回目は 200ms、3回目は 400ms…です。`.min(2 秒)` で、待ち時間がのびすぎないように上限を付けています（練習問題3）。
- 相手が本当に混んでいるなら、少しずつ間を空けるほど、相手が立ち直る時間ができます。すぐに何度もやり直すと、混んでいる相手に追い打ちをかけてしまいます。
- `max_attempts`（ここでは5回）で、やり直す回数にも上限を付けています。相手が本当に壊れているときに、永遠にやり直し続けないためです。

##### 解説3: ばらつき（ジッター）を足す

- `jitter(capped / 2)` で、待ち時間に「0 から、その半分未満」のばらつきを足しています。実行結果の 105ms・205ms の 5ms が、それです。
- 100 台のサーバーが同時に失敗して、全部がぴったり 100ms 後にやり直すと、相手には同じ瞬間に 100 個のリクエストが押し寄せます。ばらつきがあれば、やり直しの時刻が散らばります。
- ここでは、外のクレートを増やさないために、標準ライブラリの `RandomState`（`HashMap` がランダムな順番を作るために使う部品）から乱数の代わりの数を作っています。本物のアプリでは、乱数のクレート（rand など）を使うのがふつうです。

> **注意:** 再試行は、全体の待ち時間を長くします。①は3回目で成功しましたが、5回とも失敗すると、待ち時間の合計は 100 + 200 + 400 + 800 = 1500 ミリ秒（＋ばらつき）に、5回分の問い合わせの時間が加わります。画面の表示のために呼ぶ API なら、UNIT 06 の `timeout` で、再試行も含めた全体に上限を付けておきましょう。

#### 例8-5: 二重の送金を、冪等キーで防ぐ

> **▼ このコードがやること（先に日本語で）:** 「1000 円を送金する」POST を受けるサーバーを作ります。最初の1回だけ、送金した後の答えを返すのが 0.5 秒遅れる（ネットワークが詰まった）ようにします。クライアントは 0.2 秒で時間切れにして、やり直します。①冪等キー無しと、②冪等キー付き（`Idempotency-Key` ヘッダー）で、サーバーが実際に送金した回数を比べます。

```rust
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_lab::mini_server::{self, Request, Response};

// サーバー側の記録: 実際に「送金」した回数と、冪等キーごとの答え
#[derive(Default)]
struct Ledger {
    charged: u32,
    answers: HashMap<String, String>,
}

// POST /payments の処理。最初の1回だけ、答えを返すのがとても遅い（ネットワークが詰まった想定）
async fn handle_payment(ledger: Arc<Mutex<Ledger>>, request: Request) -> Response {
    let key = request.header("Idempotency-Key").map(str::to_string);
    let answer = {
        let mut ledger = ledger.lock().unwrap();
        // 同じキーで処理済みなら、記録しておいた答えをそのまま返す（もう一度は送金しない）
        if let Some(key) = &key
            && let Some(saved) = ledger.answers.get(key)
        {
            return Response::new(200, format!("{saved}（記録済みの答え）"));
        }
        ledger.charged += 1;
        let answer = format!(
            "{} を受け付けました（送金 {} 回目）",
            request.body, ledger.charged
        );
        if let Some(key) = key {
            ledger.answers.insert(key, answer.clone());
        }
        answer
    };
    // 送金はもう終わったが、答えが相手に届くまで時間がかかる
    if answer.contains("送金 1 回目") {
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Response::new(201, answer)
}

// 冪等キー（このリクエストの「整理番号」）を作る。本物では uuid クレートなどを使う
fn new_key() -> String {
    format!("{:016x}", RandomState::new().build_hasher().finish())
}

async fn pay(client: &reqwest::Client, url: &str, key: Option<&str>) {
    for attempt in 1..=3 {
        let mut request = client.post(url).body("1000 円");
        if let Some(key) = key {
            request = request.header("Idempotency-Key", key);
        }
        match request.send().await {
            Ok(response) => {
                println!("  {attempt} 回目: {}", response.text().await.unwrap());
                return;
            }
            Err(error) if error.is_timeout() => println!("  {attempt} 回目: 時間切れ → やり直す"),
            Err(error) => {
                println!("  {attempt} 回目: 失敗 {error}");
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        // 200ms で諦める（サーバーの最初の答えは 500ms かかるので、必ず時間切れになる）
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap();

    for use_key in [false, true] {
        let ledger = Arc::new(Mutex::new(Ledger::default()));
        let server_ledger = Arc::clone(&ledger);
        let address =
            mini_server::start(move |request| handle_payment(Arc::clone(&server_ledger), request))
                .await;
        let url = format!("http://{address}/payments");

        if use_key {
            println!("② 冪等キーあり");
            let key = new_key();
            pay(&client, &url, Some(&key)).await;
        } else {
            println!("① 冪等キーなし");
            pay(&client, &url, None).await;
        }
        println!(
            "  → サーバーが実際に送金した回数: {}",
            ledger.lock().unwrap().charged
        );
    }
}
```

`src/bin/u08_idempotency.rs` として保存して実行します。

```powershell
cargo run -q --bin u08_idempotency
```

**▼ 実行結果**

```text
① 冪等キーなし
  1 回目: 時間切れ → やり直す
  2 回目: 1000 円 を受け付けました（送金 2 回目）
  → サーバーが実際に送金した回数: 2
② 冪等キーあり
  1 回目: 時間切れ → やり直す
  2 回目: 1000 円 を受け付けました（送金 1 回目）（記録済みの答え）
  → サーバーが実際に送金した回数: 1
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-083" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="14" y="22" font-size="12" font-weight="700" fill="#b91c1c">① 冪等キーなし</text>
<rect x="14" y="34" width="120" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="74" y="58" font-size="11" fill="#1e40af" text-anchor="middle">POST 1000 円</text>
<rect x="150" y="34" width="120" height="40" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="210" y="52" font-size="11" fill="#b91c1c" text-anchor="middle">送金 1 回目</text>
<text x="210" y="67" font-size="9" fill="#475569" text-anchor="middle">答えが遅れて届かない</text>
<rect x="286" y="34" width="120" height="40" rx="6" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="346" y="52" font-size="11" fill="#854d0e" text-anchor="middle">時間切れ</text>
<text x="346" y="67" font-size="9" fill="#475569" text-anchor="middle">通ったか分からない</text>
<rect x="422" y="34" width="120" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="482" y="58" font-size="11" fill="#1e40af" text-anchor="middle">もう一度 POST</text>
<rect x="558" y="34" width="148" height="40" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="632" y="52" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">送金 2 回目</text>
<text x="632" y="67" font-size="9" fill="#b91c1c" text-anchor="middle">2000 円が送られた ✕</text>
<line x1="134" y1="54" x2="146" y2="54" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="270" y1="54" x2="282" y2="54" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="406" y1="54" x2="418" y2="54" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="542" y1="54" x2="554" y2="54" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<text x="14" y="122" font-size="12" font-weight="700" fill="#047857">② 冪等キーあり（キー K を毎回付ける）</text>
<rect x="14" y="134" width="120" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="74" y="152" font-size="11" fill="#1e40af" text-anchor="middle">POST 1000 円</text>
<text x="74" y="167" font-size="9" fill="#475569" text-anchor="middle">キー K</text>
<rect x="150" y="134" width="120" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="210" y="152" font-size="11" fill="#047857" text-anchor="middle">送金 1 回目</text>
<text x="210" y="167" font-size="9" fill="#475569" text-anchor="middle">K の答えを記録</text>
<rect x="286" y="134" width="120" height="40" rx="6" fill="#fefce8" stroke="#eab308" stroke-width="2"/>
<text x="346" y="152" font-size="11" fill="#854d0e" text-anchor="middle">時間切れ</text>
<text x="346" y="167" font-size="9" fill="#475569" text-anchor="middle">通ったか分からない</text>
<rect x="422" y="134" width="120" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="482" y="152" font-size="11" fill="#1e40af" text-anchor="middle">もう一度 POST</text>
<text x="482" y="167" font-size="9" fill="#475569" text-anchor="middle">同じキー K</text>
<rect x="558" y="134" width="148" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="632" y="152" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">記録済みの答え</text>
<text x="632" y="167" font-size="9" fill="#047857" text-anchor="middle">送金は 1 回だけ ✓</text>
<line x1="134" y1="154" x2="146" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="270" y1="154" x2="282" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="406" y1="154" x2="418" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<line x1="542" y1="154" x2="554" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-083)"/>
<rect x="14" y="200" width="692" height="56" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="360" y="222" font-size="11" fill="#0f172a" text-anchor="middle">時間切れは「失敗した」ではなく「結果が分からない」。相手の側ではもう終わっているかもしれない</text>
<text x="360" y="242" font-size="11" fill="#0f172a" text-anchor="middle">同じ操作に同じキーを付けて送れば、受ける側が「2回目」を見分けて、前の答えを返せる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 冪等キーの有無による違い。キーが無いと、時間切れの後のやり直しで二重に送金される。同じキーを付けて送れば、受ける側が2回目を見分けて、記録済みの答えを返す</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 時間切れは「失敗」ではなく「分からない」

- ①では、1回目の POST で、サーバーは送金を済ませていました。ただ、その答えが 0.5 秒遅れて、クライアントの 0.2 秒の上限に間に合わなかっただけです。クライアントから見ると「時間切れ」ですが、**相手の側では、もう終わっている**かもしれません。
- それを「失敗した」と思ってやり直したので、2回目の送金が起き、合計 2000 円が送られました。サーバーが実際に送金した回数は 2 です。

##### 解説2: 冪等キーは、操作ごとの「整理番号」

- ②では、送金の操作を始める前に、キー（ランダムな整理番号）を1つ作り、**やり直しのときも同じキー**を `Idempotency-Key` ヘッダーに付けて送っています。
- サーバーは、送金するときにキーと答えを記録しておきます。同じキーのリクエストがもう一度来たら、送金はせずに、記録しておいた答えを返します。実行結果の「（記録済みの答え）」が、それです。送金は1回だけでした。
- キーは「操作ごと」に作ります。やり直しのたびに新しいキーを作ってしまうと、サーバーには別の操作に見えて、意味がありません。

##### 解説3: 冪等な操作と、冪等でない操作

- 「ノートを読む」（GET）は、何回やっても結果が同じなので、冪等です。安心して再試行できます。
- 「タイトルを『買い物』に変える」（同じ値に書き換える）も、2回やっても結果は同じなので、冪等です。
- 「お金を送る」「ノートを新しく作る」（POST）は、2回やれば2回分のことが起きるので、冪等ではありません。冪等キーのような仕組みが無いなら、**時間切れでも自動では再試行しない**のが安全です。

> **注意:** この例の記録（`answers`）は、メモリの中の `HashMap` です。本物の決済のサービスでは、キーと答えを DB に保存し、「送金」と「キーの記録」を同じ transaction で行います（第7巻 UNIT 05）。メモリだけだと、サーバーの再起動で記録が消え、2回目を見分けられなくなります。外の決済の API を使うときは、その API が冪等キーを受け付けるかどうかを、ドキュメントで確かめましょう。

### よくあるエラーと読み方

#### エラー1: send() の後の .await を忘れる（E0599）

```rust
#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // send() の後の .await を忘れた
    let response = client.get("http://127.0.0.1:3000/health").send();
    println!("{}", response.status());
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0599]: no method named `status` found for opaque type `impl Future<Output = Result<Response, reqwest::Error>>` in the current scope
 --> src\bin\e08_forgot_await.rs:6:29
  |
6 |     println!("{}", response.status());
  |                             ^^^^^^ method not found in `impl Future<Output = Result<Response, reqwest::Error>>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `async_lab` (bin "e08_forgot_await") due to 1 previous error
```

- **何が起きたか:** 「`impl Future<Output = Result<Response, reqwest::Error>>` という型には、`status` というメソッドがありません」というエラーです。`send()` が返すのは、まだ送っていないリクエストの Future で、`Response` ではありません（UNIT 01 の例1-1）。
- 型の名前の中に `Output = Result<Response, …>` とあるのが手がかりです。「`.await` すれば、`Result<Response, …>` が出てくる Future」という意味です。
- **直し方:** `.send().await` で Future を待ち、出てきた `Result` を `?` か `match` で開けてから、`response.status()` を呼びます。`.send().await?.status()` の順番です。

#### エラー2: 答えの JSON の形が、struct と合わない（error decoding response body）

例8-2の `/broken` の結果です。

```text
/broken: 失敗 error decoding response body for url (http://127.0.0.1:PORT/broken)
    原因: missing field `celsius` at line 1 column 17
```

- **何が起きたか:** 答えの JSON に、struct の `celsius` の欄がありませんでした。外側のエラーだけを表示すると「本文を読み解けませんでした」としか分からないので、例8-2のように `source()` も表示すると、どの欄が問題かが分かります。
- **直し方:** まず、相手の API のドキュメントと、実際に届いた本文（`response.text().await` で文字列のまま見る）を見比べます。無いことがある欄なら `Option` にし（例8-2のヒント）、名前が違う（`"temp_c"` など）なら、struct の欄の名前を合わせるか、serde の `#[serde(rename = "temp_c")]` で読み替えます。

#### エラー3（エラーが出ない間違い）: やり直してはいけない操作を、自動でやり直す

例8-5の①が、このいちばん大事な間違いです。エラーは出ず、プログラムは「成功しました」と表示しますが、お金は2回送られています。

- 時間切れや接続の切断の後に、**冪等でない操作**（POST の作成・送金・メールの送信など）を自動で再試行すると、二重の作成・二重の送金・同じメールが2通、という事故が起きます。
- **直し方:** 再試行の関数（例8-4の `get_with_retry`）は、GET のような冪等な操作にだけ使います。冪等でない操作を再試行したいときは、相手が冪等キーを受け付けるかを確かめ、キーを付けて送ります。受け付けないなら、自動では再試行せず、「処理が完了したか分かりません。履歴を確かめてください」のように、利用者に確かめてもらう流れにします。

> **ヒント:** もう1つ、エラーにならない間違いが「すぐに、何度でもやり直す」ことです。待ち時間なし・回数の上限なしで再試行すると、相手が本当に壊れているとき、こちらのサーバーが相手に大量のリクエストを送り続けて、相手が立ち直るのを邪魔します。指数バックオフ・ジッター・回数の上限の3つは、いつもセットで使いましょう。

### RustNote ではここで使う

RustNote は、外の API を呼びません（`Cargo.toml` に reqwest は入っていません）。RustNote が話す相手は、ブラウザと DB だけです。それでも、この単元の考え方は、RustNote の**答える側**でそのまま使われています。

ノートを作る処理（`src/routes/notes.rs` の `create`）を抜き出します。

```rust
#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;

    let errors = match NewNote::parse(&input.title, &input.body) {
        Ok(new_note) => {
            let note = app_state(cx)
                .notes
                .insert(user.id, &new_note)
                .await
                .map_err(AppError::from)?;
            flash::set(cx, Flash::NoteCreated);
            // PRG: 作成できたら 303 で詳細ページ（GET）へ
            return Err(see_other(href!(id::show, id::Id(note.id)).resolve(cx)).into());
        }
        Err(errors) => errors,
    };

    // （入力に問題があったときの 400 の再表示は省略）
}
```

- `POST /notes` は、送られてくるたびに `insert` で新しいノートを作ります。**冪等ではない操作**です。RustNote には冪等キーの仕組みは入っていないので、同じフォームが2回送られれば、ノートは2つできます。
- そこで RustNote は、作成に成功したら **303 See Other** で詳細ページ（GET）へ移動させています。この形を **PRG**（Post/Redirect/Get）と呼びます。もし POST の答えとしてページをそのまま返していたら、利用者がブラウザの「再読み込み」を押したときに、ブラウザが同じ POST をもう一度送ろうとして、ノートが2つできてしまいます。303 で GET のページへ移っておけば、再読み込みは冪等な GET になります。
- ノートの編集のときの「同時に2人が編集して、片方の変更が消える」問題は、第7巻 UNIT 05 の `version` 列で防いでいます。「同じ操作が二重に、あるいは食い違って届いても、データを壊さない」ための工夫は、答える側にも必要なのです。

> **ヒント:** RustNote に、外の API を呼ぶ機能（たとえば、ノートを翻訳する API や、メールを送る API）を足すときは、この単元の形がそのまま使えます。`reqwest::Client` を `AppState` に1つ持たせて使い回し、時間の上限を付け、GET なら例8-4の再試行を、メールの送信のように冪等でない操作なら冪等キーか「自動ではやり直さない」を選びます。

### 練習問題

**問1:** 例8-4の `u08_retry.rs` で、①の `get_with_retry` の最大回数を `5` から `2` に変えると、①の結果はどうなるでしょうか。予想してから実行してください（`src/bin/u08_q1.rs`）。

**問2:** 次の答え（または失敗）のうち、`is_retryable` が `true`（やり直す）を返すのはどれでしょうか。理由も考えましょう。
1. 200 OK　2. 400 Bad Request　3. 401 Unauthorized　4. 404 Not Found　5. 429 Too Many Requests　6. 500 Internal Server Error　7. 503 Service Unavailable　8. 時間切れ（`is_timeout`）　9. JSON の形が違う（decoding のエラー）

**問3:** ばらつきを足さない `backoff_without_jitter(attempt)`（100 ミリ秒から倍々、上限 2 秒）を作り、1回目から6回目までの失敗のあとに待つ時間と、待った時間の合計を表示してください（`src/bin/u08_q3.rs`）。何回目から上限の 2 秒になるかを予想しましょう。

### 解答と解説

**問1の解答:** `get_with_retry(&client, &format!("http://{address}/flaky"), 2)` に変えて実行した結果です（②はそのまま）。

```text
① 調子の悪いサーバー（503 が2回続く）
  0.00 秒: 1 回目 → 503 Service Unavailable
  140ms 待ってやり直します
  0.15 秒: 2 回目 → 503 Service Unavailable
結果: Err("2 回やっても成功しませんでした")
② 存在しない URL（404 はやり直さない）
  0.00 秒: 1 回目 → 404 Not Found
結果: Err("HTTP status client error (404 Not Found) for url (http://127.0.0.1:PORT/nothing)")
```

- 2回とも 503 だったので、上限の2回で諦めて `Err` になりました。3回目なら成功していたはずです。
- 回数の上限は、「何回までなら相手が立ち直るのを待つ価値があるか」と「利用者をどれだけ待たせてよいか」の兼ね合いで決めます。最後のやり直しの後には待たない（`attempt < max_attempts` のときだけ待つ）ので、2回目の失敗の後に「待ってやり直します」は出ていません。

**問2の解答:** やり直すのは **5（429）・6（500）・7（503）・8（時間切れ）** の4つです。

- 1 の 200 は成功なので、やり直す必要がありません。
- 2 の 400（頼み方がおかしい）・3 の 401（ログインしていない）・4 の 404（無い）は、頼む側の問題です。同じリクエストを何回送っても、答えは変わりません。
- 5 の 429（混んでいる・頼みすぎ）と 7 の 503（一時的に使えない）は、時間をおけば成功するかもしれない、と相手が教えてくれている答えです。6 の 500（相手の中の問題）も、一時的なことがあるので、回数の上限つきでやり直します。
- 8 の時間切れは、一時的な混雑かもしれないので、やり直す価値があります。ただし、例8-5のとおり、**冪等な操作のときだけ**です。
- 9 の JSON の形の違いは、答えそのものが期待と違うので、やり直しても同じ答えが返ってくるだけです。

**問3の解答:**

```rust
use std::time::Duration;

// n 回目の失敗のあとに待つ時間（ばらつきなし）: 100ms から倍々、上限 2 秒
fn backoff_without_jitter(attempt: u32) -> Duration {
    let base = Duration::from_millis(100) * 2u32.pow(attempt - 1);
    base.min(Duration::from_secs(2))
}

fn main() {
    let mut total = Duration::ZERO;
    for attempt in 1..=6 {
        let wait = backoff_without_jitter(attempt);
        total += wait;
        println!(
            "{attempt} 回目の失敗のあと: {}ms 待つ（待った合計 {}ms）",
            wait.as_millis(),
            total.as_millis()
        );
    }
}
```

```text
1 回目の失敗のあと: 100ms 待つ（待った合計 100ms）
2 回目の失敗のあと: 200ms 待つ（待った合計 300ms）
3 回目の失敗のあと: 400ms 待つ（待った合計 700ms）
4 回目の失敗のあと: 800ms 待つ（待った合計 1500ms）
5 回目の失敗のあと: 1600ms 待つ（待った合計 3100ms）
6 回目の失敗のあと: 2000ms 待つ（待った合計 5100ms）
```

- 倍々にすると、6回目は本当なら 3200ms ですが、上限の 2000ms で止まっています。上限が無いと、10回目には 51 秒、20回目には 14 時間以上になってしまいます。
- 待った時間の合計は、6回の失敗で 5.1 秒です。再試行は、回数を増やすほど全体の時間がどんどん長くなる、ということが数字で分かります。画面のための呼び出しなら、回数は少なめ（2〜3回）にし、全体に UNIT 06 の `timeout` を付けましょう。

### まとめ

- reqwest の `Client` は1つ作って使い回し、`connect_timeout` と `timeout` で上限を付ける。`send()` の `Ok` は「答えが届いた」で、4xx・5xx は `error_for_status` で `Err` に変える。JSON は `#[derive(Deserialize)]` の struct で受け取る。
- 失敗は「つながらない（`is_connect`）・時間切れ（`is_timeout`）」と「ステータス・形の違い」に分けて考え、エラーの `source()` の連なりをたどって読む。
- 再試行は、一時的な失敗（5xx・429・時間切れ・接続できない）だけを、指数バックオフ＋ジッター＋回数の上限で行う。時間切れは「分からない」なので、冪等でない操作は冪等キーが無ければ自動でやり直さない。RustNote は、答える側で PRG（303）と `version` 列を使っている。
