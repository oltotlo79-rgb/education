## UNIT 09: コンパイラエラーとの付き合い方

### この単元のゴール

- Rust のエラーメッセージを、**エラー番号・場所・印（`^^^`）・`note:`・`help:`** の部品に分けて読める
- エラーが複数出たときに、**上から1つずつ**直す理由が分かる
- `rustc --explain` で、エラー番号の詳しい説明を読める
- **エラー（error）**と**警告（warning）**の違いが分かり、`cargo clippy` の助言を活かせる

### たとえ話でつかむ

Rust のコンパイラのエラーメッセージは、**とても親切な先生の添削（赤ペン）**です。

- 「×」だけ付けて返す先生ではありません。**どのページの何行目の何文字目か**、**何が問題か**、**なぜそう判断したか**、そして多くの場合**どう直せばよいか**まで書いてくれます。
- 添削が10か所あっても、最初の1つの書き間違いが原因で、後ろがまとめて×になっていることがあります。**最初の赤ペンから順に**直すのが近道です。
- 先生の説明が短すぎて分からないときは、**教科書の該当ページ**（`rustc --explain`）を開けば、例つきの長い説明が読めます。

最初はエラーが出るたびに「怒られた」と感じるかもしれません。でも Rust のコンパイラは、**動かしてから壊れるはずだった問題を、動かす前に見つけてくれる**相棒です。エラーをたくさん見た人ほど、Rust が上手になります。

> **用語:** **エラー（error）** … これを直さないとコンパイルが終わらない（実行ファイルが作られない）問題。
>
> **用語:** **警告（warning）** … コンパイルは終わるが、「たぶん間違いでは？」「もっと良い書き方がある」という指摘。UNIT 08 で見たとおり、バグの手がかりになることがあります。

### 図で理解する

次のエラーメッセージは「もう一歩」で実際に出すものです。部品ごとに意味があります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 420" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="420" height="400" rx="10" fill="#0c1222"/>
<g font-family="Cascadia Mono, Consolas, monospace" font-size="12.5" fill="#e2e8f0" xml:space="preserve" style="white-space: pre">
<text x="22" y="36"><tspan fill="#f87171" font-weight="700">error[E0308]</tspan>: mismatched types</text>
<text x="22" y="58"><tspan fill="#60a5fa"> --&gt;</tspan> src\main.rs:8:11</text>
<text x="22" y="80" fill="#60a5fa">  |</text>
<text x="22" y="102"><tspan fill="#60a5fa">8 |</tspan>     greet(name);</text>
<text x="22" y="124"><tspan fill="#60a5fa">  |</tspan>     <tspan fill="#60a5fa">-----</tspan> <tspan fill="#f87171">^^^^ expected `&amp;str`,</tspan></text>
<text x="22" y="146"><tspan fill="#60a5fa">  |</tspan>           <tspan fill="#f87171">found `String`</tspan></text>
<text x="22" y="168"><tspan fill="#60a5fa">  |     |</tspan></text>
<text x="22" y="190"><tspan fill="#60a5fa">  |     arguments to this function</tspan></text>
<text x="22" y="212"><tspan fill="#60a5fa">  |     are incorrect</tspan></text>
<text x="22" y="234"><tspan fill="#4ade80" font-weight="700">note</tspan>: function defined here</text>
<text x="22" y="256"><tspan fill="#60a5fa"> --&gt;</tspan> src\main.rs:2:4</text>
<text x="22" y="278"><tspan fill="#60a5fa">2 |</tspan> fn greet(name: &amp;str) {</text>
<text x="22" y="300"><tspan fill="#60a5fa">  |</tspan>    <tspan fill="#4ade80">^^^^^</tspan> <tspan fill="#60a5fa">----------</tspan></text>
<text x="22" y="322"><tspan fill="#38bdf8" font-weight="700">help</tspan>: consider borrowing here</text>
<text x="22" y="344"><tspan fill="#60a5fa">8 |</tspan>     greet(<tspan fill="#4ade80">&amp;</tspan>name);</text>
<text x="22" y="366"><tspan fill="#60a5fa">  |</tspan>           <tspan fill="#4ade80">+</tspan></text>
<text x="22" y="396">For more information ... --explain E0308</text>
</g>
<circle cx="420" cy="32" r="3" fill="#dc2626"/>
<line x1="420" y1="32" x2="452" y2="32" stroke="#dc2626" stroke-width="1.5"/>
<circle cx="420" cy="54" r="3" fill="#3b82f6"/>
<line x1="420" y1="54" x2="452" y2="80" stroke="#3b82f6" stroke-width="1.5"/>
<circle cx="420" cy="120" r="3" fill="#dc2626"/>
<line x1="420" y1="120" x2="452" y2="134" stroke="#dc2626" stroke-width="1.5"/>
<circle cx="420" cy="186" r="3" fill="#3b82f6"/>
<line x1="420" y1="186" x2="452" y2="188" stroke="#3b82f6" stroke-width="1.5"/>
<circle cx="420" cy="230" r="3" fill="#10b981"/>
<line x1="420" y1="230" x2="452" y2="246" stroke="#10b981" stroke-width="1.5"/>
<circle cx="420" cy="318" r="3" fill="#0284c7"/>
<line x1="420" y1="318" x2="452" y2="306" stroke="#0284c7" stroke-width="1.5"/>
<circle cx="420" cy="392" r="3" fill="#475569"/>
<line x1="420" y1="392" x2="452" y2="372" stroke="#475569" stroke-width="1.5"/>
<rect x="454" y="14" width="256" height="40" rx="6" fill="#fef2f2" stroke="#dc2626"/>
<text x="464" y="31" font-size="12" font-weight="700" fill="#b91c1c">① エラー番号と要約</text>
<text x="464" y="47" font-size="12" fill="#334155">E0308 = 型が合わない</text>
<rect x="454" y="62" width="256" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="464" y="79" font-size="12" font-weight="700" fill="#1e40af">② 場所（ファイル:行:列）</text>
<text x="464" y="95" font-size="12" fill="#334155">main.rs の 8 行目 11 文字目</text>
<rect x="454" y="114" width="256" height="40" rx="6" fill="#fef2f2" stroke="#dc2626"/>
<text x="464" y="131" font-size="12" font-weight="700" fill="#b91c1c">③ ^^^ = 問題の箇所と理由</text>
<text x="464" y="147" font-size="12" fill="#334155">&amp;str が欲しいのに String</text>
<rect x="454" y="168" width="256" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="464" y="185" font-size="12" font-weight="700" fill="#1e40af">④ --- = 関係する箇所</text>
<text x="464" y="201" font-size="12" fill="#334155">この関数への引数が原因</text>
<rect x="454" y="226" width="256" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="464" y="243" font-size="12" font-weight="700" fill="#047857">⑤ note: 補足情報</text>
<text x="464" y="259" font-size="12" fill="#334155">関数はここで定義されている</text>
<rect x="454" y="286" width="256" height="40" rx="6" fill="#f0f9ff" stroke="#0284c7"/>
<text x="464" y="303" font-size="12" font-weight="700" fill="#0369a1">⑥ help: 直し方の提案</text>
<text x="464" y="319" font-size="12" fill="#334155">+ の位置に &amp; を足すとよい</text>
<rect x="454" y="352" width="256" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="464" y="369" font-size="12" font-weight="700" fill="#334155">⑦ もっと詳しい説明の読み方</text>
<text x="464" y="385" font-size="12" fill="#334155">rustc --explain E0308</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-1: エラーメッセージの部品（紙面に合わせて長い行は折り返している。右端の点がその行）。①→②→③の順に読み、⑥の提案を検討する</div>
</div>

エラーが出たときの進め方は、次の流れです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-23" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="124" height="70" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="76" y="48" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">① いちばん上の</text>
<text x="76" y="68" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">エラーだけ読む</text>
<rect x="156" y="20" width="124" height="70" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="218" y="48" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">② --&gt; の場所を</text>
<text x="218" y="68" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">エディタで開く</text>
<rect x="298" y="20" width="124" height="70" rx="10" fill="#f0f9ff" stroke="#0284c7" stroke-width="2"/>
<text x="360" y="48" font-size="13" font-weight="700" fill="#0369a1" text-anchor="middle">③ help: を読み</text>
<text x="360" y="68" font-size="13" font-weight="700" fill="#0369a1" text-anchor="middle">意味を考える</text>
<rect x="440" y="20" width="124" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="502" y="48" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">④ 1か所だけ</text>
<text x="502" y="68" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">直す</text>
<rect x="582" y="20" width="124" height="70" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="644" y="48" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">⑤ cargo check</text>
<text x="644" y="68" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">でもう一度</text>
<line x1="140" y1="55" x2="152" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="282" y1="55" x2="294" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="424" y1="55" x2="436" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="566" y1="55" x2="578" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<path d="M644,92 L644,120 L76,120 L76,94" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<text x="360" y="114" font-size="12" fill="#334155" text-anchor="middle">まだエラーが残っていれば、また①へ（消えたエラーは同じ原因だった）</text>
<rect x="156" y="150" width="408" height="84" rx="10" fill="#f8fafc" stroke="#475569" stroke-dasharray="6 4"/>
<text x="360" y="174" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">③で意味が分からないとき</text>
<text x="360" y="198" font-size="12" fill="#334155" text-anchor="middle">rustc --explain エラー番号 → 例つきの説明を読む</text>
<text x="360" y="220" font-size="12" fill="#334155" text-anchor="middle">それでも分からなければ、エラー全文で検索する・AI に聞く</text>
<line x1="360" y1="92" x2="360" y2="100" stroke="#475569" stroke-width="2"/>
<line x1="330" y1="128" x2="330" y2="146" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-00-23)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-2: エラーとの付き合い方。上から1つずつ、1か所直したらすぐ確かめる</div>
</div>

### まず動かす最小コード

この単元の最小コードは、**わざと間違えたコード**です。1つずつ `src\main.rs` に貼り付けて `cargo run` し、エラーを自分の目で見てから直してください。

#### 例1: 型が合わない（E0308）

> **▼ このコードがやること（先に日本語で）:** 年齢を数値（`u32`）の変数に入れたいのに、うっかり `"20"` と引用符を付けて**文字**として書いてしまったコードです。

```rust
fn main() {
    // 年齢を数値（u32）として持ちたい
    let age: u32 = "20";
    println!("年齢: {age}");
}
```

**▼ 実行結果**

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
error[E0308]: mismatched types
 --> src\main.rs:3:20
  |
3 |     let age: u32 = "20";
  |              ---   ^^^^ expected `u32`, found `&str`
  |              |
  |              expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: 1行目は「番号」と「要約」

```text
error[E0308]: mismatched types
```

- `error` … 直さないとコンパイルが終わらない問題です。
- `[E0308]` … **エラー番号**です。同じ番号のエラーは同じ種類の問題なので、番号で検索すると説明が見つかります。
- `mismatched types` … 要約で、「型が食い違っている」という意味です。

##### 解説2: `-->` は場所

```text
 --> src\main.rs:3:20
```

- `src\main.rs` ファイルの **3行目・20文字目**です。VS Code では、`Ctrl + G` で行番号を指定して移動できます。ターミナルのこの部分を Ctrl を押しながらクリックすると、その場所が開くこともあります。

##### 解説3: `^^^` と `---` の印

```text
3 |     let age: u32 = "20";
  |              ---   ^^^^ expected `u32`, found `&str`
  |              |
  |              expected due to this
```

- `^^^^` は**問題の箇所**です。`"20"` の下に付いていて、「`u32` を期待していたのに、`&str`（文字列）が見つかった」と書かれています。
- `---` は**関係する箇所**です。`u32` の下に付いていて、「このせいで `u32` を期待した（expected due to this）」と、判断の根拠を示しています。
- 直し方は、引用符を外して `let age: u32 = 20;` とすることです。

##### 解説4: 最後の2行

```text
For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

- 1行目は「詳しい説明は `rustc --explain E0308` で読めます」という案内です（「もう一歩」で使います）。
- 2行目は Cargo のまとめで、「前に出たエラー1個のせいで、`practice` をコンパイルできなかった」という意味です。エラーの中身はこの行ではなく、**上のほう**にあります。

#### 例2: 名前の打ち間違い（E0425）

ここから先のエラーの出力では、最初の `Compiling ...` の行を省きます。例2・例3では、最後の2行（例1の解説4と同じ形の案内とまとめ）も省いています。

```rust
fn main() {
    let message = "こんにちは";
    println!("{}", mesage);
}
```

**▼ 実行結果**

```text
error[E0425]: cannot find value `mesage` in this scope
 --> src\main.rs:3:20
  |
3 |     println!("{}", mesage);
  |                    ^^^^^^
  |
help: a local variable with a similar name exists
  |
3 |     println!("{}", message);
  |                       +
```

- 「この範囲（scope）に `mesage` という値は見つからない」というエラーです。
- `help:` で「似た名前のローカル変数があります」と、`message` を提案してくれています。下の `+` は「ここに1文字足す」という印です。
- 打ち間違いは、Rust で最もよく出るエラーの1つです。`help:` の提案どおりに直せば動きます。

#### 例3: 書き換え禁止の変数を書き換えた（E0384）

```rust
fn main() {
    let count = 0;
    count = count + 1;
    println!("count = {count}");
}
```

**▼ 実行結果**

```text
error[E0384]: cannot assign twice to immutable variable `count`
 --> src\main.rs:3:5
  |
2 |     let count = 0;
  |         ----- first assignment to `count`
3 |     count = count + 1;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut count = 0;
  |         +++
```

- 「書き換え禁止（immutable）の変数 `count` に、2回目の代入はできない」というエラーです。
- `---` が「最初の代入はここ（2行目）」、`^^^` が「2回目の代入はここ（3行目）」を指しています。**問題の行と原因の行が別**であることに注意してください。
- `help:` は「この変数を書き換え可能（mutable）にしては？」と、2行目に `mut` を足す（`+++`）提案をしています。UNIT 01 で学んだ `let mut` です。

3つとも、`help:` や印のとおりに直すと動きます。

```text
年齢: 20
こんにちは
count = 1
```

（例1〜3を直したものを、それぞれ実行した結果です。）

### もう一歩: 実務に近い形

#### `note:` と `help:` の両方が出るエラー

> **▼ このコードがやること（先に日本語で）:** 名前を受け取ってあいさつする関数 `greet` を作り、`String` 型の変数を渡して呼び出そうとしたコードです。関数が受け取る型と、渡した型が少しだけ違っています。

```rust
// name さんにあいさつする関数
fn greet(name: &str) {
    println!("こんにちは、{name} さん");
}

fn main() {
    let name = String::from("Hanako");
    greet(name);
}
```

**▼ 実行結果**

```text
error[E0308]: mismatched types
 --> src\main.rs:8:11
  |
8 |     greet(name);
  |     ----- ^^^^ expected `&str`, found `String`
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> src\main.rs:2:4
  |
2 | fn greet(name: &str) {
  |    ^^^^^ ----------
help: consider borrowing here
  |
8 |     greet(&name);
  |           +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: 問題の箇所と、関係する箇所

- `^^^^` は8行目の `name` を指し、「`&str` が欲しいのに `String` が来た」と言っています。
- `-----` は `greet` を指し、「この関数への引数が正しくない（arguments to this function are incorrect）」と言っています。

##### 解説2: `note:` は別の場所の補足

```text
note: function defined here
 --> src\main.rs:2:4
```

- `note:` は「参考までに」という補足です。ここでは「その関数はここ（2行目）で定義されていて、`name: &str` という引数を受け取ると書いてある」と、**関数の側の場所**まで見せてくれています。
- エラーの原因が**2か所にまたがる**とき（呼び出す側と、定義する側）に、両方を見比べられるようにしてくれているのです。

##### 解説3: `help:` は直し方の提案

```text
help: consider borrowing here
  |
8 |     greet(&name);
  |           +
```

- 「ここで借りる（borrow）ことを検討してください」という提案で、`name` の前に `&` を足した形を見せてくれています。
- 提案どおり `greet(&name);` に直すと、`こんにちは、Hanako さん` と表示されます。
- `&` の意味（借用）は第3巻のテーマです。今は「help が具体的な直し方まで見せてくれる」ことが分かれば十分です。

> **注意:** `help:` は**多くの場合正しい**提案ですが、いつもあなたの意図どおりとは限りません。UNIT 08 の「`_` を付けて警告を消す」提案のように、意味を考えずに従うと問題を隠してしまうこともあります。「なぜその直し方で直るのか」を一言で説明できるか、自分に聞いてから直しましょう。

#### 同じ原因のエラーは、まとめて消える

```rust
fn main() {
    // 在庫の数
    let mut stock = HashMap::new();
    stock.insert("りんご", 3);

    // 値段
    let mut prices = HashMap::new();
    prices.insert("りんご", 120);

    println!("{stock:?} {prices:?}");
}
```

**▼ 実行結果**

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src\main.rs:3:21
  |
3 |     let mut stock = HashMap::new();
  |                     ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

error[E0433]: cannot find type `HashMap` in this scope
 --> src\main.rs:7:22
  |
7 |     let mut prices = HashMap::new();
  |                      ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `practice` (bin "practice") due to 2 previous errors
```

- エラーは2つですが、原因は「`HashMap` を `use` していない」の1つだけです。
- `help:` の `1 + use std::collections::HashMap;` は、「1行目にこの行を**追加**して」という意味です（`+` は追加の印）。1行足せば、**2つのエラーが同時に消えます**。
- エラーが20個出ても慌てないでください。**最初の1つを直して、もう一度コンパイル**すると、半分以下に減ることがよくあります。

#### `rustc --explain` で教科書を開く

```powershell
rustc --explain E0384
```

**▼ 実行結果**（先頭の部分）

````text
An immutable variable was reassigned.

Erroneous code example:

```
fn main() {
    let x = 3;
    x = 5; // error, reassignment of immutable variable
}
```

By default, variables in Rust are immutable. To fix this error, add the keyword
`mut` after the keyword `let` when declaring the variable. For example:

```
fn main() {
    let mut x = 3;
    x = 5;
}
```
````

- 1行目が要約（「書き換え禁止の変数に再代入した」）、続いて**間違った例**（Erroneous code example）と、**直し方の説明と正しい例**が載っています。
- 英語ですが、コードの例は英語が読めなくても比べられます。翻訳ツールにかけるのも良い方法です。

#### エラーではないが、より良い書き方: `cargo clippy`

例3を `let mut count = 0;` に直したコードに `cargo clippy` をかけると、次の助言が出ます。

```powershell
cargo clippy
```

**▼ 実行結果**（助言の部分）

```text
warning: manual implementation of an assign operation
 --> src\main.rs:3:5
  |
3 |     count = count + 1;
  |     ^^^^^^^^^^^^^^^^^ help: replace it with: `count += 1`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#assign_op_pattern
  = note: `#[warn(clippy::assign_op_pattern)]` on by default
```

**Clippy（クリッピー）** は、コンパイラより一歩踏み込んで「動くけれど、こう書くほうが Rust らしい」という助言をくれる道具です。ここでは「`count = count + 1` は `count += 1` と書けます」と教えてくれています。エラーと同じ読み方で読めます。

### よくあるエラーと読み方

#### 使い終わった値を、また使おうとした（E0382）

```rust
fn main() {
    let title = String::from("牛乳を買う");
    let saved = title;
    println!("保存したタイトル: {saved}");
    println!("元のタイトル: {title}");
}
```

**▼ 実行結果**

```text
error[E0382]: borrow of moved value: `title`
 --> src\main.rs:5:24
  |
2 |     let title = String::from("牛乳を買う");
  |         ----- move occurs because `title` has type `String`, which does not implement the `Copy` trait
3 |     let saved = title;
  |                 ----- value moved here
4 |     println!("保存したタイトル: {saved}");
5 |     println!("元のタイトル: {title}");
  |                              ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let saved = title.clone();
  |                      ++++++++
```

**何が起きたか:** ラベルを上から読むと、話の筋が分かります。

1. 2行目: `title` は `String` 型で、コピーされない（`Copy` ではない）型なので、渡すと**移動（move）**する
2. 3行目: `title` の中身が `saved` に**移動した**（value moved here）
3. 5行目: 移動した後の `title` を使おうとした（borrowed here after move）

Rust では、文字列などの値には**持ち主がちょうど1人**いて、`let saved = title;` で持ち主が `saved` に引っ越します。引っ越した後の `title` は空き家なので、使えません。これが Rust の中心にある**所有権**というルールで、第3巻で詳しく学びます。

**直し方:** `help:` の提案どおり `title.clone()` で複製を作って渡すか、`saved` だけを使うように書き直します（練習問題2）。

> **ヒント:** 日本語の文字が入った行では、`^^^^^` の印が少しずれて見えることがあります。日本語の文字が画面上で2文字分の幅を取るためです。印の位置より、`-->` の行番号と列番号を信じましょう。

#### `;` を忘れた

```rust
fn main() {
    let price = 120
    let count = 3;
    println!("合計: {}", price * count);
}
```

**▼ 実行結果**

```text
error: expected `;`, found keyword `let`
 --> src\main.rs:2:20
  |
2 |     let price = 120
  |                    ^ help: add `;` here
3 |     let count = 3;
  |     --- unexpected token

error: could not compile `practice` (bin "practice") due to 1 previous error
```

**何が起きたか:** 「`;` があるはずの所で、`let` というキーワードが見つかった」というエラーです。このように、文法の間違いにはエラー番号が付かないこともあります。

**直し方:** `help: add ; here` のとおり、2行目の終わり（`^` の位置）に `;` を足します。コンパイラは3行目の `let` を見て初めて「おかしい」と気づくので、**エラーの印が、本当に直すべき場所の次の行に付く**ことがある、と覚えておきましょう。

### RustNote ではここで使う

- RustNote では Topcoat の `view!` や `#[page]` のような**マクロ**（コードを自動で書き足してくれる仕組み）を使います。マクロの中で間違えると、少し長めのエラーが出ることがありますが、読み方は同じです。まず `-->` の場所と最初の `^^^` を見ます。
- 作業の区切りごとに `cargo check`・`cargo clippy`・`cargo test` を通す習慣を付けます。第11巻では、警告が1つでもあれば失敗にする設定（`-D warnings`）で、品質を保つしくみも作ります。

### 練習問題

**問題1:** 次のコードのエラーを読み、直してください。税込み価格（120 円の 1.08 倍）を表示したいコードです。

```rust
fn main() {
    let price: u32 = 120;
    // 税込み価格（1.08 倍）を計算したい
    let with_tax = price * 1.08;
    println!("税込み: {with_tax}");
}
```

```text
error[E0277]: cannot multiply `u32` by `{float}`
 --> src\main.rs:4:26
  |
4 |     let with_tax = price * 1.08;
  |                          ^ no implementation for `u32 * {float}`
  |
  = help: the trait `Mul<{float}>` is not implemented for `u32`
  = help: the following other types implement trait `Mul<Rhs>`:
            `&u32` implements `Mul<u32>`
            `&u32` implements `Mul`
            `u32` implements `Mul<&u32>`
            `u32` implements `Mul<Duration>`
            `u32` implements `Mul`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

**問題2:** 「よくあるエラー」の E0382 のコードを、`help:` の提案を使って直してください。

**問題3:** `rustc --explain E0425` を実行して、最初の1行に書かれている要約を日本語にしてください。

### 解答と解説

**問題1の解答:** エラーの要約は「`u32`（整数）に `{float}`（小数）を掛けることはできない」です。Rust は、整数と小数を**勝手に変換して計算しません**。どちらかに型をそろえます。

```rust
fn main() {
    let price: u32 = 120;

    // 方法 1: 整数のまま「108 を掛けて 100 で割る」（小数点以下は切り捨て）
    let with_tax = price * 108 / 100;
    println!("税込み（整数で計算）: {with_tax}");

    // 方法 2: f64（小数の型）に変換してから 1.08 倍する
    let with_tax_f64 = f64::from(price) * 1.08;
    println!("税込み（小数で計算）: {with_tax_f64}");
}
```

**▼ 実行結果**

```text
税込み（整数で計算）: 129
税込み（小数で計算）: 129.60000000000002
```

小数で計算すると `129.60000000000002` という半端な結果になりました。コンピュータは小数を2進数で近似して持つため、こうした小さな誤差が出ます。お金の計算は、方法1のように**整数（円）で扱う**のが基本です。

`= help:` のあとの長い一覧（`Mul` を持っている型の一覧）は、今は読み飛ばして大丈夫です。エラーメッセージは、**分かる部分から読めばよい**のです。

**問題2の解答:**

```rust
fn main() {
    let title = String::from("牛乳を買う");
    // clone() で「複製」を作って saved に渡す。title は手元に残る
    let saved = title.clone();
    println!("保存したタイトル: {saved}");
    println!("元のタイトル: {title}");
}
```

**▼ 実行結果**

```text
保存したタイトル: 牛乳を買う
元のタイトル: 牛乳を買う
```

**問題3の解答:** 最初の行は `An unresolved name was used.` で、「解決できない（見つからない）名前が使われた」という意味です。例2の `mesage` がまさにこれでした。

```text
An unresolved name was used.

Erroneous code examples:
```

### まとめ

- エラーメッセージは「① 番号と要約 → ② `-->` の場所 → ③ `^^^` の理由 → `note:` の補足 → `help:` の提案」の順に読みます。
- 複数のエラーは**上から1つずつ**。1か所直したらすぐ `cargo check` します。分からなければ `rustc --explain 番号`。
- 警告や Clippy の助言も読み方は同じです。`help:` は「なぜ直るのか」を考えてから使いましょう。

