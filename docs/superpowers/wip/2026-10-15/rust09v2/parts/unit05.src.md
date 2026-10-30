## UNIT 05: Cargo.toml・版の固定・features

### この単元のゴール

- RustNote の `Cargo.toml` の、直接使うクレートの版を `=` で固定する理由と、`Cargo.lock` が間接のクレートの版まで記録する役目を説明できる
- `cargo tree` で、依存の木・features（機能）・同じクレートの2つの版を確かめ、**features は依存の間でまとめられる**ことを、tokio の例で説明できる
- 版を上げるときの手順（調べる → `Cargo.toml` を直す → `cargo update -p` → テスト）を、RustNote を Topcoat 0.10.0 から 0.11.0 に上げた実際の記録で説明できる
- 古い Rust で動かしたとき（`rust-version`）・版がぶつかったとき・`--locked` で止められたとき・取り下げられた版を指定したときのエラーを読める

### たとえ話でつかむ

料理教室のレシピを考えます。

- レシピに「しょうゆ 大さじ1」とだけ書くと、家によって、こいくち・うすくち・減塩と、違うしょうゆが使われて、味が変わります。そこで、**銘柄まで**書きます（「〇〇しょうゆ こいくち」）。これが `=` で版を固定することです。
- しょうゆの中には、さらに小麦や大豆が入っていますが、それはレシピには書きません。代わりに、教室で使った材料の**仕入れの記録**を全部残しておきます。同じ記録で仕入れれば、どの家でも同じ味になります。これが `Cargo.lock` です。
- 「ねぎ（白い所だけ）」のように、材料の**使う部分**も書きます。ところが、ほかの料理（ほかのクレート）が「ねぎ（青い所も）」を頼んでいると、鍋には、まとめて両方が入ります。これが features のまとめです。
- 新しい銘柄が出たら、**試食してから**レシピを書き換えます。勝手に入れ替わることはありません。

| 料理教室 | Cargo |
|---|---|
| 銘柄まで書いたレシピ | `Cargo.toml` の `topcoat = "=0.11.0"`（直接使うクレートの版を固定） |
| 仕入れの記録 | `Cargo.lock`（間接のクレートも含めた、すべての版） |
| 材料の使う部分 | features（`tokio` の `rt-multi-thread`・`macros` など） |
| 鍋で両方まとまる | features のまとめ（ほかのクレートが頼んだ機能も入る） |
| 試食してから書き換える | 版を上げる手順（`cargo update -p` のあとでテスト） |

> **用語:** features（フィーチャー：クレートの中の、入れるかどうかを選べる機能。`Cargo.toml` の `features = […]` で選ぶ）

<!-- -->

> **用語:** Cargo.lock（カーゴ・ロック：ビルドに使ったすべてのクレートの、正確な版を記録したファイル。アプリでは Git に入れて共有する）

<!-- -->

> **用語:** MSRV（エムエスアールブイ：Minimum Supported Rust Version。そのクレートを使える、いちばん古い Rust の版。`Cargo.toml` の `rust-version` に書く）

### 図で理解する

`Cargo.toml` と `Cargo.lock` が、それぞれ何を決めるかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0501" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="300" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="170" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Cargo.toml（人が書く）</text>
<text x="40" y="72" font-size="12" fill="#0f172a" font-family="Consolas, monospace">rust-version = "1.99"</text>
<text x="40" y="96" font-size="12" fill="#0f172a" font-family="Consolas, monospace">topcoat = "=0.11.0"</text>
<text x="40" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">tokio = { version = "=1.54.1",</text>
<text x="40" y="140" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  features = ["rt-multi-thread",</text>
<text x="40" y="160" font-size="12" fill="#0f172a" font-family="Consolas, monospace">              "macros"] }</text>
<text x="40" y="190" font-size="11" fill="#475569">直接使う 9 個（と、テスト用の 1 個）の</text>
<text x="40" y="208" font-size="11" fill="#475569">版と機能、使える Rust の版</text>
<rect x="400" y="20" width="300" height="230" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="550" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Cargo.lock（Cargo が書く）</text>
<text x="420" y="72" font-size="12" fill="#0f172a" font-family="Consolas, monospace">name = "hyper"  version = "0.14.32"</text>
<text x="420" y="96" font-size="12" fill="#0f172a" font-family="Consolas, monospace">name = "hyper"  version = "1.12.0"</text>
<text x="420" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">name = "generic-array"</text>
<text x="420" y="140" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  version = "0.14.7"</text>
<text x="420" y="160" font-size="12" fill="#0f172a">… ほか全部</text>
<text x="420" y="190" font-size="11" fill="#475569">間接のものも含めた、すべてのクレートの</text>
<text x="420" y="208" font-size="11" fill="#475569">正確な版（誰の PC でも同じになる）</text>
<line x1="320" y1="135" x2="396" y2="135" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0501)"/>
<text x="358" y="125" font-size="11" fill="#475569" text-anchor="middle">cargo が</text>
<text x="358" y="155" font-size="11" fill="#475569" text-anchor="middle">版を選ぶ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: Cargo.toml と Cargo.lock。人は、直接使うクレートの版と機能を Cargo.toml に書く。Cargo は、それに合う全部のクレートの版を選んで、Cargo.lock に記録する</div>
</div>

features は、依存の間でまとめられます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0502" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="42" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">rustnote</text>
<text x="120" y="62" font-size="11" fill="#0f172a" text-anchor="middle">rt-multi-thread・macros</text>
<rect x="20" y="88" width="200" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="110" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">topcoat（の serve）</text>
<text x="120" y="130" font-size="11" fill="#0f172a" text-anchor="middle">macros・net など</text>
<rect x="20" y="156" width="200" height="56" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="120" y="178" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">libsql_replication</text>
<text x="120" y="198" font-size="11" fill="#0f172a" text-anchor="middle">full（全部）</text>
<rect x="420" y="70" width="280" height="96" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="560" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ビルドされる tokio 1.54.1 は1つ</text>
<text x="560" y="120" font-size="11" fill="#0f172a" text-anchor="middle">機能は、頼まれたものを全部まとめた形</text>
<text x="560" y="140" font-size="11" fill="#0f172a" text-anchor="middle">（full・macros・net・fs・time・signal …）</text>
<line x1="220" y1="48" x2="416" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0502)"/>
<line x1="220" y1="116" x2="416" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0502)"/>
<line x1="220" y1="184" x2="416" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0502)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: features のまとめ。同じクレートの同じ版を使う3つのクレートが、それぞれ頼んだ機能は、まとめて1つの tokio に入る。だから、自分の Cargo.toml に書いていない機能も、入っていることがある</div>
</div>

### まず動かす最小コード

#### 例5-1: RustNote の Cargo.toml と、依存の木を読む

RustNote の `Cargo.toml` です（UNIT 04 までと同じく、コピーした `rustnote` フォルダのものです）。

```toml
@@FILE:book/rustnote/Cargo.toml@@
```

直接使っているクレートだけを、木の形で見ます。

```powershell
cargo tree --depth 1 --charset ascii
```

```text
@@OUT:u05_tree@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: = は「この版だけ」という意味

- `topcoat = "=0.11.0"` の `=` は、「0.11.0 だけを使う」という意味です。`=` を付けずに `topcoat = "0.11.0"` と書くと、Cargo は「0.11.0 以上、0.12.0 未満」のうち、いちばん新しい版を選びます（`^0.11.0` と同じ意味です）。
- RustNote は教材なので、**読む人の PC でも、この本と同じ結果になる**ように、直接使うクレートを全部 `=` で固定しています。0.11 の間でも、小さな版（0.11.1 など）で表示や動きが変わることがあるからです。
- `cargo tree --depth 1` の木は、`Cargo.toml` に書いた 9 個と、テストだけで使う `tempfile`（`[dev-dependencies]`）を、`Cargo.lock` で決まった版で表示しています。

##### 解説2: rust-version は、使える Rust の下限

- `rust-version = "1.99"` は、「このクレートは Rust 1.99 以上で使ってください」という印です。Topcoat 0.11 が 1.99 以上を求めるので、RustNote も 1.99 にしています。古い Rust で動かすと、ビルドの前に止まります（エラー1）。
- 実際に使う版は、`rust-toolchain.toml` の `channel = "1.99.1"` で決めています（第1巻・この巻の巻頭）。「下限」と「実際に使う版」は別のファイルです。

##### 解説3: features で、使う機能を選ぶ

- `tokio` には、`features = ["rt-multi-thread", "macros"]` と書いています。`rt-multi-thread` は「いくつものスレッドで仕事を回すランタイム」、`macros` は `#[tokio::main]` や `#[tokio::test]` のマクロです（第5巻 UNIT 02）。
- `serde` の `features = ["derive"]` は、`#[derive(Deserialize)]` を使うための機能です（第6巻 UNIT 06）。

### もう一歩: 実務に近い形

#### 例5-2: Topcoat を 0.10.0 から 0.11.0 に上げる

この教材は、2026 年 10 月に、RustNote を Topcoat 0.10.0 から 0.11.0 に上げました。そのときの手順を、0.10.0 のころの `Cargo.toml` と `Cargo.lock` に戻したコピーで、もう一度たどります。

**① 新しい版を調べる。**

```powershell
cargo search topcoat --limit 1
cargo search tokio --limit 1
```

```text
@@OUT:u05_search@@
```

**② `Cargo.toml` を直す。** `topcoat = "=0.11.0"`・`tokio = { version = "=1.54.1", … }` に書き換え、Topcoat 0.11 が Rust 1.99 以上を求めるので、`rust-version = "1.99"` にします（変更点の説明は、Topcoat の `CHANGELOG.md` で読みます）。

**③ その2つだけを上げる。** `-p` で、上げるクレートを選びます。

```powershell
cargo update -p topcoat -p tokio
```

```text
@@OUT:u05_update@@
```

- `Updating topcoat v0.10.0 -> v0.11.0` のように、Topcoat の部品（`topcoat-router` など）がまとめて上がり、新しい部品 `topcoat-l10n` が足されました（0.11 で入った、言葉の切り替えの部品）。`-p` で選んだクレートと、それが使うものだけが変わり、ほかのクレートの版はそのままです。
- 最後の `31 unchanged dependencies behind latest` は、「新しい版が出ているのに、上げていない間接のクレートが 31 個あります」という意味です。

**④ ほかの間接のクレートも上げるかを決める。** `--dry-run` で、何が変わるかを先に見ます。

```powershell
cargo update --dry-run
```

```text
@@OUTHEAD:u05_dryrun:11@@
...
@@OUTTAIL:u05_dryrun:3@@
```

- 34 個の間接のクレートが少しずつ新しい版に上がり、1 個が足される予定です（どれも、それぞれの版の決まりの範囲の中の上げ方です）。`warning: not updating lockfile due to dry run` は「試しなので、`Cargo.lock` は書き換えていません」という意味です。

**⑤ 上げて、テストを全部動かす。**

```powershell
cargo update
cargo test
```

```text
@@OUTHEAD:u05_test:12@@
```

- テストは全部通りました（一部だけを載せています）。RustNote のコードは、1行も直さずに 0.11 で動きました。こうして作った `Cargo.lock` は、教材の RustNote の `Cargo.lock` と、1文字も違わない同じものになりました。
- 大事なのは、**版を上げる作業と、機能を足す作業を混ぜない**ことです。上げたあとで何かが壊れたとき、原因が「版」なのか「自分の変更」なのか、すぐに分かるからです。

### よくあるエラーと読み方

#### エラー1: 古い Rust で動かした（rust-version）

PC に入っている古い Rust（1.90.0）で、RustNote をビルドしようとした場合です。

```powershell
cargo +1.90.0 check
```

```text
@@OUTHEAD:u05_err_msrv:8@@
...
```

- **何が起きたか:** 「rustc 1.90.0 は、次のパッケージに対応していません」というエラーで、`rustnote@0.1.0 requires rustc 1.99`（rustnote は rustc 1.99 を求めます）のあとに、Topcoat の部品が並びました。コンパイルを始める**前に**、`rust-version` を見て止めています。
- **直し方:** `cargo +1.90.0` を付けずに、`rust-toolchain.toml` の版（1.99.1）で動かします。`rustup toolchain list` で、入っている版を確かめられます。

#### エラー2: 版が、ほかのクレートの求めとぶつかった

`serde` の版を、`=1.0.100` に下げて固定してしまった場合です。

```text
@@OUT:u05_err_conflict@@
```

- **何が起きたか:** 「`serde` の版を選べません」というエラーです。`... required by package libsql_replication v0.10.0` から下は、「libsql_replication が、`^1.0.189`（1.0.189 以上）を求めています」という、求めのつながりです。RustNote が `=1.0.100` と固定したので、両方を満たす版がありません（`all possible versions conflict`）。
- **直し方:** `=` で固定する版は、ほかのクレートの求めを満たす版にします。たいていは、`cargo update` で選ばれた版（RustNote なら 1.0.229）をそのまま固定します。

#### エラー3: --locked で、Cargo.lock の書き換えを止められた

`anyhow` の版を `Cargo.toml` で変えたのに、`Cargo.lock` を作り直さずに、`--locked` でビルドした場合です。

```text
@@OUT:u05_err_locked@@
```

- **何が起きたか:** 「`--locked` が付いているので、`Cargo.lock` を書き換えられません」というエラーです。`--locked` は、「`Cargo.lock` に書いてあるとおりにだけ作る」という指定です。`Cargo.toml` と `Cargo.lock` が食い違っていると、そこで止めます。
- **学べること:** これは**わざと止めている**エラーです。RustNote の CI（`ci.yml`）と Dockerfile は、`cargo test --locked`・`cargo build --release --locked` を使います。手元で `Cargo.toml` を直したのに、`Cargo.lock` を Git に入れ忘れると、CI がこのエラーで気づかせてくれます（第12巻 UNIT 02（準備中））。
- **直し方:** 手元で `cargo build`（`--locked` なし）をして `Cargo.lock` を作り直し、`Cargo.toml` と一緒に Git に入れます。

#### エラー4（エラーが出ない間違い）: 使っている機能を、Cargo.toml に書いていない

RustNote の `tokio` の features から、`"macros"` を消してしまった場合です。

```text
@@OUT:u05_silent_build@@
```

- **何が起きたか:** `#[tokio::main]` と `#[tokio::test]` を使っているのに、**ビルドは通りました**。`cargo tree` で、tokio の `macros` を誰が頼んでいるかを見ると、Topcoat が頼んでいました。

```text
@@OUT:u05_silent_tree@@
```

- `macros` は、`topcoat`・`tokio` の `full`・`topcoat-router` の `serve` から頼まれていました（`full` は、`libsql_replication` が頼んでいます）。features はまとめられるので、RustNote が書かなくても入ったのです。
- ところが、同じ書き方を、Topcoat を使わない小さなクレートですると、止まります。

```toml
[dependencies]
tokio = { version = "=1.54.1", features = ["rt-multi-thread"] }
```

```text
@@OUT:u05_err_macros@@
```

- 「`tokio` の中に `main` が見つかりません」（E0433）と、「`main` 関数は `async` にできません」（E0752）です。`macros` が無いと、`#[tokio::main]` そのものが無いからです。
- **学べること:** ほかのクレートが頼んでくれている機能に頼ると、そのクレートの版を上げたときに、**突然ビルドできなくなる**ことがあります。**自分が使う機能は、自分の Cargo.toml に書く**のが決まりです（第6巻 UNIT 02 で、tokio の `net` を足したのと同じです）。

### RustNote ではここで使う

- RustNote の `Cargo.toml` は、例5-1のとおり、直接使う 9 個のクレートを `=` で固定し、tokio は使う機能（`rt-multi-thread`・`macros`）だけを書いています。それぞれのクレートの上に、何のために使うかを一行ずつ書いてあります。
- `Cargo.lock` は Git に入れてあり、CI と Dockerfile は `--locked` で、記録どおりの版でビルドします（エラー3）。
- `default-run = "rustnote"` は、`examples` や `src/bin` が増えても、`cargo run` でアプリそのものを動かすための設定です（第6巻 UNIT 02 のエラー4）。
- 版を上げるときは、例5-2の手順（調べる → 直す → `-p` で上げる → テスト → 間接のものを上げる → テスト）で進め、版を上げるだけのコミットにします。

### 練習問題

**問1:** `cargo tree -d --depth 0` で、RustNote の中に、同じクレートの版が2つ以上入っているものを調べてください。`hyper` に2つの版があるのは、なぜでしょうか。

**問2:** `generic-array` は、`cargo update` をしても 0.14.7 のままで、`0.14.9 available` と表示されます（第6巻 UNIT 02）。`cargo tree -i generic-array --depth 1` で、誰が使っているかを調べてください。

**問3:** `tokio` を、`=1.54.0` に固定しようとすると、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
@@OUT:u05_q1@@
```

- 版が2つ以上あるクレートが、30 種類ありました。`hyper` は 0.14.32 と 1.12.0 です。それぞれ、誰が使っているかを `-i`（逆向きの木）で見ます。

```text
@@OUT:u05_q1_hyper@@
```

- 古い `hyper 0.14` は、`libsql` が使う `tonic 0.11`（通信の部品）が使っています。新しい `hyper 1.12` は、Topcoat（`topcoat-router`）が使っています。版の大きな番号（0.14 と 1）が違うと、Cargo は1つにまとめられないので、両方を入れます。
- 同じクレートが2つ入ると、ビルドの時間と実行ファイルの大きさが増えます。自分では直せないことが多いですが、`cargo tree -d` で、何が2つ入っているかを知っておくと、版を上げるときの判断に使えます。

**問2の解答:**

```text
@@OUT:u05_q2@@
```

- `crypto-common` などが使っています。第6巻 UNIT 02 で確かめたとおり、`crypto-common` が `generic-array` を `=0.14.7` と**固定**しているので、Cargo は 0.14.9 を選べません。間接のクレートの固定は、自分の `Cargo.toml` では変えられません。

**問3の解答:**

```text
@@OUT:u05_q3_yanked@@
```

- `version 1.54.0 is yanked`（1.54.0 は取り下げられています）で、選べませんでした。**取り下げ**（yank）は、問題が見つかった版を、作った人が「新しく使わないでください」と印を付けることです。すでにその版を記録している `Cargo.lock` はそのまま使えますが、新しく選ぶことはできません。
- 版を固定するときは、`cargo search` や crates.io で、取り下げられていない新しい版（ここでは 1.54.1）を選びます。

### まとめ

- `Cargo.toml` には、直接使うクレートの版（RustNote は `=` で固定）・使う機能（features）・使える Rust の下限（`rust-version`）を書く。`Cargo.lock` は、間接のものも含めた全部の版を記録し、Git に入れて、CI と Docker では `--locked` で使う。
- features は依存の間でまとめられるので、書いていない機能も入っていることがある。自分が使う機能は、自分の `Cargo.toml` に書く。`cargo tree`（`-e features`・`-d`・`-i`）で中身を確かめられる。
- 版を上げるときは、調べる → `Cargo.toml` を直す → `cargo update -p` → テスト → 間接のものを上げる → テスト、の順に、版を上げるだけの変更として進める。
