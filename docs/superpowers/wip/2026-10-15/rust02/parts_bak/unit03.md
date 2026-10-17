## UNIT 03: 関数・文・式・戻り値

### この単元のゴール

- `fn` で関数を定義し、引数の型と戻り値の型を書ける
- **文**（値を持たない命令）と**式**（値になるもの）の違いを説明できる
- 「最後の式にセミコロンを付けない」と戻り値になる理由を説明でき、セミコロンの付けすぎ・付け忘れのエラーを直せる
- `return` で早めに関数を抜ける書き方（早期リターン）を使える

### たとえ話でつかむ

関数は「**自動販売機**」です。

- お金を入れる口（**引数**）には形が決まっていて、硬貨の口に紙幣は入りません。Rust の関数も、引数ごとに**型**が決まっています。
- ボタンを押すと、中の仕組み（**本体**）が動いて、商品（**戻り値**）が出てきます。何が出てくるかも、型として前もって決まっています。
- 自販機の外から見えるのは「何を入れると何が出てくるか」だけです。中身を知らなくても使えるように、関数の1行目（**シグネチャ**）だけ読めば使い方が分かるようになっています。

もう1つ、「文」と「式」のたとえです。

- **式**は「答えが出る計算」です。`3 + 4` は 7 という答え（値）になります。
- **文**は「作業の指示」です。`let x = 7;` は「x を作れ」という指示で、それ自体は答えを持ちません。
- 式の後ろに `;`（セミコロン）を付けると、その式は文になり、**答えは捨てられます**。セミコロンは「答えをゴミ箱に入れる」記号だと思ってください。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">関数の1行目（シグネチャ）を部品に分ける</text>
<rect x="40" y="46" width="50" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="65" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#0f172a">fn</text>
<rect x="100" y="46" width="62" height="40" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="131" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#c2410c">add</text>
<rect x="172" y="46" width="200" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="272" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#1e40af">(a: i32, b: i32)</text>
<rect x="382" y="46" width="90" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="427" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#047857">-&gt; i32</text>
<rect x="482" y="46" width="200" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="582" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#0f172a">{ a + b }</text>
<text x="65" y="108" text-anchor="middle" font-size="12" fill="#475569">関数を作る</text>
<text x="131" y="108" text-anchor="middle" font-size="12" fill="#c2410c">関数名</text>
<text x="272" y="108" text-anchor="middle" font-size="12" fill="#1e40af">引数（名前: 型）を , 区切り</text>
<text x="427" y="108" text-anchor="middle" font-size="12" fill="#047857">戻り値の型</text>
<text x="582" y="108" text-anchor="middle" font-size="12" fill="#475569">本体。最後の式が戻り値</text>
<line x1="20" y1="132" x2="700" y2="132" stroke="#e2e8f0"/>
<text x="360" y="158" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">自動販売機として見ると</text>
<rect x="60" y="180" width="150" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="135" y="204" text-anchor="middle" font-size="13" fill="#1e40af">入れるもの</text>
<text x="135" y="224" text-anchor="middle" font-size="14" font-weight="700" fill="#1e40af">3 と 4（i32）</text>
<rect x="285" y="172" width="150" height="72" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="204" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">add</text>
<text x="360" y="226" text-anchor="middle" font-size="12" fill="#c2410c">a + b を計算</text>
<rect x="510" y="180" width="150" height="56" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="585" y="204" text-anchor="middle" font-size="13" fill="#047857">出てくるもの</text>
<text x="585" y="224" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">7（i32）</text>
<line x1="210" y1="208" x2="280" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-03)"/>
<line x1="435" y1="208" x2="505" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-03)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-1: シグネチャを読めば「何を入れると何が出てくるか」が分かる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">最後の行が「式」のブロック</text>
<rect x="20" y="40" width="320" height="120" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="40" y="70" font-size="15" font-family="Consolas, monospace" fill="#0f172a">{</text>
<text x="60" y="98" font-size="15" font-family="Consolas, monospace" fill="#0f172a">let x = 3;</text>
<text x="200" y="98" font-size="12" fill="#64748b">← 文（値なし）</text>
<text x="60" y="126" font-size="15" font-family="Consolas, monospace" fill="#047857" font-weight="700">x + 1</text>
<text x="200" y="126" font-size="12" fill="#047857">← 式（値は 4）</text>
<text x="40" y="150" font-size="15" font-family="Consolas, monospace" fill="#0f172a">}</text>
<line x1="180" y1="162" x2="180" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-04)"/>
<rect x="90" y="200" width="180" height="40" rx="8" fill="#ffffff" stroke="#047857" stroke-width="2"/>
<text x="180" y="226" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">ブロックの値 = 4</text>
<text x="540" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">最後の行に ; を付けたブロック</text>
<rect x="380" y="40" width="320" height="120" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="400" y="70" font-size="15" font-family="Consolas, monospace" fill="#0f172a">{</text>
<text x="420" y="98" font-size="15" font-family="Consolas, monospace" fill="#0f172a">let x = 3;</text>
<text x="560" y="98" font-size="12" fill="#64748b">← 文（値なし）</text>
<text x="420" y="126" font-size="15" font-family="Consolas, monospace" fill="#c2410c" font-weight="700">x + 1;</text>
<text x="560" y="126" font-size="12" fill="#c2410c">← ; で値を捨てた</text>
<text x="400" y="150" font-size="15" font-family="Consolas, monospace" fill="#0f172a">}</text>
<line x1="540" y1="162" x2="540" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-04)"/>
<rect x="430" y="200" width="220" height="40" rx="8" fill="#ffffff" stroke="#c2410c" stroke-width="2"/>
<text x="540" y="226" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">ブロックの値 = ()（空）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-2: セミコロン1つで、ブロック（や関数）の値が 4 から () に変わる</div>
</div>

> **用語:** **()（ユニット）** … 「意味のある値はありません」を表す、中身が空の値です。型の名前も `()` です。何も返さない関数は、実は `()` を返しています。

### まず動かす最小コード

#### 例1: 関数を定義して呼び出す

> **▼ このコードがやること（先に日本語で）:** 2つの数を足して返す関数 `add` と、あいさつを表示するだけの関数 `greet` を作り、`main` から呼び出します。

```rust
// 2つの整数を受け取り、その合計を返す関数
fn add(a: i32, b: i32) -> i32 {
    // 最後の行にセミコロンを付けないと、その値が戻り値になる
    a + b
}

// 何も返さない関数（-> 戻り値の型 を書かない）
fn greet(name: &str) {
    println!("こんにちは、{name}さん");
}

fn main() {
    let total = add(3, 4);
    println!("3 + 4 = {total}");
    greet("Rust");
    // 関数の呼び出し結果を、そのまま別の関数の引数にしてもよい
    println!("入れ子: {}", add(add(1, 2), add(3, 4)));
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
3 + 4 = 7
こんにちは、Rustさん
入れ子: 10
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数と戻り値の型は省略できない

```rust
fn add(a: i32, b: i32) -> i32 {
```

- `a: i32` は「`a` という名前で i32 の値を受け取る」です。`let` と違い、**関数の引数の型は必ず書きます**。
- `-> i32` は「i32 の値を返す」です。矢印 `->` の後ろが戻り値の型です。
- 型を必ず書かせるのは、**関数の1行目だけで使い方が分かるようにする**ためです。中身を読まなくても、呼ぶ側は「i32 を2つ渡せば i32 が返る」と分かります。

##### 解説2: 最後の式が戻り値

```rust
    a + b
}
```

- 本体の最後の行 `a + b` には `;` がありません。これが「この値を返す」という意味です（図3-2 の左側と同じ）。
- 他の言語のように `return a + b;` と書くこともできますが、Rust では**最後の式で返す**のがふつうの書き方です。

##### 解説3: 何も返さない関数と `&str`

```rust
fn greet(name: &str) {
    println!("こんにちは、{name}さん");
}
```

- `->` を書かない関数は、意味のある値を返しません（正確には `()` を返します）。表示だけする関数などがこの形です。
- `&str` は「文字列を借りて読む」ための型です。関数で文字列を受け取るときの基本形で、UNIT 05 で詳しく学びます。

##### 解説4: 呼び出しは式

```rust
println!("入れ子: {}", add(add(1, 2), add(3, 4)));
```

- 関数の呼び出し `add(1, 2)` は値（3）になる**式**なので、そのまま別の関数の引数に書けます。内側から順に `add(3, 7)` = 10 と計算されます。

#### 例2: ブロックも if も式

> **▼ このコードがやること（先に日本語で）:** `{ }` で囲んだブロックの計算結果や、`if` の結果を変数に入れます。最後が文で終わるブロックの値が `()` になることも確かめます。

```rust
fn main() {
    // { } のブロックも「値になる式」。最後の行の値がブロック全体の値になる
    let area = {
        let width = 3;
        let height = 4;
        width * height
    };
    println!("面積: {area}");

    // if も式なので、結果をそのまま変数に入れられる
    let size = if area >= 10 { "大" } else { "小" };
    println!("大きさ: {size}");

    // 最後が「文」（セミコロン付き）のブロックは、() という「空の値」になる
    let nothing = {
        println!("ブロックの中で表示だけする");
    };
    println!("nothing の中身: {nothing:?}");
}
```

**▼ 実行結果**

```text
面積: 12
大きさ: 大
ブロックの中で表示だけする
nothing の中身: ()
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ブロックで「作業用の変数」を閉じ込める

```rust
let area = {
    let width = 3;
    let height = 4;
    width * height
};
```

- ブロックの中の `width` と `height` は、面積を計算するためだけの変数です。ブロックを出ると消えるので、外の名前とぶつかりません。
- 最後の `width * height` の値（12）がブロックの値になり、`area` に入ります。`};` の `;` は `let area = ...;` という**文の終わり**のセミコロンです。

##### 解説2: `if` 式

```rust
let size = if area >= 10 { "大" } else { "小" };
```

- `if` も値を返す式です。条件が成り立てば `{ "大" }` の値、そうでなければ `{ "小" }` の値になります。
- 他の言語の「三項演算子（`条件 ? A : B`）」の役目を、Rust では `if` 式が担います。

##### 解説3: `()` が返るブロック

- `println!(...);` は文なので、このブロックには最後の式がありません。値がないブロックは `()` になります。`{nothing:?}` で表示すると `()` と出ます。
- もし `x + 1;` のように**計算結果を捨てる文**を書くと、Rust は次の警告で「その値、使わなくていいの？」と教えてくれます。

```text
warning: unused arithmetic operation that must be used
 --> src\main.rs:4:9
  |
4 |         x + 1;
  |         ^^^^^ the arithmetic operation produces a value
```

#### 例3: return で早めに抜ける

> **▼ このコードがやること（先に日本語で）:** 合計点と人数から平均点を計算します。人数が 0 のときは割り算ができないので、計算する前に 0 を返して関数を終えます。

```rust
// 平均点を返す。人数が 0 のときは割り算できないので、先に 0 を返して終わる
fn average(total: u32, count: u32) -> u32 {
    if count == 0 {
        // return は「ここで関数を終えて、この値を返す」
        return 0;
    }
    total / count
}

fn main() {
    println!("{}", average(270, 3));
    println!("{}", average(0, 0));
}
```

**▼ 実行結果**

```text
90
0
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 早期リターン

- `return 0;` は、その場で関数を終えて 0 を返します。後ろの `total / count` は実行されません。だから 0 での割り算（panic）が起きません。
- このように「**おかしな場合を先に片付けて、残りで本来の処理をする**」書き方を**早期リターン**（ガード節）と呼びます。条件分岐が深く入れ子にならず、読みやすくなります。
- 早期リターンには `return` を、ふつうの戻り値には最後の式を使う、という使い分けが Rust の慣習です。

#### 例4: 何も返さない関数の戻り値

> **▼ このコードがやること（先に日本語で）:** 表示するだけの関数の戻り値をあえて変数に受け取り、中身が `()` であることを確かめます。

```rust
fn log(message: &str) {
    println!("[log] {message}");
}

fn main() {
    // 戻り値の型を書かない関数は、実は () という値を返している
    let result = log("起動しました");
    println!("log の戻り値: {result:?}");
}
```

**▼ 実行結果**

```text
[log] 起動しました
log の戻り値: ()
```

- `fn log(message: &str)` は `fn log(message: &str) -> ()` と同じ意味です。`-> ()` は省略するのが決まりです。
- Rust では「すべての関数が何かの値を返す」と考えると、文と式の仕組みがすっきり理解できます。

### もう一歩: 実務に近い形

実際のプログラムでは、1つの大きな関数に全部書くのではなく、**名前の付いた小さな関数**に分けて組み合わせます。関数名がそのまま説明になり、1つずつテストもしやすくなります。

#### 小さな関数を組み合わせて支払い額を計算する

> **▼ このコードがやること（先に日本語で）:** 「税込みにする」「送料を決める」「総額を出す」の3つの関数を作り、組み合わせて支払い総額を計算します。税込み 5,000 円以上なら送料無料です。

```rust
// 税率（%）、送料無料になる金額、送料
const TAX_RATE_PERCENT: u32 = 10;
const FREE_SHIPPING_THRESHOLD: u32 = 5_000;
const SHIPPING_FEE: u32 = 500;

// 税込みの金額を返す
fn with_tax(price: u32) -> u32 {
    price * (100 + TAX_RATE_PERCENT) / 100
}

// 合計金額に応じた送料を返す。if の結果がそのまま戻り値になる
fn shipping_fee(subtotal: u32) -> u32 {
    if subtotal >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        SHIPPING_FEE
    }
}

// 小さな関数を組み合わせて、支払い総額を計算する
fn total(price: u32, quantity: u32) -> u32 {
    let subtotal = with_tax(price * quantity);
    subtotal + shipping_fee(subtotal)
}

fn main() {
    println!("1,000円 × 2個 = {} 円", total(1_000, 2));
    println!("1,000円 × 5個 = {} 円", total(1_000, 5));
}
```

**▼ 実行結果**

```text
1,000円 × 2個 = 2700 円
1,000円 × 5個 = 5500 円
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: if 式をそのまま戻り値にする

```rust
fn shipping_fee(subtotal: u32) -> u32 {
    if subtotal >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        SHIPPING_FEE
    }
}
```

- 関数の本体の最後にある `if` 式の値が、そのまま戻り値になります。`0` にも `SHIPPING_FEE` にもセミコロンを付けないのがポイントです。
- `if` と `else` のどちらの枝も `u32` を返しているので、型がそろっています（そろっていないとエラーになります。UNIT 04）。

##### 解説2: 関数の組み合わせで読みやすくする

```rust
fn total(price: u32, quantity: u32) -> u32 {
    let subtotal = with_tax(price * quantity);
    subtotal + shipping_fee(subtotal)
}
```

- `total` の中身は「税込みにして、送料を足す」と日本語のように読めます。計算の細かい部分は、それぞれの関数の中に隠れています。
- 税率が変わったら `with_tax`（と定数）だけ、送料の規則が変わったら `shipping_fee` だけを直せば済みます。

### よくあるエラーと読み方

#### エラー1: E0308 最後の式にセミコロンを付けてしまった

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(1, 2));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:1:27
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ---                    ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     a + b;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected i32, found ()` は「i32 を返すと約束したのに、`()` が返っている」。`implicitly returns () as its body has no tail or return expression` は「本体に最後の式も `return` もないので、`()` を返していることになる」という意味です。
- **何が起きたか:** `a + b;` のセミコロンで値が捨てられました（図3-2 の右側）。
- **直し方:** `help: remove this semicolon to return this value`（この値を返すにはセミコロンを消してください）のとおり、`a + b` にします。**初心者がいちばんよく出会うエラーの1つ**です。

#### エラー2: E0308 戻り値の型を書き忘れた

```rust
fn add(a: i32, b: i32) {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:2:5
  |
1 | fn add(a: i32, b: i32) {
  |                       - help: try adding a return type: `-> i32`
2 |     a + b
  |     ^^^^^ expected `()`, found `i32`

error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> src\main.rs:6:20
  |
6 |     println!("{}", add(1, 2));
  |               --   ^^^^^^^^^ `()` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 2 previous errors
```

- **読み方:** エラー1の逆です。`->` がないので「`()` を返す関数」とみなされ、最後の式 `a + b`（i32）と食い違いました。2つ目の E0277 は、その `()` を `{}` で表示しようとしたために出た**連鎖のエラー**です。
- **直し方:** `help: try adding a return type: -> i32` のとおり、`fn add(a: i32, b: i32) -> i32` にします。1つ目を直せば2つ目も消えます。

> **ヒント:** エラーが複数出たときは、**いちばん上のエラーから**直しましょう。後ろのエラーは、最初のエラーの巻き添えで出ていることがよくあります。

#### エラー3: 引数の型を書き忘れた

```rust
fn double(x) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error: expected one of `:`, `@`, or `|`, found `)`
 --> src\main.rs:1:12
  |
1 | fn double(x) -> i32 {
  |            ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
1 | fn double(x: TypeName) -> i32 {
  |            ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
1 | fn double(_: x) -> i32 {
  |           ++

error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected one of :, @, or |, found )` は「`x` の後ろには `:` などが来るはずなのに `)` が来た」という**文法のエラー**です。エラー番号がないのは、型を調べる前の「文の形」の段階で止まったからです。
- **直し方:** 1つ目の `help`（引数名なら型を付けてください）に従い、`fn double(x: i32) -> i32` と書きます。`TypeName` は「ここに型名を書く」という意味の仮の名前です。

#### エラー4: E0061 引数の数が合わない

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(1));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> src\main.rs:6:20
  |
6 |     println!("{}", add(1));
  |                    ^^^--- argument #2 of type `i32` is missing
  |
note: function defined here
 --> src\main.rs:1:4
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ^^^         ------
help: provide the argument
  |
6 |     println!("{}", add(1, /* i32 */));
  |                         +++++++++++

For more information about this error, try `rustc --explain E0061`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「この関数は引数を2つ取るのに、1つしか渡されていません」。`argument #2 of type i32 is missing` は「2番目の i32 の引数がない」。`note: function defined here` が関数の定義の場所を教えてくれます。
- **直し方:** `add(1, 2)` のように2つ渡します。Rust には「引数の省略」や「既定値付きの引数」はありません。

### RustNote ではここで使う

UNIT 01 で作った「タイトルの文字数チェック」を、**名前の付いた小さな関数**に分けます。RustNote の検証処理は、最終的にこの形を `NoteTitle::parse` という1つの入口にまとめたものになります（UNIT 09）。

> **▼ このコードがやること（先に日本語で）:** タイトルの空白を取り除き、文字数を数え、1〜80文字の範囲かどうかを判定して、結果を文章で返します。空白だけのタイトルと、81文字のタイトルも試します。

```rust
const MAX_TITLE_CHARS: usize = 80;

// 前後の空白を取り除いた版を返す
fn normalize(raw: &str) -> &str {
    raw.trim()
}

// 文字数を数える（日本語も 1 文字を 1 と数える）
fn char_count(text: &str) -> usize {
    text.chars().count()
}

// 1 以上 80 以下なら true を返す
fn is_valid_length(len: usize) -> bool {
    (1..=MAX_TITLE_CHARS).contains(&len)
}

// 判定結果を、人が読める文にして返す
fn describe(raw: &str) -> String {
    let title = normalize(raw);
    let len = char_count(title);
    if is_valid_length(len) {
        format!("OK: 「{title}」（{len}文字）")
    } else {
        format!("NG: {len}文字は 1〜{MAX_TITLE_CHARS} 文字の範囲外です")
    }
}

fn main() {
    println!("{}", describe("  買い物リスト  "));
    println!("{}", describe("    "));
    // "あ" を 81 回くり返した文字列で試す
    let long = "あ".repeat(81);
    println!("{}", describe(&long));
}
```

**▼ 実行結果**

```text
OK: 「買い物リスト」（6文字）
NG: 0文字は 1〜80 文字の範囲外です
NG: 81文字は 1〜80 文字の範囲外です
```

- `(1..=MAX_TITLE_CHARS)` は「1 以上 80 以下」という**範囲**を表します（`..=` は「終わりも含む」）。`.contains(&len)` は「その範囲に len が入っているか」を `true` / `false` で返します。`&` は「値を貸す」記号で、第3巻で詳しく学びます。
- `format!` は `println!` と同じ書き方で、表示する代わりに**文字列（`String`）を作って返す**マクロです。
- `describe(&long)` の `&` も「貸す」記号です。`repeat` で作った `String` を、`&str` を受け取る関数へ渡すときに付けます（UNIT 05）。

### 練習問題

**問題1.** 次の関数はコンパイルエラーになります。何が原因かを説明して、直してください。

```rust
fn square(x: i32) -> i32 {
    x * x;
}
```

**問題2.** 摂氏（℃）を華氏（℉）に変換する関数 `fn fahrenheit(celsius: f64) -> f64` を書き、0℃、36.5℃、100℃ を変換して表示してください。式は「℉ = ℃ × 9 ÷ 5 + 32」です。

**問題3.** 次のプログラムが表示する値を予想してください。

```rust
fn main() {
    let a = 10;
    let b = {
        let t = a * 2;
        t + 1
    };
    let c = if b > 20 { b - 20 } else { 0 };
    println!("{a} {b} {c}");
}
```

### 解答と解説

**解答1.** 最後の `x * x;` のセミコロンで値が捨てられ、関数が `()` を返すことになるため E0308 になります。セミコロンを消します。

```rust
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    println!("{}", square(7));
}
```

**▼ 実行結果**

```text
49
```

**解答2.**

```rust
// 摂氏（℃）を華氏（℉）に変換する
fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("0℃ = {}℉", fahrenheit(0.0));
    println!("36.5℃ = {:.1}℉", fahrenheit(36.5));
    println!("100℃ = {}℉", fahrenheit(100.0));
}
```

**▼ 実行結果**

```text
0℃ = 32℉
36.5℃ = 97.7℉
100℃ = 212℉
```

- `f64` どうしの計算なので、`9` ではなく `9.0` のように小数で書きます（`celsius * 9` だと UNIT 02 のエラー4と同じ E0277 になります）。

**解答3.**

```text
10 21 1
```

- `b` のブロックは `t = 20` を作り、最後の式 `t + 1` = 21 が値になります。`c` は `21 > 20` なので `21 - 20` = 1 です。

### まとめ

- 関数は `fn 名前(引数: 型) -> 戻り値の型 { 本体 }`。引数と戻り値の型は省略できない。
- 値になるのが**式**、値を持たない命令が**文**。最後の式（セミコロンなし）が、ブロックや関数の値になる。
- 異常な場合は `return` で早めに抜け、本来の処理は最後の式で返す。
