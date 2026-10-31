## UNIT 04: rustup・rustc・Cargo

### この単元のゴール

- `rustup`・`rustc`・`cargo` の3つの役目の違いを説明できる
- `rustup-init.exe` で Rust を入れ、新しい窓で `rustc --version`・`rustup show` を使って、どの版がどこに入ったかを確かめられる
- `cargo new`・`cargo run` で最初のプログラムを作って動かし、できたファイルの役目を言える
- `rust-toolchain.toml` でフォルダごとに版を決め、`rustup update`・`rustup install` で版を新しくしたり、足したりできる

### たとえ話でつかむ

**レストランの厨房**を考えます。

- **料理人**（`rustc`）は、レシピ（ソースコード）を読んで、料理（実行ファイル）を作ります。
- **段取り係**（`cargo`）は、どのレシピを、どの順番で料理人に渡すかを決め、材料（ほかの人が作った部品）を取り寄せ、できた料理を決まった棚（`target` フォルダ）に並べます。私たちがふだん話しかけるのは、この段取り係です。
- **人事係**（`rustup`）は、料理人を雇い、いろいろな年の料理人（版）をそろえておきます。厨房の入口に「この厨房では 1.99.1 年の料理人を使う」と貼り紙（`rust-toolchain.toml`）があれば、その料理人を呼びます。貼り紙が無ければ、いつもの料理人（`stable`）を呼びます。
- 厨房の受付（`.cargo\bin`）に「料理人さん」と声をかけると、受付は人事係に「どの料理人を呼ぶか」を聞いてから、その人につなぎます。

| 厨房 | Rust の道具 |
|---|---|
| 料理人 | `rustc`（コンパイラ） |
| 段取り係 | `cargo`（ビルドと部品の管理） |
| 人事係 | `rustup`（Rust の版の管理） |
| いろいろな年の料理人 | ツールチェーン（`stable-aarch64-pc-windows-msvc`・`1.99.1-…` など） |
| 厨房の入口の貼り紙 | `rust-toolchain.toml` |
| 受付 | `C:\Users\（名前）\.cargo\bin` の `rustc.exe`・`cargo.exe` |
| 料理を並べる棚 | `target` フォルダ |

> **用語:** ツールチェーン（toolchain：ある版の Rust の道具一式。`rustc`・`cargo`・標準ライブラリなどが入っている。`stable-aarch64-pc-windows-msvc` のように「版-CPU-OS-道具の種類」の名前が付く）

<!-- -->

> **用語:** コンパイラ（compiler：人が書いたソースコードを、CPU が実行できる形（機械語）に変える道具。Rust のコンパイラが `rustc`）

### 図で理解する

`rustc` と打ったとき、どの版の `rustc` が動くかは、こう決まります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-041" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="40" width="150" height="120" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="95" y="64" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">あなたが打つ</text>
<text x="95" y="92" font-size="12" fill="#0f172a" text-anchor="middle">rustc --version</text>
<text x="95" y="114" font-size="12" fill="#0f172a" text-anchor="middle">cargo run</text>
<text x="95" y="136" font-size="12" fill="#0f172a" text-anchor="middle">（今いるフォルダで）</text>
<rect x="215" y="40" width="210" height="120" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text font-family="Consolas, Yu Gothic UI, monospace" x="320" y="64" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">受付: .cargo\bin</text>
<text x="320" y="90" font-size="12" fill="#0f172a" text-anchor="middle">rustc.exe・cargo.exe・rustup.exe</text>
<text x="320" y="112" font-size="11" fill="#0369a1" text-anchor="middle">中身は3つとも rustup と同じ</text>
<text x="320" y="132" font-size="11" fill="#0369a1" text-anchor="middle">（PATH に入っているのはここ）</text>
<rect x="470" y="22" width="230" height="156" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="585" y="46" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">どの版を呼ぶか</text>
<text x="484" y="72" font-size="12" fill="#0f172a">① 今いるフォルダ（か、その上の</text>
<text x="484" y="90" font-size="12" fill="#0f172a">　フォルダ）の rust-toolchain.toml</text>
<text x="484" y="116" font-size="12" fill="#0f172a">② 無ければ、既定の版（stable）</text>
<text x="484" y="142" font-size="11" fill="#b45309">cargo +1.98.0 … のように</text>
<text x="484" y="160" font-size="11" fill="#b45309">打てば、その版を直接選べる</text>
<line x1="172" y1="100" x2="211" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-041)"/>
<line x1="427" y1="100" x2="466" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-041)"/>
<rect x="20" y="206" width="680" height="110" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text font-family="Consolas, Yu Gothic UI, monospace" x="360" y="228" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">本物の道具の置き場: C:\Users\（名前）\.rustup\toolchains</text>
<rect x="40" y="244" width="200" height="56" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.2"/>
<text x="140" y="268" font-size="12" fill="#0f172a" text-anchor="middle">stable-aarch64-…</text>
<text x="140" y="288" font-size="11" fill="#15803d" text-anchor="middle">いつもの版（今は 1.99.1）</text>
<rect x="260" y="244" width="200" height="56" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.2"/>
<text x="360" y="268" font-size="12" fill="#0f172a" text-anchor="middle">1.99.1-aarch64-…</text>
<text x="360" y="288" font-size="11" fill="#15803d" text-anchor="middle">RustNote のフォルダで使う版</text>
<rect x="480" y="244" width="200" height="56" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.2"/>
<text x="580" y="268" font-size="12" fill="#0f172a" text-anchor="middle">1.98.0-aarch64-…</text>
<text x="580" y="288" font-size="11" fill="#15803d" text-anchor="middle">古い版も、並べて置ける</text>
<line x1="585" y1="180" x2="585" y2="240" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-041)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: rustup のしくみ。受付（.cargo の bin）が、フォルダの貼り紙（rust-toolchain.toml）を見て、棚（toolchains）から版を選ぶ</div>
</div>

`cargo run` と打ったときに、中で起きることです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-042" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="140" height="90" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="90" y="56" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">hello フォルダ</text>
<text x="90" y="80" font-size="12" fill="#0f172a" text-anchor="middle">Cargo.toml</text>
<text font-family="Consolas, Yu Gothic UI, monospace" x="90" y="102" font-size="12" fill="#0f172a" text-anchor="middle">src\main.rs</text>
<rect x="200" y="30" width="120" height="90" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="260" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo</text>
<text x="260" y="90" font-size="11" fill="#0369a1" text-anchor="middle">段取りをする</text>
<rect x="360" y="30" width="120" height="90" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="420" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">rustc</text>
<text x="420" y="90" font-size="11" fill="#0369a1" text-anchor="middle">機械語にする</text>
<rect x="520" y="30" width="180" height="90" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="610" y="62" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">link.exe</text>
<text x="610" y="86" font-size="11" fill="#0369a1" text-anchor="middle">つないで .exe にする</text>
<text x="610" y="106" font-size="11" fill="#0369a1" text-anchor="middle">（UNIT 05 で入れる道具）</text>
<line x1="162" y1="75" x2="196" y2="75" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-042)"/>
<line x1="322" y1="75" x2="356" y2="75" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-042)"/>
<line x1="482" y1="75" x2="516" y2="75" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-042)"/>
<line x1="610" y1="122" x2="610" y2="146" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-042)"/>
<rect x="390" y="150" width="310" height="56" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text font-family="Consolas, Yu Gothic UI, monospace" x="545" y="174" font-size="12" fill="#0f172a" text-anchor="middle">target\debug\hello.exe ができる</text>
<text x="545" y="194" font-size="11" fill="#15803d" text-anchor="middle">cargo run は、できたらすぐに動かす</text>
<text x="20" y="160" font-size="11" fill="#475569">2回目の cargo run は、</text>
<text x="20" y="178" font-size="11" fill="#475569">ソースが変わっていなければ</text>
<text x="20" y="196" font-size="11" fill="#475569">作り直さずに、すぐ動かす</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: cargo run の流れ。cargo が rustc に頼み、link.exe がつないで、target の中に実行ファイルができる</div>
</div>

### まず動かす最小コード

#### 例4-1: rustup-init.exe で Rust を入れる

> **▼ この手順がやること（先に日本語で）:** 公式サイトから、PC の CPU に合った `rustup-init.exe` を入手して動かし、Rust を入れます。

**手順1: 入手する。** ブラウザで `https://rustup.rs/` を開きます。Windows では、CPU ごとに3つの `rustup-init.exe` が並んでいます（2026-10-31 に確かめたもの）。

| PC の CPU（UNIT 01 で確かめたもの） | 選ぶもの | 入手先 |
|---|---|---|
| ARM64（Snapdragon など） | Windows on Arm | `https://win.rustup.rs/aarch64` |
| x64（Intel・AMD） | Windows 64-bit | `https://win.rustup.rs/x86_64` |
| 32 ビットの PC | Windows 32-bit | `https://win.rustup.rs/i686` |

**手順2: 動かす。** ダウンロードした `rustup-init.exe` を、エクスプローラーでダブルクリックするか、PowerShell で動かします。

```powershell
# ダウンロードのフォルダにある rustup-init.exe を動かす
cd ~\Downloads
.\rustup-init.exe
```

この本では、PC にもともと入っている Rust を変えないように、Rust の置き場を別のフォルダ（`rust-try`）にして試しました（`RUSTUP_HOME` と `CARGO_HOME` で場所を変え、`--no-modify-path` で PATH を変えないようにしています）。ふつうに入れると、置き場は `C:\Users\（あなたの名前）\.rustup` と `.cargo` になり、`modify PATH variable` は `yes` になります。途中で聞かれたら、`1`（標準のインストール）を選んで Enter キーを押します。

**▼ 実行結果**

```text

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\...\rust-try\rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\...\rust-try\cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\...\rust-try\cargo\bin

This path needs to be in your PATH environment variable,
but will not be added automatically.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


    default host tuple: aarch64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: no

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>
info: profile set to default
info: default host tuple is aarch64-pc-windows-msvc
info: syncing channel updates for stable-aarch64-pc-windows-msvc
info: latest update on 2026-10-15 for version 1.99.1 (58cf8f9c3 2026-10-13)
info: downloading 6 components
info: default toolchain set to stable-aarch64-pc-windows-msvc

  stable-aarch64-pc-windows-msvc installed - rustc 1.99.1 (58cf8f9c3 2026-10-13)


Rust is installed now. Great!

To get started you need Cargo's bin directory (C:\Users\...\rust-try\cargo
\bin) in your PATH
environment variable. This has not been done automatically.

Press the Enter key to continue.
```

#### ▼ 1つずつ分解して解説

##### 解説1: 2つの置き場

- **Rustup home**（ふつうは `.rustup`）は、人事係の棚です。いろいろな版のツールチェーンが、ここにしまわれます。
- **Cargo home**（ふつうは `.cargo`）の中の `bin` が受付です。`rustc.exe`・`cargo.exe`・`rustup.exe` がここに置かれ、ここが PATH に足されます。
- `default host tuple: aarch64-pc-windows-msvc` は、「この PC は ARM64 の Windows で、MSVC（UNIT 05）の道具を使う」という意味です。x64 の PC では `x86_64-pc-windows-msvc` になります。

##### 解説2: 入ったのは stable の 1.99.1

- `default toolchain: stable` は、「いつもの版は、いちばん新しい安定版（stable）」という意味です。2026-10-31 の stable は **1.99.1** でした。
- `downloading 6 components` は、rustc・cargo・標準ライブラリ・rustfmt（整形）・clippy（注意）・説明書の6つです（例4-2で確かめます）。

##### 解説3: 最後は、新しい窓を開く

- `Rust is installed now. Great!` と出れば成功です。PATH が変わったのは、**これから開く窓**だけです。今開いている PowerShell の窓は、いったん閉じて、新しく開き直します。

#### 例4-2: 新しい窓で、入った版と場所を確かめる

> **▼ この手順がやること（先に日本語で）:** 新しく開いた PowerShell で、3つの道具の版と、どこにあるかを表示します。そのあと、`rustup show` で、入っているツールチェーンと、今どれが選ばれているかを表示します。

```powershell
rustc --version
cargo --version
rustup --version
where.exe rustc cargo rustup
```

**▼ 実行結果**

```text
rustc 1.99.1 (58cf8f9c3 2026-10-13)
cargo 1.99.1 (5f94df478 2026-08-27)
rustup 1.29.2 (d8c92c676 2026-10-21)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
C:\Users\...\rust-try\cargo\bin\rustc.exe
C:\Users\...\rust-try\cargo\bin\cargo.exe
C:\Users\...\rust-try\cargo\bin\rustup.exe
```

```powershell
# 入っているツールチェーンと、今どれが選ばれているか
rustup show
# 入っている部品
rustup component list --installed
# 受付の3つの .exe の中身を比べる（ハッシュの先頭 16 文字）
Get-ChildItem ~\.cargo\bin\rustup.exe, ~\.cargo\bin\rustc.exe, ~\.cargo\bin\cargo.exe | ForEach-Object { "{0,-10} {1}" -f $_.Name, (Get-FileHash $_.FullName).Hash.Substring(0, 16) }
```

**▼ 実行結果**

```text
Default host: aarch64-pc-windows-msvc
rustup home:  C:\Users\...\rust-try\rustup

installed toolchains
--------------------
stable-aarch64-pc-windows-msvc (active, default)

active toolchain
----------------
name: stable-aarch64-pc-windows-msvc
active because: it's the default toolchain
installed targets:
  aarch64-pc-windows-msvc
```

```text
cargo-aarch64-pc-windows-msvc
clippy-aarch64-pc-windows-msvc
rust-docs-aarch64-pc-windows-msvc
rust-std-aarch64-pc-windows-msvc
rustc-aarch64-pc-windows-msvc
rustfmt-aarch64-pc-windows-msvc
```

```text
rustup.exe 58005A321C5DCB9C
rustc.exe  58005A321C5DCB9C
cargo.exe  58005A321C5DCB9C
```

#### ▼ 1つずつ分解して解説

##### 解説1: 3つの版の数字

- `rustc 1.99.1` と `cargo 1.99.1` は、同じツールチェーンの中の道具なので、版がそろっています。`rustup 1.29.2` は人事係そのものの版で、Rust の版とは別に数えます（`info:` の2行が、そう教えてくれています）。
- `where.exe` の結果は、3つとも同じ `cargo\bin` のフォルダでした。ここが受付です。

##### 解説2: 受付の .exe は、中身がみな rustup

- 受付の `rustup.exe`・`rustc.exe`・`cargo.exe` は、ハッシュ（中身から計算した指紋）が同じでした。どれも中身は rustup で、「自分がどの名前で呼ばれたか」を見て、選んだツールチェーンの本物の `rustc` や `cargo` を呼び出します（図の受付）。

##### 解説3: `rustup show` の読み方

- `installed toolchains` が棚に入っている版の一覧、`active toolchain` が**今いるフォルダで**選ばれる版です。`active because: it's the default toolchain` は「既定の版なので選ばれた」という意味です。
- `installed targets` は、「どの CPU・OS 用の実行ファイルを作れるか」の一覧です（UNIT 05 で足します）。

#### 例4-3: 最初のプロジェクトを作って、動かす

> **▼ この手順がやること（先に日本語で）:** UNIT 01 で作った作業フォルダ `rust-study` の中に、`cargo new` で `hello` というプロジェクトを作ります。できたファイルを見たあと、`cargo run` を2回続けて打ちます。

```powershell
cd ~\rust-study
cargo new hello
cd hello
cargo run
cargo run
```

**▼ 実行結果**（`cargo new hello`）

```text
    Creating binary (application) `hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

できたファイルです（`.git` の中は除いています）。

```text
./.gitignore
./Cargo.toml
./src/main.rs
（.git フォルダもある）
```

`Cargo.toml`:

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`src\main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

**▼ 実行結果**（1回目の `cargo run`）

```text
   Compiling hello v0.1.0 (C:\Users\...\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target\debug\hello.exe`
Hello, world!
```

**▼ 実行結果**（2回目の `cargo run`）

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\hello.exe`
Hello, world!
```

```powershell
# できた実行ファイルを見て、直接動かす
Get-ChildItem target\debug -Filter *.exe | Select-Object Name, Length
.\target\debug\hello.exe
```

**▼ 実行結果**

```text
Name      Length
----      ------
hello.exe 131584
Hello, world!
```

#### ▼ 1つずつ分解して解説

##### 解説1: `cargo new` が作るもの

- `Cargo.toml` は、プロジェクトの**表紙**です。名前（`hello`）・版（`0.1.0`）・Rust の書き方の年版（`edition = "2024"`）と、使う部品の一覧（`[dependencies]`。今は空）が書かれています。
- `src\main.rs` が、プログラムの本体です。`fn main()` から始まり、`println!` で1行表示します。
- `.gitignore` も一緒に作られ、`cargo new` は Git のリポジトリも作ります（`.git` フォルダ。UNIT 02）。`.gitignore` には `/target` が書かれていて、できた実行ファイルを Git に入れないようにしています。

##### 解説2: 1回目と2回目の違い

- 1回目は `Compiling hello` と出て、作るのに 0.48 秒かかりました。`Finished` の行の `dev` profile は「開発用の作り方（速く作れるが、実行は遅め）」という意味です（本番用の作り方は第12巻 UNIT 01）。
- 2回目は `Compiling` の行が無く、0.01 秒でした。cargo は、ソースが変わっていなければ作り直さず、前に作った `target\debug\hello.exe` をすぐ動かします。

##### 解説3: `target` フォルダ

- できた実行ファイルは `target\debug\hello.exe`（131,584 バイト）でした。`cargo run` を使わずに、直接動かすこともできます。
- `target` には、作る途中のファイルもたくさん置かれます。消しても、次の `cargo build` でまた作られます（`cargo clean` で消せます）。

### もう一歩: 実務に近い形

#### 例4-4: フォルダごとに版を決める（rust-toolchain.toml）

> **▼ この手順がやること（先に日本語で）:** `rust-study` の中に `old` というフォルダを作り、「このフォルダでは Rust 1.98.0 を使う」と書いた `rust-toolchain.toml` を置きます。1.98.0 はまだ入っていません。そのフォルダで `rustc --version` を打つと、どうなるかを見ます。

```powershell
cd ~\rust-study
mkdir old
cd old
# 「このフォルダでは 1.98.0 を使う」という貼り紙を作る
Set-Content -Encoding ascii rust-toolchain.toml "[toolchain]`nchannel = `"1.98.0`""
rustup show active-toolchain
rustc --version
```

**▼ 実行結果**（`mkdir` が表示する、できたフォルダの一覧は省いています）

```text
info: syncing channel updates for 1.98.0-aarch64-pc-windows-msvc
info: latest update on 2026-08-20 for version 1.98.0 (88d9e12ae 2026-08-18)
info: downloading 6 components
warn: the missing active toolchain `1.98.0-aarch64-pc-windows-msvc` has been auto-installed
warn: auto-installation is deprecated for most `rustup` commands
warn: scripts relying on this behavior in `rustup` may stop working in the future
warn: to install the active toolchain, use `rustup install` instead
warn: see <https://github.com/rust-lang/rustup/issues/4836> for more info
1.98.0-aarch64-pc-windows-msvc (overridden by 'C:\Users\...\rust-study\old\rust-toolchain.toml')
rustc 1.98.0 (88d9e12ae 2026-08-18)
（かかった時間: 65 秒）
```

`hello` と `old` で、選ばれる版を比べました。

```text
stable-aarch64-pc-windows-msvc (default)
1.98.0-aarch64-pc-windows-msvc (overridden by 'C:\Users\...\rust-study\old\rust-toolchain.toml')
```

#### ▼ 1つずつ分解して解説

##### 解説1: 入っていない版は、その場で入れてくれた（けれど、その動きは古くなる）

- 1.98.0 は入っていなかったので、rustup が**その場でダウンロードして入れました**（65 秒）。そのあと、`overridden by '…\old\rust-toolchain.toml'`（このファイルで上書きされた）と、`rustc 1.98.0` が表示されました。
- ただし、`warn:` の行のとおり、この「自動で入れる」動きは、rustup 1.29.2 では**非推奨**（いずれ無くなる予定）です。ふつうのコマンドの途中で、急に大きなダウンロードが始まるのを避けるためです。これからは、フォルダに入ったら、先に `rustup install`（貼り紙に書かれた版を入れる）を打つ習慣にします。

##### 解説2: 同じ PC の中で、版を使い分ける

- `hello` では `stable`（既定）、`old` では 1.98.0（貼り紙）が選ばれました。フォルダを移るだけで、使う Rust が変わります。UNIT 01 の「RustNote のフォルダでは 1.99.1」と同じしくみです。

#### 例4-5: いつもの版（stable）を新しくする

> **▼ この手順がやること（先に日本語で）:** この本の PC のいつもの Rust（stable）は 1.97.1 のままでした。`rustup update` で、いちばん新しい stable にします。

```powershell
rustup update stable
```

**▼ 実行結果**（この本の PC の、いつもの Rust）

```text
info: syncing channel updates for stable-aarch64-pc-windows-msvc
info: latest update on 2026-10-15 for version 1.99.1 (58cf8f9c3 2026-10-13)
info: removing previous version of component rust-std for target x86_64-pc-windows-msvc
info: removing previous version of component rust-std for target wasm32-unknown-unknown
info: removing previous version of component cargo
info: removing previous version of component clippy
info: removing previous version of component rust-docs
info: removing previous version of component rust-std
info: removing previous version of component rustc
info: removing previous version of component rustfmt
info: downloading 8 components

  stable-aarch64-pc-windows-msvc updated - rustc 1.99.1 (58cf8f9c3 2026-10-13) (from rustc 1.97.1 (8bab26f4f 2026-07-14))

info: checking for self-update (current version: 1.29.2)
```

```powershell
rustup toolchain list
```

**▼ 実行結果**

```text
stable-aarch64-pc-windows-msvc (active, default)
1.90.0-aarch64-pc-windows-msvc
1.99.0-aarch64-pc-windows-msvc
1.99.1-aarch64-pc-windows-msvc
```

- stable が 1.97.1 から **1.99.1** になりました。前の版の部品を消してから、新しい部品を入れています。この PC の stable には、x64 用（`x86_64-pc-windows-msvc`）と WebAssembly 用（`wasm32-unknown-unknown`）の部品も入っていたので、それも一緒に新しくなりました。
- `rustup toolchain list` には、stable のほかに、貼り紙のために入れた版（1.90.0・1.99.0・1.99.1）が並びます。使わなくなった版は、`rustup toolchain uninstall 1.90.0` のように消せます。
- **stable は、新しい版が出るたびに `rustup update` で上がります**（だいたい6週ごと）。一方、`rust-toolchain.toml` で決めたフォルダの版は変わりません。RustNote を、決めた版で確かめ続けられるのは、このためです。

### よくあるエラーと読み方

#### エラー1: 「用語 'rustc' は、…認識されません」（PATH に入っていない）

```text
rustc : The term 'rustc' is not recognized as the name of a cmdlet, function, script file, or operable program. Check t
he spelling of the name, or if a path was included, verify that the path is correct and try again.
At line:1 char:140
+ ... ;') | Where-Object { $_ -notmatch 'cargo' }) -join ';'; rustc --versi ...
+                                                             ~~~~~
    + CategoryInfo          : ObjectNotFound: (rustc:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

- **何が起きたか:** PowerShell が PATH のフォルダを探しましたが、`rustc` が見つかりませんでした。この本の PC で、PATH から `.cargo\bin` を抜いた PowerShell で打つと、このエラーになりました。
- **よくある原因:** Rust を入れる前から開いていた窓で打っている（PATH が変わるのは、新しく開いた窓だけ）。または、インストールの途中で `modify PATH variable` を `no` にした。
- **直し方:** まず PowerShell の窓を閉じて、新しく開き直します。それでもだめなら、`$env:Path -split ';'` で、`C:\Users\（名前）\.cargo\bin` が入っているかを確かめます。

#### エラー2: 入っていない版を、黙って入れ始める（例4-4）

- エラーではありませんが、`rust-toolchain.toml` のあるフォルダで初めてコマンドを打つと、急にダウンロードが始まることがあります。ネットにつながっていないと、ここで失敗します。
- **直し方:** フォルダに入ったら、先に `rustup install` で、貼り紙の版を入れておきます。

#### エラー3: Rust を深いフォルダに入れると、リンクで LNK1104

この本で、Rust の置き場（`RUSTUP_HOME`）を、とても深いフォルダにして `cargo run` をしたときの、最後の行です（パスの途中は `...` で縮めています）。

```text
  = note: LINK : fatal error LNK1104: ファイル '...\sb\rustup\toolchains\stable-aarch64-pc-windows-msvc\lib\rustlib\aarch64-pc-windows-msvc\lib\libpanic_unwind-2c26c6f32d00640e.rlib' を開くことができません。
```

- **何が起きたか:** ツールチェーンの中の部品のパスが、267 文字になっていました。UNIT 01 で見た、259 文字の上限を超えたので、リンカーがファイルを開けませんでした。
- ツールチェーンの中だけで、約 127 文字あります。ふつうの置き場（`C:\Users\（名前）\.rustup`）なら問題ありませんが、`RUSTUP_HOME` を自分で決めるときは、短い場所にします。

### RustNote ではここで使う

- RustNote のフォルダの `rust-toolchain.toml` には、`channel = "1.99.1"` と `components = ["rustfmt", "clippy"]` が書かれています（UNIT 01）。RustNote のフォルダに入ったら、最初に `rustup install` を打つと、1.99.1 と、整形・注意の道具がそろいます。
- RustNote の `Cargo.toml` には `rust-version = "1.99"` と書かれています。Topcoat 0.11 が Rust 1.99 以上を求めるからです（UNIT 08）。
- `cargo run` で RustNote を動かすのは、UNIT 08 です。

### 練習問題

**問1:** 友だちの PC で `rustup show` を打つと、`Default host: x86_64-pc-windows-msvc` と表示されました。友だちの PC の CPU は、何でしょうか。この本の PC と、どこが違いますか。

**問2:** `rust-study\old` の中で `cargo new pinned` をして、`pinned` の中で `rustc --version` を打つと、どの版になるでしょうか。

**問3:** `hello` の `src\main.rs` の `Hello, world!` を `こんにちは` に変えて `cargo run` をしました。1回目と2回目の `cargo run` で、表示はどう違うでしょうか。

### 解答と解説

**問1の解答:**

- `x86_64` なので、x64（Intel や AMD）の CPU の PC です。この本の PC は `aarch64`（ARM64）でした。
- 名前のうしろの `pc-windows-msvc` は同じで、どちらも「Windows で、MSVC の道具を使う」という意味です（UNIT 05）。

**問2の解答:**

- **1.98.0** になります。rustup は、今いるフォルダに `rust-toolchain.toml` が無ければ、その**上のフォルダ**を順に探します。`pinned` には貼り紙がありませんが、1つ上の `old` に貼り紙があるので、それが使われます（図の①）。

**問3の解答:**

- ソースを変えたので、1回目は `Compiling hello` の行が出て作り直され、`こんにちは` と表示されます。2回目は、ソースが変わっていないので `Compiling` が無く、すぐに `こんにちは` と表示されます。

### まとめ

- `rustup` は版を管理する人事係、`rustc` はコンパイラ、`cargo` はビルドと部品の段取り係。受付（`.cargo\bin`）の `rustc.exe`・`cargo.exe` は、中身が rustup で、フォルダの `rust-toolchain.toml` か既定の stable から版を選ぶ。
- `rustup-init.exe` は CPU に合ったものを `https://rustup.rs/` から入れる。入れたら新しい窓で `rustc --version`・`rustup show` を確かめる。2026-10-31 の stable は 1.99.1。
- `cargo new` で `Cargo.toml`・`src\main.rs`・`.gitignore` ができ、`cargo run` で `target\debug\hello.exe` が作られて動く。2回目は作り直さない。
- 貼り紙の版が無いと、rustup は自動で入れるが、その動きは非推奨。`rustup install` で先に入れる。stable は `rustup update` で新しくする。
