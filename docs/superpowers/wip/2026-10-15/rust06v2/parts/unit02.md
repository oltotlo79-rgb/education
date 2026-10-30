## UNIT 02: Topcoat 0.11 の最小のアプリ

### この単元のゴール

- `cargo add` で Topcoat 0.11.0 と Tokio を足し、ページが1つだけの Web アプリを作って起動できる
- `#[page("/")]`・`Router::builder().discover().build()`・`topcoat::start` の3つが、それぞれ何をしているかを説明できる
- UNIT 01 で自分で書いた処理（`Content-Length`・404・405・HEAD）を、Topcoat が代わりにしてくれることを、`curl.exe` の表示で確かめられる
- `HOST` と `PORT` の環境変数で待ち受ける場所を変え、`.await` の書き忘れ・`use` の書き忘れ・ポートの指定の間違いのエラーを読める

### たとえ話でつかむ

UNIT 01 では、お店の窓口を1人で切り盛りしました。お客さんの申込書を1行ずつ読み、「その窓口ではできません」「その書類はありません」を自分で書き、返事の紙の枚数（`Content-Length`）まで自分で数えました。

UNIT 02 では、**受付の係を雇います**。

- 受付の係は、申込書を読み、どの担当に回すかを決め、担当がいなければ「その書類はありません」（404）、担当はいても方法が違えば「その方法は使えません」（405）と、決まった形で返事をしてくれます。返事の紙の枚数も、係が数えます。
- あなたがすることは、**担当者の名札**を書くことだけです。「`/` に見に来た人には、この人が答えます」と書いた名札を、担当者に付けておきます。
- 開店のとき、受付の係は、名札を付けた担当者を**全員集めて**、名簿を作ります。
- お店の**住所と窓口の番号**（`HOST` と `PORT`）は、開店の前に掲示板（環境変数）に書いておけば、係がそれを読みます。

| お店 | Topcoat のアプリ |
|---|---|
| 受付の係 | **Topcoat**（ルーターと、待ち受けの仕組み） |
| 担当者に付ける名札 | `#[page("/")]`（この関数は `GET /` に答える、という印） |
| 名札の付いた担当者を集めて名簿を作る | `Router::builder().discover().build()` |
| 開店して、窓口を開ける | `topcoat::start(router).await` |
| 掲示板に書いた住所と窓口の番号 | 環境変数 `HOST` と `PORT`（無ければ `127.0.0.1` と `3000`） |
| 決まった形の断り方 | 無い URL は 404、使えないメソッドは 405 と `allow` |

> **用語:** Web フレームワーク（ウェブフレームワーク：要求を読み、URL ごとの処理に振り分け、応答を返す、といった決まった仕事を引き受けてくれる道具。Topcoat もその1つ）

<!-- -->

> **用語:** ルーター（router：要求の URL とメソッドを見て、どの関数に処理させるかを決める仕組み）

### 図で理解する

最小のアプリの部品と、要求が届いてから応答を返すまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="46" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">#[page("/")]</text>
<text x="120" y="66" font-size="11" fill="#0f172a" text-anchor="middle">async fn home()</text>
<text x="120" y="82" font-size="11" fill="#475569" text-anchor="middle">→ view! で HTML</text>
<rect x="260" y="20" width="200" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="46" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Router::builder()</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">.discover()</text>
<text x="360" y="82" font-size="11" fill="#0f172a" text-anchor="middle">.build()</text>
<rect x="500" y="20" width="200" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="46" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">topcoat::start(router)</text>
<text x="600" y="66" font-size="11" fill="#0f172a" text-anchor="middle">HOST・PORT を読む</text>
<text x="600" y="82" font-size="11" fill="#0f172a" text-anchor="middle">待ち受けを始める</text>
<line x1="220" y1="55" x2="256" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
<line x1="460" y1="55" x2="496" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
<text x="238" y="44" font-size="10" fill="#475569" text-anchor="middle">集める</text>
<text x="478" y="44" font-size="10" fill="#475569" text-anchor="middle">渡す</text>
<rect x="20" y="150" width="680" height="120" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="6 4"/>
<text x="360" y="174" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">要求が届いたら（Topcoat が引き受ける）</text>
<rect x="40" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="115" y="216" font-size="12" fill="#0f172a" text-anchor="middle">GET /</text>
<text x="115" y="236" font-size="11" fill="#047857" text-anchor="middle">→ home を呼ぶ → 200</text>
<rect x="210" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="285" y="216" font-size="12" fill="#0f172a" text-anchor="middle">GET /nothing</text>
<text x="285" y="236" font-size="11" fill="#c2410c" text-anchor="middle">→ 404 not found</text>
<rect x="380" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="455" y="216" font-size="12" fill="#0f172a" text-anchor="middle">POST /</text>
<text x="455" y="236" font-size="11" fill="#c2410c" text-anchor="middle">→ 405・allow: GET, HEAD</text>
<rect x="550" y="190" width="130" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="615" y="216" font-size="12" fill="#0f172a" text-anchor="middle">HEAD /</text>
<text x="615" y="236" font-size="11" fill="#047857" text-anchor="middle">→ ヘッダーだけ</text>
<line x1="600" y1="90" x2="600" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 最小のアプリの部品。名札（#[page]）を付けた関数を discover で集めてルーターを作り、start に渡す。要求の振り分けと、404・405・HEAD の返事は Topcoat がする</div>
</div>

- 自分で書くのは、`#[page("/")]` を付けた関数 `home` と、`main` の2行だけです。
- `discover()` は、プログラムの中の `#[page]` を付けた関数を、**全部自動で集めます**。ページを増やすときは、`#[page("/about")]` を付けた関数を足すだけで、`main` を書き換える必要はありません（問1）。
- 無い URL の 404、使えないメソッドの 405 と `allow`、HEAD のときに本文を付けないこと、`Content-Length` の計算は、どれも Topcoat がします。UNIT 01 で自分で書いた処理です。

### まず動かす最小コード

#### 例2-1: プロジェクトを作り、Topcoat と Tokio を足す

> **▼ このコマンドがやること（先に日本語で）:** 新しいプロジェクトを作り、Rust の版を 1.99.1 に固定してから、`cargo add` で Topcoat（版を 0.11.0 に固定）と Tokio（非同期のランタイム）を足します。

```powershell
cargo new web_lab
cd web_lab
Set-Content rust-toolchain.toml "[toolchain]`nchannel = `"1.99.1`""
cargo add topcoat@=0.11.0
cargo add tokio@=1.54.1 --features rt-multi-thread,macros
```

**▼ 実行結果**（`cargo add topcoat@=0.11.0` の表示）

```text
    Updating crates.io index
      Adding topcoat =0.11.0 to dependencies
             Features:
             + asset
             + compression
             + cookie
             + discover
             + font
             + icon
             + router
             + runtime
             + serve
             + session
             + view
             22 deactivated features
    Updating crates.io index
     Locking 220 packages to highest Rust 1.99.1 compatible versions
      Adding generic-array v0.14.7 (available: v0.14.9)
```

- `+` の付いた features（`asset`・`router`・`view` など）が、既定で有効になる機能です。そのほかの機能（`tailwind`・`websocket`・`l10n` など）は既定では無効で、数が多いので、`22 deactivated features`（無効の機能が 22 個）とだけ表示されます。一覧は `cargo info -v topcoat@0.11.0` で見られます。必要なときに、自分で足します。
- `Locking 220 packages to highest Rust 1.99.1 compatible versions` は、「Rust 1.99.1 で使える範囲で、いちばん新しい版を選んで、220 個のクレートの版を `Cargo.lock` に書きました」という意味です。`Adding generic-array v0.14.7 (available: v0.14.9)` は、「もっと新しい 0.14.9 もありますが、0.14.7 にしました」というお知らせです。間接的に使う `crypto-common` が `=0.14.7` と版を固定しているからで、気にしなくて大丈夫です。
- `@=0.11.0` の `=` を付けているので、Topcoat に新しい版が出ても、勝手には上がりません（第13巻 UNIT 01）。Topcoat は、版が上がると書き方が変わることがあるので、版を上げるときは、自分で決めて、まとめて直します。この本も、0.10.0 から 0.11.0 に上げたときに、すべての巻のコードを動かし直しました。

できあがった `Cargo.toml` です（この単元の最後に、もう1つの機能を足したあとの形です。例2-3で説明します）。

```toml
[package]
name = "web_lab"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macros", "net"] }
topcoat = "=0.11.0"
```

#### 例2-2: ページが1つだけのアプリ

> **▼ このコードがやること（先に日本語で）:** `GET /` に答える関数 `home` に `#[page("/")]` の名札を付け、`view!` で HTML を作って返します。`main` では、名札の付いた関数を `discover()` で集めてルーターを作り、`topcoat::start` に渡して、待ち受けを始めます。

```rust
//! Topcoat 0.11 の、いちばん小さな Web アプリ。GET / に HTML を返す。
//! 実行: cargo run

use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

// URL「/」に GET が来たら、この関数が HTML を作って返す
#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"はじめての Topcoat"</title>
            </head>
            <body>
                <h1>"こんにちは、Topcoat！"</h1>
                <p>"この HTML は Rust のプログラムが作りました。"</p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    // #[page] の付いた関数を、プログラム全体から集めて、ルーターを作る
    let router = Router::builder().discover().build();
    // 環境変数 HOST と PORT の場所（無ければ 127.0.0.1:3000）で待ち受ける
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin web_lab
```

最初の1回は、Topcoat が使うクレートをコンパイルするので、少し時間がかかります（この PC では、`Compiling` の行が 164 行出て、約 22 秒でした）。

```text
   Compiling web_lab v0.1.0 (...\web_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 21.90s
```

**サーバーの窓には、何も表示されません。** `topcoat::start` は、起動しても何も表示しないからです。止まっているように見えますが、待ち受けています。2つ目の窓から、`curl.exe` で確かめます。

```powershell
curl.exe -i http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 228
date: Fri, 30 Oct 2026 11:31:05 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>はじめての Topcoat</title></head><body><h1>こんにちは、Topcoat！</h1><p>この HTML は Rust のプログラムが作りました。</p></body></html>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[page("/")] は、URL の名札

- `#[page("/")]` は、「この関数は `GET /` に答える」という印です。`#[page]` は、ふつうの関数を、ページの処理に変える**属性マクロ**です。何も指定しなければ GET に答え、HEAD にも自動で答えます。
- 関数の名前（`home`）は、URL には関係ありません。URL を決めるのは、`#[page(...)]` の中の文字列です。
- 戻り値の `Result<impl View>` は、「HTML になる何か（`View`）を返す。失敗することもある」という意味です。`view!` の中には、HTML をほぼそのまま書けます。文字は `"..."` で囲みます（UNIT 05）。

##### 解説2: discover は、名札を自動で集める

- `Router::builder().discover().build()` は、プログラムの中にある `#[page]` の関数を全部集めて、URL の名簿（ルーター）を作ります。
- `discover` は、`RouterBuilderDiscoverExt` という trait のメソッドです。だから、`use topcoat::router::RouterBuilderDiscoverExt` が要ります（エラー2）。

##### 解説3: Topcoat が付けてくれたヘッダー

- 応答のヘッダーは、`content-type: text/html; charset=utf-8`（HTML です。文字は UTF-8 です）・`content-length: 228`・`date`（応答を作った時刻、UTC）のほかに、`vary` が2行付いています（問3）。UNIT 01 では、`Content-Length` を自分で数えて、日本語で間違えました。Topcoat は、本文のバイト数を正しく数えてくれます。
- ヘッダーの名前が小文字（`content-type`）になっています。HTTP のヘッダーの名前は、大文字と小文字を区別しない決まりなので、`Content-Type` と同じ意味です。
- 本文の HTML は、`view!` に書いたとおりの1行になっています。字下げや改行は、`view!` が取り除いて、短くしています。

#### ▼ Topcoat が引き受けてくれる返事

UNIT 01 の例1-2で、自分で書いた返事を、Topcoat が返すか確かめます。

```powershell
curl.exe -i http://127.0.0.1:3000/nothing
curl.exe -i -X POST http://127.0.0.1:3000/
curl.exe -I http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
### curl.exe -i http://127.0.0.1:3000/nothing
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9
date: Fri, 30 Oct 2026 11:31:05 GMT

not found

### curl.exe -i -X POST http://127.0.0.1:3000/
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Fri, 30 Oct 2026 11:31:05 GMT



### curl.exe -I http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 228
date: Fri, 30 Oct 2026 11:31:05 GMT
```

- 無い URL（`/nothing`）は `404 Not Found` で、本文は `not found` だけです。ページに名札を付けていない URL は、Topcoat が自動で 404 にします。自分の画面（ヘッダーやトップへのリンクの付いたもの）で 404 を出したいときは、`not_found!()` とレイアウトを使います（UNIT 03・10）。
- `POST /` は `405 Method Not Allowed` と `allow: GET, HEAD` です。`#[page]` は GET（と HEAD）だけに答えるので、Topcoat が「使えるのは GET と HEAD です」と教えてくれました。UNIT 01 の問1で「本当は 405 と Allow を返すべき」と書いたことを、Topcoat は何も書かなくてもしてくれます。
- `HEAD /`（`-I`）は、GET と同じヘッダー（`content-length: 228` も同じ）で、本文はありません。

### もう一歩: 実務に近い形

#### 例2-3: 待ち受けの場所を押さえてから、「待っています」と表示する

`topcoat::start` は、何も表示しません。「起動しました」と表示したくて、`start` の前に `println!` を書くと、困ったことが起きます。ポートを押さえる**前**に表示するので、ほかのプログラムがポートを使っていて起動に失敗したときも、「起動しました」と出てしまうのです（第12巻 UNIT 01 のエラー2で、RustNote が実際にそうなっているのを見ました）。

そこで、待ち受けの場所を**自分で先に押さえて**から表示し、押さえた場所を Topcoat に渡す書き方をします。`topcoat::start` の中身（Topcoat 0.11.0 の `serve.rs`）も、`TcpListener::bind` で押さえてから `serve` を呼ぶ形になっています。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">start の前に表示する（困る形）</text>
<rect x="20" y="36" width="320" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① 「起動しました」と表示</text>
<rect x="20" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="180" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② topcoat::start の中で</text>
<text x="180" y="138" font-size="11" fill="#475569" text-anchor="middle">bind（押さえる）→ serve（さばく）</text>
<rect x="20" y="172" width="320" height="56" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="180" y="194" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">bind に失敗しても、①はもう出ている</text>
<text x="180" y="214" font-size="11" fill="#475569" text-anchor="middle">表示と本当の状態が食い違う</text>
<line x1="180" y1="76" x2="180" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="180" y1="152" x2="180" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">押さえてから表示する（例2-3）</text>
<rect x="380" y="36" width="320" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① TcpListener::bind で押さえる</text>
<rect x="380" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="540" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② 押さえられたときだけ</text>
<text x="540" y="138" font-size="11" fill="#475569" text-anchor="middle">「待っています」と表示</text>
<rect x="380" y="172" width="320" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="194" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ topcoat::serve(listener, router)</text>
<text x="540" y="214" font-size="11" fill="#475569" text-anchor="middle">失敗したら①で止まり、②は出ない</text>
<line x1="540" y1="76" x2="540" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="540" y1="152" x2="540" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 「起動しました」を表示する場所の違い。start は押さえるのとさばくのを中でまとめてするので、間に表示を入れられない。自分で押さえてから表示し、serve に渡せば、表示と状態が食い違わない</div>
</div>

> **▼ このコードがやること（先に日本語で）:** Tokio の `TcpListener::bind` で `127.0.0.1:3000` を押さえます。押さえられなかったら、そこでエラーとして終わります。押さえられたときだけ、「待っています」と表示してから、`topcoat::serve` に、押さえた場所とルーターを渡します。

```rust
//! 先に待ち受けの場所（127.0.0.1:3000）を押さえてから、「待っています」と表示する版。
//! 実行: cargo run --bin u02_serve

use tokio::net::TcpListener;
use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>"u02_serve で動いています"</h1>
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::builder().discover().build();
    // 先にアドレスとポートを押さえる。ほかのプログラムが使っていたら、ここでエラーになって止まる
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    // 押さえられたときだけ、ここまで来る
    println!(
        "http://{} で待っています（Ctrl+C で止めます）",
        listener.local_addr()?
    );
    topcoat::serve(listener, router).await
}
```

このプログラムは `tokio::net::TcpListener` を使うので、Tokio の `net` の機能を、自分の `Cargo.toml` に足します。Topcoat がすでに `net` を有効にしているので、足さなくてもビルドは通りますが、**自分が使う機能は自分の `Cargo.toml` に書く**のが決まりです（第13巻 UNIT 01）。

```powershell
cargo add tokio@=1.54.1 --features rt-multi-thread,macros,net
```

`src/bin/u02_serve.rs` として保存し、1つ目の窓で起動します。続けて、2つ目の窓で、同じものをもう1つ起動します。

```powershell
cargo run --bin u02_serve
```

**▼ 実行結果（1つ目）**

```text
http://127.0.0.1:3000 で待っています（Ctrl+C で止めます）
```

**▼ 実行結果（2つ目。ポートがもう使われている）**

```text
Error: Os { code: 10048, kind: AddrInUse, message: "通常、各ソケット アドレスに対してプロトコル、ネットワーク アドレス、またはポートのどれか 1 つのみを使用できます。" }
```

- 1つ目は、ポートを押さえてから「待っています」と表示しました。2つ目は、`TcpListener::bind` で失敗したので、「待っています」を表示せずに、エラーで終わりました。表示と、本当の状態が食い違いません。
- `Error: Os { code: 10048, kind: AddrInUse, ... }` は、`main` が返した `Err` を、Rust が `{:?}` の形で表示したものです。`AddrInUse` は「アドレスはもう使われています」という意味です（UNIT 01 のエラー3と同じ原因です）。
- `local_addr()` は、実際に押さえた場所を返します。ポートに 0 を指定すると、空いている番号を OS が選んでくれるので、そのときに「何番になったか」を知るのにも使えます。

### よくあるエラーと読み方

#### エラー1: start の .await を書き忘れた（E0599）

`topcoat::start(router)` の次の行の `.await` を消してしまった場合です。

```rust
    topcoat::start(router)
        .expect("サーバーを起動できませんでした");
```

```powershell
cargo build
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0599]: no method named `expect` found for opaque type `impl Future<Output = Result<(), std::io::Error>>` in the current scope
  --> src\main.rs:34:10
   |
33 | /     topcoat::start(router)
34 | |         .expect("サーバーを起動できませんでした");
   | |         -^^^^^^ method not found in `impl Future<Output = Result<(), std::io::Error>>`
   | |_________|
   |
   |
help: consider `await`ing on the `Future` and calling the method on its `Output`
   |
34 |         .await.expect("サーバーを起動できませんでした");
   |          ++++++

For more information about this error, try `rustc --explain E0599`.
error: could not compile `web_lab` (bin "web_lab") due to 1 previous error
```

- **何が起きたか:** 「`impl Future<Output = Result<(), std::io::Error>>` に、`expect` というメソッドはありません」というエラーです。`topcoat::start` は async の関数なので、呼んだだけでは、まだ動いていない「これから動く仕事」（`Future`）が返るだけです。`expect` は `Result` のメソッドなので、`Future` には使えません（第5巻 UNIT 01）。
- **直し方:** `help:` のとおり、`.await` を付けます。`.await` で仕事を最後まで動かすと、`Result` が手に入り、`expect` が使えます。

#### エラー2: discover の trait を use し忘れた（E0599）

`use topcoat::router::{Router, RouterBuilderDiscoverExt, page}` から、`RouterBuilderDiscoverExt` を消してしまった場合です。

```powershell
cargo build
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0599]: no method named `discover` found for struct `RouterBuilder` in the current scope
  --> src\main.rs:31:36
   |
31 |     let router = Router::builder().discover().build();
   |                                    ^^^^^^^^
   |
  ::: ...\router.rs:14:8
   |
14 |     fn discover(self) -> Self;
   |        -------- the method is available for `RouterBuilder` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `RouterBuilderDiscoverExt` which provides `discover` is implemented but not in scope; perhaps you want to import it
   |
 4 + use topcoat::router::RouterBuilderDiscoverExt;
   |
help: there is a method `discover_fonts` with a similar name
   |
31 |     let router = Router::builder().discover_fonts().build();
   |                                            ++++++

For more information about this error, try `rustc --explain E0599`.
error: could not compile `web_lab` (bin "web_lab") due to 1 previous error
```

- **何が起きたか:** 「`RouterBuilder` に、`discover` というメソッドが見つかりません」というエラーです。`discover` は、`RouterBuilder` 自身のメソッドではなく、`RouterBuilderDiscoverExt` という trait が足しているメソッドです。trait のメソッドは、その trait を `use` しているときだけ使えます（`help: items from traits can only be used if the trait is in scope`）。
- **直し方:** 2つ目の `help:` のとおり、`use topcoat::router::RouterBuilderDiscoverExt;` を足します。3つ目の `help:` の `discover_fonts`（名前が似ているメソッド）は、別の機能なので、案内に従わないでください。

#### エラー3: PORT に数ではない値を入れた

PowerShell で `$env:PORT = "abc"` としてから起動した場合です。

```powershell
$env:PORT = "abc"
cargo run --bin web_lab
```

```text

thread 'main' (202724) panicked at src\main.rs:35:10:
サーバーを起動できませんでした: Custom { kind: InvalidInput, error: "PORT must be a valid port number: invalid digit found in string" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `topcoat::start` が、環境変数 `PORT` を数として読めずに、`InvalidInput`（入力の値が不正）のエラーを返しました。`PORT must be a valid port number: invalid digit found in string` は「PORT は正しいポートの番号でなければなりません。文字列に数字ではない文字があります」です。`main` の `.expect(...)` が、このエラーを受けて panic しました。
- **直し方:** `Remove-Item Env:PORT` で環境変数を消す（既定の 3000 に戻る）か、`$env:PORT = "3001"` のように正しい番号を入れます。PowerShell で入れた環境変数は、その窓を閉じるまで残るので、ほかのプログラムにも影響することがあります。

#### エラー4: 実行ファイルが2つになって、cargo run が迷った

`src/bin/u02_serve.rs` を足したあとに、`--bin` を付けずに `cargo run` とした場合です。

```powershell
cargo run
```

```text
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: u02_serve, web_lab
```

- **何が起きたか:** 「`cargo run` は、どの実行ファイルを動かせばよいか決められません。`--bin` で指定するか、`default-run` を書いてください」というエラーです。`src/main.rs`（`web_lab`）と `src/bin/u02_serve.rs`（`u02_serve`）の2つの実行ファイルがあるからです。
- **直し方:** `cargo run --bin web_lab` のように指定します。いつも同じものを動かすなら、`Cargo.toml` の `[package]` に `default-run = "web_lab"` と書きます。RustNote の `Cargo.toml` に `default-run = "rustnote"` があるのは、このためです（第13巻 UNIT 01）。

### RustNote ではここで使う

RustNote の `src/main.rs` の最後の部分です（抜き出しています）。

```rust
    let router = routes::build_router(AppState::new(db), assets);

    eprintln!(
        "RustNote を起動しました: http://{}",
        config::listen_address()
    );
    topcoat::start(router)
        .await
        .context("サーバーを起動できませんでした（HOST / PORT を確認してください）")?;
```

- 例2-2と同じく、`topcoat::start(router).await` で待ち受けています。`HOST` と `PORT` を読むのも同じです。公開するとき（Fly.io）は `HOST=0.0.0.0`・`PORT=8080` にします（第12巻）。
- 例2-3で見たとおり、`eprintln!` で「起動しました」を、`start` の**前**に表示しています。ポートが使われていると、この表示のあとにエラーで止まります。例2-3の形（`TcpListener::bind` で押さえてから表示し、`topcoat::serve` に渡す）にすれば直せます（RustNote のコードは変えていません）。
- RustNote のルーターは、`discover()` ではなく、`module_router!()` で作っています（`src/routes.rs` の `build_router`）。ファイルの並びをそのまま URL の並びにする書き方で、UNIT 03で扱います。

### 練習問題

**問1:** 例2-2のアプリに、2つ目のページ `GET /about` を足してください。トップページから `/about` へのリンクも付けます。足したら、`curl.exe -i http://127.0.0.1:3000/about` で確かめてください。

**問2:** `$env:PORT = "3001"` としてから例2-2のアプリを起動し、`http://127.0.0.1:3001/` と `http://127.0.0.1:3000/` に `curl.exe` で要求を送ってください。それぞれ、どうなるでしょうか。

**問3:** Topcoat の応答には、`vary: accept-encoding` というヘッダーが付いていました。`curl.exe -i --compressed http://127.0.0.1:3000/about` で、圧縮してほしいと伝えて要求を送り、ヘッダーがどう変わるかを確かめてください。`vary` は何を伝えているのでしょうか。

### 解答と解説

**問1の解答:**

`#[page("/about")]` を付けた関数を足すだけです（足したものを `src/bin/u02_q1.rs` として保存しました）。`main` は変えません。

```rust
//! 問1の解答: 例2-1のアプリに、2つ目のページ（/about）を足したもの。
//! 実行: cargo run --bin u02_q1

use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"はじめての Topcoat"</title>
            </head>
            <body>
                <h1>"こんにちは、Topcoat！"</h1>
                <p><a href="/about">"このアプリについて"</a></p>
            </body>
        </html>
    })
}

// 2つ目のページ。#[page] に URL を書いた関数を足すだけで、discover が見つけてくれる
#[page("/about")]
async fn about() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"このアプリについて"</title>
            </head>
            <body>
                <h1>"このアプリについて"</h1>
                <p>"第6巻 UNIT 02 の練習で作りました。"</p>
                <p><a href="/">"トップへ戻る"</a></p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = Router::builder().discover().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u02_q1
```

```text
### curl.exe -i http://127.0.0.1:3000/about
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 258
date: Fri, 30 Oct 2026 11:31:29 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>このアプリについて</title></head><body><h1>このアプリについて</h1><p>第6巻 UNIT 02 の練習で作りました。</p><p><a href="/">トップへ戻る</a></p></body></html>
```

- `discover()` が、新しい `about` も自動で集めたので、`main` を書き換えずに、`/about` が `200 OK` になりました。
- トップページのリンクは、`<a href="/about">` と URL を文字列で書いています。ページの名前を変えたときに、リンクを直し忘れても、コンパイルでは気づけません。Topcoat には、関数を指定してリンクの URL を作る `href!` があり、RustNote はそちらを使っています（第9巻 UNIT 04・この巻の UNIT 03）。

**問2の解答:**

```powershell
$env:PORT = "3001"
cargo run --bin web_lab
```

別の窓から:

```text
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3001/
200
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/
000
(curl の終了コード: 7)
```

- 3001 番は `200` でした。3000 番は、何も待ち受けていないので、`-w "%{http_code}"` の表示は `000`（応答が無い）で、curl は終了コード 7（接続できない）で終わりました（UNIT 01 のエラー2）。
- `topcoat::start` は、`PORT` を読んで、その番号で待ち受けます。プログラムを書き換えずに、待ち受ける場所を変えられるのです。確かめたら、`Remove-Item Env:PORT` で元に戻します。

**問3の解答:**

```powershell
curl.exe -i --compressed http://127.0.0.1:3000/about
```

```text
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-encoding: gzip
transfer-encoding: chunked
date: Fri, 30 Oct 2026 11:31:29 GMT
```

- `--compressed` を付けると、curl は `Accept-Encoding`（受け取れる圧縮の形式）のヘッダーを付けて送ります。Topcoat は、本文を gzip で圧縮し、`content-encoding: gzip`（本文は gzip で圧縮してあります）を付けて返しました。圧縮したので、`content-length` の代わりに、`transfer-encoding: chunked`（本文を区切って少しずつ送ります）になっています。curl は、受け取った本文を元に戻して表示します（ここでは、ヘッダーの部分だけを載せています）。
- `vary: accept-encoding` は、「この応答は、要求の `Accept-Encoding` によって中身が変わります」という印です。途中で応答を覚えておく仕組み（キャッシュ）が、圧縮できない相手に、圧縮した応答を渡してしまわないように付けます。`vary: Accept` も同じで、要求の `Accept`（受け取れる種類）によって中身が変わることがある、という印です。
- ブラウザは、いつも `Accept-Encoding` を付けて送るので、ブラウザから見るときは、圧縮された応答が届きます。Topcoat の既定の機能（`compression`）が、これをしています。

### まとめ

- `cargo add topcoat@=0.11.0` と Tokio を足し、`#[page("/")]` を付けた関数と、`Router::builder().discover().build()`・`topcoat::start(router).await` の数行で、Web アプリが動く。版は `=` で固定する（新しい版が出ても、勝手には上がらない）。
- 要求の振り分け・`Content-Length`・404・405 と `allow`・HEAD・圧縮は、Topcoat がする。`topcoat::start` は `HOST` と `PORT` を読み、起動しても何も表示しない。
- 「待っています」と表示したいときは、`TcpListener::bind` で先に押さえてから表示し、`topcoat::serve` に渡す。`.await` の書き忘れ、trait の `use` の書き忘れ、実行ファイルが2つあるときの `--bin` のエラーを読めるようにしておく。
