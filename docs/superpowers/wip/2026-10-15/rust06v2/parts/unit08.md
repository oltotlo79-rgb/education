## UNIT 08: アセット・CSS・束

### この単元のゴール

- CSS や画像のような静的なファイル（アセット）を `asset!` で宣言し、`view!` の中でリンク先として使える
- `topcoat asset bundle` で「束」（ハッシュ付きの名前のファイルと目録）を作り、`AssetBundle::load()` で読み込んで配れる
- ファイル名にハッシュが付く理由と、`cache-control: …, immutable` で長く覚えてもらえる理由を、CSS を書き換えて確かめられる
- 束を作り忘れた・パスを打ち間違えた・束が古いままのときのエラーと、`topcoat dev` で自動で作り直す流れを説明できる

### たとえ話でつかむ

お店のチラシを、近所に配る場面を考えます。

- チラシの原稿（CSS）は、お店の中にあります。配る前に、**印刷所で刷って、束にします**。束には、どのチラシが何枚入っているかの**目録**を付けます。
- 刷ったチラシには、**版の番号**が印刷されています。原稿を少しでも直すと、番号が変わります。お客さんは、「この番号のチラシなら、もう持っている」と、同じ番号のチラシを取っておけます（何度も取りに来なくてよい）。
- お店の入口の案内（HTML）には、「チラシは〇〇番」と、**今の版の番号**が書いてあります。原稿を直したら、案内の番号も新しくなるので、お客さんは新しいチラシを取りに来ます。
- 印刷所に行き忘れたり、原稿の場所を間違えたりすると、束ができません。原稿を足したのに刷り直さないと、案内には書いてあるのに、束に入っていないチラシができてしまいます。

| チラシ | Topcoat のアセット |
|---|---|
| チラシの原稿 | `assets/site.css`（自分で書いた CSS） |
| 「このチラシを配る」と決める | `asset!("assets/site.css")` |
| 印刷所で刷って束にする | `topcoat asset bundle`（`target\debug\assets\` に作る） |
| 束の目録 | `manifest.toml`（どの宣言が、どのファイルか） |
| 版の番号 | ファイル名のハッシュ（`site-7becc8b6682e6054.css`） |
| 入口の案内に書いた番号 | HTML の `<link href="/_topcoat/assets/site-….css">` |

> **用語:** アセット（asset：CSS・画像・フォントなど、プログラムと一緒に配る、中身が決まったファイル）

<!-- -->

> **用語:** ハッシュ（hash：ファイルの中身から計算した、決まった長さの値。中身が1文字でも違えば、まったく違う値になる。第8巻 UNIT 02）

<!-- -->

> **用語:** キャッシュ（cache：一度受け取ったファイルを、ブラウザが手元に取っておき、次は取りに行かずに使うこと）

### 図で理解する

`asset!` で宣言してから、ブラウザが CSS を受け取るまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0801" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① 宣言（コード）</text>
<text x="120" y="66" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">asset!("assets/site.css")</text>
<text x="120" y="82" font-size="10" fill="#475569" text-anchor="middle">番号（ID）だけが実行ファイルに入る</text>
<rect x="260" y="20" width="200" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">② topcoat asset bundle</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">ビルドして宣言を集め、</text>
<text x="360" y="82" font-size="11" fill="#0f172a" text-anchor="middle">ハッシュ付きの名前で書き出す</text>
<rect x="500" y="20" width="200" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle" font-family="Consolas, monospace">target\debug\assets\</text>
<text x="600" y="66" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">manifest.toml（目録）</text>
<text x="600" y="82" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">site-7becc8b6682e6054.css</text>
<line x1="220" y1="55" x2="256" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<line x1="460" y1="55" x2="496" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<rect x="260" y="140" width="200" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="164" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">③ 起動のとき</text>
<text x="360" y="184" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">AssetBundle::load()</text>
<line x1="600" y1="90" x2="464" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<rect x="20" y="210" width="330" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="232" font-size="12" fill="#0f172a" text-anchor="middle">④ HTML: &lt;link href="/_topcoat/assets/</text>
<text x="185" y="252" font-size="12" fill="#0f172a" text-anchor="middle">site-7becc8b6682e6054.css"&gt;</text>
<rect x="370" y="210" width="330" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="535" y="232" font-size="12" fill="#0f172a" text-anchor="middle">⑤ GET その URL → CSS と</text>
<text x="535" y="252" font-size="11" fill="#047857" text-anchor="middle">cache-control: public, max-age=31536000, immutable</text>
<line x1="330" y1="200" x2="230" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<line x1="390" y1="200" x2="490" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: アセットの流れ。コードには宣言だけを書き、topcoat asset bundle がハッシュ付きの名前のファイルと目録を作る。起動のときに目録を読み、HTML には今のファイル名の URL が入る</div>
</div>

- 実行ファイルの中に入るのは、CSS の中身ではなく、**宣言の番号（ID）**だけです。CSS のファイルは、束として、実行ファイルの隣に置きます。
- 起動のときに目録（`manifest.toml`）を読み、「この ID は、このファイル名」と分かるので、`view!` に `(SITE_CSS)` と書くと、今のファイル名の URL になります。

### まず動かす最小コード

#### 例8-1: CSS を束にして配る

`web_lab` に `assets` フォルダを作り、`assets/site.css` を置きます。

```css
/* UNIT 08 の練習用の見た目 */
body {
  font-family: sans-serif;
  margin: 2rem;
}

h1 {
  color: #1e40af;
}
```

> **▼ このコードがやること（先に日本語で）:** `asset!("assets/site.css")` で CSS を宣言し、ページの `<link rel="stylesheet">` の `href` に入れます。`main` では、`AssetBundle::load()` で、実行ファイルの隣の束を読み込み、`.assets(…)` でルーターに渡します。

```rust
//! 第6巻 UNIT 08: CSS を asset! で宣言し、topcoat asset bundle で束にして配る。
//! 実行: topcoat asset bundle --bin u08_assets のあとで cargo run --bin u08_assets

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

/// クレートの assets/site.css（Cargo.toml のあるフォルダからの場所）
const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 08"</title>
                <link rel="stylesheet" href=(SITE_CSS)>
            </head>
            <body>
                <h1>"アセットの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // 実行ファイルの隣の assets/ から、束を読み込む
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

起動する前に、束を作ります。実行ファイルがいくつもある `web_lab` では、`--bin` で、どの実行ファイルの束かを選びます。

```powershell
topcoat asset bundle --bin u08_assets
```

```text
bundled assets into ...\web_lab\target\debug\assets
```

```powershell
Get-ChildItem target\debug\assets -Name
Get-Content target\debug\assets\manifest.toml
```

```text
manifest.toml
site-7becc8b6682e6054.css
```

```toml
version = 1

[[assets]]
id = 6866230526714045621
file = "site-7becc8b6682e6054.css"
hash = "7becc8b6682e6054d01e53b6fa67af40714b1145b9a9b5191b328ac6e4c53d2e"
content_type = "text/css"
```

起動して、ページと CSS を開きます。

```powershell
cargo run --bin u08_assets
```

```text
### curl.exe -s http://127.0.0.1:3000/
<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 08</title><link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css"></head><body><h1>アセットの練習</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/_topcoat/assets/site-7becc8b6682e6054.css
HTTP/1.1 200 OK
content-type: text/css
cache-control: public, max-age=31536000, immutable
content-length: 117

/* UNIT 08 の練習用の見た目 */
body {
  font-family: sans-serif;
  margin: 2rem;
}

h1 {
  color: #1e40af;
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: asset! は「このファイルを配る」という宣言

- `asset!("assets/site.css")` の `assets/site.css` は、`Cargo.toml` のあるフォルダ（`web_lab`）からの場所です。`./` で始めると、そのソースファイルからの場所になります。
- `const SITE_CSS: Asset = …` と名前を付けておくと、いくつものページから同じ CSS を使えます。

##### 解説2: 束には、ハッシュ付きの名前が付く

- `topcoat asset bundle` は、プログラムを一度ビルドして、`asset!` の宣言を集め、`target\debug\assets\` に書き出しました。ファイル名は `site-7becc8b6682e6054.css` で、`-` の後ろは、CSS の中身から計算したハッシュ（`manifest.toml` の `hash` の最初の 16 文字）です。
- 目録の `id` は宣言の番号、`file` はファイル名、`content_type` は配るときの種類です。

##### 解説3: 配るときは、1年間覚えてよいと伝える

- ページの HTML には、`href="/_topcoat/assets/site-7becc8b6682e6054.css"` が入りました。`.assets(…)` を登録すると、Topcoat は `/_topcoat/assets/` の下で束のファイルを配ります。
- CSS の返事には、`cache-control: public, max-age=31536000, immutable` が付いています。「誰が覚えてもよい・31,536,000 秒（365 日）覚えてよい・中身は変わらない」という意味です。ブラウザは、2回目からは CSS を取りに来ません。
- 中身が変わったら**ファイル名（URL）が変わる**ので、長く覚えてもらっても困りません（問2）。

### もう一歩: 実務に近い形

#### 例8-2: topcoat dev で、保存するたびに作り直す

CSS を直すたびに `topcoat asset bundle` と起動をくり返すのは手間です。開発中は、`topcoat dev` で動かします。

> **▼ このコードがやること（先に日本語で）:** 例8-1と同じページに、`topcoat::dev::script()` を足します。`topcoat dev` で動かしているときだけ、ブラウザに「作り直したら読み直す」ためのスクリプトが入ります。

```rust
//! 第6巻 UNIT 08 の例8-2: topcoat dev で動かすと、保存するたびに作り直し、ブラウザも読み直す。
//! 実行: topcoat dev --bin u08_dev

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <link rel="stylesheet" href=(SITE_CSS)>
                // topcoat dev で動かしているときだけ、自動で読み直すためのスクリプトが入る
                topcoat::dev::script()
            </head>
            <body>
                <h1>"開発サーバーの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
topcoat dev --bin u08_dev
```

**▼ topcoat dev の窓の表示（最後に止めるまで）**

```text

  topcoat dev server
  watching for file changes...

  ready on http://127.0.0.1:3000

  ready on http://127.0.0.1:3000
```

2つ目の窓からページを開くと、`dev.js` のスクリプトが入っています（`59039` の番号は、起動するたびに変わります）。

```text
### curl.exe -s http://127.0.0.1:3000/
<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css"><script src="http://127.0.0.1:59039/dev.js"></script></head><body><h1>開発サーバーの練習</h1></body></html>
```

`topcoat dev` を動かしたまま、`assets/site.css` の `h1` の色を変えて保存し、もう一度開きます。

```text
### （h1 の色を #047857 に変えて保存したあと）curl.exe -s http://127.0.0.1:3000/ の <link>
<link rel="stylesheet" href="/_topcoat/assets/site-86109e8897909764.css">
### （色を #1e40af に戻して保存したあと）
<link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css">
```

- 保存しただけで、CSS のファイル名が `site-86109e8897909764.css` に変わりました。`topcoat dev` が、ファイルの変更を見つけて、ビルドと束の作り直しと、アプリの起動のし直しを、自動でしてくれたのです。ブラウザで開いていれば、`dev.js` が、ページを読み直します。
- 色を元に戻すと、ファイル名も `site-7becc8b6682e6054.css` に戻りました。**同じ中身なら、同じハッシュ**です。
- `topcoat::dev::script()` は、`topcoat dev` 以外で動かしたとき（`cargo run` や本番）には、何も出しません。RustNote のレイアウトにも、同じ1行があります。

> **注意:** 本番用の束は、本番用のビルドで作ります。`topcoat asset bundle --release --bin u08_assets` は `target\release\assets\` に作り、`target\debug\assets\` の束とは別です（第12巻 UNIT 03（準備中））。ほかのビルドで作った束を使い回すと、宣言の番号が合わないことがあるので、**実行ファイルと束は、いつも同じビルドで作ります**。

```text
### topcoat asset bundle --release --bin u08_assets
bundled assets into ...\web_lab\target\release\assets
### Get-ChildItem target\release\assets -Name
manifest.toml
site-7becc8b6682e6054.css
### Get-ChildItem target\debug\assets -Name
manifest.toml
site-7becc8b6682e6054.css
```

### よくあるエラーと読み方

#### エラー1: 束を作らずに起動した

`topcoat asset clean`（束を消すコマンド）のあとに、束を作らずに起動した場合です。

```text
### topcoat asset clean
removed ...\web_lab\target\debug\assets
### cargo run --bin u08_assets

thread 'main' (201628) panicked at src\bin\u08_assets.rs:35:37:
アセットの束を読み込めません: Custom { kind: NotFound, error: "no asset bundle at ...\\web_lab\\target\\debug\\assets" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `AssetBundle::load()` が、「`target\debug\assets` に束がありません」（`NotFound`）を返し、`main` の `.expect(…)` で止まりました。
- **直し方:** `topcoat asset bundle --bin u08_assets` で束を作ってから起動するか、`topcoat dev --bin u08_assets` で起動します。RustNote の `main` は、`.context("アセットの束（assets/）がありません。先に topcoat asset bundle を実行するか、topcoat dev で起動してください")` で、**何をすればよいか**を表示してから止まります。

#### エラー2: asset! のパスを打ち間違えた

`asset!("assets/sitee.css")` と、`e` を1つ多く書いた場合です（`errs/e08_typo.rs`）。

```text
### topcoat asset bundle --bin e08_typo
failed to bundle assets: io error for asset at ...\web_lab\assets\sitee.css: 指定されたファイルが見つかりません。 (os error 2)
```

- **何が起きたか:** コンパイルは通りますが、`topcoat asset bundle` が「`assets\sitee.css` のファイルが見つかりません」で失敗しました。`asset!` は、ファイルがあるかをコンパイルのときには確かめず、**束を作るとき**に確かめます。
- **直し方:** パスを `assets/site.css` に直します。`topcoat asset list --bin …` で、どのファイルが束に入る予定かを確かめられます（問1）。

#### エラー3: asset! を足したのに、束を作り直さなかった

`assets/print.css`（印刷用の CSS）を足し、`asset!("assets/print.css")` も書いて `cargo build` したのに、束を作り直さずに起動した場合です（`errs/e08_stale.rs`）。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (212564) panicked at ...\renderer.rs:90:44:
failed to resolve asset Asset { id: AssetId(582545301376341735) } in the asset catalog
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 起動はできましたが、ページを開くと 500 でした。「`Asset { id: … }` を、アセットの目録の中で見つけられません」という panic です。新しい宣言の番号が、古い目録に無いからです。
- **直し方:** 束を作り直します（`topcoat asset bundle --bin …`）。`asset!` を足した・変えたら、**必ず束を作り直す**と覚えておきます。`topcoat dev` なら、自動で作り直されます。

### RustNote ではここで使う

RustNote の CSS は、`src/views.rs` で宣言しています（抜き出しています）。

```rust
/// 画面全体のスタイルシート（assets/style.css）。
/// `topcoat asset bundle`（または `topcoat dev`）で、ハッシュ付きのファイル名で配信される。
pub const STYLESHEET: Asset = asset!("assets/style.css");
```

- `routes.rs` のレイアウトで `<link rel="stylesheet" href=(STYLESHEET)>` と使い、`topcoat::dev::script()` も置いています（UNIT 03）。

束は、`src/main.rs` で、DB を開く**前に**読み込んでいます（抜き出しています）。

```rust
    // `topcoat asset bundle`（または `topcoat dev`）が実行ファイルの隣に作る assets/ を読む。
    // 無ければ DB に触る前に止める
    let assets = AssetBundle::load().context(
        "アセットの束（assets/）がありません。先に `topcoat asset bundle` を実行するか、`topcoat dev` で起動してください",
    )?;
```

- 束が無いと分かったら、DB を開く前に止まります。準備の足りない状態で、途中まで動いてしまわないようにする順番です（第9巻 UNIT 06（準備中））。
- テストでは、束を作らずに済むように、CSS の URL だけを決めた目録（`AssetConfig::hosted_at("/test-assets", manifest)`）を渡しています（`tests/common/mod.rs` の `test_assets`）。本番の Dockerfile では、`cargo build --release` のあとに `topcoat asset bundle --release` を実行し、`assets/` を実行ファイルの隣にコピーしています（第12巻 UNIT 02・03（準備中））。

### 練習問題

**問1:** `topcoat asset list --bin u08_assets` を実行すると、何が表示されるでしょうか。束を作る前に、何を確かめるのに使えるでしょうか。

**問2:** `assets/site.css` の `h1` の色を変えてから、`topcoat asset bundle --bin u08_assets` を実行し、起動し直して、ページの `<link>` と、前のファイル名の URL（`site-7becc8b6682e6054.css`）を開いてください。どうなるでしょうか。

**問3:** `web_lab` には、実行ファイルがいくつもあります。`topcoat asset bundle --bin u02_serve`（アセットを使っていない実行ファイル）を実行してから、`u08_assets` を起動して `/` を開くと、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
...\web_lab\assets\site.css
```

- `asset!` で宣言された、束に入る予定のファイルが表示されました。`topcoat asset list` も、プログラムを一度ビルドして宣言を集めます。パスを打ち間違えていないか、使わなくなった宣言が残っていないかを、束を作る前に確かめられます。
- 使われていない宣言（どこからも使っていない `const`）は、コンパイルのときに取り除かれることがあり、そのときは束にも入りません。

**問2の解答:**

```text
### （h1 の色を変えてから）topcoat asset bundle --bin u08_assets
bundled assets into ...\web_lab\target\debug\assets
### Get-ChildItem target\debug\assets -Name
manifest.toml
site-7b62d7c4c2b2ded6.css
```

```text
### curl.exe -s http://127.0.0.1:3000/
<link rel="stylesheet" href="/_topcoat/assets/site-7b62d7c4c2b2ded6.css">
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/_topcoat/assets/site-7becc8b6682e6054.css
404
```

- ファイル名が `site-7b62d7c4c2b2ded6.css` に変わり、古い `site-7becc8b6682e6054.css` は束から消えて、開くと 404 でした。
- ブラウザは、古い CSS を 365 日覚えていますが、新しいページの `<link>` は新しい URL を指すので、新しい CSS を取りに来ます。**中身が変わったら名前が変わる**ので、「直したのに、ブラウザが古い CSS を使い続ける」ということが起きません。
- 本番では、古い URL が 404 になるので、新しい版を出したときに、古い HTML を開いたままの人が、CSS を受け取れないことがあります。束を外の置き場（CDN）に置く場合は、古いファイルをしばらく残しておく、といった工夫をします（`AssetConfig::hosted_at`）。

**問3の解答:**

```text

thread 'tokio-rt-worker' (215508) panicked at ...\renderer.rs:90:44:
failed to resolve asset Asset { id: AssetId(6866230526714045621) } in the asset catalog
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `/` は 500 で、エラー3と同じ「目録の中で見つけられません」でした。`u02_serve` で束を作ると、`target\debug\assets\` の中身が `u02_serve` の束（アセットは無し: `assets = []`）に置き換わります。束の置き場は、**同じビルドの実行ファイルの間で1つ**だからです。
- 実行ファイルを切り替えるときは、その実行ファイルで束を作り直します。RustNote のように、アセットを使う実行ファイルが1つなら、`--bin` を付けなくても困りません。

### まとめ

- CSS や画像は `asset!("assets/…")` で宣言し、`view!` に入れると、束の中の URL（`/_topcoat/assets/名前-ハッシュ.css`）になる。束は `topcoat asset bundle --bin …` で作り、`AssetBundle::load()` で読み込んで `.assets(…)` で配る。
- ファイル名のハッシュは中身から計算されるので、中身が変わると URL も変わる。だから `cache-control: …, immutable` で1年間覚えてもらえる。
- 束が無い・パスが違う・束が古いときは、それぞれ起動時・束を作るとき・ページを開いたときに止まる。開発中は `topcoat dev` で、保存するたびに作り直す。実行ファイルと束は、いつも同じビルドで作る。
