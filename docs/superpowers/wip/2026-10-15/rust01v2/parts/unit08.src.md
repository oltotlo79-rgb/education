## UNIT 08: Topcoat 0.11 と CLI

### この単元のゴール

- Topcoat の CLI（`topcoat`）を、版を決めて `cargo install … --locked` で入れ、`topcoat --version` で確かめられる
- `topcoat new` で新しいアプリを作り、できたファイルの役目を言える
- `topcoat dev`（開発用のサーバー）で動かし、ファイルを保存すると作り直されることを確かめられる。`cargo run` だけでは動かない理由（アセットの束）を説明できる
- 古い Rust（1.99 より前）や、使われているポートのエラーを読んで直せる

### たとえ話でつかむ

**組み立て家具のお店**を考えます。

- **Topcoat** は、Web アプリを組み立てるための**部品の箱**（ライブラリ）です。アプリの `Cargo.toml` に「Topcoat 0.11.0 を使う」と書くと、cargo が取り寄せます。
- **Topcoat の CLI**（`topcoat` コマンド）は、**お店の店員さん**です。「新しい家具を作りたい」と言うと、最初の形を用意してくれます（`topcoat new`）。組み立て中に、ねじを1本替えると、すぐに組み直して見せてくれます（`topcoat dev`）。
- 家具には、**説明書と飾り**（CSS などのアセット）も要ります。アプリの本体（実行ファイル）とは別に、飾りを**束**にしてそろえておかないと、アプリは「飾りがありません」と言って止まります。店員さん（`topcoat dev`・`topcoat asset bundle`）は、この束も作ってくれます。
- 部品の箱（Topcoat 0.11）は、**新しい工具**（Rust 1.99 以上）でないと開けられません。

| 家具のお店 | この単元の言葉 |
|---|---|
| 部品の箱 | Topcoat（ライブラリ。`Cargo.toml` の `topcoat = …`） |
| 店員さん | Topcoat の CLI（`topcoat`。`cargo install topcoat-cli` で入れる） |
| 最初の形を用意する | `topcoat new` |
| すぐ組み直して見せる | `topcoat dev` |
| 説明書と飾りの束 | アセットの束（`target\debug\assets`） |
| 新しい工具 | Rust 1.99 以上 |

> **用語:** Topcoat（トップコート：Rust で Web アプリを作るためのライブラリ。このシリーズの RustNote は Topcoat 0.11 で作る。くわしくは第6巻）

<!-- -->

> **用語:** アセット（asset：CSS・画像・フォントなど、ページと一緒に配るファイル。Topcoat は、ビルドのときに束（bundle）にまとめる）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-081" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="110" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="120" y="46" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">hello-topcoat</text>
<text x="120" y="70" font-size="11" fill="#0f172a" text-anchor="middle">Cargo.toml（topcoat 0.11.0）</text>
<text x="120" y="90" font-size="11" fill="#0f172a" text-anchor="middle">src の .rs・styles.css</text>
<text x="120" y="112" font-size="11" fill="#475569" text-anchor="middle">topcoat new で作る</text>
<rect x="270" y="20" width="200" height="110" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="370" y="46" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">topcoat dev</text>
<text x="370" y="70" font-size="11" fill="#0f172a" text-anchor="middle">① cargo でビルド</text>
<text x="370" y="90" font-size="11" fill="#0f172a" text-anchor="middle">② アセットの束を作る</text>
<text x="370" y="110" font-size="11" fill="#0f172a" text-anchor="middle">③ アプリを動かす</text>
<rect x="520" y="20" width="180" height="110" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="610" y="50" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<text x="610" y="76" font-size="11" fill="#15803d" text-anchor="middle">http://127.0.0.1:3000</text>
<text x="610" y="100" font-size="11" fill="#15803d" text-anchor="middle">保存すると、</text>
<text x="610" y="116" font-size="11" fill="#15803d" text-anchor="middle">作り直して見せ直す</text>
<line x1="222" y1="75" x2="266" y2="75" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-081)"/>
<line x1="472" y1="75" x2="516" y2="75" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-081)"/>
<rect x="20" y="160" width="680" height="96" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="360" y="186" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo run だけだと</text>
<text x="360" y="210" font-size="12" fill="#0f172a" text-anchor="middle">①と③はするが、②（アセットの束）を作らない → 「no asset bundle」で止まる（例8-3）</text>
<text x="360" y="234" font-size="12" fill="#b45309" text-anchor="middle">先に topcoat asset bundle で束を作れば、cargo run でも動く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: topcoat dev がすること。ビルド・アセットの束・起動をまとめてして、保存のたびにやり直す</div>
</div>

### まず動かす最小コード

#### 例8-1: Topcoat の CLI を入れる

> **▼ この手順がやること（先に日本語で）:** `cargo install` で、Topcoat の CLI（パッケージの名前は `topcoat-cli`）の **0.11.0** を入れます。`--locked` は、CLI を作った人が確かめた部品の版（`Cargo.lock`）のとおりに作る、という指定です。

```powershell
cargo install topcoat-cli --version 0.11.0 --locked
topcoat --version
where.exe topcoat
topcoat --help
```

この本では、PC のいつもの Rust とは別の置き場（`rsb`）の Rust で試しました（UNIT 04 の `rust-try` と同じ考え方です）。

**▼ 実行結果**（`cargo install`。最初と最後の行だけ）

```text
@@OUTHEAD:u08_install:4@@
（ここに、部品をダウンロードする行と、243 個の部品の Compiling の行が続く）
@@OUTTAIL:u08_install:6@@
```

```text
@@OUT:u08_help@@
```

#### ▼ 1つずつ分解して解説

##### 解説1: `cargo install` は、その場でビルドする

- `cargo install` は、crates.io から `topcoat-cli` のソースを取ってきて、この PC で**ビルド**してから、`.cargo\bin` に置きます。243 個の部品をビルドして、107 秒かかりました。入ったのは `topcoat.exe` と `cargo-topcoat.exe` の2つです。
- `.cargo\bin` は UNIT 04 の受付と同じフォルダなので、新しい窓を開かなくても、すぐに `topcoat` と打てます。

##### 解説2: 版を決めて入れる

- `--version 0.11.0` を付けないと、そのときのいちばん新しい版が入ります。アプリの `Cargo.toml` の Topcoat の版と、CLI の版は**そろえます**（版が違うと、CLI が注意を出します。第12巻 UNIT 03）。
- `topcoat --help` の `Commands:` が、店員さんのできることの一覧です。この単元では `new`・`dev`・`asset` を使います。

#### 例8-2: 新しいアプリを作る

> **▼ この手順がやること（先に日本語で）:** `rust-study` で、`topcoat new` を使って `hello-topcoat` というアプリを作ります。`--minimal` は「いちばん小さい組み合わせ」、`--no-interactive` は「質問をしない」という指定です（`--help` で確かめられます）。

```powershell
cd ~\rust-study
topcoat new hello-topcoat --minimal --no-interactive
```

**▼ 実行結果**

```text
@@OUT:u08_new@@
```

できたファイルです（`.git` の中は除いています）。

```text
@@OUT:u08_tree@@
```

`Cargo.toml`:

```toml
@@FILE:u08_app_snapshot/Cargo.toml@@
```

`src\app.rs`:

```rust
@@FILE:u08_app_snapshot/src/app.rs@@
```

- `Cargo.toml` には `topcoat = { version = "0.11.0", … }` と、非同期の部品の `tokio` が書かれています。
- `src\app.rs` の `#[page] async fn home()` が、トップページ（`/`）です。`view! { … }` の中に、HTML の形でページを書きます（第6巻）。`asset!("styles.css")` は、`styles.css` を束に入れて、その URL を書き込む、という意味です。
- `Next steps` のとおり、次は `cd hello-topcoat` と `topcoat dev` です。

#### 例8-3: まず `cargo run` で動かしてみる

> **▼ この手順がやること（先に日本語で）:** UNIT 04 と同じ `cargo run` で動かしてみます。

```powershell
cd hello-topcoat
cargo run
```

**▼ 実行結果**（部品のダウンロードとビルドの行は除いています。101 個をダウンロードし、108 個をビルドしました）

```text
@@OUTGREP:u08_run:Updating|Locking|Finished|Running|panicked|Custom|error:@@
```

- ビルドはできましたが、動かした直後に `panicked`（プログラムが止まった）になりました。`no asset bundle at …\target\debug\assets` は、「`target\debug\assets` に、アセットの束が無い」という意味です。`src\app.rs` の 13 行目の `AssetBundle::load().unwrap()` で止まっています。
- `cargo run` は Rust のビルドだけをして、Topcoat のアセットの束は作らないからです（図）。束は `topcoat asset bundle` か `topcoat dev` が作ります。

```powershell
# 束を作ってから、もう一度 cargo run
topcoat asset bundle
Get-ChildItem target\debug\assets -Name
cargo run
```

**▼ 実行結果**

```text
@@OUT:u08_bundle@@
```

```text
@@OUT:u08_bundle_ls@@
```

```text
@@OUTHEAD:u08_run2:2@@
```

別の窓から開きました。

```powershell
curl.exe -s -i http://127.0.0.1:3000/
```

**▼ 実行結果**（最初の3行）

```text
@@OUT:u08_run2_curl@@
```

- 束（`manifest.toml` と、指紋付きの名前の CSS）ができたあとは、`cargo run` でも動き、トップページが 200 で返りました。ブラウザで `http://127.0.0.1:3000` を開くと、`hello-topcoat` の見出しが出ます。止めるときは、`cargo run` の窓で `Ctrl + C` を押します。

### もう一歩: 実務に近い形

#### 例8-4: topcoat dev で、保存のたびに作り直す

> **▼ この手順がやること（先に日本語で）:** `topcoat dev` で動かします。ページの見出しを確かめてから、`src\app.rs` の見出しを `"こんにちは、Topcoat"` に書き換えて保存し、もう一度ページを開きます。

```powershell
topcoat dev
```

**▼ 実行結果**（`topcoat dev` の窓）

```text
@@OUT:u08_dev@@
```

別の窓から、書き換える前と後で、ページを開きました。

```powershell
curl.exe -s http://127.0.0.1:3000/
```

**▼ 実行結果**（書き換える前。ページの中の見出し `<h1>` の所だけを載せています）

```text
@@OUT:u08_dev_h1_before@@
```

**▼ 実行結果**（`src\app.rs` を書き換えて保存した後。見出しの所だけ）

```text
@@OUT:u08_dev_h1_after@@
```

- `topcoat dev` は、ファイルの変化を見張り（`watching for file changes...`）、保存されると、ビルド・束・起動をやり直します。`ready on http://127.0.0.1:3000` が2回出ているのは、最初の起動と、保存のあとの起動です。
- `src\app.rs` の `<head>` の中の `topcoat::dev::script()` は、`topcoat dev` で動いているときだけ、小さなスクリプトを入れます。Topcoat 0.11 のソースの説明によると、作り直しが成功すると、ブラウザのページを自動で新しくします。`topcoat dev` で動いていないときは、何も入れません。

### よくあるエラーと読み方

#### エラー1: 古い Rust で、Topcoat 0.11 を使う

この本の PC には、古い Rust 1.90.0 も入っています（UNIT 04 の `rustup toolchain list`）。RustNote のフォルダで、`cargo +1.90.0 check` と、版を指定して確かめました。

```powershell
cargo +1.90.0 check
```

```text
@@OUTHEAD:u08_old_rust:6@@
（Topcoat の部品の名前が、同じように続く）
```

- **何が起きたか:** 「rustc 1.90.0 では使えない部品がある」というエラーです。RustNote も、Topcoat 0.11 の部品も、`requires rustc 1.99`（Rust 1.99 以上が要る）と書かれていました。部品の `Cargo.toml` の `rust-version` を、cargo がビルドの前に確かめています。
- **直し方:** `rustup update` で Rust を新しくします（UNIT 04）。RustNote のフォルダでは、`rust-toolchain.toml` の 1.99.1 が使われるので、`+1.90.0` のように**わざと古い版を選ばない限り**、このエラーにはなりません。

#### エラー2: ポートが使われている（10048）

1つ目の `hello-topcoat` が動いている間に、2つ目を動かしました。

```powershell
.\target\debug\hello-topcoat.exe
```

```text
@@OUT:u08_port_busy@@
```

- **何が起きたか:** `src\main.rs` の 6 行目（`topcoat::start(...).await.unwrap()`）で止まりました。`AddrInUse`（アドレスが使用中）、`code: 10048` は、ポート 3000 が、もう使われているという意味です。
- **直し方:** 前に動かしたアプリを `Ctrl + C` で止めます。両方動かしたいときは、`$env:PORT = "3001"` のように、別のポートにします（Topcoat の `start` は、`HOST`・`PORT` を読みます）。なお、`topcoat dev` は、3000 が使われていると、自分で 3001 を使います。

### RustNote ではここで使う

- RustNote の `Cargo.toml` には `topcoat = "=0.11.0"` と、`=` 付きで版が固定されています。CLI も 0.11.0 をそろえます。
- RustNote を開発するときは、RustNote のフォルダで `topcoat dev` を使います。本番用の束は `topcoat asset bundle --release` で作ります（第12巻 UNIT 03）。
- RustNote の画面や部品の書き方は、第6巻（Topcoat 0.11 完全ガイド）でくわしく学びます。

### 練習問題

**問1:** `topcoat-cli` を `--version` を付けずに入れると、どうなる可能性があるでしょうか。

**問2:** 例8-3で `cargo run` が止まったとき、エラーの中のどの言葉から、原因にたどり着けるでしょうか。

**問3:** `topcoat dev` を動かしたまま、`styles.css` の色を変えて保存すると、どうなると考えられますか。

### 解答と解説

**問1の解答:**

- そのときのいちばん新しい版が入ります。アプリの Topcoat が 0.11.0 なのに、CLI が新しい版になると、束の作り方が合わないことがあります。第12巻 UNIT 03 では、版が違うと CLI が注意を出すことを確かめました。版は `--version 0.11.0` でそろえます。

**問2の解答:**

- `no asset bundle at …\target\debug\assets` です。「束（bundle）が無い」と、探した場所が書かれています。そのフォルダが無いことを `Get-ChildItem` で確かめ、`topcoat asset bundle` で作ります。

**問3の解答:**

- `topcoat dev` は、ファイルの変化を見張っているので、作り直して、束も作り直します。CSS のファイルの名前の指紋（`styles-…css` の `…` の部分）は中身から計算されるので、新しい名前になり、ブラウザは新しい CSS を読みます（第12巻 UNIT 03）。

### まとめ

- Topcoat の CLI は `cargo install topcoat-cli --version 0.11.0 --locked` で入れる（243 個の部品をビルドして 107 秒）。`topcoat --version` で確かめる。アプリの Topcoat と CLI の版はそろえる。
- `topcoat new` で、`Cargo.toml`・`src\app.rs`・`styles.css` などができる。`cargo run` だけだとアセットの束が無くて止まる。`topcoat asset bundle` か `topcoat dev` で束を作る。
- `topcoat dev` は、ビルド・束・起動をして、保存のたびにやり直す。見出しを書き換えて保存すると、ページも変わった。
- Topcoat 0.11 は Rust 1.99 以上が要る（`requires rustc 1.99`）。同じポートは `AddrInUse`（10048）。
