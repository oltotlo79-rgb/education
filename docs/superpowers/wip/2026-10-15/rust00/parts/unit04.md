## UNIT 04: ソースコードから実行まで

### この単元のゴール

- **ソースコード**・**コンパイラ**・**実行ファイル**の関係を、流れ図で説明できる
- `rustc`（コンパイラ）と `cargo`（プロジェクト管理の道具）の役割の違いが分かる
- `cargo new`・`cargo run`・`cargo build`・`cargo check`・`--release` を使い分けられる
- **コンパイル時の失敗**と**実行時の失敗**を区別できる

### たとえ話でつかむ

プログラムが動くまでの流れは、**本の翻訳と出版**にたとえられます。

- **ソースコードは日本語の原稿**です。人間が読み書きしやすい言葉（Rust）で書かれていますが、コンピュータ（CPU）はこのままでは読めません。CPU が読めるのは **機械語**（0 と 1 の並び）だけです。
- **コンパイラ（rustc）は翻訳者**です。原稿を機械語に翻訳します。ただし Rust の翻訳者はとても厳しく、**翻訳する前に原稿を隅々まで校閲**します。「ここで数字と文字を足していますよ」「この変数は書き換え禁止ですよ」と矛盾を1つでも見つけると、翻訳を断って赤ペンの指摘（**コンパイルエラー**）を返します。
- **実行ファイル（.exe）は翻訳済みの本**です。CPU がそのまま読めるので、翻訳者がいなくても何度でも読めます（実行できます）。
- **Cargo（カーゴ）は編集者**です。原稿の置き場所を決め、翻訳者に依頼し、必要な参考資料（他の人が作った部品＝**クレート**）を取り寄せ、できた本を決まった棚にしまってくれます。私たちはふだん編集者の Cargo に頼み、翻訳者の rustc を直接呼ぶことはあまりありません。

> **用語:** **コンパイル** … ソースコードを、CPU が実行できる形（機械語）に翻訳すること。翻訳する道具を **コンパイラ** と呼びます。Rust のコンパイラの名前は `rustc`（ラストシー）です。
>
> **用語:** **クレート（crate）** … Rust のプログラムや部品のひとまとまり。「木箱」という意味です。他の人が作ったクレートは crates.io というサイトで公開されていて、Cargo が自動で取り寄せてくれます（Topcoat もクレートの1つです）。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="12" y="40" width="140" height="90" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="82" y="66" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ソースコード</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="82" y="88" font-size="12" fill="#334155" text-anchor="middle">src\main.rs</text>
<text x="82" y="108" font-size="12" fill="#334155" text-anchor="middle">人が読める文字</text>
<rect x="196" y="40" width="140" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="266" y="66" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">コンパイル</text>
<text x="266" y="88" font-size="12" fill="#334155" text-anchor="middle">rustc が校閲して</text>
<text x="266" y="108" font-size="12" fill="#334155" text-anchor="middle">機械語に翻訳</text>
<rect x="380" y="40" width="140" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="450" y="66" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">実行ファイル</text>
<text x="450" y="88" font-size="12" fill="#334155" text-anchor="middle">practice.exe</text>
<text x="450" y="108" font-size="12" fill="#334155" text-anchor="middle">CPU が読める</text>
<rect x="564" y="40" width="144" height="90" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="636" y="66" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">実行（プロセス）</text>
<text x="636" y="88" font-size="12" fill="#334155" text-anchor="middle">OS が読み込んで</text>
<text x="636" y="108" font-size="12" fill="#334155" text-anchor="middle">動かす</text>
<line x1="154" y1="85" x2="192" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="338" y1="85" x2="376" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="522" y1="85" x2="560" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="266" y1="132" x2="266" y2="176" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-00-11)"/>
<rect x="166" y="180" width="200" height="80" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="266" y="204" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">コンパイルエラー</text>
<text x="266" y="226" font-size="12" fill="#334155" text-anchor="middle">矛盾があると .exe は作られない</text>
<text x="266" y="246" font-size="12" fill="#334155" text-anchor="middle">例: 型の不一致（UNIT 09）</text>
<line x1="636" y1="132" x2="636" y2="176" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-00-11)"/>
<rect x="506" y="180" width="202" height="80" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="607" y="204" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">実行時エラー（パニック）</text>
<text x="607" y="226" font-size="12" fill="#334155" text-anchor="middle">動かして初めて分かる失敗</text>
<text x="607" y="246" font-size="12" fill="#334155" text-anchor="middle">例: 範囲外の番号・変換失敗</text>
<text x="20" y="24" font-size="12" fill="#64748b">cargo build ＝ 左の2つの矢印まで　／　cargo run ＝ 右の矢印（実行）まで全部</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: ソースコードが実行されるまで。失敗には「コンパイル時」と「実行時」の2種類がある</div>
</div>

Cargo のプロジェクトは、次のような決まった形のフォルダになっています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-12" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="14" width="330" height="272" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="30" y="40" font-size="14" font-weight="700" fill="#334155">hello\（プロジェクトのフォルダ）</text>
<rect x="34" y="54" width="290" height="44" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="46" y="73" font-size="13" font-weight="700" fill="#9a3412">Cargo.toml</text>
<text x="46" y="90" font-size="12" fill="#334155">名前・版・使うクレートの一覧（設定）</text>
<rect x="34" y="106" width="290" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="125" font-size="13" font-weight="700" fill="#1e40af">src\main.rs</text>
<text x="46" y="142" font-size="12" fill="#334155">ソースコード（あなたが書く）</text>
<rect x="34" y="158" width="290" height="44" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="46" y="177" font-size="13" font-weight="700" fill="#334155">Cargo.lock</text>
<text x="46" y="194" font-size="12" fill="#334155">実際に使ったクレートの版の記録（自動）</text>
<rect x="34" y="210" width="290" height="62" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="229" font-size="13" font-weight="700" fill="#047857">target\（自動で作られる）</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="247" font-size="12" fill="#334155">debug\hello.exe　…　cargo build</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="264" font-size="12" fill="#334155">release\hello.exe … --release 付き</text>
<rect x="420" y="40" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="64" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo new hello</text>
<text x="436" y="88" font-size="12" fill="#334155">フォルダ・Cargo.toml・main.rs を作る</text>
<rect x="420" y="124" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="148" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo build</text>
<text x="436" y="172" font-size="12" fill="#334155">src を読んで target に .exe を作る</text>
<rect x="420" y="208" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="232" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo run</text>
<text x="436" y="256" font-size="12" fill="#334155">必要なら build してから .exe を実行</text>
<line x1="416" y1="73" x2="328" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
<line x1="416" y1="157" x2="328" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
<line x1="416" y1="241" x2="328" y2="246" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: Cargo のプロジェクトの形と、主なコマンドが触る場所</div>
</div>

### まず動かす最小コード

#### `cargo new` でプロジェクトを作り、`cargo run` で動かす

> **▼ このコマンドがやること（先に日本語で）:** `hello` という名前の Rust プロジェクトを作り、中に入って、最初から用意されているプログラムをコンパイルして実行します。もう一度実行して、2回目は翻訳（コンパイル）が省かれることも確かめます。

UNIT 03 で作った `rust-study` フォルダの中で、次のコマンドを打ちます。

```powershell
cargo new hello
cd hello
cat Cargo.toml
cat src\main.rs
cargo run
cargo run
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study> cargo new hello
    Creating binary (application) `hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
PS C:\Users\you\rust-study> cd hello
PS C:\Users\you\rust-study\hello> cat Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
PS C:\Users\you\rust-study\hello> cat src\main.rs
fn main() {
    println!("Hello, world!");
}
PS C:\Users\you\rust-study\hello> cargo run
   Compiling hello v0.1.0 (C:\Users\you\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\hello.exe`
Hello, world!
PS C:\Users\you\rust-study\hello> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\hello.exe`
Hello, world!
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: `cargo new` が作ったもの

```text
    Creating binary (application) `hello` package
```

- 「`hello` という名前の、**実行できるアプリ（binary）**のパッケージを作りました」という報告です。
- 2行目の `note:` は「`Cargo.toml` に書ける設定の一覧はこのページにあります」という案内で、エラーではありません。
- 作られたのは `Cargo.toml`・`src\main.rs`・`.gitignore`（Git 用の設定）と、隠しフォルダの `.git` です。

##### 解説2: `Cargo.toml` はプロジェクトの「表紙」

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `[package]` の下はこのプロジェクト自身の情報です。`name` は名前、`version` はこのプログラムの版です。
- `edition = "2024"` は「Rust 2024 年版の文法で書いています」という宣言です。Rust は数年ごとに書き方の細かい決まりを見直していて、このシリーズは最新の 2024 年版を使います。
- `[dependencies]`（依存関係）の下には、使いたいクレートを書きます。今は空です。第6巻では、ここに Topcoat が加わります。

> **用語:** **TOML（トムル）** … 設定ファイルの書き方の1つ。`名前 = 値` を並べ、`[見出し]` でまとまりを作ります。

##### 解説3: `cargo run` の3行の報告

```text
   Compiling hello v0.1.0 (C:\Users\you\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\hello.exe`
```

- `Compiling` … 「`hello` の 0.1.0 版をコンパイルしています」。
- `Finished` … 「終わりました」。`dev` プロファイルは**開発用の設定**で、`unoptimized`（最適化なし）・`debuginfo`（調査用の情報つき）という意味です。かかった時間は 0.30 秒でした。
- `Running` … 「できた `target\debug\hello.exe` を実行します」。この後の `Hello, world!` が、プログラム自身の出力です。
- 2回目は `Compiling` の行がありません。ソースコードが変わっていないので、Cargo が「翻訳し直す必要なし」と判断し、前回の `.exe` をそのまま使ったからです。

##### 解説4: できた実行ファイルを確かめる

`cargo run` の後に一覧を見ると、`target` フォルダと `Cargo.lock` が増えています。

```text
PS C:\Users\you\rust-study\hello> ls


    ディレクトリ: C:\Users\you\rust-study\hello


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        2026/10/17     11:30                src
d-----        2026/10/17     11:30                target
-a----        2026/10/17     11:30              8 .gitignore
-a----        2026/10/17     11:30            149 Cargo.lock
-a----        2026/10/17     11:30             76 Cargo.toml


PS C:\Users\you\rust-study\hello> .\target\debug\hello.exe
Hello, world!
```

- `target\debug\hello.exe` が翻訳済みの本です。**Cargo を通さずに直接実行しても動きます**。
- 実行ファイルは、そのまま別の Windows PC にコピーしても動きます（同じ種類の CPU なら）。Rust を入れていない PC でも動くのが、コンパイルする言語の特徴です。
- `target` フォルダは何度でも作り直せるので、Git での記録の対象外です（`.gitignore` にそう書かれています）。

### もう一歩: 実務に近い形

#### 開発用（debug）と本番用（release）の速さを比べる

> **▼ このコードがやること（先に日本語で）:** 1 から 2 億までの数について「7 で割った余り」を足し合わせ、かかった時間を表示します。同じコードを、開発用の設定と本番用の設定でコンパイルして、速さを比べます。

```rust
use std::hint::black_box;
use std::time::Instant;

fn main() {
    // 計算を始めた時刻を覚えておく
    let start = Instant::now();

    // 2 億回のくり返しで、1 から順に「7 で割った余り」を足していく
    let mut total: u64 = 0;
    for n in 1..=200_000_000u64 {
        // black_box は「この計算を省略しないで」とコンパイラに伝えるための道具
        total += black_box(n) % 7;
    }

    // かかった時間を表示する
    println!("合計: {total}");
    println!("かかった時間: {:?}", start.elapsed());
}
```

```powershell
cargo run
cargo run --release
```

**▼ 実行結果**（`Compiling` などの行も載せています）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target\debug\practice.exe`
合計: 599999998
かかった時間: 1.3268229s
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `release` profile [optimized] target(s) in 0.71s
     Running `target\release\practice.exe`
合計: 599999998
かかった時間: 545.5482ms
```

答えは同じですが、`--release` 付きは2倍以上速くなりました（時間は PC や、そのとき他に動いているアプリによって変わります）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 時間を計る

```rust
let start = Instant::now();
// ...
println!("かかった時間: {:?}", start.elapsed());
```

- `Instant::now()` は「今この瞬間」を記録します。`start.elapsed()` は「`start` から何秒たったか」です。
- `{:?}` で表示すると、`1.3268229s`（秒）や `545.5482ms`（ミリ秒＝1000分の1秒）のように単位つきで出ます。

##### 解説2: 大きな数と型

```rust
let mut total: u64 = 0;
for n in 1..=200_000_000u64 {
```

- `u64` は「0 以上の整数で、とても大きな数まで入る型」です。合計が約 6 億になるので、大きめの型を選んでいます。型は第2巻で学びます。
- `200_000_000` の `_` は読みやすくするための区切りで、`200000000` と同じです。

##### 解説3: debug と release の違い

| | `cargo build`（debug） | `cargo build --release` |
|---|---|---|
| 置き場所 | `target\debug\` | `target\release\` |
| コンパイルの速さ | 速い | 遅め（最適化に時間をかける） |
| 実行の速さ | 遅め | 速い |
| 使う場面 | 開発中、何度も直して試すとき | 本番に出すとき、速さを測るとき |

- **最適化**とは、コンパイラが「同じ結果になる、もっと速いやり方」に機械語を作り変えることです。
- `black_box` は、計算を丸ごと省略される（最適化されすぎる）のを防ぐための道具です。速さ比べの実験でだけ使います。

#### コンパイラを直接呼ぶ: `rustc`

Cargo は裏で `rustc` を呼んでいます。1ファイルだけなら、`rustc` を直接使うこともできます。

```rust
fn main() {
    println!("rustc だけでコンパイルしました");
}
```

この内容で `main.rs` を作ったフォルダで、次を実行します。

```powershell
rustc main.rs
ls
.\main.exe
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study\rustc-only> rustc main.rs
PS C:\Users\you\rust-study\rustc-only> ls


    ディレクトリ: C:\Users\you\rust-study\rustc-only


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30         131072 main.exe
-a----        2026/10/17     11:30        1396736 main.pdb
-a----        2026/10/15     20:38             74 main.rs


PS C:\Users\you\rust-study\rustc-only> .\main.exe
rustc だけでコンパイルしました
```

`rustc` は**何も表示せずに**終わり、`main.exe` ができました。成功したときに黙っているのは、多くのコマンドに共通する習慣です。`main.pdb` は、問題を調べるときに使う補助のファイルです。実際の開発では、クレートの取り寄せや設定の管理をしてくれる Cargo を使います。

#### よく使う Cargo のコマンド

| コマンド | すること | いつ使うか |
|---|---|---|
| `cargo new 名前` | 新しいプロジェクトを作る | 始めるとき |
| `cargo run` | コンパイルして実行する | 動かして確かめるとき |
| `cargo build` | コンパイルだけする | 実行ファイルだけ欲しいとき |
| `cargo check` | 校閲（エラー探し）だけして、`.exe` は作らない | 書きながら何度も確かめるとき（いちばん速い） |
| `cargo test` | テストを実行する | 直したら毎回（UNIT 10） |
| `cargo fmt` | コードの見た目（字下げなど）を整える | 保存する前 |
| `cargo clippy` | 「もっと良い書き方」を助言してもらう | 区切りのいいところで |

### よくあるエラーと読み方

#### プロジェクトの外で `cargo run` した

```text
PS C:\Users\you\rust-study> cargo run
error: could not find `Cargo.toml` in `C:\Users\you\rust-study` or any parent directory
```

**何が起きたか:** 「`C:\Users\you\rust-study` にも、その親のフォルダにも `Cargo.toml` が見つからない」というエラーです。Cargo は `Cargo.toml` のある場所をプロジェクトとみなすので、プロジェクトの**外**にいると何をコンパイルすればよいか分かりません。

**直し方:** `cd hello` のように、`Cargo.toml` のあるフォルダへ移動してから実行します。プロンプトで今いる場所を確かめましょう（UNIT 02・03）。

#### コンパイルは通ったのに、実行して止まった

> **▼ このコードがやること（先に日本語で）:** 文字の `"42"` と `"4x2"` を、それぞれ数値に変換しようとします。1つ目は成功しますが、2つ目は数値ではないので、実行中に失敗します。

```rust
fn main() {
    // 文字列を数値に変換する。どちらもコンパイルは通る
    let ok: i32 = "42".parse().unwrap();
    println!("1 つ目: {ok}");

    // "4x2" は数値ではないので、実行してみて初めて失敗が分かる
    let ng: i32 = "4x2".parse().unwrap();
    println!("2 つ目: {ng}");
}
```

**▼ 実行結果**

```text
1 つ目: 42

thread 'main' (53536) panicked at src\main.rs:7:33:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:**

- コンパイルは成功し、1つ目の `println!` までは実行されました（`1 つ目: 42`）。
- 7行目で、`"4x2"` を数値にしようとして失敗しました。`.parse()` は「成功（`Ok`）か失敗（`Err`）」を返し、`.unwrap()` は「成功なら中身を取り出す、**失敗ならパニックする**」という道具です。
- `ParseIntError { kind: InvalidDigit }` は「整数への変換エラー、種類は『数字でない文字がある』」という意味です。

**直し方:** 「失敗するかもしれない」と分かっている処理では、`unwrap()` を使わず、成功と失敗の両方の場合を書きます（練習問題3）。フォームに入力された値などは、利用者が何を打つか分からないので、必ずこの書き方をします。

> **つまずきポイント:** 「コンパイルが通った ＝ 正しく動く」ではありません。コンパイラが見つけるのは**書き方の矛盾**（型が合わない、変数が無い など）で、**実行してみないと分からない値の問題**（ファイルが無い、入力が数字でない など）は、実行時の失敗になります。Rust はコンパイル時に見つけられる問題を最大限見つけてくれるので、実行時の失敗はかなり少なくなります。

### RustNote ではここで使う

- 開発中は `cargo run` で RustNote のサーバーを起動し、ブラウザで `http://127.0.0.1:3000` を開いて確かめます。Topcoat には、ファイルを保存するたびに自動でビルドし直してくれる `topcoat dev` というコマンドもあります（第6巻）。
- `Cargo.toml` の `[dependencies]` に、Topcoat・データベース用のクレート・パスワード処理用のクレートなどを書きます。どのクレートのどの版を使うかは第9巻で決めます。
- 公開するときは `cargo build --release` で本番用の実行ファイルを作り、Docker という箱に入れて Fly.io に送ります（第12巻）。

### 練習問題

**問題1:** `cargo new greet` を実行すると、どんなファイルとフォルダが作られるでしょうか。また、`cargo run` の後に増えるものは何でしょうか。

**問題2:** 次の場面では、`cargo build`・`cargo run`・`cargo check` のどれを使うのがよいでしょうか。

- a. コードを書いている途中で、エラーが無いかだけを素早く確かめたい
- b. 書き終えたので、動かして結果を見たい
- c. 実行ファイルだけを作って、あとで別の PC で動かしたい

**問題3:** 「よくあるエラー」の `"4x2"` の例を、パニックせずに「変換できません」と表示して最後まで動くように書き直してください。

### 解答と解説

**問題1の解答:** `greet` フォルダの中に `Cargo.toml`・`src\main.rs`・`.gitignore`（と隠しフォルダ `.git`）が作られます。`cargo run` の後には、コンパイル結果の入った `target` フォルダと、使ったクレートの版を記録する `Cargo.lock` が増えます。

**問題2の解答:** a は `cargo check`（`.exe` を作らないぶん最も速い）、b は `cargo run`、c は `cargo build`（本番用なら `cargo build --release`）。

**問題3の解答:**

```rust
fn main() {
    for text in ["42", "4x2"] {
        // parse の結果は「成功(Ok)」か「失敗(Err)」のどちらか
        match text.parse::<i32>() {
            Ok(n) => println!("{text} -> 数値 {n}"),
            Err(e) => println!("{text} -> 変換できません（{e}）"),
        }
    }
}
```

**▼ 実行結果**

```text
42 -> 数値 42
4x2 -> 変換できません（invalid digit found in string）
```

`match` で `Ok` と `Err` の両方の場合を書いたので、失敗しても止まらずに最後まで進みます。`{e}` には、エラーの説明文「invalid digit found in string（文字列の中に数字でない文字があった）」が入ります。`parse::<i32>()` の `::<i32>` は「`i32`（整数）に変換して」という指定です。

### まとめ

- ソースコードは rustc が**校閲してから**機械語に翻訳し、実行ファイルができます。Cargo はその作業全体を取り仕切る編集者です。
- `cargo run` は「必要ならコンパイル → 実行」、`cargo check` は校閲だけ、`--release` は速い本番用です。
- 失敗には**コンパイル時**（書き方の矛盾、`.exe` が作られない）と**実行時**（パニックなど、動かして初めて分かる）の2種類があります。

