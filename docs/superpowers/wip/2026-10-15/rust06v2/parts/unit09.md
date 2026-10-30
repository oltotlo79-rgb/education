## UNIT 09: runtime・signal・procedure・shard

### この単元のゴール

- Topcoat の runtime（ブラウザの中で動く部分）を有効にし、`signal`（ブラウザが持つ状態）・`@click`（イベント）・`:hidden`（属性の結び付け）で、ページを読み直さずに画面を変えられる
- `#[procedure]` で、ブラウザのボタンからサーバーの関数を呼べる。その通信が、実際には `POST /api/…` であることを、ブラウザと curl で確かめられる
- **ブラウザから届く値は利用者の入力**であり、procedure と shard には**ページとは別の入口**があるので、中で値を確かめ、権限を確かめなければならない理由を、実際の穴で説明できる
- 構造体を `signal` に入れたとき（E0277）・`.discover()` を忘れたとき（404）・あふれる値が来たとき（500）を読める

> **注意:** Topcoat の公式の説明でも、runtime は **「とても実験的」**（highly experimental）とされています。使える Rust の型や書き方が限られ、版が上がると変わることがあります。この単元では、Topcoat 0.11.0 で実際に動かした範囲を扱います。RustNote は runtime を使っていません（後で理由を説明します）。

### たとえ話でつかむ

レストランのテーブルを考えます。

- テーブルには、**お客さんの手元のメモ**（「水のおかわり 2 回目」）があります。お客さんが自分で書き換えます。厨房に聞かなくても変えられます。これが `signal` と、ブラウザの中で動く `@click` です。
- 料理の注文は、手元では作れないので、**厨房に伝票を出します**。厨房が作って、返してくれます。これが `#[procedure]` です。
- 厨房の入口は、**お客さんの席とは別のドア**です。「この席は会員の席だから」と席の前で確かめても、厨房のドアに直接伝票を持ってくる人は、その確かめを通りません。だから厨房でも、**伝票を受け取るたびに確かめます**。
- 伝票に「パンを 1800 京個」と書かれても、そのまま作ろうとしてはいけません。書いてあることは、お客さんが書いたことです。

| レストラン | Topcoat の runtime |
|---|---|
| 手元のメモ | `signal(cx, || 1usize)`（ブラウザが持つ状態。最初の値はサーバーが決める） |
| 手元で書き換える | `@click=$(|_e| count.increment())`（ブラウザの中だけで動く） |
| 厨房に伝票を出す | `#[procedure("/api/double")]` を `.await` で呼ぶ（`POST /api/double`） |
| 厨房の別のドア | procedure の URL。ページの確かめ（`admin_page` の中の確かめ）を通らない |
| 伝票を受け取るたびに確かめる | procedure の中で、値と権限を確かめる |

> **用語:** runtime（ランタイム：ここでは、Topcoat がページに入れる、ブラウザの中で動くプログラム（JavaScript）。`$( … )` に書いた式を、ブラウザでも動かす）

<!-- -->

> **用語:** signal（シグナル：ブラウザが持つ状態。値が変わると、その値を使っている式が、ブラウザの中で計算し直される）

<!-- -->

> **用語:** procedure（プロシージャ：ブラウザから呼べる、サーバーの async 関数。1つずつに URL があり、`POST` で呼ばれる）

### 図で理解する

ページを開いてから、ボタンを押すまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0901" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="250" height="260" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">サーバー（Rust）</text>
<text x="40" y="74" font-size="12" fill="#0f172a">① 最初の HTML を作る</text>
<text x="56" y="94" font-size="11" fill="#475569">数: 1（signal の最初の値）</text>
<text x="56" y="112" font-size="11" fill="#475569">ボタンの処理を JavaScript にして</text>
<text x="56" y="130" font-size="11" fill="#475569">HTML の属性に入れる</text>
<rect x="40" y="200" width="210" height="60" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="224" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">#[procedure] double</text>
<text x="145" y="244" font-size="11" fill="#475569" text-anchor="middle">POST /api/double に答える</text>
<rect x="450" y="20" width="250" height="260" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="575" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ブラウザ（runtime の JS）</text>
<text x="470" y="74" font-size="12" fill="#0f172a">② +1・説明を押す</text>
<text x="486" y="94" font-size="11" fill="#475569">signal を書き換え、</text>
<text x="486" y="112" font-size="11" fill="#475569">数や hidden をその場で直す</text>
<text x="486" y="130" font-size="11" fill="#047857">（通信なし）</text>
<text x="470" y="200" font-size="12" fill="#0f172a">③ 2倍を押す</text>
<text x="486" y="220" font-size="11" fill="#475569">サーバーに数を送り、</text>
<text x="486" y="238" font-size="11" fill="#475569">返ってきた値を signal に入れる</text>
<line x1="270" y1="100" x2="446" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="358" y="92" font-size="11" fill="#475569" text-anchor="middle">HTML と runtime の JS</text>
<line x1="446" y1="215" x2="254" y2="215" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="350" y="207" font-size="11" fill="#475569" text-anchor="middle">POST /api/double（[数]）</text>
<line x1="254" y1="245" x2="446" y2="245" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="350" y="265" font-size="11" fill="#475569" text-anchor="middle">200（2倍の値）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: runtime の流れ。サーバーは最初の HTML と、ボタンの処理（JavaScript）を送る。signal の書き換えはブラウザの中だけで済み、procedure を呼んだときだけ POST の通信が起きる</div>
</div>

procedure には、ページとは別の入口があります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0902" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="170" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="105" y="56" font-size="12" fill="#0f172a" text-anchor="middle">ブラウザで</text>
<text x="105" y="76" font-size="12" fill="#0f172a" text-anchor="middle">/admin を開く</text>
<rect x="20" y="140" width="170" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="105" y="166" font-size="12" fill="#0f172a" text-anchor="middle">curl などで</text>
<text x="105" y="186" font-size="12" fill="#0f172a" text-anchor="middle">直接 POST する</text>
<rect x="260" y="30" width="200" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="56" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ページの確かめ</text>
<text x="360" y="76" font-size="11" fill="#0f172a" text-anchor="middle">is_admin(cx) → だめなら 403</text>
<rect x="520" y="85" width="180" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="610" y="112" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">POST /api/clear</text>
<text x="610" y="132" font-size="11" fill="#0f172a" text-anchor="middle">（procedure の入口）</text>
<line x1="190" y1="60" x2="256" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<line x1="460" y1="70" x2="516" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<text x="488" y="76" font-size="10" fill="#475569">ボタン</text>
<line x1="190" y1="170" x2="516" y2="135" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<text x="350" y="178" font-size="11" fill="#b91c1c" text-anchor="middle">ページの確かめを通らずに、直接届く</text>
<text x="610" y="190" font-size="11" fill="#047857" text-anchor="middle">→ procedure の中でも確かめる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: procedure の入口。ボタンはページの確かめのあとで押されるが、procedure の URL には、ページを通らずに直接 POST できる。だから、確かめは procedure の中にも書く</div>
</div>

### まず動かす最小コード

#### 例9-1: 数を足すボタンと、サーバーで計算するボタン

> **▼ このコードがやること（先に日本語で）:** `count`（最初は 1）と `open`（最初は false）の2つの signal を作ります。「+1」はブラウザの中だけで `count` に 1 を足し、「2倍（サーバーで計算）」はサーバーの `double` を呼んで、返ってきた値を `count` に入れます。「説明」は `open` を切り替え、`:hidden` で説明の文を出したり消したりします。`main` では、`.discover()` で procedure を集め、`.assets(…)` と `.runtime()` を登録します。

```rust
//! 第6巻 UNIT 09: ブラウザの中で動く部分（signal・イベント・procedure）。
//! 実行: topcoat asset bundle --bin u09_runtime のあとで cargo run --bin u09_runtime

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page},
    runtime::{RouterBuilderRuntimeExt, procedure, signal},
    view::{View, view},
};

/// サーバーで動く関数。ブラウザから POST /api/double で呼ばれる
#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    Ok(value * 2)
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // ブラウザが持つ状態。最初の値はサーバーが決めて送る
    let count = signal(cx, || 1usize);
    let open = signal(cx, || false);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 09"</title>
                // ブラウザで動く部分（runtime）のスクリプト
                topcoat::runtime::script()
            </head>
            <body>
                <p>"数: " $(count.get())</p>
                // ブラウザの中だけで 1 足す（サーバーには送らない）
                <button @click=$(|_e| count.increment())>"+1"</button>
                // サーバーの double を呼んで、結果を入れる
                <button @click=$(async |_e| {
                    let doubled = double(count.get()).await;
                    count.set(doubled);
                })>
                    "2倍（サーバーで計算）"
                </button>
                <button @click=$(|_e| open.set(!open.get()))>"説明"</button>
                <p :hidden=$(!open.get())>"この文は、ブラウザの中だけで出したり消したりします"</p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // procedure（double）を集めて登録する
        .discover()
        // runtime のスクリプトは、アセットの束から配る
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

runtime のスクリプトは、アセットの束から配るので、起動の前に束を作ります（UNIT 08）。

```powershell
topcoat asset bundle --bin u09_runtime
cargo run --bin u09_runtime
```

ブラウザで `http://127.0.0.1:3000/` を開き、ボタンを押しました。

**▼ ブラウザで確かめた結果**

```text
（開いた直後）                数: 1
「+1」を2回押す               数: 3
「2倍（サーバーで計算）」     数: 6
「説明」を押す                「この文は、ブラウザの中だけで出したり消したりします」が出る
--- ここから、ブラウザの通信の記録を取りながら ---
「2倍」をもう一度             数: 12   通信: POST http://127.0.0.1:3000/api/double → 200
「+1」と「説明」              数: 13・説明の文が消える   通信: 0 件
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: $( … ) は、ブラウザでも動く式

- `$(count.get())` は、**runtime の式**です。サーバーは最初の HTML を作るときにこの式を計算し（数: 1）、さらに同じ式を JavaScript にしてページに入れます。ブラウザは、`count` が変わるたびに、その JavaScript で計算し直します。
- curl でページを見ると、サーバーがどう送っているかが分かります（1行で返ってくるので、読みやすいようにタグごとに改行し、最初の部分と「+1」のボタンだけを載せています）。

```powershell
curl.exe -s http://127.0.0.1:3000/
```

```text
### curl.exe -s http://127.0.0.1:3000/
<!--::topcoat::signal({&quot;t&quot;:&quot;signal&quot;,&quot;id&quot;:&quot;b4419cc41caf26e5432cdcb030a5c804&quot;,&quot;v&quot;:{&quot;t&quot;:&quot;usize&quot;,&quot;bits&quot;:64,&quot;v&quot;:&quot;1&quot;}})-->
<!--::topcoat::signal({&quot;t&quot;:&quot;signal&quot;,&quot;id&quot;:&quot;ec97a701a0af29d73734d8de751e9b94&quot;,&quot;v&quot;:false})-->
<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<title>UNIT 09</title>
<script type="module" src="/_topcoat/assets/topcoat-f8e40bad0d18978e.js" data-topcoat-usize-bits="64">
</script>
…
<button data-topcoat-on:click="(__local0) => (cx.hydrate({&quot;t&quot;:&quot;Signal&quot;,&quot;id&quot;:&quot;b4419cc41caf26e5432cdcb030a5c804&quot;})).increment()">+1</button>
…（以下、ほかのボタンと説明の文が続きます）
```

- 先頭の `<!--::topcoat::signal(…)-->` が、2つの signal の最初の値（`"v":"1"` と `false`）です。`<script type="module" src="/_topcoat/assets/topcoat-….js">` が runtime の本体で、ボタンの `data-topcoat-on:click="…"` に、`@click` の処理を JavaScript にしたものが入っています。**ブラウザに送ったものは、利用者に全部見えます**。

##### 解説2: @click と :hidden は、ブラウザの中だけで動く

- 「+1」と「説明」を押したときは、ブラウザの通信は **0 件**でした。`count.increment()` も `open.set(!open.get())` も、ブラウザの中だけで signal を書き換えています。
- `:hidden=$(!open.get())` は**結び付けの属性**です。最初は `hidden=""`（隠れている）で送られ、`open` が変わるたびに、ブラウザが `hidden` を付けたり外したりします。

##### 解説3: procedure は、POST の通信になる

- 「2倍」を押したときは、`POST http://127.0.0.1:3000/api/double` が1件あり、数は 6 → 12 になりました。`double(count.get()).await` は、Rust の関数を呼んでいるように見えますが、ブラウザから**サーバーへの HTTP の要求**です。
- だから、curl からも同じ要求を送れます。数は、`{"t":"usize","bits":64,"v":"21"}` という形の JSON で送ります（Rust の `usize` を、JavaScript でも正しく扱うための形です）。

```powershell
Set-Content args21.json '[{"t":"usize","bits":64,"v":"21"}]' -Encoding ascii
curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
```

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 200 OK
content-type: application/json
content-length: 32

{"t":"usize","bits":64,"v":"42"}
```

### もう一歩: 実務に近い形

#### 例9-2: ページの確かめは、procedure を守らない

管理者だけが開けるページに、「メモを全部消す」ボタンを置きます。ボタンが呼ぶ procedure を、**確かめない版**（`/api/clear`）と、**確かめる版**（`/api/clear-safe`）の2つ作って比べます。

> **▼ このコードがやること（先に日本語で）:** `is_admin(cx)` は、Cookie に `admin=yes` があれば管理者とみなします（説明を短くするための仮の仕組みです。本物はセッションで確かめます。UNIT 10）。`/admin` のページは、管理者でなければ 403 にします。`clear_unsafe` は確かめずにメモを全部消し、`clear_safe` は中で `is_admin` を確かめてから消します。どちらも、消した件数を返します。

```rust
//! 第6巻 UNIT 09 の例9-2: ページの確かめは、procedure を守らない。procedure の中でも確かめる。
//! 実行: topcoat asset bundle --bin u09_guard のあとで cargo run --bin u09_guard

use std::sync::Mutex;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::{Cx, app_context},
    router::{RouterBuilderDiscoverExt, error::forbidden, module_router, page, request::headers},
    runtime::{RouterBuilderRuntimeExt, procedure},
    view::{View, view},
};

/// 保存したメモ
struct Memos(Mutex<Vec<String>>);

/// 管理者か（この例では、Cookie: admin=yes が付いていれば管理者とみなす。本物はセッションで確かめる）
fn is_admin(cx: &Cx) -> bool {
    headers(cx)
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|cookie| cookie.split("; ").any(|c| c == "admin=yes"))
}

/// してはいけない例: 中で確かめずに、メモを全部消す
#[procedure("/api/clear")]
async fn clear_unsafe(cx: &Cx) -> Result<usize> {
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

/// 正しい例: procedure の中でも、管理者かを確かめる
#[procedure("/api/clear-safe")]
async fn clear_safe(cx: &Cx) -> Result<usize> {
    if !is_admin(cx) {
        return Err(forbidden().into());
    }
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

mod admin {
    use super::*;

    /// GET /admin: 管理者だけが開けるページ。ボタンで procedure を呼ぶ
    #[page]
    async fn admin_page(cx: &Cx) -> Result<impl View> {
        if !is_admin(cx) {
            return Err(forbidden().into());
        }
        let count = app_context::<Memos>(cx).0.lock().unwrap().len();
        Ok(view! {
            <head>topcoat::runtime::script()</head>
            <p>"メモは " (count) " 件"</p>
            <button @click=$(async |_e| {
                clear_safe().await;
            })>
                "全部消す"
            </button>
        })
    }
}

#[tokio::main]
async fn main() {
    let memos = vec!["牛乳".to_string(), "本".to_string(), "宿題".to_string()];
    let router = module_router!()
        .discover()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .app_context(Memos(Mutex::new(memos)))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
topcoat asset bundle --bin u09_guard
cargo run --bin u09_guard
```

まず、確かめない版を試します（`noargs.json` は `[]`、引数なしの意味です）。

```text
### curl.exe -i http://127.0.0.1:3000/admin
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 3 件</p>

### curl.exe -i -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear
HTTP/1.1 200 OK
content-type: application/json
content-length: 31

{"t":"usize","bits":64,"v":"3"}


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 0 件</p>
```

- Cookie の無い要求で `/admin` を開くと 403 で、ページの確かめは働いています。
- ところが、同じ Cookie の無い要求で `/api/clear` に直接 POST すると、**200 で、メモが3件消えました**。管理者として `/admin` を開き直すと、0 件です。`/api/clear` は、`/admin` のページを通らない、**別の入口**だからです。

次に、サーバーを起動し直して（メモが3件に戻ります）、確かめる版を試します。

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear-safe
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -i -H "Cookie: admin=yes" -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear-safe
HTTP/1.1 200 OK
content-type: application/json
content-length: 31

{"t":"usize","bits":64,"v":"3"}
```

- Cookie が無ければ 403、あれば 200 で3件消えました。procedure の**中で**確かめたので、どこから呼ばれても守られます。
- Topcoat の説明にも、「procedure の引数を確かめ、中で権限を確かめなさい。呼ぶ人は、自分で要求を作って送れるので」と書かれています。shard（サーバーで描き直す部品）も、同じく自分の入口を持ち、ページやレイアウトの確かめは働きません。

> **ヒント:** 「ページにボタンを出していないから、押されない」は、守りになりません。URL は、ページの HTML（解説1の `data-topcoat-on:click`）を見れば分かります。**入口ごとに確かめる**のが決まりです（第8巻 UNIT 07（準備中））。

### よくあるエラーと読み方

#### エラー1: 自分の構造体を、そのまま signal に入れた（E0277）

```rust
//! 自分で作った構造体を、そのまま signal に入れた
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{module_router, page},
    runtime::{RouterBuilderRuntimeExt, signal},
    view::{View, view},
};

#[derive(Clone)]
struct Cart {
    items: usize,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let cart = signal(cx, || Cart { items: 0 });
    Ok(view! {
        <head>topcoat::runtime::script()</head>
        <p>"かご: " $(cart.get().items)</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0277]: the trait bound `Cart: SignalValue` is not satisfied
   --> src\bin\e09_struct.rs:18:16
    |
 18 |     let cart = signal(cx, || Cart { items: 0 });
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `Surrogated` is not implemented for `Cart`
   --> src\bin\e09_struct.rs:12:1
    |
 12 | struct Cart {
    | ^^^^^^^^^^^
    = help: the following other types implement trait `Surrogated`:
              &'__lifetime Option<T>
              &'__lifetime Result<T, E>
              &'__lifetime Signal<T>
              &'__lifetime Vec<T>
              &'__lifetime [T; N]
              &'__lifetime [T]
              &'__lifetime bool
```

（このあとに、`signal` の決まり `T: SignalValue` を指す `note:` と、同じ形のエラーが続きます）

- **何が起きたか:** 「`Cart` に `SignalValue` が実装されていません」というエラーです。signal の値は、ブラウザ（JavaScript）とサーバー（Rust）の間を行き来するので、runtime が両方で扱える型（数・文字・`bool`・`Vec`・`Option` など。`help:` に一部が並んでいます）でなければなりません。
- **直し方:** 構造体に `#[record]` を付けます（`use topcoat::runtime::record;`）。`#[record]` と `#[derive(Clone)]` を付けた `Cart` で、ビルドが通ることを確かめました。項目の型も、runtime が扱える型にします。

#### エラー2: .discover() を書き忘れた

`main` の `.discover()` を消してしまった場合です（`errs/e09_nodiscover.rs`）。コンパイルも起動も通り、ページも開けます。

```text
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/
200
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

- **何が起きたか:** `/api/double` が 404 でした。`#[procedure]` は、ルーターに登録しないと、入口ができません。`module_router!()` は `#[page]` などは集めますが、procedure は集めないので、`.discover()`（または `.route(double)`）が要ります。ブラウザで「2倍」を押しても、通信が 404 で失敗し、数は変わりません。
- **直し方:** `.discover()` を足します。「押しても何も起きない」ときは、ブラウザの開発者ツールの「ネットワーク」で、procedure の要求が何番で返っているかを見ます。

#### エラー3: あふれる値が届いて、500 になった

例9-1の `double` に、`usize` の最大の値（18446744073709551615）を送った場合です（`argsbig.json`）。

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@argsbig.json" http://127.0.0.1:3000/api/double
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (214492) panicked at src\bin\u09_runtime.rs:16:8:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `value * 2` が `usize` に入りきらず、「掛け算であふれました」（`attempt to multiply with overflow`）で panic し、500 になりました。デバッグのビルドでは、あふれると panic する決まりです。リリースのビルドでは、あふれた分が**黙って切り捨てられ**、おかしな値が返ります（第2巻 UNIT 02）。
- **学べること:** ブラウザのボタンは 1・2・4・… と数を倍にするだけですが、procedure の入口には、**どんな値でも**送れます。ブラウザから届く値は、利用者が書いた入力と同じに扱い、サーバーで確かめます（問1）。

### RustNote ではここで使う

**RustNote は、runtime を使っていません。** 画面の操作は、すべて HTML のフォームと、303 で移る PRG（UNIT 06）で作っています。理由は次の3つです。

- runtime は、まだ実験的で、版が上がると書き方が変わることがあります。教材の完成形は、長く同じ形で読めるようにしたいからです。
- フォームと PRG なら、JavaScript が動かないブラウザでも、すべての操作ができます。入口は `#[page(POST)]` だけなので、確かめる場所（`require_user`）も、ページの関数の中の1か所で済みます。
- 守りの考え方は同じです。RustNote の POST（作成・編集・完了・削除・ログアウト）は、どれもページの関数の最初で `require_user(cx)` を呼び、ほかのサイトからの POST は、Topcoat の OriginPolicy が 403 で止めます（第8巻 UNIT 01。問3）。

ノートの検索を、入力のたびに結果が変わる形にしたいとき（いまはボタンで検索）などに、shard を使う余地があります。そのときも、shard の中で `require_user(cx)` を呼び、検索の言葉を `NoteQuery::new` で確かめてから使います。

### 練習問題

**問1:** 例9-1の `double` を、2倍が `usize` に入りきらないときは 400（`bad_request`）を返すように直し、エラー3と同じ値を送って確かめてください。

**問2:** 例9-1のページの HTML を curl で見ると、`count` の最初の値や、ボタンの処理が見えました。runtime を使うページに、**ブラウザに送ってはいけないもの**は何でしょうか。

**問3:** 例9-2の確かめない版の `/api/clear` に、**ほかのサイトのページから** POST させると、どうなるでしょうか（curl で `Origin: https://evil.example` と `Sec-Fetch-Site: cross-site` を付けて試します）。それでも、確かめない版を使ってはいけないのはなぜでしょうか。

### 解答と解説

**問1の解答:**

`checked_mul` は、あふれるときに `None` を返します。`None` なら `bad_request(…)` を返します（`src/bin/u09_q1.rs`。変えた所だけを抜き出しています）。

```rust
#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    // ブラウザから来た値は、利用者の入力。入りきらないなら断る
    value
        .checked_mul(2)
        .ok_or_else(|| bad_request("2倍にすると大きすぎます").into())
}
```

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 200 OK
content-type: application/json
content-length: 32

{"t":"usize","bits":64,"v":"42"}


### curl.exe -i -H "Content-Type: application/json" --data-binary "@argsbig.json" http://127.0.0.1:3000/api/double
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 47

bad request: 2倍にすると大きすぎます
```

- 21 は 42 になり、最大の値は 400 と「2倍にすると大きすぎます」になりました。サーバーは panic せず、窓にも何も出ません。
- `bad_request` の文は、送った人に見えます。中のこと（SQL やファイルの場所など）を書かないようにします。

**問2の解答:**

- `signal` の値・`$( … )` の式・ボタンの処理は、どれも HTML に入って、ブラウザに送られます。だから、**秘密の値**（パスワードのハッシュ・API のトークン・ほかの人のメールアドレス・管理者だけが見てよい数）を、signal に入れたり、`$( … )` の式の中で使ったりしてはいけません。
- 「ブラウザで隠している（`hidden`）」も、**見えていない**だけで、HTML には入っています。見せてはいけないものは、サーバーで、そもそも HTML に入れない（`if` で出し分ける）ようにします。

**問3の解答:**

```text
### curl.exe -i -H "Origin: https://evil.example" -H "Sec-Fetch-Site: cross-site" -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 3 件</p>
```

- 403 で止まり、メモは3件のままでした。Topcoat の OriginPolicy は、**ほかのサイトのページから、ブラウザが送る**状態を変える要求（POST など）を、すべての入口で止めます（procedure も含みます。例9-1の `/api/double` でも、同じく 403 でした）。
- それでも、確かめない版を使ってはいけません。OriginPolicy が止めるのは「ほかのサイトのページから、利用者のブラウザに送らせる」攻撃（CSRF。UNIT 10）だけで、**ブラウザを使わずに、自分で要求を作る人**（例9-2の curl）は止めません。その人が管理者でなければ断る、という確かめは、procedure の中にしか書けません。

### まとめ

- runtime は、`signal` と `$( … )` の式で、ブラウザの中だけで画面を変える仕組み。`@click` はイベント、`:hidden` などは結び付けの属性。使うには `topcoat::runtime::script()`・`.runtime()`・アセットの束が要る。Topcoat 0.11 でも「とても実験的」な機能。
- `#[procedure]` は、ブラウザから `POST` で呼ばれるサーバーの関数。`.discover()` などで登録しないと 404。signal の値の型は、runtime が両方で扱える型か `#[record]` の構造体。
- ブラウザに送ったものは全部見え、ブラウザから届く値は利用者の入力。procedure と shard には別の入口があるので、**中で値と権限を確かめる**。OriginPolicy は、ほかのサイトからの POST は止めるが、直接の要求は止めない。
