## UNIT 09: Result・エラー伝播・?演算子

### この単元のゴール

- `Result<T, E>`（`Ok(値)` か `Err(理由)`）で「失敗するかもしれない処理」を表し、`match` で扱える
- `?` 演算子で、失敗をその場で呼び出し元へ返す（**エラー伝播**）書き方ができる
- 失敗の種類を enum で作り、`Display` で利用者向けのメッセージを付けられる
- `map_err` / `ok_or` でエラーの型をそろえ、`main` から `Result` を返せる。`unwrap` が panic する理由を説明できる

### たとえ話でつかむ

- **Result** は「**注文の結果のお知らせ**」です。「ご注文の品をお届けします（`Ok(商品)`）」か「在庫切れのためお届けできません（`Err(理由)`）」のどちらかが必ず届きます。UNIT 08 の Option（あるか・ないか）と違い、**失敗したときに理由が付いてくる**のがポイントです。
- **`?` 演算子**は「**問題が起きたら、すぐ上司に報告して作業を打ち切る**」という職場のルールです。各作業の後に「うまくいった？」と確かめ、だめならその報告書（`Err`）を持って上司（呼び出し元）のところへ戻ります。うまくいったら、そのまま次の作業へ進みます。
- **`unwrap()`** は「失敗するはずがない」と決めつけて作業を続けることです。もし失敗していたら、その場で工場の非常ベルが鳴ります（panic）。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">"42".parse::&lt;i32&gt;() の結果は Result&lt;i32, ParseIntError&gt;</text>
<rect x="40" y="46" width="300" height="120" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="190" y="74" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">Ok(42)（成功）</text>
<rect x="140" y="90" width="100" height="44" rx="6" fill="#ffffff" stroke="#047857"/>
<text x="190" y="118" text-anchor="middle" font-size="18" font-weight="700" fill="#0f172a">42</text>
<text x="190" y="154" text-anchor="middle" font-size="12" fill="#047857">中身: 成功した値（i32）</text>
<rect x="380" y="46" width="300" height="120" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="530" y="74" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">Err(e)（失敗）</text>
<rect x="400" y="90" width="260" height="44" rx="6" fill="#ffffff" stroke="#c2410c"/>
<text x="530" y="117" text-anchor="middle" font-size="13" fill="#0f172a">invalid digit found in string</text>
<text x="530" y="154" text-anchor="middle" font-size="12" fill="#c2410c">中身: 失敗の理由（ParseIntError）</text>
<text x="360" y="196" text-anchor="middle" font-size="13" fill="#334155">Option は「あるか・ないか」、Result は「成功の値か・失敗の理由か」</text>
<text x="360" y="218" text-anchor="middle" font-size="12" fill="#64748b">どちらも enum なので、match で両方の場合を書く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-1: Result は成功（Ok）と失敗（Err）のどちらか。失敗にも中身（理由）がある</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-12" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-02-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="360" y="22" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">fn add_strings(a, b) -&gt; Result&lt;i32, ParseIntError&gt; の中の流れ</text>
<rect x="30" y="44" width="270" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="165" y="71" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#1e40af">let x = a.parse()?;</text>
<rect x="30" y="128" width="270" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="165" y="155" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#1e40af">let y = b.parse()?;</text>
<rect x="30" y="212" width="270" height="44" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="165" y="239" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#047857">Ok(x + y)</text>
<line x1="165" y1="88" x2="165" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-12)"/>
<text x="175" y="112" font-size="12" fill="#047857">Ok なら中身を x へ</text>
<line x1="165" y1="172" x2="165" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-12)"/>
<text x="175" y="196" font-size="12" fill="#047857">Ok なら中身を y へ</text>
<path d="M300,66 L420,66 L420,140" fill="none" stroke="#c2410c" stroke-width="2"/>
<path d="M300,150 L420,150" fill="none" stroke="#c2410c" stroke-width="2"/>
<line x1="420" y1="140" x2="420" y2="226" stroke="#c2410c" stroke-width="2"/>
<line x1="420" y1="232" x2="476" y2="232" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-02-13)"/>
<path d="M420,226 L420,232" fill="none" stroke="#c2410c" stroke-width="2"/>
<text x="330" y="58" font-size="12" fill="#c2410c">Err なら</text>
<text x="330" y="142" font-size="12" fill="#c2410c">Err なら</text>
<rect x="480" y="190" width="220" height="84" rx="10" fill="#fff7ed" stroke="#c2410c" stroke-width="2"/>
<text x="590" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">return Err(e);</text>
<text x="590" y="240" text-anchor="middle" font-size="12" fill="#334155">残りの行は実行せず</text>
<text x="590" y="260" text-anchor="middle" font-size="12" fill="#334155">呼び出し元へ失敗を渡す</text>
<rect x="480" y="44" width="220" height="110" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="590" y="70" text-anchor="middle" font-size="12" fill="#334155">? 1文字で、次の4行と同じ意味</text>
<text x="500" y="94" font-size="12" font-family="Consolas, monospace" fill="#0f172a">match a.parse() {</text>
<text x="512" y="112" font-size="12" font-family="Consolas, monospace" fill="#0f172a">Ok(v) =&gt; v,</text>
<text x="512" y="130" font-size="12" font-family="Consolas, monospace" fill="#0f172a">Err(e) =&gt; return Err(e),</text>
<text x="500" y="148" font-size="12" font-family="Consolas, monospace" fill="#0f172a">}</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-2: ? は「成功なら中身を取り出して次へ、失敗ならすぐ return」。失敗の流れが1か所にまとまる</div>
</div>

### まず動かす最小コード

#### 例1: parse の Result を match で扱う

> **▼ このコードがやること（先に日本語で）:** 4つの文字列を整数に変換し、成功したら数を、失敗したら理由を表示します。最後に、失敗の中身をプログラマー向けの形でも表示します。

```rust
fn main() {
    for raw in ["42", "-7", "abc", ""] {
        // parse は Result を返す。成功なら Ok(値)、失敗なら Err(理由)
        let result: Result<i32, std::num::ParseIntError> = raw.parse::<i32>();
        match result {
            Ok(n) => println!("{raw:?} → 数 {n}"),
            // {e} で、失敗の理由を人向けの文で表示する
            Err(e) => println!("{raw:?} → 失敗: {e}"),
        }
    }
    // {:?} だと、プログラマー向けの詳しい形で表示される
    println!("{:?}", "abc".parse::<i32>());
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
"42" → 数 42
"-7" → 数 -7
"abc" → 失敗: invalid digit found in string
"" → 失敗: cannot parse integer from empty string
Err(ParseIntError { kind: InvalidDigit })
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `Result<T, E>`

```rust
let result: Result<i32, std::num::ParseIntError> = raw.parse::<i32>();
```

- `Result<i32, ParseIntError>` は「成功なら `i32`、失敗なら `ParseIntError`（整数の解析エラー）」という型です。標準ライブラリで次のように定義された enum です。

```text
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `std::num::ParseIntError` は「標準ライブラリ（`std`）の `num` モジュールにある `ParseIntError`」という、型の住所付きの名前です（モジュールは UNIT 10）。

##### 解説2: `{e}` と `{:?}` の違い

- `{e}` は**利用者向け**の表示（`Display`）で、`invalid digit found in string`（数字でない文字があります）、`cannot parse integer from empty string`（空の文字列は整数にできません）のような文になります。
- `{:?}` は**プログラマー向け**の表示（`Debug`）で、`Err(ParseIntError { kind: InvalidDigit })` のように型の中身がそのまま見えます。ログや調査に向いています。

#### 例2: ? 演算子で失敗を呼び出し元へ返す

> **▼ このコードがやること（先に日本語で）:** 2つの文字列を数に変換して足す関数を作ります。どちらかの変換に失敗したら、その時点で失敗を返します。

```rust
use std::num::ParseIntError;

// 2つの文字列を数にして足す。どちらかが数でなければ、その時点で Err を返す
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // ? は「Ok なら中身を取り出して続ける。Err ならその Err をすぐ返す」
    let x: i32 = a.trim().parse()?;
    let y: i32 = b.trim().parse()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", " 32 "));
    println!("{:?}", add_strings("10", "abc"));
}
```

**▼ 実行結果**

```text
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `?` がしていること

```rust
let x: i32 = a.trim().parse()?;
```

- 末尾の `?` は、「`Ok(v)` なら中身の `v` を取り出す。`Err(e)` なら、関数から `Err(e)` をすぐに返す」という意味です（図9-2）。
- `?` を使わずに書くと、次のようになります（実行結果は同じです）。

```rust
use std::num::ParseIntError;

// ? を使わずに書いた add_strings（中身は同じ動き）
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = match a.trim().parse() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let y: i32 = match b.trim().parse() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", " 32 "));
    println!("{:?}", add_strings("10", "abc"));
}
```

- 失敗のたびに `match` を書くと、本来の処理（足し算）が埋もれてしまいます。このコードに `cargo clippy` をかけると、まさに「`?` に置き換えられます」と提案されます。`?` なら**正常な流れだけを上から読める**ようになります。

##### 解説2: `use` と最後の `Ok(...)`

- 1行目の `use std::num::ParseIntError;` は、長い名前を `ParseIntError` だけで書けるようにする宣言です（UNIT 10）。
- 関数の最後は、成功の値を `Ok(x + y)` で包んで返します。`x + y` だけだと型が合いません（UNIT 08 の `Some` の包み忘れと同じです）。

#### 例3: 自分のエラー型を作る

> **▼ このコードがやること（先に日本語で）:** タイトルの検証で起こる失敗「空」「長すぎる」を enum で表し、利用者に見せる日本語のメッセージを付けます。3つのタイトルを検証して、結果を表示します。

```rust
use std::fmt;

const MAX_TITLE_CHARS: usize = 80;

// タイトルの検証で起こりうる失敗を enum で列挙する
#[derive(Debug, PartialEq)]
enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

// 利用者に見せるメッセージを決める（{} で表示できるようになる）
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { max, actual } => {
                write!(
                    f,
                    "タイトルは{max}文字以内にしてください（現在{actual}文字）"
                )
            }
        }
    }
}

fn check_title(raw: &str) -> Result<String, TitleError> {
    let title = raw.trim();
    let count = title.chars().count();
    if count == 0 {
        return Err(TitleError::Empty);
    }
    if count > MAX_TITLE_CHARS {
        return Err(TitleError::TooLong {
            max: MAX_TITLE_CHARS,
            actual: count,
        });
    }
    Ok(title.to_string())
}

fn main() {
    let long = "あ".repeat(81);
    for raw in ["  牛乳を買う ", "   ", long.as_str()] {
        match check_title(raw) {
            Ok(title) => println!("OK: {title}"),
            Err(e) => println!("NG: {e}  /  {e:?}"),
        }
    }
    // PartialEq を derive したので、結果を == で比べられる（テストで便利）
    println!("{}", check_title("") == Err(TitleError::Empty));
}
```

**▼ 実行結果**

```text
OK: 牛乳を買う
NG: タイトルを入力してください  /  Empty
NG: タイトルは80文字以内にしてください（現在81文字）  /  TooLong { max: 80, actual: 81 }
true
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 失敗の種類を enum で列挙する

```rust
enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}
```

- 失敗の理由を文字列で返すこともできますが、enum にすると、呼ぶ側が `match` で**理由ごとに処理を変えられ**ます。「長すぎる」場合には、上限と実際の文字数も一緒に渡せます。

##### 解説2: `impl fmt::Display` で利用者向けの表示を決める

```rust
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            ...
        }
    }
}
```

- `Display` は「`{}` で表示するときの方法」を決める**トレイト**（型に共通の機能を付ける仕組み。第4巻で詳しく学びます）です。今は「この形を書けば `{e}` で日本語のメッセージが出る」と覚えれば十分です。
- `write!(f, ...)` は `format!` と同じ書き方で、表示する文を書き込みます。
- `Debug`（`{e:?}`）は derive で自動、`Display`（`{e}`）は人に見せる文なので自分で書く、という分担です。

##### 解説3: `PartialEq` でエラーを比べる

- `check_title("") == Err(TitleError::Empty)` のように、結果を `==` で比べられます。テスト（第4巻・第11巻）で「この入力ならこのエラーになる」と確かめるのに使います。

#### 例4: main から Result を返す

> **▼ このコードがやること（先に日本語で）:** `main` の中で `?` を使います。1つ目の変換は成功し、2つ目で失敗するので、そこでプログラムがエラーを表示して終わります。

```rust
use std::num::ParseIntError;

// main も Result を返せる。Err で終わると、エラーを表示して「失敗」として終了する
fn main() -> Result<(), ParseIntError> {
    let count: u32 = "12".parse()?;
    println!("count = {count}");
    let broken: u32 = "12a".parse()?;
    println!("ここには来ない: {broken}");
    Ok(())
}
```

**▼ 実行結果**

```text
count = 12
Error: ParseIntError { kind: InvalidDigit }
error: process didn't exit successfully: `target\debug\basics.exe` (exit code: 1)
```

- `fn main() -> Result<(), ParseIntError>` と書くと、`main` の中でも `?` が使えます。成功のときは最後に `Ok(())`（中身のない成功）を返します。
- `main` が `Err` を返すと、Rust は `Error: ` に続けてエラーの `Debug` 表示を出し、**終了コード 1**（失敗）で終わります。最後の行は cargo が「プログラムが失敗で終わった」と知らせているものです。panic（終了コード 101）とは違い、**想定どおりの失敗として**終わっています。

### もう一歩: 実務に近い形

設定値の読み込みでは、「値がない（Option）」「数でない（ParseIntError）」「範囲外（TryFromIntError）」と、**種類の違う失敗**が混ざります。`?` は関数の戻り値と同じエラー型でないと使えないので、`ok_or` と `map_err` で自分のエラー型にそろえます。

#### 種類の違う失敗を1つのエラー型にまとめる

> **▼ このコードがやること（先に日本語で）:** ポート番号の設定を検証します。設定がない・数でない・65535 を超える、の3種類の失敗をそれぞれ自分のエラー型で表し、種類ごとにメッセージを出し分けます。

```rust
// 設定の読み込みで起こりうる失敗
#[derive(Debug)]
enum ConfigError {
    Missing,
    NotANumber(String),
    OutOfRange(u32),
}

// ポート番号の文字列（ないかもしれない）を検証して u16 にする
fn parse_port(raw: Option<&str>) -> Result<u16, ConfigError> {
    // ok_or: None を Err(...) に変える。? で、なければすぐ返す
    let text = raw.ok_or(ConfigError::Missing)?;
    // map_err: ParseIntError を、自分のエラー型 ConfigError に変える
    let value: u32 = text
        .trim()
        .parse()
        .map_err(|_| ConfigError::NotANumber(text.to_string()))?;
    // u16 に入らない値（65536 以上）を弾く
    u16::try_from(value).map_err(|_| ConfigError::OutOfRange(value))
}

fn main() {
    for raw in [Some("8080"), None, Some("abc"), Some("70000")] {
        // エラーの種類ごとに、中のデータを使ってメッセージを作る
        match parse_port(raw) {
            Ok(port) => println!("{raw:?} → ポート {port}"),
            Err(ConfigError::Missing) => println!("{raw:?} → 設定がありません"),
            Err(ConfigError::NotANumber(text)) => {
                println!("{raw:?} → 「{text}」は数ではありません")
            }
            Err(ConfigError::OutOfRange(n)) => {
                println!("{raw:?} → {n} はポート番号の範囲外です")
            }
        }
    }
}
```

**▼ 実行結果**

```text
Some("8080") → ポート 8080
None → 設定がありません
Some("abc") → 「abc」は数ではありません
Some("70000") → 70000 はポート番号の範囲外です
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `ok_or` で Option を Result にする

```rust
let text = raw.ok_or(ConfigError::Missing)?;
```

- `ok_or(エラー)` は、`Some(v)` を `Ok(v)` に、`None` を `Err(エラー)` に変えます。その後ろの `?` で、なければすぐに `Err(ConfigError::Missing)` を返します。

##### 解説2: `map_err` でエラーの型を変える

```rust
let value: u32 = text
    .trim()
    .parse()
    .map_err(|_| ConfigError::NotANumber(text.to_string()))?;
```

- `parse()` の失敗は `ParseIntError` ですが、この関数のエラー型は `ConfigError` です。`map_err` は**失敗のときだけ中身を変換する**メソッドで（Option の `map` の、エラー版です）、`|_| ...` は「元のエラーは使わずに、`ConfigError::NotANumber(...)` を作る」というクロージャです。
- 型をそろえてから `?` を使います。そろえ忘れると、下の「よくあるエラー」の E0277 になります。

##### 解説3: 最後の式も Result

```rust
u16::try_from(value).map_err(|_| ConfigError::OutOfRange(value))
```

- `try_from` の結果（`Result<u16, TryFromIntError>`）を `map_err` で `Result<u16, ConfigError>` に変え、それをそのまま戻り値にしています。最後なので `?` も `Ok(...)` も要りません。

> **ヒント:** エラー型に `From` という変換を実装しておくと、`map_err` を書かなくても `?` が自動で変換してくれます。エラー型の設計（`thiserror` などのクレートも含む）は第4巻で学びます。

### よくあるエラーと読み方

#### エラー1: Err を unwrap して panic

```rust
fn main() {
    let raw = "abc";
    let count: u32 = raw.parse().unwrap();
    println!("{count}");
}
```

**▼ 実行結果**

```text
thread 'main' (49636) panicked at src\main.rs:3:34:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\basics.exe` (exit code: 101)
```

- **読み方:** `called Result::unwrap() on an Err value: ParseIntError { kind: InvalidDigit }`（`Err` の値に対して `unwrap()` が呼ばれました。中身は「数字でない文字」のエラーでした）。
- **何が起きたか:** `unwrap()` は Ok なら中身を返し、Err なら panic します。外から来た文字列は数とは限らないので、ここで `unwrap()` を使うのは危険です。
- **直し方:** `match` で失敗を扱うか、関数の戻り値を `Result` にして `?` で呼び出し元に任せます。値がなくても進めてよいなら `unwrap_or(既定値)` も使えます（UNIT 01 の設定値の例）。

#### エラー2: E0277 Result を返さない関数で ? を使った

```rust
fn main() {
    let count: u32 = "12".parse()?;
    println!("{count}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src\main.rs:2:34
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let count: u32 = "12".parse()?;
  |                                  ^ cannot use the `?` operator in a function that returns `()`
  |
help: consider adding return type
  |
1 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
2 |     let count: u32 = "12".parse()?;
3 |     println!("{count}");
4 +     Ok(())
  |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`?` 演算子は、`Result` か `Option` を返す関数の中でしか使えません」。`main` は `()` を返す関数なので、失敗を返す先がありません。
- **直し方:** `help` のように `main` の戻り値を `Result` にします。`Box<dyn std::error::Error>` は「どんな種類のエラーでも入る箱」で、いろいろなエラーが混ざる `main` でよく使われます（第4巻）。ここでは `Result<(), ParseIntError>` でも十分です（練習問題3）。

#### エラー3: E0277 ? がエラーの型を変換できない

```rust
#[derive(Debug)]
enum ConfigError {
    Missing,
}

fn parse_port(raw: Option<&str>) -> Result<u16, ConfigError> {
    let text = raw.ok_or(ConfigError::Missing)?;
    let port = text.parse::<u16>()?;
    Ok(port)
}

fn main() {
    println!("{:?}", parse_port(Some("8080")));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0277]: `?` couldn't convert the error to `ConfigError`
 --> src\main.rs:8:35
  |
6 | fn parse_port(raw: Option<&str>) -> Result<u16, ConfigError> {
  |                                     ------------------------ expected `ConfigError` because of this
7 |     let text = raw.ok_or(ConfigError::Missing)?;
8 |     let port = text.parse::<u16>()?;
  |                     --------------^ the trait `From<ParseIntError>` is not implemented for `ConfigError`
  |                     |
  |                     this can't be annotated with `?` because it has type `Result<_, ParseIntError>`
  |
note: `ConfigError` needs to implement `From<ParseIntError>`
 --> src\main.rs:2:1
  |
2 | enum ConfigError {
  | ^^^^^^^^^^^^^^^^
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

For more information about this error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`?` がエラーを `ConfigError` に変換できません」。関数は `ConfigError` を返すと約束しているのに、`parse` の失敗は `ParseIntError` です。`note` は「`?` は `From` を使ってエラーを暗黙に変換する」と説明しています。
- **直し方:** `.map_err(|_| ConfigError::NotANumber(text.to_string()))?` のように自分で変換します（「もう一歩」のコード）。または `ConfigError` に `From<ParseIntError>` を実装します（第4巻）。

#### 警告: Result を使わずに捨てた

```rust
fn check(raw: &str) -> Result<u32, std::num::ParseIntError> {
    raw.parse()
}

fn main() {
    check("abc");
    println!("チェックしました");
}
```

```text
warning: unused `Result` that must be used
 --> src\main.rs:6:5
  |
6 |     check("abc");
  |     ^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
6 |     let _ = check("abc");
  |     +++++++

warning: `basics` (bin "basics") generated 1 warning
```

- **読み方:** 「使わなければならない `Result` が使われていません」。`note: this Result may be an Err variant, which should be handled`（この Result は Err かもしれないので、処理すべきです）。警告なので実行はされ、「チェックしました」と表示されます。
- **何が起きたか:** 検証に失敗していても、結果を見ずに先へ進んでいます。Rust は `Result` を**無視すると警告する**ので、エラー処理の書き忘れに気づけます。
- **直し方:** `match` や `?` で結果を扱います。本当に結果が要らないときだけ、`let _ = check("abc");` と書いて「わざと捨てる」ことを明示します。

> **つまずきポイント:** 「`unwrap()` を書けば警告もエラーも消える」ので多用しがちです。`unwrap()` は**エラーを解決したのではなく、失敗したら止まるようにしただけ**です。使ってよいのは、テストのコードや、「ここで失敗するならプログラムの前提が壊れている」と言い切れる場所だけにしましょう。

### RustNote ではここで使う

いよいよ RustNote の `NoteTitle::parse` です。UNIT 01 の定数、UNIT 05 の文字数、UNIT 07 の newtype、この UNIT の Result を組み合わせて、「**1〜80文字に収まったタイトルしか入らない型**」を作ります。いったん `NoteTitle` を作れたら、その後の処理は「タイトルが正しいか」を確かめ直す必要がありません。

> **▼ このコードがやること（先に日本語で）:** フォームから届いたタイトルと本文から「新しいノート」を作ります。タイトルが正しければ作成し、空や81文字のときは理由付きの入力エラーにします。

```rust
use std::fmt;

const MAX_TITLE_CHARS: usize = 80;

#[derive(Debug, PartialEq)]
enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { max, actual } => {
                write!(
                    f,
                    "タイトルは{max}文字以内にしてください（現在{actual}文字）"
                )
            }
        }
    }
}

// 検証に通ったタイトルだけを入れる型
#[derive(Debug)]
struct NoteTitle(String);

impl NoteTitle {
    // 1〜80 文字に収まっていれば NoteTitle を作る。そうでなければ理由を返す
    fn parse(raw: &str) -> Result<Self, TitleError> {
        let title = raw.trim();
        let count = title.chars().count();
        if count == 0 {
            return Err(TitleError::Empty);
        }
        if count > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: count,
            });
        }
        Ok(Self(title.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

// フォームから届いた「新しいノート」を検証済みの形で持つ
struct NewNote {
    title: NoteTitle,
    body: String,
}

impl NewNote {
    fn from_form(title: &str, body: &str) -> Result<Self, TitleError> {
        // タイトルが不正なら、? でその Err をそのまま返す
        let title = NoteTitle::parse(title)?;
        Ok(Self {
            title,
            body: body.trim().to_string(),
        })
    }
}

fn main() {
    let long = "長".repeat(81);
    let forms = [
        ("  牛乳を買う ", "低脂肪のもの"),
        ("", "本文だけ"),
        (long.as_str(), ""),
    ];
    for (title, body) in forms {
        match NewNote::from_form(title, body) {
            Ok(note) => println!(
                "作成: 「{}」（本文 {} 文字）",
                note.title.as_str(),
                note.body.chars().count()
            ),
            Err(e) => println!("入力エラー: {e}"),
        }
    }
}
```

**▼ 実行結果**

```text
作成: 「牛乳を買う」（本文 6 文字）
入力エラー: タイトルを入力してください
入力エラー: タイトルは80文字以内にしてください（現在81文字）
```

- `NoteTitle::parse` が返すのは `Result<NoteTitle, TitleError>` です。成功したときだけ `NoteTitle` が手に入るので、**正しくないタイトルを持った `NoteTitle` は存在しえません**。このように「検証済みであること」を型で表す考え方を、本書では何度も使います。
- `NewNote::from_form` の中の `NoteTitle::parse(title)?` は、タイトルが不正ならその `TitleError` をそのまま返します。RustNote の画面では、このメッセージ（「タイトルを入力してください」など）をフォームの上に表示します。
- 今のままでは、外から `NoteTitle(String::new())` と直接書けば空のタイトルも作れてしまいます。UNIT 10 のモジュールと `pub` で、中身を外から触れないようにします。

### 練習問題

**問題1.** 年齢の文字列を検証する `fn parse_age(raw: &str) -> Result<u8, String>` を書いてください。数でなければ「「abc」は 0〜255 の数ではありません」、151 以上なら「200 歳は大きすぎます」のようなメッセージを `Err` で返します。`map_err` と `?` を使いましょう。

**問題2.** 「時間」と「分」の文字列から合計の分数を返す `fn total_minutes(hours: &str, minutes: &str) -> Result<u32, ParseIntError>` を、`?` を使って書いてください。

**問題3.** 「よくあるエラー」のエラー2を、`main` の戻り値を `Result<(), ParseIntError>` にして直してください。

### 解答と解説

**解答1.**

```rust
// 年齢の文字列を検証して u8 にする
fn parse_age(raw: &str) -> Result<u8, String> {
    let age: u8 = raw
        .trim()
        .parse()
        .map_err(|_| format!("「{raw}」は 0〜255 の数ではありません"))?;
    if age > 150 {
        return Err(format!("{age} 歳は大きすぎます"));
    }
    Ok(age)
}

fn main() {
    for raw in ["20", "abc", "200", "300"] {
        match parse_age(raw) {
            Ok(age) => println!("OK: {age} 歳"),
            Err(message) => println!("NG: {message}"),
        }
    }
}
```

**▼ 実行結果**

```text
OK: 20 歳
NG: 「abc」は 0〜255 の数ではありません
NG: 200 歳は大きすぎます
NG: 「300」は 0〜255 の数ではありません
```

- 300 は `u8`（0〜255）に入らないので、`parse` の時点で失敗します。200 は `u8` には入るので、2つ目のチェック（151 以上）で弾かれます。型による検査と、規則による検査の2段構えです。

**解答2.**

```rust
use std::num::ParseIntError;

// 「時間」と「分」の文字列から、合計の分数を求める
fn total_minutes(hours: &str, minutes: &str) -> Result<u32, ParseIntError> {
    let h: u32 = hours.parse()?;
    let m: u32 = minutes.parse()?;
    Ok(h * 60 + m)
}

fn main() {
    println!("{:?}", total_minutes("2", "30"));
    println!("{:?}", total_minutes("2", "半"));
}
```

**▼ 実行結果**

```text
Ok(150)
Err(ParseIntError { kind: InvalidDigit })
```

**解答3.**

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let count: u32 = "12".parse()?;
    println!("{count}");
    Ok(())
}
```

**▼ 実行結果**

```text
12
```

- `main` の最後に `Ok(())` を返すのを忘れないようにしましょう。

### まとめ

- `Result<T, E>` は `Ok(成功の値)` か `Err(失敗の理由)`。`{e}` は利用者向け、`{e:?}` はプログラマー向けの表示。
- `?` は「Ok なら中身を取り出し、Err ならすぐ return」。使える関数の戻り値は Result（か Option）で、エラーの型は `map_err` / `ok_or` でそろえる。
- 失敗の種類は enum で列挙して `Display` でメッセージを付ける。`unwrap()` は失敗で panic するので、外から来たデータには使わない。
