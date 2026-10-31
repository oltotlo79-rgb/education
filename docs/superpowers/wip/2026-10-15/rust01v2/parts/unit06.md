## UNIT 06: Cargo の品質コマンド

### この単元のゴール

- `cargo check`・`cargo fmt`・`cargo clippy`・`cargo test` の4つを、目的ごとに使い分けられる
- `cargo fmt --check` の差分、`cargo clippy` の注意、`cargo test` の失敗を読んで、コードを直せる
- `-D warnings`（注意をエラーとして扱う）と終了コードの意味を説明できる
- rustfmt・clippy が入っていないときのエラーを読み、`rustup component add` で足せる

### たとえ話でつかむ

**レポートを提出する前の、4つの見直し**を考えます。

- **`cargo check`** は、**誤字脱字の見直し**です。文として成り立っているか（型が合っているか）だけを、急いで確かめます。清書（実行ファイル作り）はしません。
- **`cargo fmt`** は、**体裁を整える係**です。字下げや空白を、決まりの形にそろえます。中身は変えません。
- **`cargo clippy`** は、**先生の赤ペン**です。「間違いではないけれど、こう書く方が分かりやすい」という所に、印を付けてくれます。
- **`cargo test`** は、**答え合わせ**です。「この問題には、この答えになるはず」というテストを動かして、本当に合っているかを確かめます。

この4つは、得意なことが違います。誤字脱字が無くても（check が通っても）、答えが間違っていることはあります（test で見つかる）。

| レポートの見直し | Cargo のコマンド | 見るもの |
|---|---|---|
| 誤字脱字 | `cargo check` | 型と文法（実行ファイルは作らない） |
| 体裁 | `cargo fmt` | 字下げ・空白・改行の形 |
| 赤ペン | `cargo clippy` | よくある書き方の注意 |
| 答え合わせ | `cargo test` | 動かした結果が、期待どおりか |

> **用語:** rustfmt（ラスト・フォーマット：Rust の整形の道具。`cargo fmt` で呼ぶ）

<!-- -->

> **用語:** 終了コード（exit code：コマンドが終わるときに返す数。0 なら成功、0 以外なら失敗。PowerShell では `$LASTEXITCODE` で読める）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-061" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">提出（コミット・公開）の前に、4つを順に通す</text>
<rect x="20" y="40" width="150" height="120" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="95" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo fmt --check</text>
<text x="95" y="92" font-size="11" fill="#0f172a" text-anchor="middle">体裁がそろっているか</text>
<text x="95" y="114" font-size="11" fill="#0369a1" text-anchor="middle">直すのは cargo fmt</text>
<text x="95" y="136" font-size="11" fill="#0369a1" text-anchor="middle">いちばん速い</text>
<rect x="195" y="40" width="150" height="120" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="270" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo check</text>
<text x="270" y="92" font-size="11" fill="#0f172a" text-anchor="middle">型と文法</text>
<text x="270" y="114" font-size="11" fill="#0369a1" text-anchor="middle">.exe は作らない</text>
<text x="270" y="136" font-size="11" fill="#0369a1" text-anchor="middle">書きながら何度も</text>
<rect x="370" y="40" width="150" height="120" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="445" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo clippy</text>
<text x="445" y="92" font-size="11" fill="#0f172a" text-anchor="middle">check ＋ 赤ペン</text>
<text x="445" y="114" font-size="11" fill="#b45309" text-anchor="middle">-- -D warnings で</text>
<text x="445" y="136" font-size="11" fill="#b45309" text-anchor="middle">注意も失敗にする</text>
<rect x="545" y="40" width="155" height="120" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="622" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">cargo test</text>
<text x="622" y="92" font-size="11" fill="#0f172a" text-anchor="middle">答え合わせ</text>
<text x="622" y="114" font-size="11" fill="#15803d" text-anchor="middle">作って、動かして</text>
<text x="622" y="136" font-size="11" fill="#15803d" text-anchor="middle">結果を比べる</text>
<line x1="172" y1="100" x2="191" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-061)"/>
<line x1="347" y1="100" x2="366" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-061)"/>
<line x1="522" y1="100" x2="541" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-061)"/>
<rect x="20" y="182" width="680" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="204" font-size="12" fill="#1d4ed8">どれかが失敗すると、終了コードが 0 以外になる。CI（第11巻）は fmt・clippy・test の順に動かす</text>
<text x="34" y="226" font-size="12" fill="#1d4ed8">check が通っても、答えが間違っていることはある（例6-4）。4つは、見ている物が違う</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 4つの品質コマンド。速くて安い物から順に通し、最後にテストで答え合わせをする</div>
</div>

### まず動かす最小コード

#### 例6-1: 練習のプロジェクトを作り、cargo check をかける

> **▼ この手順がやること（先に日本語で）:** `rust-study` に、ライブラリ（ほかから使われる部品）のプロジェクト `quality` を作ります。`src\lib.rs` を、わざと3つの問題を入れた次のコードに書き換えて、まず `cargo check` をかけます。

```powershell
cd ~\rust-study
cargo new --lib quality
cd quality
```

```text
    Creating library `quality` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

`src\lib.rs`（わざと問題を入れたもの）:

```rust
/// ノートの題の文字数を数える
pub fn title_length(title:&str)->usize{
    return title.len();
}

/// 題が空（空白だけ）かどうか
pub fn is_blank(title: &str) -> bool {
    title.trim().len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_title() {
        assert_eq!(title_length("買い物"), 3);
    }

    #[test]
    fn blank_title() {
        assert!(is_blank("   "));
        assert!(!is_blank("メモ"));
    }
}
```

```powershell
cargo check
```

**▼ 実行結果**

```text
    Checking quality v0.1.0 (C:\Users\...\rust-study\quality)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
```

- `Checking quality` と出て、0.37 秒ほどで終わりました。`exit=0`（終了コード 0）は成功です。この本では、終了コードを `'exit=' + $LASTEXITCODE` で表示しています。
- **3つの問題があるのに、check は通りました。** 型と文法は正しいからです。体裁・書き方・答えの間違いは、ほかの3つのコマンドで見つけます。

#### 例6-2: cargo fmt で体裁を整える

```powershell
# 整っていない所を表示するだけ（ファイルは変えない）
cargo fmt --check
# 整える（ファイルを書き換える）
cargo fmt
cargo fmt --check
```

**▼ 実行結果**（1回目の `cargo fmt --check`）

```text
Diff in \\?\C:\Users\...\rust-study\quality\src\lib.rs:1:
 /// ノートの題の文字数を数える
-pub fn title_length(title:&str)->usize{
+pub fn title_length(title: &str) -> usize {
     return title.len();
 }
```

**▼ 実行結果**（`cargo fmt` のあとの `cargo fmt --check`）

```text

```

整えたあとの `src\lib.rs` の先頭です。

```rust
/// ノートの題の文字数を数える
pub fn title_length(title: &str) -> usize {
    return title.len();
}

/// 題が空（空白だけ）かどうか
pub fn is_blank(title: &str) -> bool {
    title.trim().len() == 0
}
```

- `--check` は、`-`（今の形）と `+`（整えた形）を並べて見せ、整っていなければ終了コード 1 で終わります。`Diff in` のパスの前の `\\?\` は、Windows の長いパスの書き方で、気にしなくて大丈夫です。
- `cargo fmt` は、`title:&str)->usize{` を `title: &str) -> usize {` に整えました。**中身（`return title.len();`）は変えていません。** 体裁の係なので、書き方の良し悪しは見ません。

#### 例6-3: cargo clippy の注意を読む

```powershell
cargo clippy
cargo clippy -- -D warnings
```

**▼ 実行結果**（`cargo clippy`）

```text
    Checking quality v0.1.0 (C:\Users\...\rust-study\quality)
warning: unneeded `return` statement
 --> src\lib.rs:3:5
  |
3 |     return title.len();
  |     ^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_return
  = note: `#[warn(clippy::needless_return)]` on by default
help: remove `return`
  |
3 -     return title.len();
3 +     title.len()
  |

warning: length comparison to zero
 --> src\lib.rs:8:5
  |
8 |     title.trim().len() == 0
  |     ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `title.trim().is_empty()`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#len_zero
  = note: `#[warn(clippy::len_zero)]` on by default

warning: `quality` (lib) generated 2 warnings (run `cargo clippy --fix --lib -p quality -- ` to apply 2 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
```

**▼ 実行結果**（`cargo clippy -- -D warnings`）

```text
    Checking quality v0.1.0 (C:\Users\...\rust-study\quality)
error: unneeded `return` statement
 --> src\lib.rs:3:5
  |
3 |     return title.len();
  |     ^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#needless_return
  = note: `-D clippy::needless-return` implied by `-D warnings`
  = help: to override `-D warnings` add `#[allow(clippy::needless_return)]`
help: remove `return`
  |
3 -     return title.len();
3 +     title.len()
  |

error: length comparison to zero
 --> src\lib.rs:8:5
  |
8 |     title.trim().len() == 0
  |     ^^^^^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `title.trim().is_empty()`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#len_zero
  = note: `-D clippy::len-zero` implied by `-D warnings`
  = help: to override `-D warnings` add `#[allow(clippy::len_zero)]`

error: could not compile `quality` (lib) due to 2 previous errors
```

#### ▼ 1つずつ分解して解説

##### 解説1: 注意の読み方

- 1つ目の `warning: unneeded `return` statement` は、「最後の式なら `return` は要らない」という注意です。`-->` の行が場所（`src\lib.rs` の 3 行目 5 文字目）、`help: remove `return`` の下が、直した形です。
- 2つ目の `length comparison to zero` は、「長さを 0 と比べるより、`is_empty()` の方が分かりやすい」という注意です。
- `help: for further information visit …` の URL を開くと、その注意の説明（なぜそう書くのか）が読めます。注意の名前（`needless_return`・`len_zero`）で検索することもできます。

##### 解説2: 注意だけなら成功、-D warnings なら失敗

- ふつうの `cargo clippy` は、注意があっても `exit=0` でした。注意は「失敗」ではないからです。
- `-- -D warnings`（`--` のうしろは clippy への指示。`-D` は deny＝許さない）を付けると、注意が `error` になり、`exit=101` で失敗しました。CI（第11巻）では、注意を残したまま先へ進まないように、こちらを使います。

#### 例6-4: cargo test で答え合わせ

```powershell
cargo test
```

**▼ 実行結果**

```text
   Compiling quality v0.1.0 (C:\Users\...\rust-study\quality)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src\lib.rs (target\debug\deps\quality-c31da7b9e94c39ed.exe)

running 2 tests
test tests::blank_title ... ok
test tests::counts_japanese_title ... FAILED

failures:

---- tests::counts_japanese_title stdout ----

thread 'tests::counts_japanese_title' (69460) panicked at src\lib.rs:17:9:
assertion `left == right` failed
  left: 9
 right: 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::counts_japanese_title

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass `--lib`
```

- `counts_japanese_title` が `FAILED` になりました。`left: 9`（実際の値）、`right: 3`（期待した値）です。
- `title.len()` は**バイト数**を返します。日本語の1文字は UTF-8 で 3 バイトなので、「買い物」は 9 でした。文字数を数えるには `title.chars().count()` を使います（第2巻）。
- check も fmt も clippy も、この間違いには気づきませんでした。**答えの間違いは、テストでしか見つかりません。**

### もう一歩: 実務に近い形

#### 例6-5: 直して、4つを順に通す

3つの問題を直した `src\lib.rs` です。

```rust
/// ノートの題の文字数を数える
pub fn title_length(title: &str) -> usize {
    title.chars().count()
}

/// 題が空（空白だけ）かどうか
pub fn is_blank(title: &str) -> bool {
    title.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_title() {
        assert_eq!(title_length("買い物"), 3);
    }

    #[test]
    fn blank_title() {
        assert!(is_blank("   "));
        assert!(!is_blank("メモ"));
    }
}
```

```powershell
cargo fmt --check; 'fmt: ' + $LASTEXITCODE
cargo clippy -q -- -D warnings; 'clippy: ' + $LASTEXITCODE
cargo test -q; 'test: ' + $LASTEXITCODE
```

**▼ 実行結果**

```text
fmt: 0
clippy: 0

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

test: 0
```

- 3つとも終了コード 0 になりました。`-q`（quiet＝静かに）を付けると、`Compiling` などの途中の行が出ず、結果だけが表示されます。テストは2つとも通り（`..` の1つの点が1つのテスト）、2つ目の `running 0 tests` は、説明の中の例（ドキュメントテスト）の分です。
- **順番:** 速くて安い物から先に通します。体裁（fmt）は一瞬で、clippy は check の分の時間、test は作って動かす分の時間がかかります。

#### 例6-6: 大きなプロジェクトで、check と build の時間を比べる

RustNote で、何も作っていない（`target` が空の）状態から、`cargo check` と `cargo build` の時間を測りました（この本の PC のいつもの Rust。測った時間は、PC によって違います）。

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 59.49s
cargo check: 61.3 s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 04s
cargo build: 64.9 s
rustnote.exe in check target: 0
rustnote.exe in build target: 2
```

続けて、`src\main.rs` を保存し直して（中身は変えずに）、もう一度それぞれをかけました。

```text
--- cargo check（src\main.rs を保存し直したあと）
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
--- cargo build（src\main.rs を保存し直したあと）
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.40s
```

- 最初は、check も build も約 1 分でした。どちらも、使う部品（クレート）を全部用意する時間がほとんどだからです。check の `target` には `rustnote.exe` が1つもできず、build の方には2つ（`target\debug` と、途中の置き場の `deps`）できました。
- 自分のコードを少し直したあとは、check が 0.95 秒、build が 3.40 秒でした。書きながら確かめるときは check（VS Code の rust-analyzer も、保存のたびに check か clippy をかけています。UNIT 03）、動かすときに build や run、と使い分けます。

### よくあるエラーと読み方

#### エラー: `'cargo-fmt.exe' is not installed`（部品が入っていない）

rustup で、いちばん小さい組み合わせ（`--profile minimal`）で入れた Rust には、rustfmt と clippy が入っていません。この本で、1つ前の版の `1.99.0` をそうやって入れ、`cargo +1.99.0 fmt --check` を打ったときです。

```powershell
rustup toolchain install 1.99.0 --profile minimal
cargo +1.99.0 fmt --check
cargo +1.99.0 clippy
```

**▼ 実行結果**

```text
info: syncing channel updates for 1.99.0-aarch64-pc-windows-msvc
info: latest update on 2026-10-01 for version 1.99.0 (b940084d7 2026-09-28)
info: downloading 3 components

  1.99.0-aarch64-pc-windows-msvc installed - rustc 1.99.0 (b940084d7 2026-09-28)

info: checking for self-update (current version: 1.29.2)
```

```text
error: 'cargo-fmt.exe' is not installed for the toolchain '1.99.0-aarch64-pc-windows-msvc'.
help: run `rustup component add --toolchain 1.99.0-aarch64-pc-windows-msvc rustfmt` to install it
```

```text
error: 'cargo-clippy.exe' is not installed for the toolchain '1.99.0-aarch64-pc-windows-msvc'.
help: run `rustup component add --toolchain 1.99.0-aarch64-pc-windows-msvc clippy` to install it
```

- **何が起きたか:** `downloading 3 components`（rustc・cargo・標準ライブラリだけ）で、整形の `cargo-fmt.exe` と、注意の `cargo-clippy.exe` がありませんでした。`cargo +1.99.0` の `+1.99.0` は、「この版で動かす」という指定です（UNIT 04 の図）。
- **直し方:** `help:` のとおり、`rustup component add` で足します。

```powershell
rustup component add rustfmt clippy --toolchain 1.99.0
cargo +1.99.0 fmt --check
```

**▼ 実行結果**

```text
info: downloading 2 components
```

```text

```

### RustNote ではここで使う

- RustNote の `rust-toolchain.toml` には `components = ["rustfmt", "clippy"]` と書かれています（UNIT 01）。RustNote のフォルダで使う 1.99.1 には、この2つが必ずそろうので、このエラーにはなりません。
- RustNote の CI の見本（`ci.yml`）は、`cargo fmt --check`・`cargo clippy --all-targets --locked -- -D warnings`・`cargo test --locked` を順に動かします（第11巻）。clippy は check の確かめも一緒にするので、check は別には動かしていません。
- VS Code の設定で `rust-analyzer.check.command` を `clippy` にしたのは（UNIT 03）、保存のたびに、この赤ペンを見るためです。

### 練習問題

**問1:** `cargo check` が通ったのに、`cargo test` が失敗することはあるでしょうか。例を挙げて答えてください。

**問2:** `cargo fmt` をかけたら、コードの意味が変わってしまうことはあるでしょうか。

**問3:** 例6-3の2つの注意のうち、1つだけを「このプロジェクトでは気にしない」ことにしたいとき、どうすればよいでしょうか。注意の出力の中に、手がかりがあります。

### 解答と解説

**問1の解答:**

- あります。例6-4の `title.len()` がそうです。型も文法も正しいので check は通りますが、文字数ではなくバイト数を返すので、テストで期待した 3 にならず、失敗しました。

**問2の解答:**

- 変わりません。rustfmt は、字下げ・空白・改行の形だけを整える道具です。例6-2でも、`return title.len();` という中身は、そのまま残っていました。書き方そのものを良くするのは clippy の仕事です。

**問3の解答:**

- `-D warnings` のときの出力に、`help: to override `-D warnings` add `#[allow(clippy::needless_return)]`` と書かれていました。その関数の上に `#[allow(clippy::needless_return)]` を書くと、そこでは、その注意を出さなくなります。
- ただし、注意には理由があるので、まずは直す方を考えます。気にしないことにするなら、なぜかをコメントに書いておきます。

### まとめ

- `cargo check`（型と文法）・`cargo fmt`（体裁）・`cargo clippy`（書き方の注意）・`cargo test`（答え合わせ）は、見ている物が違う。3つの問題を入れたコードも、check は通った。
- `cargo fmt --check` は差分を見せて終了コード 1、`cargo clippy -- -D warnings` は注意をエラーにして終了コード 101、`cargo test` は `left`（実際）と `right`（期待）を見せて失敗する。
- RustNote では、何も無い状態からは check も build も約 1 分。少し直したあとは check 0.95 秒・build 3.40 秒。書きながらは check、動かすときに build。
- rustfmt・clippy が無いと `'cargo-fmt.exe' is not installed`。`rustup component add rustfmt clippy` で足す。
