## UNIT 01: 変数・mut・シャドーイング・定数

### この単元のゴール

- `let` で値に名前を付け、`println!` で表示できる
- Rust の変数が「既定で変更できない（不変）」理由を説明し、必要なときだけ `mut` を付けられる
- 同じ名前で値を作り直す**シャドーイング**と、`mut` による書き換えの違いを説明できる
- 「プログラム全体で変わらない値」を `const`（定数）で書ける

### たとえ話でつかむ

変数は「**箱に貼った名札**」だと考えてください。

- `let x = 5;` は、5 を入れた箱を用意して「x」という名札を貼り、**ふたをテープで閉じる**ことです。中身を見ることはできますが、入れ替えはできません。
- `let mut y = 5;` の `mut`（ミュート：mutable ＝ 変更可能 の略）は、「**ふたを開けたままにしておく**」という宣言です。あとから中身を入れ替えられます。
- **シャドーイング**（shadowing：影にする、の意味）は、同じ名札を**新しい箱に貼り替える**ことです。古い箱はそのまま残りますが、名札がなくなったので、もうその名前では呼べません。新しい箱には別の種類（型）のものを入れてもかまいません。
- `const`（コンスト：constant ＝ 定数）は、**金属のプレートに刻印した値**です。プログラムを作る（コンパイルする）時点で決まっていて、二度と変わりません。

なぜ「ふたが閉じている」のが基本なのでしょうか。大きなプログラムでは、「この値はどこかで書き換えられていないか？」を疑いながら読むのはとても疲れます。Rust は**変えない値を標準**にし、変える値にだけ `mut` という目印を付けさせることで、読む人が安心できるようにしています。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="250" rx="12" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="40" text-anchor="middle" font-size="16" font-weight="700" fill="#1e40af">let x = 5;（既定は不変）</text>
<rect x="120" y="96" width="140" height="84" rx="8" fill="#ffffff" stroke="#1e40af" stroke-width="2"/>
<rect x="110" y="82" width="160" height="16" rx="4" fill="#1e40af"/>
<line x1="190" y1="82" x2="190" y2="98" stroke="#f97316" stroke-width="6"/>
<text x="190" y="150" text-anchor="middle" font-size="30" font-weight="700" fill="#0f172a">5</text>
<rect x="40" y="122" width="60" height="32" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="70" y="144" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">x</text>
<line x1="100" y1="138" x2="120" y2="138" stroke="#f97316" stroke-width="2"/>
<text x="180" y="212" text-anchor="middle" font-size="13" fill="#334155">ふたがテープで閉じている</text>
<text x="180" y="236" text-anchor="middle" font-size="13" fill="#c2410c">x = 6; → コンパイルエラー（E0384）</text>
<rect x="370" y="10" width="340" height="250" rx="12" fill="#ecfdf5" stroke="#10b981"/>
<text x="540" y="40" text-anchor="middle" font-size="16" font-weight="700" fill="#047857">let mut y = 5;（変更可能）</text>
<rect x="480" y="96" width="140" height="84" rx="8" fill="#ffffff" stroke="#047857" stroke-width="2"/>
<rect x="470" y="80" width="160" height="16" rx="4" fill="#047857" transform="rotate(-10 470 96)"/>
<text x="530" y="150" text-anchor="middle" font-size="26" font-weight="700" fill="#94a3b8" text-decoration="line-through">5</text>
<text x="575" y="150" text-anchor="middle" font-size="30" font-weight="700" fill="#0f172a">6</text>
<rect x="400" y="122" width="60" height="32" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="430" y="144" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">y</text>
<line x1="460" y1="138" x2="480" y2="138" stroke="#f97316" stroke-width="2"/>
<text x="540" y="212" text-anchor="middle" font-size="13" fill="#334155">ふたが開いている（mut の目印）</text>
<text x="540" y="236" text-anchor="middle" font-size="13" fill="#047857">y = 6; → OK。中身を入れ替えられる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 変数は「名札を貼った箱」。let だけならふたが閉じ、mut を付けるとふたが開く</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">シャドーイング: 同じ名札を「新しい箱」に貼り替える</text>
<rect x="20" y="70" width="170" height="80" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="105" y="108" text-anchor="middle" font-size="18" fill="#94a3b8">"&#160;&#160;42&#160;&#160;"</text>
<text x="105" y="134" text-anchor="middle" font-size="12" fill="#94a3b8">型: &amp;str（文字列）</text>
<rect x="275" y="70" width="170" height="80" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="360" y="108" text-anchor="middle" font-size="18" fill="#94a3b8">"42"</text>
<text x="360" y="134" text-anchor="middle" font-size="12" fill="#94a3b8">型: &amp;str（文字列）</text>
<rect x="530" y="70" width="170" height="80" rx="8" fill="#eff6ff" stroke="#1e40af" stroke-width="2"/>
<text x="615" y="108" text-anchor="middle" font-size="22" font-weight="700" fill="#0f172a">42</text>
<text x="615" y="134" text-anchor="middle" font-size="12" fill="#1e40af">型: u32（数）</text>
<line x1="190" y1="110" x2="270" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-01)"/>
<text x="230" y="100" text-anchor="middle" font-size="13" fill="#475569">.trim()</text>
<line x1="445" y1="110" x2="525" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-01)"/>
<text x="485" y="100" text-anchor="middle" font-size="13" fill="#475569">.parse()</text>
<rect x="580" y="40" width="70" height="26" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="615" y="58" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">input</text>
<text x="105" y="178" text-anchor="middle" font-size="12" fill="#64748b">1つ目の input</text>
<text x="360" y="178" text-anchor="middle" font-size="12" fill="#64748b">2つ目の input</text>
<text x="615" y="178" text-anchor="middle" font-size="12" fill="#1e40af">3つ目の input（今はこれ）</text>
<text x="360" y="212" text-anchor="middle" font-size="13" fill="#334155">古い箱は名札を失い、もう input という名前では呼べない（型が変わってもよい）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: let を書き直すたびに新しい箱ができ、名札だけが移っていく</div>
</div>

4つの書き方を表にまとめます。

| 書き方 | 中身の入れ替え | 型を変える | いつ使うか |
|---|---|---|---|
| `let x = 5;` | できない | できない | ほとんどの値（まずはこれ） |
| `let mut x = 5;` | できる | できない | 数を数える、少しずつ組み立てるなど、本当に変わる値 |
| `let x = x + 1;`（シャドーイング） | 新しい箱を作る | できる | 「文字列 → 空白除去 → 数」のように段階的に変換するとき |
| `const MAX: u32 = 80;` | できない | できない | プログラム全体で共有する決まった値（上限・既定値） |

### まず動かす最小コード

この巻のコードは、すべて次の手順で動かせます（詳しくは巻頭の「この巻の進め方」）。

```powershell
# 最初に1回だけ: 練習用プロジェクトを作って中に入る
cargo new basics
cd basics
# 以後は src/main.rs の中身を例のコードに置き換えて実行する
cargo run
```

#### 例1: let で名前を付けて表示する

> **▼ このコードがやること（先に日本語で）:** 文字と数にそれぞれ名前を付けて、画面に表示します。`println!` の中の `{}` が「ここに値を差し込む」印です。

```rust
fn main() {
    // title という名前に、文字列 "Rust入門" を結び付ける
    let title = "Rust入門";
    // count という名前に、数 3 を結び付ける
    let count = 3;

    // {} の位置に、後ろに書いた値が入って表示される
    println!("タイトル: {}", title);
    // {count} のように、波かっこの中へ変数名を直接書いてもよい
    println!("ノートの数: {count}");
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
タイトル: Rust入門
ノートの数: 3
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `fn main() { ... }`

```rust
fn main() {
    // ここに書いた文が上から順に実行される
}
```

- `fn`（エフエヌ：function ＝ 関数 の略）は「ひとまとまりの処理を作る」キーワードです。
- `main` という名前の関数は特別で、プログラムを起動すると**最初に呼ばれる入口**です。
- `{ }`（波かっこ）の中が関数の中身です。関数については UNIT 03 で詳しく学びます。

##### 解説2: `let 名前 = 値;`

```rust
let title = "Rust入門";
let count = 3;
```

- `let`（レット）は「この値に名前を付ける」宣言です。この「名前と値を結び付けること」を**束縛**（バインディング）とも呼びます。
- `"..."` で囲んだものは文字列、`3` は数です。どちらも型（値の種類）を書いていませんが、Rust が右側の値から自動で決めます。これを**型推論**（かたすいろん）と呼びます。
- 文の終わりには `;`（セミコロン）を付けます。

> **用語:** **型（かた）** … 値の種類のこと。「整数」「文字列」「真偽値」などがあります。Rust はすべての値の型をコンパイルの時点で決めます。

##### 解説3: `println!` と `{}`

```rust
println!("タイトル: {}", title);
println!("ノートの数: {count}");
```

- `println!`（プリントライン）は画面に1行表示する命令です。名前の最後の `!` は、これが関数ではなく**マクロ**（コードを自動で組み立てる仕組み）であることを表します。今は「表示するときは `println!`」と覚えれば十分です。
- `{}` は「ここに値を差し込む」穴です。後ろに `, title` と書いた値が入ります。
- `{count}` のように、穴の中へ変数名を直接書くこともできます。本書ではこちらを多く使います。

#### 例2: mut で中身を入れ替える

> **▼ このコードがやること（先に日本語で）:** 完了したノートの数を数える変数を作り、1を足したり2を足したりして、そのたびに表示します。数が変わる変数なので `mut` を付けます。

```rust
fn main() {
    // mut を付けると、あとから中身を入れ替えられる変数になる
    let mut done_count = 0;
    println!("はじめ: {done_count}");

    // 今の値に 1 を足して、同じ変数へ入れ直す
    done_count = done_count + 1;
    println!("1件完了: {done_count}");

    // += は「自分に足して入れ直す」の短い書き方
    done_count += 2;
    println!("さらに2件完了: {done_count}");
}
```

**▼ 実行結果**

```text
はじめ: 0
1件完了: 1
さらに2件完了: 3
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `let mut`

```rust
let mut done_count = 0;
```

- `mut` を付けた変数だけが、あとから `=` で別の値を入れ直せます。これを**再代入**（さいだいにゅう）と呼びます。
- `mut` は「この値はこの先変わります」という**読む人へのお知らせ**でもあります。

##### 解説2: 再代入と `+=`

```rust
done_count = done_count + 1;
done_count += 2;
```

- 1行目は「右側を先に計算（0 + 1 = 1）してから、その結果を `done_count` に入れる」という意味です。数学の「等しい」ではなく「入れる」です。
- `+=` は `done_count = done_count + 2` の短縮形です。`-=` `*=` `/=` も同じように使えます。
- 再代入できるのは**同じ型の値だけ**です。数の変数に文字列を入れることはできません（下の「よくあるエラー」で実際に試します）。

#### 例3: シャドーイングで段階的に変換する

> **▼ このコードがやること（先に日本語で）:** 画面から届いたような「前後に空白のある文字列」を、空白を取り除き、さらに数へ変換します。3段階とも `input` という同じ名前を使い、そのたびに新しい変数を作り直しています。

```rust
fn main() {
    // 段階1: 画面から受け取った文字列（前後に空白がある）
    let input = "  42  ";
    println!("[{input}]");

    // 段階2: 同じ名前で「前後の空白を取り除いた版」を作り直す
    let input = input.trim();
    println!("[{input}]");

    // 段階3: さらに同じ名前で「数（u32 型）に変換した版」を作り直す
    let input: u32 = input.parse().expect("数字ではありません");
    // 数になったので足し算ができる
    println!("{}", input + 1);
}
```

**▼ 実行結果**

```text
[  42  ]
[42]
43
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: もう一度 `let` を書くと「新しい変数」になる

```rust
let input = "  42  ";
let input = input.trim();
```

- 2行目の右側の `input` は「1つ目の input」、左側の `input` は「2つ目の input」です。右側を先に計算してから、新しい箱に名札を貼ります。
- `mut` を付けていないのにエラーにならないのは、**書き換えではなく新しく作っている**からです（図1-2）。
- `.trim()`（トリム）は文字列の前後の空白を取り除いた結果を返します。表示の `[` `]` は空白が見えるように付けた飾りです。

##### 解説2: 型を変えられるのがシャドーイングの強み

```rust
let input: u32 = input.parse().expect("数字ではありません");
```

- `: u32` は「この変数の型は u32（0以上の整数）です」という**型注釈**（かたちゅうしゃく）です。
- `.parse()`（パース：解析する）は文字列を別の型に変換します。どの型にするかを `: u32` から読み取ります。
- `.expect("...")` は「変換に失敗したらこのメッセージを出してプログラムを止める」という意味です。失敗の扱い方は UNIT 09 で丁寧に学びます。
- 文字列から数へと**型が変わっている**ことに注目してください。`mut` の再代入ではこれはできません。

#### 例4: ブロックの中だけのシャドーイング

> **▼ このコードがやること（先に日本語で）:** `{ }` で囲んだ範囲の中でだけ同じ名前の変数を作り直し、範囲を出たら元の値が見えることを確かめます。

```rust
fn main() {
    let level = 1;
    {
        // このブロック { } の中だけで有効な、新しい level を作る
        let level = level * 10;
        println!("ブロックの中: {level}");
    }
    // ブロックを出ると、新しい level は消え、元の level が見える
    println!("ブロックの外: {level}");
}
```

**▼ 実行結果**

```text
ブロックの中: 10
ブロックの外: 1
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 変数には「有効範囲」がある

- 変数が使える範囲を**スコープ**（scope：範囲）と呼びます。`let` で作った変数は、それを囲む `{ }` の終わりまで有効です。
- ブロックの中で作った2つ目の `level` は `}` で消えます。すると、隠れていた1つ目の `level`（値は 1）がまた見えるようになります。
- 1つ目の `level` は一度も書き換えられていません。だから `mut` は不要です。

#### 例5: const で決まった値に名前を付ける

> **▼ このコードがやること（先に日本語で）:** タイトルの最大文字数やセッションの日数など、プログラム全体で変わらない値を定数にして使います。定数同士の計算もしてみます。

```rust
// 定数はファイルのどこからでも使える。型の注釈（: usize など）が必須
const MAX_TITLE_CHARS: usize = 80;
// 定数の値は、コンパイルするときに計算できる式なら書ける
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
const SESSION_DAYS: u64 = 30;

fn main() {
    println!("タイトルは最大 {MAX_TITLE_CHARS} 文字");

    let session_seconds = SESSION_DAYS * SECONDS_PER_DAY;
    println!("セッションの有効期限: {session_seconds} 秒");

    // 数字の中の _ は読みやすくするための区切りで、値は変わらない
    let big = 1_000_000;
    println!("百万: {big}");
}
```

**▼ 実行結果**

```text
タイトルは最大 80 文字
セッションの有効期限: 2592000 秒
百万: 1000000
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `const` の決まりごと

```rust
const MAX_TITLE_CHARS: usize = 80;
```

- 名前は**すべて大文字＋アンダースコア**（`MAX_TITLE_CHARS`）で書くのが Rust の習慣です。変数は小文字（`done_count`）なので、見ただけで区別できます。
- `: usize` のような**型注釈が必須**です。`let` と違い、定数では型を省略できません。
- `fn main()` の外（ファイルの一番上）に書けるので、どの関数からでも使えます。
- `mut` は付けられません。値は**コンパイルの時点で決まり**、実行中に変わることはありません。

> **用語:** **usize / u64** … どちらも「0以上の整数」の型です。`usize` は長さや個数、`u64` は大きな数に使います。整数の型は UNIT 02 で詳しく学びます。

> **ヒント:** `const` によく似た `static`（スタティック）もあります。`static` は「プログラムの実行中ずっと同じ場所に1つだけ存在する値」です。初心者のうちは、決まった値には `const` を使えば困りません。

### もう一歩: 実務に近い形

バックエンドでは、「環境変数などから**文字列で**届いた設定値を、空白を取り除き、数に変換し、失敗したら既定値を使う」という処理がよく出てきます。段階的な変換はシャドーイング、変化する状態（試行回数）だけを `mut`、決まった値は `const` にします。

#### 設定値を段階的に整える

> **▼ このコードがやること（先に日本語で）:** 文字列のポート番号を数へ変換します。数字でなかったときは既定値 8080 を使います。最後に、接続を3回まで試すふりをして、試行回数を数えます。

```rust
// 設定値の「既定値」と「上限」は定数にしておく
const DEFAULT_PORT: u16 = 8080;
const MAX_RETRIES: u32 = 3;

fn main() {
    // 環境変数などから届いた「文字列の」ポート番号を想定する
    let port = " 3000 ";
    // 段階1: 空白を取り除く（文字列のまま）
    let port = port.trim();
    // 段階2: 数（u16）へ変換する。変換に失敗したら既定値を使う
    let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
    println!("待ち受けポート: {port}");

    // 数字ではない文字列だった場合も試す
    let broken = "abc";
    let broken: u16 = broken.trim().parse().unwrap_or(DEFAULT_PORT);
    println!("壊れた設定のときのポート: {broken}");

    // 変化する状態（試行回数）だけを mut にする
    let mut attempts = 0;
    while attempts < MAX_RETRIES {
        attempts += 1;
        println!("{attempts} 回目の接続を試みます");
    }
    println!("上限の {MAX_RETRIES} 回に達しました");
}
```

**▼ 実行結果**

```text
待ち受けポート: 3000
壊れた設定のときのポート: 8080
1 回目の接続を試みます
2 回目の接続を試みます
3 回目の接続を試みます
上限の 3 回に達しました
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `unwrap_or(既定値)`

```rust
let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
```

- `.parse()` は成功することも失敗することもあります。`.unwrap_or(DEFAULT_PORT)` は「成功したらその値、失敗したら `DEFAULT_PORT`」を選びます。
- `"abc"` は数にできないので、2つ目の結果は 8080 になりました。プログラムは止まりません。この「成功か失敗か」の仕組み（`Result`）は UNIT 09 で学びます。

##### 解説2: `mut` は狭い範囲に閉じ込める

```rust
let mut attempts = 0;
while attempts < MAX_RETRIES {
    attempts += 1;
}
```

- `while` は「条件が成り立つ間くり返す」構文です（UNIT 04 で詳しく）。
- 実際に変化するのは `attempts` だけなので、`mut` もこれだけに付けます。`port` は一度決まったら変えないので `mut` は付けません。「変わるものが少ない」ほど、プログラムは読みやすく、間違いにくくなります。

### よくあるエラーと読み方

#### エラー1: E0384 不変の変数に2回代入した

```rust
fn main() {
    let count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0384]: cannot assign twice to immutable variable `count`
 --> src\main.rs:4:5
  |
2 |     let count = 1;
  |         ----- first assignment to `count`
3 |     println!("{count}");
4 |     count = 2;
  |     ^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut count = 1;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 1行目の `error[E0384]` がエラーの番号と要約です（「不変の変数 `count` に2回代入することはできません」）。`--> src\main.rs:4:5` は「src\main.rs の4行目5文字目」という場所です。`^^^^` の下線が問題の箇所、`-----` が関係する箇所（最初の代入）を指しています。
- **何が起きたか:** `mut` を付けずに作った `count` に、`count = 2;` で入れ直そうとしました。
- **直し方:** `help:` の提案どおり `let mut count = 1;` にします。または「入れ直す」のではなく「作り直す」意味でよければ `let count = 2;`（シャドーイング）にします。
- `rustc --explain E0384` を実行すると、このエラーの詳しい説明（英語）が読めます。

#### エラー2: E0308 mut の変数に別の型の値を入れた

```rust
fn main() {
    let mut input = " 42 ";
    input = input.trim();
    input = input.parse::<u32>().expect("数字ではありません");
    println!("{input}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:4:13
  |
2 |     let mut input = " 42 ";
  |                     ------ expected due to this value
3 |     input = input.trim();
4 |     input = input.parse::<u32>().expect("数字ではありません");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `u32`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `mismatched types` は「型が合っていません」。`expected &str, found u32` は「&str（文字列）が来るはずなのに u32（数）が来た」です。`expected due to this value` が、型が決まった場所（2行目の `" 42 "`）を指しています。
- **何が起きたか:** `mut` は「中身を入れ替えてよい」だけで、**型は変えられません**。`input` は2行目で文字列型に決まっています。
- **直し方:** 4行目を `let input: u32 = input.parse().expect("数字ではありません");` にしてシャドーイングします。そうすると `mut` も不要になります。

#### エラー3: 定数に型を書き忘れた

```rust
const MAX_TITLE_CHARS = 80;

fn main() {
    println!("{MAX_TITLE_CHARS}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error: missing type for `const` item
 --> src\main.rs:1:22
  |
1 | const MAX_TITLE_CHARS = 80;
  |                      ^ help: provide a type for the constant: `: i32`

error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** `const` には型注釈が必須です。
- **直し方:** `help` は `: i32` を提案していますが、これは「とりあえずの案」です。文字数の上限なら長さを表す `usize` が適切なので、`const MAX_TITLE_CHARS: usize = 80;` と書きます。提案をそのまま貼るのではなく、意味を考えて型を選びましょう。

#### 警告: 使っていない mut

```rust
fn main() {
    let mut name = "RustNote";
    println!("{name}");
}
```

```text
warning: variable does not need to be mutable
 --> src\main.rs:2:9
  |
2 |     let mut name = "RustNote";
  |         ----^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `basics` (bin "basics") generated 1 warning (run `cargo fix --bin "basics" -p basics` to apply 1 suggestion)
```

- `warning`（警告）はエラーではないので、プログラムは実行されます（`RustNote` と表示されます）。
- 「この変数は変更されないので `mut` は不要です」という意味です。不要な `mut` は読む人を「どこで変わるの？」と迷わせるので、`help: remove this mut` に従って消しましょう。

> **つまずきポイント:** 「とりあえず全部 `mut` にしておけばエラーが出ない」と考えがちですが、逆効果です。まず `mut` なしで書き、E0384 が出たところで「本当に変わる値か？」を考えてから付けるのが Rust 流です。

### RustNote ではここで使う

この教材で作るメモアプリ **RustNote** では、ノートのタイトルを「1〜80文字」に制限します。上限 80 は**プログラム全体で共有する決まった値**なので `const` にします。また、フォームから届いたタイトルは前後に空白が付いていることがあるので、シャドーイングで「整えた版」を作り直します。

> **▼ このコードがやること（先に日本語で）:** 空白付きのタイトルを整え、その文字数を上限と並べて表示します。

```rust
const MAX_TITLE_CHARS: usize = 80;

fn main() {
    // フォームから届いたタイトル（前後に空白がついている）
    let title = "   牛乳を買う   ";
    // 前後の空白を取り除いた版を、同じ名前で作り直す（シャドーイング）
    let title = title.trim();
    // 文字数を数える（日本語も 1 文字を 1 と数える方法。UNIT 05 で詳しく）
    let chars = title.chars().count();
    println!("タイトル「{title}」は {chars} 文字です（上限 {MAX_TITLE_CHARS} 文字）");
}
```

**▼ 実行結果**

```text
タイトル「牛乳を買う」は 5 文字です（上限 80 文字）
```

この「整える → 数える → 上限と比べる」の流れは、UNIT 05 で文字数の数え方を、UNIT 09 で「範囲外ならエラーを返す」書き方を足して、`NoteTitle::parse` という検証関数に育てていきます。

### 練習問題

**問題1.** 次のコードは E0384 になります。（a）`mut` を使う方法、（b）シャドーイングを使う方法の2通りで直してください。

```rust
fn main() {
    let count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

**問題2.** `let price = "1200";` から始めて、シャドーイングを使って数（`u32`）に変換し、10% の税を足した金額 `税込み: 1320 円` を表示してください。

**問題3.** 次のコードを実行すると何が表示されるでしょうか。実行する前に予想してください。

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("内側: {x}");
    }
    println!("外側: {x}");
}
```

### 解答と解説

**解答1.**

```rust
// (a) mut を使う方法
fn main() {
    let mut count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

```rust
// (b) シャドーイングを使う方法
fn main() {
    let count = 1;
    println!("{count}");
    let count = 2;
    println!("{count}");
}
```

**▼ 実行結果**（どちらも同じ）

```text
1
2
```

- (a) は「同じ箱の中身を入れ替える」、(b) は「新しい箱を作って名札を移す」です。結果は同じですが、「数を数えていく」なら (a)、「別の意味の値に作り直す」なら (b) が自然です。

**解答2.**

```rust
fn main() {
    let price = "1200";
    // 文字列を数に変換し、同じ名前で作り直す
    let price: u32 = price.parse().expect("数字ではありません");
    // 10% の税を足す（整数のまま計算するため、先に 110 を掛けてから 100 で割る）
    let price = price * 110 / 100;
    println!("税込み: {price} 円");
}
```

**▼ 実行結果**

```text
税込み: 1320 円
```

- 整数の割り算は小数点以下が切り捨てられるので、`price / 100 * 110` の順だと端数が失われることがあります。**先に掛けてから割る**のが整数計算のコツです（UNIT 02）。

**解答3.**

```text
内側: 12
外側: 6
```

- 2行目で `x` は 6 の新しい箱になります。ブロックの中では `x * 2` = 12 の箱が作られますが、`}` で消えるので、外側では 6 が表示されます。

### まとめ

- `let` は値に名前を付ける。既定では**変更できない**ので、変わる値にだけ `mut` を付ける。
- シャドーイングは同じ名前で**新しい変数を作り直す**こと。型も変えられるので、段階的な変換に向く。
- プログラム全体で変わらない値は、型注釈付きの `const`（大文字の名前）にする。
